#include "HoardProject.h"

#include "HAL/FileManager.h"
#include "HAL/PlatformFileManager.h"
#include "HoardClient.h"
#include "HoardJson.h"
#include "Misc/FileHelper.h"
#include "Misc/Paths.h"

using HoardJson::Str;

static bool Save(const FString& Path, const FString& Text)
{
	return FFileHelper::SaveStringToFile(Text, *Path, FFileHelper::EEncodingOptions::ForceUTF8WithoutBOM);
}

static int64 SizeOf(const FString& Path)
{
	return FPlatformFileManager::Get().GetPlatformFile().FileSize(*Path);
}

/** A path's folders and name, leaving out empty ones and "..", each made safe. */
static TArray<FString> Segments(const FString& Path)
{
	TArray<FString> Parts;
	Path.ParseIntoArray(Parts, TEXT("/"), true);
	TArray<FString> Out;
	for (const FString& P : Parts)
	{
		if (P != TEXT(".."))
		{
			Out.Add(FHoardProject::Sanitize(P));
		}
	}
	return Out;
}

static FString Join(const TArray<FString>& Parts)
{
	FString Out;
	for (const FString& P : Parts)
	{
		if (!P.IsEmpty())
		{
			Out += Out.IsEmpty() ? P : TEXT("/") + P;
		}
	}
	return Out;
}

FHoardProject::FHoardProject(const FString& InRoot, const FString& DefaultOutDir)
	: Root(InRoot.Replace(TEXT("\\"), TEXT("/"))), OutDir(DefaultOutDir)
{
	while (Root.Len() > 1 && Root.EndsWith(TEXT("/")))
	{
		Root.LeftChopInline(1);
	}
	Project = FPaths::GetCleanFilename(Root);
	ReadConfig();
}

FString FHoardProject::OutPath() const
{
	FString Dir = OutDir.StartsWith(TEXT("./")) ? OutDir.Mid(2) : OutDir;
	while (Dir.EndsWith(TEXT("/")) || Dir.EndsWith(TEXT("\\")))
	{
		Dir.LeftChopInline(1);
	}
	return FPaths::IsRelative(Dir) ? Root / Dir : Dir.Replace(TEXT("\\"), TEXT("/"));
}

/** A TOML string's contents, or a bare value (true, 3) without any comment after it. */
static FString Value(const FString& Raw)
{
	const FString S = Raw.TrimStartAndEnd();
	if (S.StartsWith(TEXT("\"")) || S.StartsWith(TEXT("'")))
	{
		const int32 End = S.Find(S.Left(1), ESearchCase::CaseSensitive, ESearchDir::FromStart, 1);
		return End > 0 ? S.Mid(1, End - 1) : S.Mid(1);
	}
	int32 Hash;
	return (S.FindChar(TEXT('#'), Hash) ? S.Left(Hash) : S).TrimStartAndEnd();
}

void FHoardProject::ReadConfig()
{
	FString Text;
	if (!FFileHelper::LoadFileToString(Text, *TomlPath()))
	{
		return;
	}
	TArray<FString> Lines;
	Text.ParseIntoArrayLines(Lines, false);
	// The settings `hoard sync` reads from the top of hoard.toml, before its first [table].
	for (const FString& Line : Lines)
	{
		const FString L = Line.TrimStartAndEnd();
		if (L.StartsWith(TEXT("[")))
		{
			break;
		}
		int32 Eq;
		if (L.StartsWith(TEXT("#")) || !L.FindChar(TEXT('='), Eq))
		{
			continue;
		}
		const FString Key = L.Left(Eq).TrimStartAndEnd();
		const FString V = Value(L.Mid(Eq + 1));
		if (Key == TEXT("server")) Server = V;
		else if (Key == TEXT("project")) Project = V;
		else if (Key == TEXT("out_dir")) OutDir = V;
		else if (Key == TEXT("layout")) Layout = V;
		else if (Key == TEXT("sidecars")) bSidecars = V == TEXT("true");
	}
}

