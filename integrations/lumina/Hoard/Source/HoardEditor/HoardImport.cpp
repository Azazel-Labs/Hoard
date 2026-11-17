#include "HoardImport.h"

#include "Asset/AssetOps.h"
#include "Assets/AssetEvents.h"
#include "Assets/AssetRegistry/AssetRegistry.h"
#include "Core/Object/ObjectCore.h"
#include "Core/Object/Package/Package.h"
#include "FileSystem/FileSystem.h"
#include "Paths/Paths.h"
#include "Tools/Import/Importer.h"

#include <set>

namespace Lumina::HoardImport
{
    namespace
    {
        FString ToFString(const std::string& Text)
        {
            return FString(Text.data(), static_cast<int32>(Text.size()));
        }

        std::string ToStd(FStringView Text)
        {
            return std::string(Text.GetData(), static_cast<size_t>(Text.Len()));
        }

        // Each missing level is made in turn, since CreateFolder only makes one.
        bool MakeFolders(const std::string& Path, std::string& OutError)
        {
            std::vector<std::string> Missing;
            FStringView Walk(Path);
            while (!Walk.IsEmpty() && !VFS::Exists(Walk))
            {
                Missing.push_back(ToStd(Walk));
                const FStringView Up = VFS::Parent(Walk, true);
                // Parent hands back the same view at the top.
                if (Up.Len() >= Walk.Len())
                {
                    break;
                }
                Walk = Up;
            }
            for (auto It = Missing.rbegin(); It != Missing.rend(); ++It)
            {
                const AssetOps::FPathOpResult Made = AssetOps::CreateFolder(FStringView(*It));
                if (!Made.bSucceeded)
                {
                    OutError = ToStd(Made.Error);
                    return false;
                }
            }
            return true;
        }

        // Updating in place keeps the asset's GUID, so whatever uses it still does.
        bool Reimport(CImporter* Importer, const FFixedString& Destination, const FString& Source, FResult& Out,
            const std::string& Label)
        {
            const FAssetDataPtr Data = FAssetRegistry::Get().GetAssetByPath(FStringView(*Destination, Destination.Len()));
            if (Data == nullptr)
            {
                return false;
            }
            CObject* Asset = LoadObject<CObject>(Data->AssetGUID);
            if (Asset == nullptr || !Importer->CanReimport(Asset->GetClass()))
            {
                return false;
            }
            const FImportRequest Request{ FFixedString(*Source), FFixedString() };
            FString ParseError;
            const bool bDone = Importer->ParseSource(Request, ParseError, nullptr) && Importer->ReimportAsset(Asset, Request, nullptr);
            Importer->ReleaseSourceData();
            if (!bDone)
            {
                Out.Failed.push_back(Label + ": couldn't reimport it" + (ParseError.IsEmpty() ? std::string() : " (" + ToStd(ParseError) + ")"));
                return true;
            }
            CPackage* Package = Asset->GetPackage();
            if (Package == nullptr || !CPackage::SavePackage(Package, Package->GetPackagePath()))
            {
                Out.Failed.push_back(Label + ": reimported but couldn't be saved");
                return true;
            }
            FAssetRegistry::Get().AssetSaved(Asset);
            AssetEvents::BroadcastAssetDataChanged(Asset);
            Out.Imported.push_back(ToStd(FStringView(*Destination, Destination.Len())));
            return true;
        }

        void ImportFile(const std::string& File, const std::string& Folder, FResult& Out, const std::string& Label)
        {
            FString Source = ToFString(File);
            Paths::Normalize(Source);
            CImporter* ImporterCDO = CImporterRegistry::Get().FindImporterForExtension(VFS::Extension(FStringView(Source)));
            if (ImporterCDO == nullptr)
            {
                return;
            }
            // Named as CPackage names a package, so the same file finds its asset again next time.
            const FFixedString Destination = SanitizeObjectName(Paths::Combine(FStringView(Folder), VFS::FileName(FStringView(Source), true)));
            CImporter* Importer = CImporterRegistry::CreateImporterOfClass(ImporterCDO->GetClass());
            if (Importer == nullptr)
            {
                return;
            }
            if (Reimport(Importer, Destination, Source, Out, Label))
            {
                CImporterRegistry::DestroyImporter(Importer);
                return;
            }
            const FImportRequest Request{ FFixedString(*Source), Destination, true };
            FImportResult Result;
            FString ParseError;
            if (Importer->ParseSource(Request, ParseError, nullptr))
            {
                Importer->BuildAssets(Request, Result, nullptr);
            }
            else
            {
                Result.Error = ParseError.IsEmpty() ? FString("couldn't read it") : ParseError;
            }
            Importer->ReleaseSourceData();
            // A file holding several meshes makes one asset each, named after the file and the mesh.
            std::vector<std::string> Made;
            for (const TStrongObjectPtr<CObject>& Created : Result.CreatedObjects)
            {
                if (Created.IsValid() && Created->GetPackage() != nullptr)
                {
                    Made.push_back(ToStd(Created->GetPackage()->GetPackagePath()));
                }
            }
            Result.Finish(Request);
            CImporterRegistry::DestroyImporter(Importer);
            if (!Result.Succeeded())
            {
                Out.Failed.push_back(Label + ": " + ToStd(Result.Error));
                return;
            }
            if (Made.empty())
            {
                Made.push_back(ToStd(FStringView(*Destination, Destination.Len())));
            }
            Out.Imported.insert(Out.Imported.end(), Made.begin(), Made.end());
        }
    }

    std::string ContentPathFor(const std::string& Rel)
    {
        // Rel's folders are already safe names on disk, which is all a content folder asks.
        std::string Out = ContentPath;
        const size_t Slash = Rel.rfind('/');
        if (Slash != std::string::npos && Slash > 0)
        {
            Out += "/" + Rel.substr(0, Slash);
        }
        return Out;
    }

    FResult Import(const Hoard::Project& Project, const std::vector<Hoard::Placed>& Downloaded)
    {
        // Kinds that import on their own without questions.
        static const std::set<std::string> Kinds = { "model", "texture", "audio", "font" };
        FResult Out;
        const std::filesystem::path Root = Project.OutPath();
        for (const Hoard::Placed& P : Downloaded)
        {
            const std::string Bundle = P.Entry.Text("bundle");
            // What a model needs (its materials and textures) comes in with it.
            if (!Bundle.empty() && Bundle != P.Entry.Text("id"))
            {
                continue;
            }
            if (!Kinds.contains(P.Entry.Text("kind")))
            {
                continue;
            }
            const std::string Folder = ContentPathFor(P.Rel);
            std::string Error;
            if (!MakeFolders(Folder, Error))
            {
                Out.Failed.push_back(P.Rel + ": " + Error);
                continue;
            }
            ImportFile(Hoard::Utf8(Root / Hoard::Fs(P.Rel)), Folder, Out, P.Rel);
        }
        return Out;
    }
}
