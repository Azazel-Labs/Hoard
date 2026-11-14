#include "HoardExport.h"

#include "AssetExportTask.h"
#include "AssetRegistry/AssetRegistryModule.h"
#include "AssetRegistry/IAssetRegistry.h"
#include "EditorFramework/AssetImportData.h"
#include "Engine/SkeletalMesh.h"
#include "Engine/StaticMesh.h"
#include "Engine/Texture2D.h"
#include "Engine/World.h"
#include "Exporters/Exporter.h"
#include "GenericPlatform/GenericPlatformHttp.h"
#include "HAL/FileManager.h"
#include "HoardClient.h"
#include "HoardJson.h"
#include "Misc/FileHelper.h"
#include "Misc/Guid.h"
#include "Misc/PackageName.h"
#include "Misc/Paths.h"
#include "Sound/SoundWave.h"
#include "UObject/StrongObjectPtr.h"
#include "UObject/UnrealType.h"

/** Absolute, with forward slashes and no slash at the end. */
static FString Full(const FString& Path)
{
	FString Out = FPaths::ConvertRelativePathToFull(Path);
	FPaths::NormalizeFilename(Out);
	FPaths::CollapseRelativeDirectories(Out);
	while (Out.Len() > 1 && Out.EndsWith(TEXT("/")))
	{
		Out.LeftChopInline(1);
	}
	return Out;
}

static bool IsHoardPackage(const FString& Name)
{
	const FString Hoard = HoardDefaults::ContentPath;
	return Name.Equals(Hoard, ESearchCase::IgnoreCase) || Name.StartsWith(Hoard + TEXT("/"), ESearchCase::IgnoreCase);
}

/** "/Game/Props/Wood/Crate" -> "Props/Wood/", the folders it goes in in the pack. */
static FString FolderOf(const FString& Package)
{
	FString Path = FPaths::GetPath(Package);
	Path = Path.Equals(TEXT("/Game"), ESearchCase::IgnoreCase) ? FString() : Path.Mid(6);
	return Path.IsEmpty() ? Path : Path + TEXT("/");
}

static IAssetRegistry& Registry()
{
	return FModuleManager::LoadModuleChecked<FAssetRegistryModule>(TEXT("AssetRegistry")).Get();
}

/** The package's own asset, the one named after it (or its first). */
static bool MainAsset(const FName& Package, FAssetData& Out)
{
	TArray<FAssetData> Found;
	Registry().GetAssetsByPackageName(Package, Found, true);
	if (Found.Num() == 0)
	{
		return false;
	}
	const FName Short(*FPackageName::GetShortName(Package));
	const FAssetData* Named = Found.FindByPredicate([&](const FAssetData& A) { return A.AssetName == Short; });
	Out = Named ? *Named : Found[0];
	return true;
}

/** The files an asset was imported from that are still there, from whatever import data it keeps. */
static TArray<FString> SourcesOf(UObject* Asset)
{
	TArray<FString> Out;
	for (TFieldIterator<FObjectProperty> It(Asset->GetClass()); It; ++It)
	{
		if (!It->PropertyClass || !It->PropertyClass->IsChildOf(UAssetImportData::StaticClass()))
		{
			continue;
		}
		if (const UAssetImportData* Data = Cast<UAssetImportData>(It->GetObjectPropertyValue_InContainer(Asset)))
		{
			for (const FString& F : Data->ExtractFilenames())
			{
				const FString Abs = Full(F);
				if (!F.IsEmpty() && FPaths::FileExists(Abs))
				{
					Out.AddUnique(Abs);
				}
			}
		}
	}
	return Out;
}

/** A copy other tools open of a mesh, texture or sound made in Unreal (or whose file is gone): FBX (or OBJ), PNG or WAV. */
static bool Copy(UObject* Asset, TArray<uint8>& OutBytes, FString& OutExtension)
{
	TArray<const TCHAR*> Formats;
	if (Asset->IsA<UStaticMesh>() || Asset->IsA<USkeletalMesh>())
	{
		Formats = {TEXT("fbx"), TEXT("obj")};
	}
	else if (Asset->IsA<UTexture2D>())
	{
		Formats = {TEXT("png")};
	}
	else if (Asset->IsA<USoundWave>())
	{
		Formats = {TEXT("wav")};
	}
	const FString Dir = Full(FPaths::ProjectSavedDir() / TEXT("Hoard") / TEXT("Send") / FGuid::NewGuid().ToString());
	for (const TCHAR* Format : Formats)
	{
		UExporter* Exporter = UExporter::FindExporter(Asset, Format);
		if (!Exporter)
		{
			continue;
		}
		const FString File = Dir / FString::Printf(TEXT("%s.%s"), *Asset->GetName(), Format);
		IFileManager::Get().MakeDirectory(*Dir, true);
		TStrongObjectPtr<UAssetExportTask> Task(NewObject<UAssetExportTask>());
		Task->Object = Asset;
		Task->Exporter = Exporter;
		Task->Filename = File;
		Task->bAutomated = true;
		Task->bPrompt = false;
		Task->bReplaceIdentical = true;
		const bool bMade = UExporter::RunAssetExportTask(Task.Get()) && FFileHelper::LoadFileToArray(OutBytes, *File) && OutBytes.Num() > 0;
		IFileManager::Get().DeleteDirectory(*Dir, false, true);
		if (bMade)
		{
			OutExtension = FString(TEXT(".")) + Format;
			return true;
		}
	}
	return false;
}