FString FHoardProject::Sanitize(const FString& Segment)
{
	static const FString Bad = TEXT("<>:\"/\\|?*");
	FString Out;
	for (const TCHAR C : Segment)
	{
		int32 At;
		const bool bControl = C < 32 || (C >= 127 && C < 160);
		Out.AppendChar(bControl || Bad.FindChar(C, At) ? TEXT('_') : C);
	}
	Out.TrimStartAndEndInline();
	while (Out.StartsWith(TEXT(".")))
	{
		Out.RightChopInline(1);
	}
	while (Out.EndsWith(TEXT(".")))
	{
		Out.LeftChopInline(1);
	}
	return Out.IsEmpty() ? FString(TEXT("_")) : Out;
}

FString FHoardProject::FileName(const TSharedPtr<FJsonObject>& Entry)
{
	FString Name = Str(Entry, TEXT("file_name"));
	if (Name.IsEmpty())
	{
		const FString Path = Str(Entry, TEXT("path"));
		int32 Slash;
		Name = Path.FindLastChar(TEXT('/'), Slash) ? Path.Mid(Slash + 1) : Path;
	}
	return Sanitize(Name);
}

FString FHoardProject::FolderFor(const TSharedPtr<FJsonObject>& Entry) const
{
	if (Layout == TEXT("flat"))
	{
		return FString();
	}
	if (Layout == TEXT("pack"))
	{
		const FString Path = Str(Entry, TEXT("path"));
		int32 Slash;
		TArray<FString> Parts = {Sanitize(Str(Entry, TEXT("pack_name")))};
		Parts.Append(Segments(Path.FindLastChar(TEXT('/'), Slash) ? Path.Left(Slash) : FString()));
		return Join(Parts);
	}
	const FString Category = Str(Entry, TEXT("category"));
	TArray<FString> Parts;
	(Category.IsEmpty() ? FString(TEXT("misc")) : Category).ParseIntoArray(Parts, TEXT("/"), false);
	for (FString& P : Parts)
	{
		P = Sanitize(P);
	}
	return FString::Join(Parts, TEXT("/"));
}

FString FHoardProject::DestFor(const TSharedPtr<FJsonObject>& Entry) const
{
	const FString Folder = FolderFor(Entry);
	return Folder.IsEmpty() ? FileName(Entry) : Folder + TEXT("/") + FileName(Entry);
}

FString FHoardProject::BundleDest(const TSharedPtr<FJsonObject>& BundleRoot, const FString& BundlePath) const
{
	const FString RootFile = FileName(BundleRoot);
	int32 Dot;
	const FString Stem = RootFile.FindLastChar(TEXT('.'), Dot) ? RootFile.Left(Dot) : RootFile;
	return Join({FolderFor(BundleRoot), Stem, Join(Segments(BundlePath))});
}

TArray<FHoardPlaced> FHoardProject::Place(const TArray<TSharedPtr<FJsonValue>>& Entries, const FHoardLock& Taken) const
{
	TArray<TSharedPtr<FJsonObject>> All;
	for (const TSharedPtr<FJsonValue>& V : Entries)
	{
		if (const TSharedPtr<FJsonObject> O = HoardJson::Obj(V))
		{
			All.Add(O);
		}
	}
	TMap<FString, TSharedPtr<FJsonObject>> Roots;
	TSet<FString> InBundles;
	for (const TSharedPtr<FJsonObject>& E : All)
	{
		const FString Bundle = Str(E, TEXT("bundle"));
		if (Bundle.IsEmpty())
		{
			continue;
		}
		if (Bundle == Str(E, TEXT("id")))
		{
			Roots.Add(Bundle, E);
		}
		else
		{
			InBundles.Add(Str(E, TEXT("id")));
		}
	}
	TArray<FHoardPlaced> Wanted;
	TMap<FString, int32, FDefaultSetAllocator, THoardPathKeyFuncs<int32>> Index;
	for (const TSharedPtr<FJsonObject>& E : All)
	{
		const FString Id = Str(E, TEXT("id"));
		const FString Hash = Str(E, TEXT("hash"));
		const FString Bundle = Str(E, TEXT("bundle"));
		if (Bundle.IsEmpty() && InBundles.Contains(Id))
		{
			continue;
		}
		const FString BundlePath = Str(E, TEXT("bundle_path"));
		const TSharedPtr<FJsonObject>* BundleRoot = Bundle.IsEmpty() || BundlePath.IsEmpty() ? nullptr : Roots.Find(Bundle);
		FString Rel = BundleRoot ? BundleDest(*BundleRoot, BundlePath) : DestFor(E);
		FString ThereId, ThereHash;
		bool bThere = false;
		if (const int32* At = Index.Find(Rel))
		{
			ThereId = Str(Wanted[*At].Entry, TEXT("id"));
			ThereHash = Str(Wanted[*At].Entry, TEXT("hash"));
			bThere = true;
		}
		else if (const FHoardLockEntry* Had = Taken.Find(Rel))
		{
			ThereId = Had->Id;
			ThereHash = Had->Hash;
			bThere = true;
		}
		if (bThere && ThereHash != Hash && ThereId != Id)
		{
			int32 Dot = INDEX_NONE, Slash = INDEX_NONE;
			Rel.FindLastChar(TEXT('.'), Dot);
			Rel.FindLastChar(TEXT('/'), Slash);
			Rel = Dot > Slash ? Rel.Left(Dot) + TEXT(".") + Hash.Left(8) + Rel.Mid(Dot) : Rel + TEXT(".") + Hash.Left(8);
		}
		if (!Index.Contains(Rel))
		{
			Index.Add(Rel, Wanted.Num());
			Wanted.Add({Rel, E});
		}
	}
	return Wanted;
}

