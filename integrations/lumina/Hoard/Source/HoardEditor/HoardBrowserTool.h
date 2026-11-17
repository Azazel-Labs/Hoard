#pragma once

#include "Core/Threading/Thread.h"
#include "HoardCore.h"
#include "UI/Tools/EditorTool.h"

#include <chrono>
#include <condition_variable>
#include <deque>
#include <mutex>

namespace Lumina
{
    /** Runs jobs one after another on a thread of its own, so the editor never waits on the server. */
    class FHoardWorker
    {
    public:

        void Start(const char* Name);
        void Post(std::function<void()> Job);
        /** Drop what's waiting and wait for what's running. */
        void Stop();

    private:

        std::mutex Mutex;
        std::condition_variable Wake;
        std::deque<std::function<void()>> Jobs;
        bool bStopping = false;
        FThread Thread;
    };

    /** The Hoard drawer: search a Hoard server, and add what's picked to the project with what it needs. */
    class HOARDEDITOR_API FHoardBrowserTool : public FEditorTool
    {
    public:

        LUMINA_SINGLETON_EDITOR_TOOL(FHoardBrowserTool)

        explicit FHoardBrowserTool(IEditorToolContext* Context)
            : FEditorTool(Context, "Hoard", nullptr)
        {}

        bool IsSingleWindowTool() const override { return true; }
        const char* GetTitlebarIcon() const override { return LE_ICON_TREASURE_CHEST; }

        void OnInitialize() override;
        void OnDeinitialize(const FUpdateContext& UpdateContext) override;
        void Update(const FUpdateContext& UpdateContext) override;

    private:

        struct FItem
        {
            Hoard::Json Asset;
            /** The thumbnail on disk once it's here, as PNG or JPEG. */
            std::string Thumb;
            float Aspect = 1.0f;
            bool bSelected = false;
        };

        using FClock = std::chrono::steady_clock;

        void DrawWindow();
        void DrawSettings();
        void DrawTiles();
        void DrawFooter();

        Hoard::Project MakeProject() const;
        Hoard::Client MakeClient(const Hoard::Project& Project) const;
        void SearchSoon();
        void Search(bool bMore);
        void FetchThumbs(size_t From, int Gen);
        void AddSelected();
        void AddOne(size_t Index);
        void Add(std::vector<Hoard::Json> Assets);

        void Say(std::string Text, bool bError = false);

        /** Hand something to the main thread, from a worker. */
        void Finish(std::function<void()> Then);

        FHoardWorker SearchWorker;
        FHoardWorker ThumbWorker;
        FHoardWorker AddWorker;
        std::atomic<bool> bCancel = false;
        std::atomic<int> Generation = 0;

        std::mutex DoneMutex;
        std::vector<std::function<void()>> Done;

        std::vector<FItem> Items;
        int64_t Total = 0;
        size_t Anchor = 0;
        bool bSearching = false;
        bool bAdding = false;
        std::string Status;
        bool bStatusError = false;
        /** The server hoard.toml names, shown as the field's hint. */
        std::string TomlServer;

        char Query[256] = {};
        int Kind = 0;
        bool bSettingsOpen = false;
        char ServerField[512] = {};
        char TokenField[512] = {};
        bool bSearchPending = false;
        FClock::time_point SearchAt;
    };
}
