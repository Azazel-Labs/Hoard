#include "HoardBrowserTool.h"

#include "Core/Engine/Engine.h"
#include "HoardImport.h"
#include "Log/Log.h"
#include "Tools/UI/ImGui/EditorColors.h"
#include "Tools/UI/ImGui/ImGuiX.h"

#include <algorithm>
#include <cstdio>
#include <cstring>

namespace Lumina
{
    namespace HoardBrowserInternal
    {
        const char* const KindValues[] = { "", "texture", "vector", "model", "material", "audio", "animation", "font", "shader", "data" };
        const char* const KindNames[] = { "Anything", "Textures", "Vector art", "Models", "Materials", "Sounds", "Animations", "Fonts", "Shaders", "Data" };
        constexpr int Page = 40;
        constexpr float TileSize = 96.0f;
        constexpr auto Debounce = std::chrono::milliseconds(350);

        void Copy(char* To, size_t Capacity, std::string_view From)
        {
            const size_t Len = std::min(From.size(), Capacity - 1);
            std::memcpy(To, From.data(), Len);
            To[Len] = '\0';
        }

        std::string Size(int64_t Bytes)
        {
            const char* Units[] = { "B", "KB", "MB", "GB" };
            double Value = static_cast<double>(Bytes);
            int Unit = 0;
            while (Value >= 1024.0 && Unit < 3)
            {
                Value /= 1024.0;
                ++Unit;
            }
            char Out[32];
            std::snprintf(Out, sizeof(Out), Unit == 0 ? "%.0f %s" : "%.1f %s", Value, Units[Unit]);
            return Out;
        }

        uint32_t Big16(const std::string& B, size_t At)
        {
            return (static_cast<uint32_t>(static_cast<unsigned char>(B[At])) << 8) | static_cast<unsigned char>(B[At + 1]);
        }

        /** A PNG's or JPEG's size from its header, so a thumbnail keeps its shape. Empty extension when it's neither. */
        const char* ImageInfo(const std::string& B, uint32_t& Width, uint32_t& Height)
        {
            if (B.size() > 24 && std::memcmp(B.data(), "\x89PNG\r\n\x1a\n", 8) == 0)
            {
                Width = (Big16(B, 16) << 16) | Big16(B, 18);
                Height = (Big16(B, 20) << 16) | Big16(B, 22);
                return ".png";
            }
            if (B.size() > 4 && static_cast<unsigned char>(B[0]) == 0xFF && static_cast<unsigned char>(B[1]) == 0xD8)
            {
                size_t At = 2;
                while (At + 9 < B.size() && static_cast<unsigned char>(B[At]) == 0xFF)
                {
                    const unsigned char Marker = static_cast<unsigned char>(B[At + 1]);
                    // A start-of-frame marker holds the size; DHT (C4), JPG (C8) and DAC (CC) share the range but don't.
                    if (Marker >= 0xC0 && Marker <= 0xCF && Marker != 0xC4 && Marker != 0xC8 && Marker != 0xCC)
                    {
                        Height = Big16(B, At + 5);
                        Width = Big16(B, At + 7);
                        return ".jpg";
                    }
                    At += 2 + Big16(B, At + 2);
                }
                Width = Height = 0;
                return ".jpg";
            }
            return "";
        }

        /** Text cut to fit Width, with an ellipsis, never splitting a character. */
        std::string Fit(const std::string& Text, float Width)
        {
            if (ImGui::CalcTextSize(Text.c_str()).x <= Width)
            {
                return Text;
            }
            std::string Cut = Text;
            while (!Cut.empty())
            {
                Cut.pop_back();
                while (!Cut.empty() && (static_cast<unsigned char>(Cut.back()) & 0xC0) == 0x80)
                {
                    Cut.pop_back();
                }
                if (!Cut.empty() && (static_cast<unsigned char>(Cut.back()) & 0x80) != 0)
                {
                    Cut.pop_back();
                }
                const std::string Shown = Cut + "...";
                if (ImGui::CalcTextSize(Shown.c_str()).x <= Width)
                {
                    return Shown;
                }
            }
            return "...";
        }
    }

    using namespace HoardBrowserInternal;

    void FHoardWorker::Start(const char* Name)
    {
        Thread = FThread(FThreadOptions{ .Name = Name, .Group = Threading::ThreadGroup_Worker, .Priority = EThreadPriority::Background }, [this]()
        {
            for (;;)
            {
                std::function<void()> Job;
                {
                    std::unique_lock Lock(Mutex);
                    Wake.wait(Lock, [this] { return bStopping || !Jobs.empty(); });
                    if (bStopping)
                    {
                        return;
                    }
                    Job = std::move(Jobs.front());
                    Jobs.pop_front();
                }
                Job();
            }
        });
    }

