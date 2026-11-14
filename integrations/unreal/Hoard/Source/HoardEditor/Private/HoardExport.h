// Sending a project's own assets to Hoard as a new pack. Only what other tools can open goes: the file an asset was
// imported from (with the files a model names, like an OBJ's materials and their pictures), or, for a mesh, texture or
// sound with no such file, a copy as FBX, PNG or WAV. Blueprints, materials, levels and whatever else only Unreal reads
// aren't sent, but what they use is. Nothing that came from Hoard goes back.
#pragma once

#include "CoreMinimal.h"
#include "HoardProject.h"

class FHoardClient;

/** One file of the pack: a file on disk (Path), or a copy made for sending (Bytes). */
struct FHoardExportFile
{
	/** Where it goes in the pack. */
	FString Rel;
	FString Path;
	TArray<uint8> Bytes;
	/** The package it stands for, like /Game/Props/Crate. */
	FString From;
	bool IsCopy() const { return Path.IsEmpty(); }
};

struct FHoardSendResult
{
	bool bOk = false;
	FString Error;
	FString Pack;
	FString Name;
	FString Link;
	TArray<FString> Sent;
};

class FHoardExport
{
public:
	explicit FHoardExport(const FHoardProject& InProject);

	FHoardProject Project;
	/** What will go, in the order found. */
	TArray<FHoardExportFile> Files;
	/** Left out because Hoard gave them to this project. */
	TArray<FString> FromHoard;
	/** Left out because only Unreal opens them (what they use was still looked at). */
	TArray<FString> EngineOnly;

	/** Work out what sending these packages (/Game/Props/Crate) and folders (/Game/Props) means, with everything they
	 *  use. Call once, then look at Files before Send. */
	void Plan(const TArray<FString>& Picked);

	/** Upload Files as a new pack called PackName, in one request. */
	void Send(const TSharedRef<FHoardClient>& Client, const FString& PackName, TFunction<void(const FHoardSendResult&)> Done) const;

	FString WhyNothing() const;
	/** "3 files; left out: 2 that came from Hoard, 1 only Unreal opens". */
	FString Summary() const;

	/** The files a model names that it can't be opened without: an OBJ's material libraries and their pictures, a
	 *  glTF's buffers and images. Full paths, only those that exist. */
	static TArray<FString> Companions(const FString& Path);

private:
	void Visit(const FName& Package, bool bWithCompanions);
	void Add(const FString& Rel, const FString& Path, TArray<uint8>&& Bytes, const FString& From);
	bool IsFromHoard(const FString& File) const;

	TSet<FName> Seen;
	TSet<FString> Rels;
	TSet<FString> Paths;
	FString OutAbs;
};
