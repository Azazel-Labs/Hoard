#include "HoardAddCommandlet.h"

#include "HAL/PlatformMisc.h"
#include "HoardClient.h"
#include "HoardImport.h"
#include "HoardJson.h"
#include "HoardProject.h"
#include "Misc/Paths.h"

#include <cstdio>

UHoardAddCommandlet::UHoardAddCommandlet()
{
	IsClient = false;
	IsServer = false;
	IsEditor = true;
	LogToConsole = true;
	ShowErrorCount = false;
}

int32 UHoardAddCommandlet::Main(const FString& Params)
{
	const TSharedRef<FHoardClient> Client = MakeShared<FHoardClient>();
	Client->Server = FPlatformMisc::GetEnvironmentVariable(TEXT("HOARD_URL"));
	Client->Token = FPlatformMisc::GetEnvironmentVariable(TEXT("HOARD_TOKEN"));
	FString Dir = FPlatformMisc::GetEnvironmentVariable(TEXT("HOARD_PROJECT"));
	if (Dir.IsEmpty())
	{
		Dir = FPaths::ConvertRelativePathToFull(FPaths::ProjectDir());
	}

	bool bDone = false;
	FHoardReply Found;
	Client->Search(FPlatformMisc::GetEnvironmentVariable(TEXT("HOARD_QUERY")), FPlatformMisc::GetEnvironmentVariable(TEXT("HOARD_KIND")), 0, 40,
		[&](const FHoardReply& R)
		{
			Found = R;
			bDone = true;
		});
	HoardWait(bDone);

	TArray<TSharedPtr<FJsonObject>> Assets;
	for (const TSharedPtr<FJsonValue>& V : HoardJson::List(Found.Object(), TEXT("items")))
	{
		if (const TSharedPtr<FJsonObject> O = HoardJson::Obj(V))
		{
			Assets.Add(O);
		}
	}
	// The thumbnails as the tab shows them: fetched and decoded.
	int32 Thumbnails = 0;
	for (const TSharedPtr<FJsonObject>& A : Assets)
	{
		bDone = false;
		Client->Thumbnail(A, [&](const TArray<uint8>& Bytes)
		{
			TArray<uint8> Pixels;
			int32 W = 0, H = 0;
			Thumbnails += HoardImport::Decode(Bytes, Pixels, W, H) ? 1 : 0;
			bDone = true;
		});
		HoardWait(bDone);
	}

	FHoardAddResult Added;
	Added.Error = Found.bOk ? TEXT("nothing found") : Found.Error;
	TArray<FString> Imported;
	if (Assets.Num() > 0)
	{
		bDone = false;
		const FHoardProject Project(Dir, HoardDefaults::OutDir);
		Project.Add(Client, Assets, nullptr, [&](const FHoardAddResult& R)
		{
			Added = R;
			bDone = true;
		});
		HoardWait(bDone, 600.0);
		if (!bDone)
		{
			Added.Error = TEXT("it took too long");
		}
		if (Added.bOk)
		{
			Imported = HoardImport::Import(Project, Added.Downloaded);
		}
	}

	const TSharedRef<FJsonObject> Result = MakeShared<FJsonObject>();
	Result->SetBoolField(TEXT("ok"), Added.bOk);
	Result->SetField(TEXT("error"), Added.bOk ? MakeShared<FJsonValueNull>() : StaticCastSharedRef<FJsonValue>(MakeShared<FJsonValueString>(Added.Error)));
	Result->SetNumberField(TEXT("files"), Added.Files);
	Result->SetNumberField(TEXT("fetched"), Added.Downloaded.Num());
	Result->SetBoolField(TEXT("reported"), Added.bReported);
	Result->SetNumberField(TEXT("matched"), Assets.Num());
	Result->SetNumberField(TEXT("thumbnails"), Thumbnails);
	TArray<TSharedPtr<FJsonValue>> Made;
	for (const FString& P : Imported)
	{
		Made.Add(MakeShared<FJsonValueString>(P));
	}
	Result->SetArrayField(TEXT("imported"), Made);
	const FString Line = TEXT("RESULT ") + HoardJson::Write(MakeShared<FJsonValueObject>(Result));
	std::printf("\n%s\n", TCHAR_TO_UTF8(*Line));
	std::fflush(stdout);
	return Added.bOk ? 0 : 1;
}