    void FHoardWorker::Post(std::function<void()> Job)
    {
        {
            std::scoped_lock Lock(Mutex);
            Jobs.push_back(std::move(Job));
        }
        Wake.notify_one();
    }

    void FHoardWorker::Stop()
    {
        {
            std::scoped_lock Lock(Mutex);
            bStopping = true;
            Jobs.clear();
        }
        Wake.notify_all();
        if (Thread.Joinable())
        {
            Thread.Join();
        }
    }

    void FHoardBrowserTool::OnInitialize()
    {
        SearchWorker.Start("Hoard search");
        ThumbWorker.Start("Hoard thumbnails");
        AddWorker.Start("Hoard add");

        const Hoard::Settings Saved = Hoard::Settings::Load();
        Copy(ServerField, sizeof(ServerField), Saved.Server);
        Copy(TokenField, sizeof(TokenField), Saved.Token);

        CreateToolWindow("Hoard", [this](bool)
        {
            DrawWindow();
        });

        TomlServer = MakeProject().Server;
        if (TomlServer.empty() && Hoard::Trim(ServerField).empty())
        {
            bSettingsOpen = true;
            Say("Set the server's address (and a token, if it wants one) to search.");
        }
        else
        {
            Search(false);
        }
    }

    void FHoardBrowserTool::OnDeinitialize(const FUpdateContext&)
    {
        bCancel = true;
        SearchWorker.Stop();
        ThumbWorker.Stop();
        AddWorker.Stop();
    }

    void FHoardBrowserTool::Update(const FUpdateContext& UpdateContext)
    {
        std::vector<std::function<void()>> Ready;
        {
            std::scoped_lock Lock(DoneMutex);
            Ready.swap(Done);
        }
        for (std::function<void()>& Then : Ready)
        {
            Then();
        }
        if (bSearchPending && FClock::now() >= SearchAt)
        {
            Search(false);
        }
        FEditorTool::Update(UpdateContext);
    }

    void FHoardBrowserTool::Say(std::string Text, bool bError)
    {
        Status = std::move(Text);
        bStatusError = bError;
    }

    void FHoardBrowserTool::Finish(std::function<void()> Then)
    {
        std::scoped_lock Lock(DoneMutex);
        Done.push_back(std::move(Then));
    }

    Hoard::Project FHoardBrowserTool::MakeProject() const
    {
        const FStringView Root = Global::GEngine != nullptr ? Global::GEngine->GetProjectPath() : FStringView();
        return Hoard::Project(std::string_view(Root.GetData(), static_cast<size_t>(Root.Len())), "Hoard");
    }

    Hoard::Client FHoardBrowserTool::MakeClient(const Hoard::Project& Project) const
    {
        Hoard::Client With;
        const std::string Typed = Hoard::Trim(ServerField);
        With.Server = Typed.empty() ? Project.Server : Typed;
        With.Token = Hoard::Trim(TokenField);
        With.Cancel = &bCancel;
        return With;
    }

    void FHoardBrowserTool::SearchSoon()
    {
        bSearchPending = true;
        SearchAt = FClock::now() + Debounce;
    }

    void FHoardBrowserTool::Search(bool bMore)
    {
        bSearchPending = false;
        const Hoard::Client With = MakeClient(MakeProject());
        if (!bMore)
        {
            ++Generation;
            Items.clear();
            Total = 0;
            Anchor = 0;
        }
        const int Gen = Generation;
        const size_t Offset = Items.size();
        bSearching = true;
        Say("Searching...");
        SearchWorker.Post([this, With, Gen, Offset, Q = std::string(Query), K = std::string(KindValues[Kind])]()
        {
            if (Gen != Generation)
            {
                return;
            }
            Hoard::Reply R = With.Search(Q, K, static_cast<int>(Offset), Page);
            Finish([this, Gen, R = std::move(R)]()
            {
                if (Gen != Generation)
                {
                    return;
                }
                bSearching = false;
                if (!R.bOk)
                {
                    Say(R.Error, true);
                    return;
                }
                Total = R.Data.Long("total");
                const size_t From = Items.size();
                for (const Hoard::Json& Asset : R.Data.List("items"))
                {
                    if (Asset.IsObject())
                    {
                        Items.push_back(FItem{ Asset });
                    }
                }
                Say(Total == 0 ? std::string("Nothing found.")
                    : std::to_string(Items.size()) + " of " + std::to_string(Total) + ". Double-click, or pick some and Add to project.");
                FetchThumbs(From, Gen);
            });
        });
    }