FHoardLock FHoardProject::ReadLock() const
{
	FHoardLock Files;
	FString Text;
	if (!FFileHelper::LoadFileToString(Text, *LockPath()))
	{
		return Files;
	}
	const TSharedPtr<FJsonObject> Doc = HoardJson::Obj(HoardJson::Parse(Text));
	if (const TSharedPtr<FJsonObject> Map = HoardJson::Obj(Doc.IsValid() ? Doc->TryGetField(TEXT("files")) : nullptr))
	{
		for (const TPair<FString, TSharedPtr<FJsonValue>>& Kv : Map->Values)
		{
			const TSharedPtr<FJsonObject> O = HoardJson::Obj(Kv.Value);
			Files.Add(Kv.Key, {Str(O, TEXT("id")), Str(O, TEXT("hash")), HoardJson::Long(O, TEXT("size"))});
		}
	}
	return Files;
}

void FHoardProject::WriteLock(const FHoardLock& Files) const
{
	// Written out by hand to match `hoard sync`'s lock to the byte: serde_json's pretty printing, paths in byte order.
	TArray<FString> Keys;
	Files.GetKeys(Keys);
	Keys.Sort([](const FString& A, const FString& B)
	{
		const FTCHARToUTF8 UA(*A);
		const FTCHARToUTF8 UB(*B);
		const int32 C = FMemory::Memcmp(UA.Get(), UB.Get(), FMath::Min(UA.Length(), UB.Length()));
		return C != 0 ? C < 0 : UA.Length() < UB.Length();
	});
	FString Text = TEXT("{\n  \"version\": 1,\n  \"files\": {");
	for (int32 I = 0; I < Keys.Num(); ++I)
	{
		const FHoardLockEntry& E = Files[Keys[I]];
		Text += I > 0 ? TEXT(",\n") : TEXT("\n");
		Text += FString::Printf(TEXT("    %s: {\n      \"id\": %s,\n      \"hash\": %s,\n      \"size\": %lld\n    }"),
			*HoardJson::Quote(Keys[I]), *HoardJson::Quote(E.Id), *HoardJson::Quote(E.Hash), static_cast<long long>(E.Size));
	}
	Text += Keys.Num() > 0 ? TEXT("\n  }\n}\n") : TEXT("}\n}\n");
	Save(LockPath(), Text);
}

