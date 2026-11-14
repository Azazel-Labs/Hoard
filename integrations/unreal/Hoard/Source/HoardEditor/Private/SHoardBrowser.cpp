#include "SHoardBrowser.h"

#include "Brushes/SlateDynamicImageBrush.h"
#include "HoardClient.h"
#include "HoardImport.h"
#include "HoardJson.h"
#include "HoardProject.h"
#include "HoardSend.h"
#include "Misc/ConfigCacheIni.h"
#include "Misc/Paths.h"
#include "Styling/AppStyle.h"
#include "Widgets/Images/SImage.h"
#include "Widgets/Input/SButton.h"
#include "Widgets/Input/SComboBox.h"
#include "Widgets/Input/SEditableTextBox.h"
#include "Widgets/Input/SSearchBox.h"
#include "Widgets/Layout/SBorder.h"
#include "Widgets/Layout/SBox.h"
#include "Widgets/Layout/SGridPanel.h"
#include "Widgets/Layout/SScaleBox.h"
#include "Widgets/SBoxPanel.h"
#include "Widgets/SOverlay.h"
#include "Widgets/Text/STextBlock.h"
#include "Widgets/Views/STableRow.h"

#define LOCTEXT_NAMESPACE "Hoard"

static const TCHAR* Section = TEXT("Hoard");
static const TCHAR* KindValues[] = {TEXT(""), TEXT("texture"), TEXT("model"), TEXT("material"), TEXT("audio"), TEXT("animation"), TEXT("font"), TEXT("shader"), TEXT("data")};
static const TCHAR* KindNames[] = {TEXT("Anything"), TEXT("Textures"), TEXT("Models"), TEXT("Materials"), TEXT("Sounds"), TEXT("Animations"), TEXT("Fonts"), TEXT("Shaders"), TEXT("Data")};
static constexpr int32 Page = 40;

static FString Size(int64 Bytes)
{
	return FText::AsMemory(Bytes).ToString();
}

