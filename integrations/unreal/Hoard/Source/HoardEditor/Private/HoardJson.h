// JSON the way `hoard sync` writes it (serde_json), and small readers for the server's answers.
#pragma once

#include "CoreMinimal.h"
#include "Dom/JsonObject.h"
#include "Dom/JsonValue.h"

namespace HoardJson
{
	/** Parse JSON text, or null when it isn't JSON. */
	TSharedPtr<FJsonValue> Parse(const FString& Text);
	/** UTF-8 bytes as text. */
	FString Utf8(const TArray<uint8>& Bytes);
	/** Write a value as serde_json does: compact, or pretty with two-space indents and ": ". */
	FString Write(const TSharedPtr<FJsonValue>& Value, bool bPretty = false);
	/** A JSON (and TOML) string literal, escaped as serde_json escapes it. */
	FString Quote(const FString& Text);
	/** Percent-encoding for a URL's path segment or query value. */
	FString Escape(const FString& Text);

	FString Str(const TSharedPtr<FJsonObject>& Object, const TCHAR* Key);
	int64 Long(const TSharedPtr<FJsonObject>& Object, const TCHAR* Key);
	TArray<TSharedPtr<FJsonValue>> List(const TSharedPtr<FJsonObject>& Object, const TCHAR* Key);
	TSharedPtr<FJsonObject> Obj(const TSharedPtr<FJsonValue>& Value);
}