void FHoardProject::Pin(const TArray<FString>& Ids, const TArray<FString>& Names, const FString& DefaultServer) const
{
	FString Text;
	FFileHelper::LoadFileToString(Text, *TomlPath());
	if (Text.IsEmpty())
	{
		Text = TEXT("# What this project takes from Hoard. `hoard sync` fetches it; the editor plugin adds to it.\n");
		if (!DefaultServer.IsEmpty())
		{
			Text += FString::Printf(TEXT("server = %s\n"), *HoardJson::Quote(DefaultServer));
		}
		Text += FString::Printf(TEXT("out_dir = %s\n"), *HoardJson::Quote(OutDir));
	}
	TArray<FString> Fresh;
	TArray<FString> Labels;
	for (int32 I = 0; I < Ids.Num(); ++I)
	{
		if (Text.Contains(TEXT("\"") + Ids[I] + TEXT("\""), ESearchCase::CaseSensitive))
		{
			continue;
		}
		Fresh.Add(HoardJson::Quote(Ids[I]));
		Labels.Add(Names.IsValidIndex(I) ? Names[I].Replace(TEXT("\n"), TEXT(" ")) : FString());
	}
	if (Fresh.Num() == 0)
	{
		return;
	}
	if (!Text.EndsWith(TEXT("\n")))
	{
		Text += TEXT("\n");
	}
	Text += FString::Printf(TEXT("\n[[select]]\nids = [%s]  # %s\n"), *FString::Join(Fresh, TEXT(", ")), *FString::Join(Labels, TEXT(", ")));
	Save(TomlPath(), Text);
}

/** One Add, from the manifest through the downloads to telling the server; it keeps itself alive until it's done. */
class FHoardAddJob : public TSharedFromThis<FHoardAddJob>
{
public:
	FHoardAddJob(const FHoardProject& InProject, const TSharedRef<FHoardClient>& InClient)
		: Project(InProject), Client(InClient)
	{
	}

	FHoardProject Project;
	TSharedRef<FHoardClient> Client;
	TArray<TSharedPtr<FJsonObject>> Assets;
	TFunction<void(int32, int32)> Progress;
	TFunction<void(const FHoardAddResult&)> Done;

	FHoardLock Lock;
	TArray<FHoardPlaced> Wanted;
	TArray<int32> Todo;
	int32 Next = 0;
	FHoardAddResult Result;

	void Start()
	{
		TArray<TSharedPtr<FJsonValue>> Ids;
		for (const TSharedPtr<FJsonObject>& A : Assets)
		{
			Ids.Add(MakeShared<FJsonValueString>(Str(A, TEXT("id"))));
		}
		const TSharedRef<FJsonObject> Payload = MakeShared<FJsonObject>();
		Payload->SetArrayField(TEXT("ids"), Ids);
		Payload->SetBoolField(TEXT("include_deps"), true);
		Client->Json(TEXT("/api/manifest"), TEXT("POST"), MakeShared<FJsonValueObject>(Payload), [Self = AsShared()](const FHoardReply& R)
		{
			Self->Planned(R);
		});
	}

	void Fail(const FString& Error)
	{
		Result.bOk = false;
		Result.Error = Error;
		Done(Result);
	}

	void Planned(const FHoardReply& R)
	{
		if (!R.bOk)
		{
			Fail(R.Error);
			return;
		}
		Lock = Project.ReadLock();
		Wanted = Project.Place(HoardJson::List(R.Object(), TEXT("entries")), Lock);
		const FString Out = Project.OutPath();
		for (int32 I = 0; I < Wanted.Num(); ++I)
		{
			const FHoardPlaced& W = Wanted[I];
			const FHoardLockEntry* Had = Lock.Find(W.Rel);
			const bool bHere = Had && Had->Hash == Str(W.Entry, TEXT("hash")) && SizeOf(Out / W.Rel) == HoardJson::Long(W.Entry, TEXT("size"));
			if (!bHere)
			{
				Todo.Add(I);
			}
		}
		DownloadNext();
	}

	void DownloadNext()
	{
		if (Next >= Todo.Num())
		{
			Finish();
			return;
		}
		const FHoardPlaced& W = Wanted[Todo[Next]];
		const FString Path = Project.OutPath() / W.Rel;
		IFileManager::Get().MakeDirectory(*FPaths::GetPath(Path), true);
		const FString Partial = Path + TEXT(".hoard-partial");
		Client->Send(Str(W.Entry, TEXT("url")), TEXT("GET"), FString(), [Self = AsShared(), Path, Partial](const FHoardReply& R)
		{
			const FHoardPlaced& Got = Self->Wanted[Self->Todo[Self->Next]];
			if (!R.bOk || SizeOf(Partial) != HoardJson::Long(Got.Entry, TEXT("size")))
			{
				IFileManager::Get().Delete(*Partial, false, true, true);
				Self->Fail(FString::Printf(TEXT("Couldn't download %s: %s"), *Got.Rel, R.bOk ? TEXT("it came out the wrong size.") : *R.Error));
				return;
			}
			if (!IFileManager::Get().Move(*Path, *Partial, true, true))
			{
				IFileManager::Get().Delete(*Partial, false, true, true);
				Self->Fail(FString::Printf(TEXT("Couldn't put %s in place."), *Path));
				return;
			}
			Self->Result.Downloaded.Add(Got);
			++Self->Next;
			if (Self->Progress)
			{
				Self->Progress(Self->Next, Self->Todo.Num());
			}
			Self->DownloadNext();
		}, Partial);
	}

