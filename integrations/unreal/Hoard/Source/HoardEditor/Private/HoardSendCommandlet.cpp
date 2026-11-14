#include "HoardSendCommandlet.h"

#include "AssetImportTask.h"
#include "AssetRegistry/AssetRegistryModule.h"
#include "AssetToolsModule.h"
#include "HAL/PlatformMisc.h"
#include "HoardClient.h"
#include "HoardExport.h"
#include "HoardJson.h"
#include "HoardProject.h"
#include "Misc/Paths.h"
#include "UObject/StrongObjectPtr.h"

#include <cstdio>

UHoardSendCommandlet::UHoardSendCommandlet()
{
	IsClient = false;
	IsServer = false;
	IsEditor = true;
	LogToConsole = true;
	ShowErrorCount = false;
}

static FString Env(const TCHAR* Name)
{
	return FPlatformMisc::GetEnvironmentVariable(Name);
}

static TArray<FString> Lines(const FString& Text)
{
	TArray<FString> Out;
	Text.ParseIntoArrayLines(Out, true);
	for (FString& L : Out)
	{
		L.TrimStartAndEndInline();
	}
	Out.RemoveAll([](const FString& L) { return L.IsEmpty(); });
	return Out;
}

static TSharedRef<FJsonValue> Strings(const TArray<FString>& Values)
{
	TArray<TSharedPtr<FJsonValue>> Out;
	for (const FString& V : Values)
	{
		Out.Add(MakeShared<FJsonValueString>(V));
	}
	return MakeShared<FJsonValueArray>(Out);
}

static int32 Done(const TSharedRef<FJsonObject>& Result)
{
	const FString Line = TEXT("RESULT ") + HoardJson::Write(MakeShared<FJsonValueObject>(Result));
	std::printf("\n%s\n", TCHAR_TO_UTF8(*Line));
	std::fflush(stdout);
	return Result->GetBoolField(TEXT("ok")) ? 0 : 1;
}

/** Import files as the editor would, into one content folder, and say what that made. */
static int32 Import(const TArray<FString>& Files, const FString& To)
{
	TArray<TStrongObjectPtr<UAssetImportTask>> Tasks;
	TArray<UAssetImportTask*> Raw;
	for (const FString& F : Files)
	{
		TStrongObjectPtr<UAssetImportTask> Task(NewObject<UAssetImportTask>());
		Task->Filename = F;
		Task->DestinationPath = To;
		Task->bAutomated = true;
		Task->bReplaceExisting = true;
		Task->bSave = true;
		Raw.Add(Task.Get());
		Tasks.Add(MoveTemp(Task));
	}
	FModuleManager::LoadModuleChecked<FAssetToolsModule>("AssetTools").Get().ImportAssetTasks(Raw);
	TArray<FString> Made;
	for (const UAssetImportTask* T : Raw)
	{
		for (const FString& P : T->ImportedObjectPaths)
		{
			Made.AddUnique(P);
		}
	}
	const TSharedRef<FJsonObject> Result = MakeShared<FJsonObject>();
	Result->SetBoolField(TEXT("ok"), Made.Num() > 0);
	Result->SetField(TEXT("imported"), Strings(Made));
	return Done(Result);
}

int32 UHoardSendCommandlet::Main(const FString& Params)
{
	FString Dir = Env(TEXT("HOARD_PROJECT"));
	if (Dir.IsEmpty())
	{
		Dir = FPaths::ConvertRelativePathToFull(FPaths::ProjectDir());
	}
	FModuleManager::LoadModuleChecked<FAssetRegistryModule>(TEXT("AssetRegistry")).Get().SearchAllAssets(true);
	const TArray<FString> ToImport = Lines(Env(TEXT("HOARD_IMPORT")));
	if (ToImport.Num() > 0)
	{
		const FString To = Env(TEXT("HOARD_IMPORT_TO"));
		return Import(ToImport, To.IsEmpty() ? FString(TEXT("/Game/Imported")) : To);
	}

	const TSharedRef<FHoardClient> Client = MakeShared<FHoardClient>();
	Client->Server = Env(TEXT("HOARD_URL"));
	Client->Token = Env(TEXT("HOARD_TOKEN"));
	FHoardExport Export(FHoardProject(Dir, HoardDefaults::OutDir));
	Export.Plan(Lines(Env(TEXT("HOARD_EXPORT"))));
	bool bDone = false;
	FHoardSendResult Sent;
	Sent.Error = TEXT("it took too long");
	Export.Send(Client, Env(TEXT("HOARD_PACK_NAME")), [&](const FHoardSendResult& R)
	{
		Sent = R;
		bDone = true;
	});
	HoardWait(bDone, 600.0);

	TArray<FString> Planned;
	for (const FHoardExportFile& F : Export.Files)
	{
		Planned.Add(F.Rel);
	}
	const TSharedRef<FJsonObject> Result = MakeShared<FJsonObject>();
	Result->SetBoolField(TEXT("ok"), Sent.bOk);
	Result->SetField(TEXT("error"), Sent.bOk ? MakeShared<FJsonValueNull>() : StaticCastSharedRef<FJsonValue>(MakeShared<FJsonValueString>(Sent.Error)));
	Result->SetStringField(TEXT("pack"), Sent.Pack);
	Result->SetStringField(TEXT("link"), Sent.Link);
	Result->SetField(TEXT("sent"), Strings(Sent.Sent));
	Result->SetField(TEXT("planned"), Strings(Planned));
	Result->SetNumberField(TEXT("from_hoard"), Export.FromHoard.Num());
	Result->SetNumberField(TEXT("engine_only"), Export.EngineOnly.Num());
	TArray<FString> LeftOut = Export.FromHoard;
	LeftOut.Append(Export.EngineOnly);
	Result->SetField(TEXT("left_out"), Strings(LeftOut));
	return Done(Result);
}