    void FHoardBrowserTool::FetchThumbs(size_t From, int Gen)
    {
        const Hoard::Client With = MakeClient(MakeProject());
        const std::filesystem::path Cache = std::filesystem::temp_directory_path() / "hoard-thumbs";
        for (size_t I = From; I < Items.size(); ++I)
        {
            const Hoard::Json& Asset = Items[I].Asset;
            const Hoard::Json* Has = Asset.Find("has_thumb");
            if (Has == nullptr || !Has->Bool)
            {
                continue;
            }
            ThumbWorker.Post([this, With, Cache, Gen, I, Asset]()
            {
                if (Gen != Generation)
                {
                    return;
                }
                const std::string Bytes = With.Thumbnail(Asset);
                uint32_t Width = 0;
                uint32_t Height = 0;
                const std::string Ext = ImageInfo(Bytes, Width, Height);
                if (Ext.empty())
                {
                    return;
                }
                // Named for the thumbnail's revision, since the editor keeps a texture per path for as long as it runs.
                const Hoard::Json* Rev = Asset.Find("thumb_rev");
                const std::string Name = Hoard::Project::Sanitize(Asset.Text("id") + "-" + (Rev != nullptr ? Rev->Write() : std::string("0"))) + Ext;
                const std::filesystem::path File = Cache / Hoard::Fs(Name);
                std::error_code Ec;
                if (!std::filesystem::exists(File, Ec) && !Hoard::WriteFile(File, Bytes))
                {
                    return;
                }
                const float Aspect = Width > 0 && Height > 0 ? static_cast<float>(Width) / static_cast<float>(Height) : 1.0f;
                Finish([this, Gen, I, Aspect, Path = Hoard::Utf8(File)]()
                {
                    if (Gen == Generation && I < Items.size())
                    {
                        Items[I].Thumb = Path;
                        Items[I].Aspect = Aspect;
                    }
                });
            });
        }
    }

    void FHoardBrowserTool::AddSelected()
    {
        std::vector<Hoard::Json> Assets;
        for (const FItem& Item : Items)
        {
            if (Item.bSelected)
            {
                Assets.push_back(Item.Asset);
            }
        }
        Add(std::move(Assets));
    }

    void FHoardBrowserTool::AddOne(size_t Index)
    {
        Add({ Items[Index].Asset });
    }

    void FHoardBrowserTool::Add(std::vector<Hoard::Json> Assets)
    {
        if (Assets.empty() || bAdding)
        {
            return;
        }
        if (Global::GEngine == nullptr || !Global::GEngine->HasLoadedProject())
        {
            Say("Open a project to add to it.");
            return;
        }
        const Hoard::Project Project = MakeProject();
        const Hoard::Client With = MakeClient(Project);
        bAdding = true;
        Say("Working out what they need...");
        AddWorker.Post([this, Project, With, Assets = std::move(Assets)]()
        {
            Hoard::AddResult R = Project.Add(With, Assets, [this](int Got, int Of)
            {
                Finish([this, Got, Of]() { Say("Downloaded " + std::to_string(Got) + " of " + std::to_string(Of) + "..."); });
            });
            Finish([this, Project, R = std::move(R)]()
            {
                bAdding = false;
                if (!R.bOk)
                {
                    Say(R.Error, true);
                    return;
                }
                const HoardImport::FResult Made = HoardImport::Import(Project, R.Downloaded);
                Say("Added " + std::to_string(R.Files) + (R.Files == 1 ? " file" : " files") + " to " + R.OutDir + " ("
                    + std::to_string(R.Downloaded.size()) + " downloaded), imported " + std::to_string(Made.Imported.size()) + " into "
                    + HoardImport::ContentPath + ".");
                if (!Made.Failed.empty())
                {
                    Status += " " + std::to_string(Made.Failed.size()) + " didn't import: " + Made.Failed.front()
                        + (Made.Failed.size() > 1 ? " (and more in the log)." : ".");
                    for (const std::string& Why : Made.Failed)
                    {
                        LOG_WARN("Hoard: {}", Why.c_str());
                    }
                }
                if (!R.bReported)
                {
                    Status += " The server didn't hear what the project uses; the next hoard sync will tell it.";
                }
            });
        });
    }

