// The Hoard tab: search the library with thumbnails, pick, and Add to project, which brings what's picked with what it
// needs to where `hoard sync` would put it, pins it in hoard.toml, merges hoard.lock, tells the server, and imports it.
#pragma once

#include "CoreMinimal.h"
#include "Widgets/SCompoundWidget.h"
#include "Widgets/Views/STileView.h"

class FHoardClient;
class FHoardProject;
class FJsonObject;
struct FSlateDynamicImageBrush;

struct FHoardTile
{
	TSharedPtr<FJsonObject> Asset;
	TSharedPtr<FSlateDynamicImageBrush> Thumb;
};

class SHoardBrowser : public SCompoundWidget
{
public:
	SLATE_BEGIN_ARGS(SHoardBrowser) {}
	SLATE_END_ARGS()

	void Construct(const FArguments& InArgs);

private:
	using FItem = TSharedPtr<FHoardTile>;

	TSharedRef<ITableRow> MakeTile(FItem Item, const TSharedRef<STableViewBase>& Owner);
	FHoardProject MakeProject() const;
	void SearchSoon();
	EActiveTimerReturnType SearchNow(double, float);
	void Search(bool bMore);
	void AddSelected();
	void SaveSettings();

	TSharedPtr<FHoardClient> Client;
	TArray<FItem> Items;
	TSharedPtr<STileView<FItem>> Tiles;
	TArray<TSharedPtr<FString>> KindLabels;
	int32 Kind = 0;
	FString Query;
	int32 Total = 0;
	int32 Generation = 0;
	bool bBusy = false;
	bool bSettings = false;
	FText Status;
	FString ServerField;
	FString TokenField;
	TSharedPtr<FActiveTimerHandle> Pending;
};