FHoardExport::FHoardExport(const FHoardProject& InProject)
	: Project(InProject), OutAbs(Full(InProject.OutPath()))
{
}

bool FHoardExport::IsFromHoard(const FString& File) const
{
	const FString Abs = Full(File);
	return Abs.Equals(OutAbs, ESearchCase::IgnoreCase) || Abs.StartsWith(OutAbs + TEXT("/"), ESearchCase::IgnoreCase);
}

void FHoardExport::Plan(const TArray<FString>& Picked)
{
	IAssetRegistry& Assets = Registry();
	TArray<FName> Queue;
	for (const FString& P : Picked)
	{
		FString Path = P.TrimStartAndEnd();
		while (Path.Len() > 1 && Path.EndsWith(TEXT("/")))
		{
			Path.LeftChopInline(1);
		}
		if (Path.IsEmpty())
		{
			continue;
		}
		if (IsHoardPackage(Path))
		{
			FromHoard.AddUnique(Path);
			continue;
		}
		// A package, or a folder of them.
		TArray<FAssetData> Found;
		Assets.GetAssetsByPackageName(FName(*Path), Found, true);
		if (Found.Num() > 0)
		{
			Queue.AddUnique(FName(*Path));
			continue;
		}
		Assets.GetAssetsByPath(FName(*Path), Found, true, true);
		TArray<FName> Names;
		for (const FAssetData& A : Found)
		{
			Names.AddUnique(A.PackageName);
		}
		Names.Sort(FNameLexicalLess());
		for (const FName& N : Names)
		{
			Queue.AddUnique(N);
		}
	}
	// Everything they use, all the way down, in a stable order.
	for (int32 I = 0; I < Queue.Num(); ++I)
	{
		if (!Queue[I].ToString().StartsWith(TEXT("/Game/")))
		{
			continue;
		}
		TArray<FName> Uses;
		Assets.GetDependencies(Queue[I], Uses);
		Uses.Sort(FNameLexicalLess());
		for (const FName& U : Uses)
		{
			Queue.AddUnique(U);
		}
	}
	// Models first, so what they name goes beside them in the pack, as they name it.
	for (const FName& N : Queue)
	{
		Visit(N, true);
	}
	for (const FName& N : Queue)
	{
		Visit(N, false);
	}
}

void FHoardExport::Visit(const FName& Package, bool bModelsOnly)
{
	if (Seen.Contains(Package))
	{
		return;
	}
	const FString Name = Package.ToString();
	FAssetData Data;
	if (!Name.StartsWith(TEXT("/Game/")) || !MainAsset(Package, Data))
	{
		Seen.Add(Package);
		return;
	}
	if (IsHoardPackage(Name))
	{
		Seen.Add(Package);
		FromHoard.Add(Name);
		return;
	}
	// A level is only ever Unreal's; what it uses was queued already.
	UObject* Asset = Data.AssetClassPath == UWorld::StaticClass()->GetClassPathName() ? nullptr : Data.GetAsset();
	const TArray<FString> Sources = Asset ? SourcesOf(Asset) : TArray<FString>();
	const TArray<FString> Named = Sources.Num() > 0 ? Companions(Sources[0]) : TArray<FString>();
	if (bModelsOnly && Named.Num() == 0)
	{
		return;
	}
	Seen.Add(Package);
	const FString Folder = FolderOf(Name);
	if (Sources.Num() > 0)
	{
		if (IsFromHoard(Sources[0]))
		{
			FromHoard.Add(Name);
			return;
		}
		for (const FString& S : Sources)
		{
			Add(Folder + FPaths::GetCleanFilename(S), S, TArray<uint8>(), Name);
		}
		const FString Dir = FPaths::GetPath(Sources[0]) + TEXT("/");
		for (const FString& C : Named)
		{
			if (!IsFromHoard(C))
			{
				// Where the model finds it, when that's below it; beside it otherwise.
				Add(Folder + (C.StartsWith(Dir, ESearchCase::IgnoreCase) ? C.Mid(Dir.Len()) : FPaths::GetCleanFilename(C)), C, TArray<uint8>(), Name);
			}
		}
		return;
	}
	TArray<uint8> Bytes;
	FString Extension;
	if (Asset && Copy(Asset, Bytes, Extension))
	{
		Add(Folder + Data.AssetName.ToString() + Extension, FString(), MoveTemp(Bytes), Name);
	}
	else
	{
		EngineOnly.Add(Name);
	}
}

