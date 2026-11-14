// The project side of adding assets: hoard.toml, hoard.lock and where files go, the same as `hoard sync` does it, so a
// later sync finds everything already in place.
#pragma once

#include "CoreMinimal.h"
#include "Dom/JsonObject.h"
#include "Dom/JsonValue.h"

class FHoardClient;

namespace HoardDefaults
{
	/** Where files go when hoard.toml doesn't say: beside Content, not in it, so Unreal never imports them by itself. */
	inline const TCHAR* OutDir = TEXT("Hoard");
	/** Where they're imported to. */
	inline const TCHAR* ContentPath = TEXT("/Game/Hoard");
}

/** Paths in hoard.lock are compared as `hoard sync` compares them: exactly, case and all. */
template <typename ValueType>
struct THoardPathKeyFuncs : TDefaultMapKeyFuncs<FString, ValueType, false>
{
	using typename TDefaultMapKeyFuncs<FString, ValueType, false>::KeyInitType;
	static bool Matches(KeyInitType A, KeyInitType B) { return A.Equals(B, ESearchCase::CaseSensitive); }
	static uint32 GetKeyHash(KeyInitType Key) { return FCrc::StrCrc32(*Key); }
};

struct FHoardLockEntry
{
	FString Id;
	FString Hash;
	int64 Size = 0;
};

using FHoardLock = TMap<FString, FHoardLockEntry, FDefaultSetAllocator, THoardPathKeyFuncs<FHoardLockEntry>>;

/** A manifest entry and where it goes under out_dir. */
struct FHoardPlaced
{
	FString Rel;
	TSharedPtr<FJsonObject> Entry;
};

/** What adding came to: how many files the assets have, which were downloaded, and whether the server heard what the
 *  project uses. */
struct FHoardAddResult
{
	bool bOk = false;
	FString Error;
	int32 Files = 0;
	bool bReported = false;
	FString OutDir;
	TArray<FHoardPlaced> Downloaded;
};

class FHoardProject
{
public:
	/** Root is the folder holding hoard.toml (the project's). */
	FHoardProject(const FString& InRoot, const FString& DefaultOutDir);

	FString Root;
	FString Server;
	FString Project;
	FString OutDir;
	FString Layout = TEXT("category");
	bool bSidecars = false;

	FString TomlPath() const { return Root / TEXT("hoard.toml"); }
	FString LockPath() const { return Root / TEXT("hoard.lock"); }
	FString OutPath() const;

	static FString Sanitize(const FString& Segment);
	static FString FileName(const TSharedPtr<FJsonObject>& Entry);
	FString FolderFor(const TSharedPtr<FJsonObject>& Entry) const;
	FString DestFor(const TSharedPtr<FJsonObject>& Entry) const;
	FString BundleDest(const TSharedPtr<FJsonObject>& BundleRoot, const FString& BundlePath) const;

	/** Where each file of a manifest goes under out_dir, as `hoard sync` places them. Taken is what hoard.lock already
	 *  holds, so a different asset at the same path gets a hash in its name instead. */
	TArray<FHoardPlaced> Place(const TArray<TSharedPtr<FJsonValue>>& Entries, const FHoardLock& Taken) const;

	FHoardLock ReadLock() const;
	void WriteLock(const FHoardLock& Files) const;

	/** Pin these assets in hoard.toml (making it if there's none), so `hoard sync` keeps them. */
	void Pin(const TArray<FString>& Ids, const TArray<FString>& Names, const FString& DefaultServer) const;

	/** Add assets to the project with what they need: resolve them, download what isn't here yet, pin them in hoard.toml,
	 *  update hoard.lock and tell the server what the project now uses. Progress gets (done, total) downloads. */
	void Add(const TSharedRef<FHoardClient>& Client, const TArray<TSharedPtr<FJsonObject>>& Assets,
		TFunction<void(int32, int32)> Progress, TFunction<void(const FHoardAddResult&)> Done) const;

private:
	void ReadConfig();
};