    void FHoardBrowserTool::DrawWindow()
    {
        const float Spacing = ImGui::GetStyle().ItemSpacing.x;
        const float Cog = ImGui::GetFrameHeight();
        const float KindWidth = 130.0f * ImGuiX::GetUIScale();
        ImGui::SetNextItemWidth(std::max(80.0f, ImGui::GetContentRegionAvail().x - KindWidth - Cog - Spacing * 2));
        if (ImGui::InputTextWithHint("##Query", LE_ICON_MAGNIFY " Search Hoard", Query, sizeof(Query)))
        {
            SearchSoon();
        }
        ImGui::SameLine();
        ImGui::SetNextItemWidth(KindWidth);
        if (ImGui::Combo("##Kind", &Kind, KindNames, IM_ARRAYSIZE(KindNames)))
        {
            Search(false);
        }
        ImGui::SameLine();
        if (ImGui::Button(LE_ICON_COG "##Settings", ImVec2(Cog, Cog)))
        {
            bSettingsOpen = !bSettingsOpen;
            TomlServer = MakeProject().Server;
        }
        ImGui::SetItemTooltip("Server and token");

        if (bSettingsOpen)
        {
            DrawSettings();
        }

        const float Footer = ImGui::GetFrameHeightWithSpacing() + ImGui::GetStyle().ItemSpacing.y;
        if (ImGui::BeginChild("##Tiles", ImVec2(0, -Footer)))
        {
            DrawTiles();
        }
        ImGui::EndChild();
        DrawFooter();
    }

    void FHoardBrowserTool::DrawSettings()
    {
        if (ImGui::BeginTable("##Settings", 2, ImGuiTableFlags_SizingStretchProp))
        {
            ImGui::TableSetupColumn("Label", ImGuiTableColumnFlags_WidthFixed);
            ImGui::TableSetupColumn("Field", ImGuiTableColumnFlags_WidthStretch);
            ImGui::TableNextRow();
            ImGui::TableNextColumn();
            ImGui::AlignTextToFramePadding();
            ImGui::TextUnformatted("Server");
            ImGui::TableNextColumn();
            ImGui::SetNextItemWidth(-FLT_MIN);
            ImGui::InputTextWithHint("##Server", TomlServer.empty() ? "http://hoard.local:4627" : TomlServer.c_str(), ServerField, sizeof(ServerField));
            ImGui::TableNextRow();
            ImGui::TableNextColumn();
            ImGui::AlignTextToFramePadding();
            ImGui::TextUnformatted("Token");
            ImGui::TableNextColumn();
            ImGui::SetNextItemWidth(-FLT_MIN);
            ImGui::InputTextWithHint("##Token", "One that can look is enough to add", TokenField, sizeof(TokenField), ImGuiInputTextFlags_Password);
            ImGui::EndTable();
        }
        if (ImGui::Button("Save and search"))
        {
            Hoard::Settings Saved;
            Saved.Server = Hoard::Trim(ServerField);
            Saved.Token = Hoard::Trim(TokenField);
            if (!Saved.Save())
            {
                Say("Couldn't save the settings to " + Hoard::Utf8(Hoard::Settings::File()) + ".", true);
                return;
            }
            bSettingsOpen = false;
            Search(false);
        }
        ImGui::Separator();
    }