void FHoardExport::Add(const FString& Rel, const FString& Path, TArray<uint8>&& Bytes, const FString& From)
{
	if (!Path.IsEmpty())
	{
		bool bAlready = false;
		Paths.Add(Path.ToLower(), &bAlready);
		if (bAlready)
		{
			return;
		}
	}
	FString R = Rel;
	for (int32 N = 2; Rels.Contains(R.ToLower()); ++N)
	{
		const FString Ext = FPaths::GetExtension(Rel, true);
		R = FString::Printf(TEXT("%s %d%s"), *Rel.LeftChop(Ext.Len()), N, *Ext);
	}
	Rels.Add(R.ToLower());
	FHoardExportFile& F = Files.AddDefaulted_GetRef();
	F.Rel = R;
	F.Path = Path;
	F.Bytes = MoveTemp(Bytes);
	F.From = From;
}

/** The pictures a material library uses: the last word of each map line, after any options. */
static void MtlMaps(const FString& Mtl, TFunctionRef<void(const FString&)> Take)
{
	TArray<FString> Lines;
	FFileHelper::LoadFileToStringArray(Lines, *Mtl);
	const FString Dir = FPaths::GetPath(Mtl);
	for (const FString& Line : Lines)
	{
		TArray<FString> Words;
		Line.TrimStartAndEnd().ParseIntoArrayWS(Words);
		if (Words.Num() < 2)
		{
			continue;
		}
		const FString Key = Words[0].ToLower();
		if (Key.StartsWith(TEXT("map_")) || Key == TEXT("bump") || Key == TEXT("disp") || Key == TEXT("decal") || Key == TEXT("refl") || Key == TEXT("norm"))
		{
			Take(Dir / Words.Last().Replace(TEXT("\\"), TEXT("/")));
		}
	}
}

TArray<FString> FHoardExport::Companions(const FString& Path)
{
	TArray<FString> Out;
	const FString Dir = FPaths::GetPath(Path);
	auto Take = [&Out](const FString& P)
	{
		const FString F = Full(P);
		if (FPaths::FileExists(F))
		{
			Out.AddUnique(F);
		}
	};
	const FString Ext = FPaths::GetExtension(Path).ToLower();
	if (Ext == TEXT("obj"))
	{
		TArray<FString> Lines;
		FFileHelper::LoadFileToStringArray(Lines, *Path);
		for (const FString& Line : Lines)
		{
			const FString L = Line.TrimStartAndEnd();
			if (!L.StartsWith(TEXT("mtllib "), ESearchCase::CaseSensitive))
			{
				continue;
			}
			const FString Mtl = Full(Dir / L.Mid(7).TrimStartAndEnd());
			if (FPaths::FileExists(Mtl) && !Out.Contains(Mtl))
			{
				Out.Add(Mtl);
				MtlMaps(Mtl, Take);
			}
		}
	}
	else if (Ext == TEXT("mtl"))
	{
		MtlMaps(Path, Take);
	}
	else if (Ext == TEXT("gltf"))
	{
		FString Text;
		FFileHelper::LoadFileToString(Text, *Path);
		const TSharedPtr<FJsonObject> Data = HoardJson::Obj(HoardJson::Parse(Text));
		for (const TCHAR* Key : {TEXT("buffers"), TEXT("images")})
		{
			for (const TSharedPtr<FJsonValue>& V : HoardJson::List(Data, Key))
			{
				const FString Uri = HoardJson::Str(HoardJson::Obj(V), TEXT("uri"));
				if (!Uri.IsEmpty() && !Uri.StartsWith(TEXT("data:")))
				{
					Take(Dir / FGenericPlatformHttp::UrlDecode(Uri));
				}
			}
		}
	}
	return Out;
}

