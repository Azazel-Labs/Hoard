#include "HoardImport.h"

#include "AssetImportTask.h"
#include "AssetToolsModule.h"
#include "Factories/Factory.h"
#include "HoardJson.h"
#include "HoardProject.h"
#include "IImageWrapper.h"
#include "IImageWrapperModule.h"
#include "Misc/Paths.h"
#include "Modules/ModuleManager.h"
#include "ObjectTools.h"
#include "UObject/StrongObjectPtr.h"
#include "UObject/UObjectIterator.h"

namespace HoardImport
{
	FString ContentPathFor(const FString& Rel)
	{
		TArray<FString> Parts;
		FPaths::GetPath(Rel).ParseIntoArray(Parts, TEXT("/"), true);
		FString Out = HoardDefaults::ContentPath;
		for (const FString& P : Parts)
		{
			Out += TEXT("/") + ObjectTools::SanitizeInvalidChars(P, INVALID_LONGPACKAGE_CHARACTERS);
		}
		return Out;
	}

	/** The file extensions some factory in this editor imports. */
	static const TSet<FString>& Importable()
	{
		static const TSet<FString> Extensions = []
		{
			TSet<FString> Out;
			for (TObjectIterator<UClass> It; It; ++It)
			{
				if (!It->IsChildOf(UFactory::StaticClass()) || It->HasAnyClassFlags(CLASS_Abstract | CLASS_Deprecated))
				{
					continue;
				}
				const UFactory* Factory = It->GetDefaultObject<UFactory>();
				if (Factory && Factory->bEditorImport)
				{
					TArray<FString> Found;
					Factory->GetSupportedFileExtensions(Found);
					for (const FString& E : Found)
					{
						Out.Add(E.ToLower());
					}
				}
			}
			return Out;
		}();
		return Extensions;
	}

	TArray<FString> Import(const FHoardProject& Project, const TArray<FHoardPlaced>& Downloaded)
	{
		// Kinds that import on their own without questions. Animations need a skeleton picked, data a row type.
		static const TSet<FString> Kinds = {TEXT("model"), TEXT("texture"), TEXT("audio"), TEXT("font")};
		TArray<TStrongObjectPtr<UAssetImportTask>> Tasks;
		for (const FHoardPlaced& P : Downloaded)
		{
			const FString Bundle = HoardJson::Str(P.Entry, TEXT("bundle"));
			// What a model needs (its materials and textures) comes in with it.
			if (!Bundle.IsEmpty() && Bundle != HoardJson::Str(P.Entry, TEXT("id")))
			{
				continue;
			}
			if (!Kinds.Contains(HoardJson::Str(P.Entry, TEXT("kind"))) || !Importable().Contains(FPaths::GetExtension(P.Rel).ToLower()))
			{
				continue;
			}
			TStrongObjectPtr<UAssetImportTask> Task(NewObject<UAssetImportTask>());
			Task->Filename = Project.OutPath() / P.Rel;
			Task->DestinationPath = ContentPathFor(P.Rel);
			Task->bAutomated = true;
			Task->bReplaceExisting = true;
			Task->bSave = true;
			Tasks.Add(Task);
		}
		TArray<FString> Made;
		if (Tasks.Num() == 0)
		{
			return Made;
		}
		TArray<UAssetImportTask*> Raw;
		for (const TStrongObjectPtr<UAssetImportTask>& T : Tasks)
		{
			Raw.Add(T.Get());
		}
		FModuleManager::LoadModuleChecked<FAssetToolsModule>("AssetTools").Get().ImportAssetTasks(Raw);
		for (const UAssetImportTask* T : Raw)
		{
			for (const UObject* O : T->GetObjects())
			{
				if (O)
				{
					Made.AddUnique(O->GetPathName());
				}
			}
			for (const FString& Path : T->ImportedObjectPaths)
			{
				Made.AddUnique(Path);
			}
		}
		return Made;
	}

	bool Decode(const TArray<uint8>& Bytes, TArray<uint8>& OutBGRA, int32& OutWidth, int32& OutHeight)
	{
		if (Bytes.Num() == 0)
		{
			return false;
		}
		IImageWrapperModule& Images = FModuleManager::LoadModuleChecked<IImageWrapperModule>("ImageWrapper");
		const EImageFormat Format = Images.DetectImageFormat(Bytes.GetData(), Bytes.Num());
		if (Format != EImageFormat::PNG && Format != EImageFormat::JPEG)
		{
			return false;
		}
		const TSharedPtr<IImageWrapper> Image = Images.CreateImageWrapper(Format);
		if (!Image.IsValid() || !Image->SetCompressed(Bytes.GetData(), Bytes.Num()) || !Image->GetRaw(ERGBFormat::BGRA, 8, OutBGRA))
		{
			return false;
		}
		OutWidth = static_cast<int32>(Image->GetWidth());
		OutHeight = static_cast<int32>(Image->GetHeight());
		return OutWidth > 0 && OutHeight > 0;
	}
}
