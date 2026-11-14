// Talks to a Hoard server: searches, thumbnails, manifests, downloads and usage reports. A token that can only look is
// enough for all of it. Answers arrive on the game thread.
#pragma once

#include "CoreMinimal.h"
#include "Dom/JsonObject.h"
#include "Dom/JsonValue.h"

/** What came back: the parsed JSON (Data) or the raw bytes (Body), or why not (Error). */
struct FHoardReply
{
	bool bOk = false;
	int32 Code = 0;
	TSharedPtr<FJsonValue> Data;
	TArray<uint8> Body;
	FString Error;

	TSharedPtr<FJsonObject> Object() const;
};

using FHoardDone = TFunction<void(const FHoardReply&)>;

class FHoardClient
{
public:
	FString Server;
	FString Token;

	/** A path on the server, or a full URL as it is. */
	FString Url(const FString& Path) const;

	/** Send a request; with DownloadTo, stream the body into that file instead of memory. */
	void Send(const FString& Path, const FString& Verb, const FString& Payload, FHoardDone Done, const FString& DownloadTo = FString()) const;

	/** Send JSON (or nothing) and parse what comes back into Data. */
	void Json(const FString& Path, const FString& Verb, const TSharedPtr<FJsonValue>& Payload, FHoardDone Done) const;

	/** A page of search results: Data is {items, total}. */
	void Search(const FString& Query, const FString& Kind, int32 Offset, int32 Limit, FHoardDone Done) const;

	/** An asset's thumbnail, as PNG or JPEG bytes, or nothing. */
	void Thumbnail(const TSharedPtr<FJsonObject>& Asset, TFunction<void(const TArray<uint8>&)> Done) const;

private:
	static FString Why(int32 Code, const FString& Body);
};