    void FHoardBrowserTool::DrawTiles()
    {
        const float Scale = ImGuiX::GetUIScale();
        const float Tile = TileSize * Scale;
        const float Pad = 4.0f * Scale;
        const float TextHeight = ImGui::GetTextLineHeight();
        const ImVec2 CellSize(Tile + Pad * 2, Tile + TextHeight + Pad * 3);
        const float Gap = ImGui::GetStyle().ItemSpacing.x;
        const int Columns = std::max(1, static_cast<int>((ImGui::GetContentRegionAvail().x + Gap) / (CellSize.x + Gap)));
        ImDrawList* Draw = ImGui::GetWindowDrawList();
        const ImU32 Dim = ImGui::GetColorU32(EditorColors::TextDim());
        const ImU32 Text = ImGui::GetColorU32(ImGuiCol_Text);
        std::optional<size_t> Open;

        for (size_t I = 0; I < Items.size(); ++I)
        {
            FItem& Item = Items[I];
            const Hoard::Json& A = Item.Asset;
            if (I % Columns != 0)
            {
                ImGui::SameLine();
            }
            ImGui::PushID(static_cast<int>(I));
            const ImVec2 Min = ImGui::GetCursorScreenPos();
            if (ImGui::Selectable("##Tile", Item.bSelected, ImGuiSelectableFlags_AllowDoubleClick, CellSize))
            {
                const ImGuiIO& Io = ImGui::GetIO();
                if (ImGui::IsMouseDoubleClicked(ImGuiMouseButton_Left))
                {
                    Open = I;
                }
                else if (Io.KeyShift)
                {
                    const size_t Lo = std::min(Anchor, I);
                    const size_t Hi = std::max(Anchor, I);
                    for (size_t J = 0; J < Items.size(); ++J)
                    {
                        Items[J].bSelected = (J >= Lo && J <= Hi) || (Io.KeyCtrl && Items[J].bSelected);
                    }
                }
                else if (Io.KeyCtrl)
                {
                    Item.bSelected = !Item.bSelected;
                    Anchor = I;
                }
                else
                {
                    for (FItem& Other : Items)
                    {
                        Other.bSelected = false;
                    }
                    Item.bSelected = true;
                    Anchor = I;
                }
            }
            const std::string Category = A.Text("category");
            const std::string AssetKind = A.Text("kind");
            if (ImGui::IsItemHovered(ImGuiHoveredFlags_DelayShort))
            {
                ImGui::SetTooltip("%s\n%s \xC2\xB7 %s \xC2\xB7 %s", A.Text("name").c_str(), A.Text("pack_name").c_str(),
                    (Category.empty() ? AssetKind : Category).c_str(), Size(A.Long("size")).c_str());
            }

            // The thumbnail fits the square and keeps its shape.
            const ImVec2 Box(Min.x + Pad, Min.y + Pad);
            if (!Item.Thumb.empty())
            {
                const float W = Item.Aspect >= 1.0f ? Tile : Tile * Item.Aspect;
                const float H = Item.Aspect >= 1.0f ? Tile / Item.Aspect : Tile;
                const ImVec2 At(Box.x + (Tile - W) * 0.5f, Box.y + (Tile - H) * 0.5f);
                Draw->AddImage(ImGuiX::ToImTextureRef(FStringView(Item.Thumb.c_str())), At, ImVec2(At.x + W, At.y + H));
            }
            else
            {
                const ImVec2 Label = ImGui::CalcTextSize(AssetKind.c_str());
                Draw->AddText(ImVec2(Box.x + (Tile - Label.x) * 0.5f, Box.y + (Tile - Label.y) * 0.5f), Dim, AssetKind.c_str());
            }
            const std::string Name = Fit(A.Text("name"), Tile);
            const float NameWidth = ImGui::CalcTextSize(Name.c_str()).x;
            Draw->AddText(ImVec2(Box.x + (Tile - NameWidth) * 0.5f, Box.y + Tile + Pad), Text, Name.c_str());
            ImGui::PopID();
        }

        if (Open)
        {
            AddOne(*Open);
        }
    }

    void FHoardBrowserTool::DrawFooter()
    {
        size_t Picked = 0;
        for (const FItem& Item : Items)
        {
            Picked += Item.bSelected ? 1 : 0;
        }
        const std::string AddLabel = Picked > 1 ? "Add " + std::to_string(Picked) + " to project" : std::string("Add to project");
        const bool bMore = !bSearching && static_cast<int64_t>(Items.size()) < Total;
        const ImGuiStyle& Style = ImGui::GetStyle();
        const float AddWidth = ImGui::CalcTextSize(AddLabel.c_str()).x + Style.FramePadding.x * 2;
        const float MoreWidth = bMore ? ImGui::CalcTextSize("More").x + Style.FramePadding.x * 2 + Style.ItemSpacing.x : 0.0f;
        const float StatusWidth = ImGui::GetContentRegionAvail().x - AddWidth - MoreWidth - Style.ItemSpacing.x;

        const float LineX = ImGui::GetCursorPosX();
        const float Avail = ImGui::GetContentRegionAvail().x;
        ImGui::AlignTextToFramePadding();
        const ImVec2 Start = ImGui::GetCursorScreenPos();
        ImGui::PushClipRect(Start, ImVec2(Start.x + std::max(0.0f, StatusWidth), Start.y + ImGui::GetFrameHeight()), true);
        ImGui::TextColored(bStatusError ? EditorColors::Danger() : EditorColors::TextDim(), "%s", Status.c_str());
        ImGui::PopClipRect();
        if (ImGui::IsItemHovered() && ImGui::CalcTextSize(Status.c_str()).x > StatusWidth)
        {
            ImGui::SetTooltip("%s", Status.c_str());
        }

        ImGui::SameLine(LineX + Avail - AddWidth - MoreWidth);
        if (bMore)
        {
            if (ImGui::Button("More"))
            {
                Search(true);
            }
            ImGui::SameLine();
        }
        ImGui::BeginDisabled(Picked == 0 || bAdding);
        if (ImGui::Button(AddLabel.c_str()))
        {
            AddSelected();
        }
        ImGui::EndDisabled();
    }
}
