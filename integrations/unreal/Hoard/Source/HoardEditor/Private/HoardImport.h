// Turning what Hoard downloaded into Unreal assets, and thumbnails into pixels.
#pragma once

#include "CoreMinimal.h"

class FHoardProject;
struct FHoardPlaced;

namespace HoardImport
{
	/** The content folder a file under out_dir is imported to: /Game/Hoard, then its folders, each a valid name. */
	FString ContentPathFor(const FString& Rel);

	/** Import what was just downloaded that Unreal makes assets of: models (their materials and textures come along),
	 *  and pictures, sounds and fonts that stand alone. Returns the paths of the assets it made. */
	TArray<FString> Import(const FHoardProject& Project, const TArray<FHoardPlaced>& Downloaded);

	/** PNG or JPEG bytes as BGRA pixels. */
	bool Decode(const TArray<uint8>& Bytes, TArray<uint8>& OutBGRA, int32& OutWidth, int32& OutHeight);
}