static void Append(TArray<uint8>& Body, const FString& Text)
{
	const FTCHARToUTF8 Utf8(*Text);
	Body.Append(reinterpret_cast<const uint8*>(Utf8.Get()), Utf8.Length());
}

void FHoardExport::Send(const TSharedRef<FHoardClient>& Client, const FString& PackName, TFunction<void(const FHoardSendResult&)> Done) const
{
	if (Files.Num() == 0)
	{
		FHoardSendResult R;
		R.Error = FString::Printf(TEXT("Nothing here to send: %s."), *WhyNothing());
		Done(R);
		return;
	}
	const FString Boundary = TEXT("hoard") + FGuid::NewGuid().ToString(EGuidFormats::Digits).ToLower();
	TArray<uint8> Body;
	auto Field = [&](const TCHAR* Name, const FString& Value)
	{
		Append(Body, FString::Printf(TEXT("--%s\r\nContent-Disposition: form-data; name=\"%s\"\r\n\r\n%s\r\n"), *Boundary, Name, *Value));
	};
	const FString Name = PackName.TrimStartAndEnd();
	Field(TEXT("name"), Name.IsEmpty() ? TEXT("From ") + Project.Project : Name);
	Field(TEXT("source"), TEXT("Unreal project: ") + Project.Project);
	for (const FHoardExportFile& F : Files)
	{
		Field(TEXT("relpath"), F.Rel);
		Append(Body, FString::Printf(TEXT("--%s\r\nContent-Disposition: form-data; name=\"file\"; filename=\"%s\"\r\nContent-Type: application/octet-stream\r\n\r\n"),
			*Boundary, *FPaths::GetCleanFilename(F.Rel).Replace(TEXT("\""), TEXT("_"))));
		if (F.IsCopy())
		{
			Body.Append(F.Bytes);
		}
		else
		{
			TArray<uint8> Bytes;
			if (!FFileHelper::LoadFileToArray(Bytes, *F.Path))
			{
				FHoardSendResult R;
				R.Error = TEXT("Couldn't read ") + F.Path;
				Done(R);
				return;
			}
			Body.Append(Bytes);
		}
		Append(Body, TEXT("\r\n"));
	}
	Append(Body, FString::Printf(TEXT("--%s--\r\n"), *Boundary));
	TArray<FString> Sent;
	for (const FHoardExportFile& F : Files)
	{
		Sent.Add(F.Rel);
	}
	Client->Upload(TEXT("/api/packs"), MoveTemp(Body), TEXT("multipart/form-data; boundary=") + Boundary, [Client, Sent, Done](const FHoardReply& Reply)
	{
		FHoardSendResult R;
		if (!Reply.bOk)
		{
			R.Error = Reply.Code == 403 ? TEXT("Sending needs a token that can add to the library (an editor's or an admin's); this one can only look.") : Reply.Error;
			Done(R);
			return;
		}
		const TSharedPtr<FJsonObject> Pack = HoardJson::Obj(HoardJson::Parse(HoardJson::Utf8(Reply.Body)));
		R.Pack = HoardJson::Str(Pack, TEXT("id"));
		if (R.Pack.IsEmpty())
		{
			R.Error = TEXT("The server's answer made no sense.");
			Done(R);
			return;
		}
		R.bOk = true;
		R.Name = HoardJson::Str(Pack, TEXT("name"));
		R.Link = Client->Url(FString::Printf(TEXT("/?pack=%s&pack_name=%s"), *HoardJson::Escape(R.Pack), *HoardJson::Escape(R.Name)));
		R.Sent = Sent;
		Done(R);
	});
}

FString FHoardExport::WhyNothing() const
{
	if (FromHoard.Num() > 0 && EngineOnly.Num() == 0)
	{
		return TEXT("it all came from Hoard");
	}
	if (EngineOnly.Num() > 0)
	{
		return TEXT("only Unreal opens what's picked, and it uses nothing else");
	}
	return TEXT("nothing is picked");
}

FString FHoardExport::Summary() const
{
	FString S = Files.Num() == 1 ? FString(TEXT("1 file")) : FString::Printf(TEXT("%d files"), Files.Num());
	TArray<FString> Left;
	if (FromHoard.Num() > 0)
	{
		Left.Add(FString::Printf(TEXT("%d that came from Hoard"), FromHoard.Num()));
	}
	if (EngineOnly.Num() > 0)
	{
		Left.Add(FString::Printf(TEXT("%d only Unreal opens"), EngineOnly.Num()));
	}
	return Left.Num() > 0 ? S + TEXT("; left out: ") + FString::Join(Left, TEXT(", ")) : S;
}