void SHoardBrowser::Construct(const FArguments& InArgs)
{
	Client = MakeShared<FHoardClient>();
	// The editor's own settings, not the project's: the token is yours, and the project is shared.
	GConfig->GetString(Section, TEXT("Server"), ServerField, GEditorSettingsIni);
	GConfig->GetString(Section, TEXT("Token"), TokenField, GEditorSettingsIni);
	for (const TCHAR* Name : KindNames)
	{
		KindLabels.Add(MakeShared<FString>(Name));
	}
	bSettings = ServerField.IsEmpty() && MakeProject().Server.IsEmpty();

	ChildSlot
	[
		SNew(SVerticalBox)
		+ SVerticalBox::Slot().AutoHeight().Padding(4)
		[
			SNew(SHorizontalBox)
			+ SHorizontalBox::Slot().FillWidth(1.f)
			[
				SNew(SSearchBox)
				.HintText(LOCTEXT("Hint", "Search Hoard"))
				.OnTextChanged_Lambda([this](const FText& Text)
				{
					Query = Text.ToString();
					SearchSoon();
				})
				.OnTextCommitted_Lambda([this](const FText& Text, ETextCommit::Type How)
				{
					Query = Text.ToString();
					if (How == ETextCommit::OnEnter)
					{
						Search(false);
					}
				})
			]
			+ SHorizontalBox::Slot().AutoWidth().Padding(4, 0, 0, 0)
			[
				SNew(SComboBox<TSharedPtr<FString>>)
				.OptionsSource(&KindLabels)
				.InitiallySelectedItem(KindLabels[0])
				.OnGenerateWidget_Lambda([](TSharedPtr<FString> Label)
				{
					return SNew(STextBlock).Text(FText::FromString(*Label));
				})
				.OnSelectionChanged_Lambda([this](TSharedPtr<FString> Label, ESelectInfo::Type)
				{
					Kind = FMath::Max(0, KindLabels.IndexOfByKey(Label));
					Search(false);
				})
				[
					SNew(STextBlock).Text_Lambda([this] { return FText::FromString(*KindLabels[Kind]); })
				]
			]
			+ SHorizontalBox::Slot().AutoWidth().Padding(4, 0, 0, 0)
			[
				SNew(SButton)
				.ToolTipText(LOCTEXT("SettingsTip", "Server and token"))
				.OnClicked_Lambda([this]
				{
					bSettings = !bSettings;
					return FReply::Handled();
				})
				[
					SNew(SImage).Image(FAppStyle::GetBrush("Icons.Settings")).ColorAndOpacity(FSlateColor::UseForeground())
				]
			]
		]
		+ SVerticalBox::Slot().AutoHeight().Padding(4, 0, 4, 4)
		[
			SNew(SBorder)
			.BorderImage(FAppStyle::GetBrush("ToolPanel.GroupBorder"))
			.Padding(8)
			.Visibility_Lambda([this] { return bSettings ? EVisibility::Visible : EVisibility::Collapsed; })
			[
				SNew(SGridPanel).FillColumn(1, 1.f)
				+ SGridPanel::Slot(0, 0).VAlign(VAlign_Center).Padding(0, 0, 8, 4)
				[
					SNew(STextBlock).Text(LOCTEXT("Server", "Server"))
				]
				+ SGridPanel::Slot(1, 0).Padding(0, 0, 0, 4)
				[
					SNew(SEditableTextBox)
					.Text_Lambda([this] { return FText::FromString(ServerField); })
					.HintText_Lambda([this]
					{
						const FString FromToml = MakeProject().Server;
						return FromToml.IsEmpty() ? LOCTEXT("ServerHint", "http://hoard.local:4627") : FText::FromString(FromToml);
					})
					.OnTextChanged_Lambda([this](const FText& T) { ServerField = T.ToString(); })
				]
				+ SGridPanel::Slot(0, 1).VAlign(VAlign_Center).Padding(0, 0, 8, 4)
				[
					SNew(STextBlock).Text(LOCTEXT("Token", "Token"))
				]
				+ SGridPanel::Slot(1, 1).Padding(0, 0, 0, 4)
				[
					SNew(SEditableTextBox)
					.IsPassword(true)
					.HintText(LOCTEXT("TokenHint", "One that can look is enough to add; sending needs an editor's"))
					.Text_Lambda([this] { return FText::FromString(TokenField); })
					.OnTextChanged_Lambda([this](const FText& T) { TokenField = T.ToString(); })
				]
				+ SGridPanel::Slot(1, 2).HAlign(HAlign_Right)
				[
					SNew(SButton)
					.Text(LOCTEXT("Save", "Save and search"))
					.OnClicked_Lambda([this]
					{
						SaveSettings();
						bSettings = false;
						Search(false);
						return FReply::Handled();
					})
				]
			]
		]
		+ SVerticalBox::Slot().FillHeight(1.f).Padding(4, 0)
		[
			SAssignNew(Tiles, STileView<FItem>)
			.ListItemsSource(&Items)
			.OnGenerateTile(this, &SHoardBrowser::MakeTile)
			.ItemWidth(116)
			.ItemHeight(148)
			.SelectionMode(ESelectionMode::Multi)
			.OnMouseButtonDoubleClick_Lambda([this](FItem)
			{
				AddSelected();
			})
		]
		+ SVerticalBox::Slot().AutoHeight().Padding(4)
		[
			SNew(SHorizontalBox)
			+ SHorizontalBox::Slot().FillWidth(1.f).VAlign(VAlign_Center)
			[
				SNew(STextBlock)
				.Text_Lambda([this] { return Status; })
				.AutoWrapText(true)
			]
			+ SHorizontalBox::Slot().AutoWidth().Padding(4, 0, 0, 0)
			[
				SNew(SButton)
				.Text(LOCTEXT("SendButton", "Send..."))
				.ToolTipText(LOCTEXT("SendButtonTip", "Send what's picked in the Content Browser to Hoard as a new pack (it needs an editor's token)."))
				.OnClicked_Lambda([this]
				{
					SaveSettings();
					HoardSend::Open(HoardSend::Selection());
					return FReply::Handled();
				})
			]
			+ SHorizontalBox::Slot().AutoWidth().Padding(4, 0, 0, 0)
			[
				SNew(SButton)
				.Text(LOCTEXT("More", "More"))
				.Visibility_Lambda([this] { return Items.Num() < Total ? EVisibility::Visible : EVisibility::Collapsed; })
				.IsEnabled_Lambda([this] { return !bBusy; })
				.OnClicked_Lambda([this]
				{
					Search(true);
					return FReply::Handled();
				})
			]
			+ SHorizontalBox::Slot().AutoWidth().Padding(4, 0, 0, 0)
			[
				SNew(SButton)
				.ButtonStyle(FAppStyle::Get(), "PrimaryButton")
				.Text_Lambda([this]
				{
					const int32 N = Tiles.IsValid() ? Tiles->GetNumItemsSelected() : 0;
					return N > 1 ? FText::Format(LOCTEXT("AddN", "Add {0} to project"), N) : LOCTEXT("Add", "Add to project");
				})
				.IsEnabled_Lambda([this] { return !bBusy && Tiles.IsValid() && Tiles->GetNumItemsSelected() > 0; })
				.OnClicked_Lambda([this]
				{
					AddSelected();
					return FReply::Handled();
				})
			]
		]
	];

	if (!bSettings)
	{
		RegisterActiveTimer(0.f, FWidgetActiveTimerDelegate::CreateSP(this, &SHoardBrowser::SearchNow));
	}
	else
	{
		Status = LOCTEXT("NoServer", "Set the server's address (and a token, if it wants one) to search.");
	}
}