	void Finish()
	{
		const FString Out = Project.OutPath();
		if (Project.bSidecars)
		{
			for (const FHoardPlaced& W : Wanted)
			{
				const TSharedPtr<FJsonObject>& E = W.Entry;
				const TSharedRef<FJsonObject> Side = MakeShared<FJsonObject>();
				Side->SetStringField(TEXT("id"), Str(E, TEXT("id")));
				Side->SetStringField(TEXT("hash"), Str(E, TEXT("hash")));
				Side->SetStringField(TEXT("name"), Str(E, TEXT("name")));
				Side->SetStringField(TEXT("kind"), Str(E, TEXT("kind")));
				const TSharedPtr<FJsonValue> Category = E->TryGetField(TEXT("category"));
				Side->SetField(TEXT("category"), Category.IsValid() ? Category : MakeShared<FJsonValueNull>());
				Side->SetArrayField(TEXT("tags"), HoardJson::List(E, TEXT("tags")));
				Side->SetStringField(TEXT("pack"), Str(E, TEXT("pack_name")));
				Side->SetStringField(TEXT("source_path"), Str(E, TEXT("path")));
				const TSharedPtr<FJsonValue> Meta = E->TryGetField(TEXT("meta"));
				Side->SetField(TEXT("meta"), Meta.IsValid() ? Meta : MakeShared<FJsonValueObject>(MakeShared<FJsonObject>()));
				Save(Out / W.Rel + TEXT(".hoard.json"), HoardJson::Write(MakeShared<FJsonValueObject>(Side), true));
			}
		}
		TArray<FString> Ids;
		TArray<FString> Names;
		for (const TSharedPtr<FJsonObject>& A : Assets)
		{
			Ids.Add(Str(A, TEXT("id")));
			Names.Add(Str(A, TEXT("name")));
		}
		Project.Pin(Ids, Names, Client->Server);
		for (const FHoardPlaced& W : Wanted)
		{
			Lock.Add(W.Rel, {Str(W.Entry, TEXT("id")), Str(W.Entry, TEXT("hash")), HoardJson::Long(W.Entry, TEXT("size"))});
		}
		Project.WriteLock(Lock);
		Result.Files = Wanted.Num();
		Result.OutDir = Out;
		// So the library knows what this project uses. The files are in place either way.
		TArray<TSharedPtr<FJsonValue>> Used;
		for (const TPair<FString, FHoardLockEntry>& Kv : Lock)
		{
			const TSharedRef<FJsonObject> U = MakeShared<FJsonObject>();
			U->SetStringField(TEXT("id"), Kv.Value.Id);
			U->SetStringField(TEXT("path"), Kv.Key);
			Used.Add(MakeShared<FJsonValueObject>(U));
		}
		const TSharedRef<FJsonObject> Payload = MakeShared<FJsonObject>();
		Payload->SetArrayField(TEXT("assets"), Used);
		Client->Json(TEXT("/api/projects/") + HoardJson::Escape(Project.Project), TEXT("PUT"), MakeShared<FJsonValueObject>(Payload), [Self = AsShared()](const FHoardReply& R)
		{
			Self->Result.bOk = true;
			Self->Result.bReported = R.bOk;
			Self->Done(Self->Result);
		});
	}
};

void FHoardProject::Add(const TSharedRef<FHoardClient>& Client, const TArray<TSharedPtr<FJsonObject>>& Assets,
	TFunction<void(int32, int32)> Progress, TFunction<void(const FHoardAddResult&)> Done) const
{
	const TSharedRef<FHoardAddJob> Job = MakeShared<FHoardAddJob>(*this, Client);
	Job->Assets = Assets;
	Job->Progress = MoveTemp(Progress);
	Job->Done = MoveTemp(Done);
	Job->Start();
}