FHoardProject SHoardBrowser::MakeProject() const
{
	return FHoardProject(FPaths::ConvertRelativePathToFull(FPaths::ProjectDir()), HoardDefaults::OutDir);
}

void SHoardBrowser::SaveSettings()
{
	GConfig->SetString(Section, TEXT("Server"), *ServerField.TrimStartAndEnd(), GEditorSettingsIni);
	GConfig->SetString(Section, TEXT("Token"), *TokenField.TrimStartAndEnd(), GEditorSettingsIni);
	GConfig->Flush(false, GEditorSettingsIni);
}

void SHoardBrowser::SearchSoon()
{
	if (Pending.IsValid())
	{
		UnRegisterActiveTimer(Pending.ToSharedRef());
	}
	Pending = RegisterActiveTimer(0.35f, FWidgetActiveTimerDelegate::CreateSP(this, &SHoardBrowser::SearchNow));
}

EActiveTimerReturnType SHoardBrowser::SearchNow(double, float)
{
	Pending.Reset();
	Search(false);
	return EActiveTimerReturnType::Stop;
}

void SHoardBrowser::Search(bool bMore)
{
	if (Pending.IsValid())
	{
		UnRegisterActiveTimer(Pending.ToSharedRef());
		Pending.Reset();
	}
	const FHoardProject Project = MakeProject();
	Client->Server = ServerField.TrimStartAndEnd().IsEmpty() ? Project.Server : ServerField.TrimStartAndEnd();
	Client->Token = TokenField;
	const int32 Gen = bMore ? Generation : ++Generation;
	if (!bMore)
	{
		Items.Reset();
		Total = 0;
		Tiles->ClearSelection();
		Tiles->RequestListRefresh();
	}
	bBusy = true;
	Status = LOCTEXT("Searching", "Searching...");
	const TWeakPtr<SHoardBrowser> Weak = SharedThis(this);
	Client->Search(Query, KindValues[Kind], Items.Num(), Page, [Weak, Gen](const FHoardReply& R)
	{
		const TSharedPtr<SHoardBrowser> Self = Weak.Pin();
		if (!Self.IsValid() || Gen != Self->Generation)
		{
			return;
		}
		Self->bBusy = false;
		if (!R.bOk)
		{
			Self->Status = FText::FromString(R.Error);
			return;
		}
		Self->Total = static_cast<int32>(HoardJson::Long(R.Object(), TEXT("total")));
		for (const TSharedPtr<FJsonValue>& V : HoardJson::List(R.Object(), TEXT("items")))
		{
			const TSharedPtr<FJsonObject> Asset = HoardJson::Obj(V);
			if (!Asset.IsValid())
			{
				continue;
			}
			const FItem Item = MakeShared<FHoardTile>();
			Item->Asset = Asset;
			Self->Items.Add(Item);
			Self->Client->Thumbnail(Asset, [Weak, Gen, Item](const TArray<uint8>& Bytes)
			{
				const TSharedPtr<SHoardBrowser> Still = Weak.Pin();
				TArray<uint8> Pixels;
				int32 W = 0, H = 0;
				if (!Still.IsValid() || Gen != Still->Generation || !HoardImport::Decode(Bytes, Pixels, W, H))
				{
					return;
				}
				static int32 Made = 0;
				Item->Thumb = FSlateDynamicImageBrush::CreateWithImageData(FName(*FString::Printf(TEXT("Hoard.Thumb.%d"), ++Made)), FVector2D(W, H), Pixels);
			});
		}
		Self->Tiles->RequestListRefresh();
		Self->Status = Self->Total == 0
			? LOCTEXT("Nothing", "Nothing found.")
			: FText::Format(LOCTEXT("Count", "{0} of {1}. Double-click, or pick some and Add to project."), Self->Items.Num(), Self->Total);
	});
}

TSharedRef<ITableRow> SHoardBrowser::MakeTile(FItem Item, const TSharedRef<STableViewBase>& Owner)
{
	const TSharedPtr<FJsonObject>& A = Item->Asset;
	const FString Category = HoardJson::Str(A, TEXT("category"));
	const FString AssetKind = HoardJson::Str(A, TEXT("kind"));
	const FString Tip = FString::Printf(TEXT("%s\n%s · %s · %s"), *HoardJson::Str(A, TEXT("name")), *HoardJson::Str(A, TEXT("pack_name")),
		Category.IsEmpty() ? *AssetKind : *Category, *Size(HoardJson::Long(A, TEXT("size"))));
	return SNew(STableRow<FItem>, Owner)
		.Padding(4)
		.ToolTipText(FText::FromString(Tip))
		[
			SNew(SVerticalBox)
			+ SVerticalBox::Slot().AutoHeight().HAlign(HAlign_Center)
			[
				SNew(SBox)
				.WidthOverride(100)
				.HeightOverride(100)
				[
					SNew(SOverlay)
					+ SOverlay::Slot().HAlign(HAlign_Center).VAlign(VAlign_Center)
					[
						SNew(STextBlock)
						.Text(FText::FromString(AssetKind))
						.ColorAndOpacity(FSlateColor::UseSubduedForeground())
						.Visibility_Lambda([Item] { return Item->Thumb.IsValid() ? EVisibility::Collapsed : EVisibility::Visible; })
					]
					+ SOverlay::Slot()
					[
						SNew(SScaleBox)
						.Stretch(EStretch::ScaleToFit)
						.Visibility_Lambda([Item] { return Item->Thumb.IsValid() ? EVisibility::Visible : EVisibility::Collapsed; })
						[
							SNew(SImage).Image_Lambda([Item]() -> const FSlateBrush* { return Item->Thumb.Get(); })
						]
					]
				]
			]
			+ SVerticalBox::Slot().AutoHeight().Padding(0, 4, 0, 0)
			[
				SNew(STextBlock)
				.Text(FText::FromString(HoardJson::Str(A, TEXT("name"))))
				.Justification(ETextJustify::Center)
				.OverflowPolicy(ETextOverflowPolicy::Ellipsis)
			]
		];
}

void SHoardBrowser::AddSelected()
{
	const TArray<FItem> Picked = Tiles->GetSelectedItems();
	if (Picked.Num() == 0 || bBusy)
	{
		return;
	}
	TArray<TSharedPtr<FJsonObject>> Assets;
	for (const FItem& I : Picked)
	{
		Assets.Add(I->Asset);
	}
	const FHoardProject Project = MakeProject();
	bBusy = true;
	Status = LOCTEXT("Adding", "Working out what they need...");
	const TWeakPtr<SHoardBrowser> Weak = SharedThis(this);
	Project.Add(Client.ToSharedRef(), Assets,
		[Weak](int32 Done, int32 Of)
		{
			if (const TSharedPtr<SHoardBrowser> Self = Weak.Pin())
			{
				Self->Status = FText::Format(LOCTEXT("Downloading", "Downloaded {0} of {1}..."), Done, Of);
			}
		},
		[Weak, Project](const FHoardAddResult& R)
		{
			// Imported even if the tab was closed meanwhile: the files are in the project either way.
			const TArray<FString> Made = R.bOk ? HoardImport::Import(Project, R.Downloaded) : TArray<FString>();
			const TSharedPtr<SHoardBrowser> Self = Weak.Pin();
			if (!Self.IsValid())
			{
				return;
			}
			Self->bBusy = false;
			if (!R.bOk)
			{
				Self->Status = FText::FromString(R.Error);
				return;
			}
			FString Text = FString::Printf(TEXT("Added %d %s to %s (%d downloaded), imported %d into %s."), R.Files, R.Files == 1 ? TEXT("file") : TEXT("files"),
				*R.OutDir, R.Downloaded.Num(), Made.Num(), HoardDefaults::ContentPath);
			if (!R.bReported)
			{
				Text += TEXT(" The server didn't hear what the project uses; the next hoard sync will tell it.");
			}
			Self->Status = FText::FromString(Text);
		});
}

#undef LOCTEXT_NAMESPACE
