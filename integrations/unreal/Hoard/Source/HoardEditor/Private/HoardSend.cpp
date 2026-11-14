#include "HoardSend.h"

#include "ContentBrowserDataSubsystem.h"
#include "ContentBrowserMenuContexts.h"
#include "ContentBrowserModule.h"
#include "Framework/Application/SlateApplication.h"
#include "Framework/Notifications/NotificationManager.h"
#include "HAL/PlatformProcess.h"
#include "HoardClient.h"
#include "HoardExport.h"
#include "HoardProject.h"
#include "IContentBrowserDataModule.h"
#include "IContentBrowserSingleton.h"
#include "Misc/ConfigCacheIni.h"
#include "Misc/PackageName.h"
#include "Misc/Paths.h"
#include "Misc/ScopedSlowTask.h"
#include "Styling/AppStyle.h"
#include "ToolMenus.h"
#include "Widgets/Input/SButton.h"
#include "Widgets/Input/SEditableTextBox.h"
#include "Widgets/Layout/SScrollBox.h"
#include "Widgets/Notifications/SNotificationList.h"
#include "Widgets/SBoxPanel.h"
#include "Widgets/SWindow.h"
#include "Widgets/Text/STextBlock.h"

#define LOCTEXT_NAMESPACE "Hoard"

namespace HoardSend
{
	static FDelegateHandle Startup;

	static FHoardProject Project()
	{
		return FHoardProject(FPaths::ConvertRelativePathToFull(FPaths::ProjectDir()), HoardDefaults::OutDir);
	}

	TSharedRef<FHoardClient> SettingsClient()
	{
		// Where the Hoard tab keeps them: the editor's own settings, not the project's.
		FString Server, Token;
		GConfig->GetString(TEXT("Hoard"), TEXT("Server"), Server, GEditorSettingsIni);
		GConfig->GetString(TEXT("Hoard"), TEXT("Token"), Token, GEditorSettingsIni);
		const TSharedRef<FHoardClient> Client = MakeShared<FHoardClient>();
		Client->Server = Server.TrimStartAndEnd().IsEmpty() ? Project().Server : Server.TrimStartAndEnd();
		Client->Token = Token;
		return Client;
	}

	static void Notify(const FText& Text, bool bOk, const FString& Link = FString())
	{
		FNotificationInfo Info(Text);
		Info.ExpireDuration = bOk ? 8.f : 12.f;
		Info.bUseSuccessFailIcons = true;
		Info.bFireAndForget = true;
		if (!Link.IsEmpty())
		{
			Info.Hyperlink = FSimpleDelegate::CreateLambda([Link] { FPlatformProcess::LaunchURL(*Link, nullptr, nullptr); });
			Info.HyperlinkText = LOCTEXT("OpenPack", "Open it in Hoard");
		}
		if (const TSharedPtr<SNotificationItem> Item = FSlateNotificationManager::Get().AddNotification(Info))
		{
			Item->SetCompletionState(bOk ? SNotificationItem::CS_Success : SNotificationItem::CS_Fail);
		}
	}

	TArray<FString> Selection()
	{
		TArray<FString> Out;
		IContentBrowserSingleton& Browser = FModuleManager::LoadModuleChecked<FContentBrowserModule>("ContentBrowser").Get();
		TArray<FAssetData> Assets;
		Browser.GetSelectedAssets(Assets);
		for (const FAssetData& A : Assets)
		{
			Out.AddUnique(A.PackageName.ToString());
		}
		TArray<FString> Folders;
		Browser.GetSelectedFolders(Folders);
		if (Folders.Num() == 0 && Assets.Num() == 0)
		{
			Browser.GetSelectedPathViewFolders(Folders);
		}
		const UContentBrowserDataSubsystem* Data = IContentBrowserDataModule::Get().GetSubsystem();
		for (const FString& Virtual : Folders)
		{
			FString Internal;
			if (Data && Data->TryConvertVirtualPath(Virtual, Internal) == EContentBrowserPathType::Internal)
			{
				Out.AddUnique(Internal);
			}
		}
		return Out;
	}

	static void Send(const TSharedRef<FHoardExport>& Export, const FString& PackName)
	{
		Notify(FText::Format(LOCTEXT("Sending", "Sending {0} to Hoard..."), FText::FromString(Export->Summary())), true);
		Export->Send(SettingsClient(), PackName, [Export](const FHoardSendResult& R)
		{
			if (R.bOk)
			{
				Notify(FText::Format(LOCTEXT("Sent", "Sent {0} to Hoard as {1}."), R.Sent.Num(), FText::FromString(R.Name)), true, R.Link);
			}
			else
			{
				Notify(FText::FromString(R.Error), false);
			}
		});
	}

	void Open(const TArray<FString>& Picked)
	{
		if (Picked.Num() == 0)
		{
			Notify(LOCTEXT("PickFirst", "Pick assets or folders in the Content Browser to send."), false);
			return;
		}
		const TSharedRef<FHoardExport> Export = MakeShared<FHoardExport>(Project());
		{
			FScopedSlowTask Working(0.f, LOCTEXT("Working", "Working out what to send..."));
			Working.MakeDialogDelayed(0.5f);
			Export->Plan(Picked);
		}
		if (Export->Files.Num() == 0)
		{
			Notify(FText::FromString(FString::Printf(TEXT("Nothing here to send: %s."), *Export->WhyNothing())), false);
			return;
		}
		const TSharedRef<FString> PackName = MakeShared<FString>(FString::Printf(TEXT("%s: %s"), *Export->Project.Project, *FPaths::GetBaseFilename(Picked[0])));
		FString Listing = Export->Summary() + TEXT(":");
		for (int32 I = 0; I < Export->Files.Num() && I < 12; ++I)
		{
			const FHoardExportFile& F = Export->Files[I];
			Listing += TEXT("\n  ") + F.Rel + (F.IsCopy() ? TEXT("  (a copy)") : TEXT(""));
		}
		if (Export->Files.Num() > 12)
		{
			Listing += FString::Printf(TEXT("\n  and %d more"), Export->Files.Num() - 12);
		}

		const TSharedRef<SWindow> Window = SNew(SWindow)
			.Title(LOCTEXT("SendTitle", "Send to Hoard"))
			.SizingRule(ESizingRule::UserSized)
			.ClientSize(FVector2D(520, 380))
			.SupportsMinimize(false)
			.SupportsMaximize(false);
		const TWeakPtr<SWindow> WeakWindow = Window;
		Window->SetContent(
			SNew(SVerticalBox)
			+ SVerticalBox::Slot().AutoHeight().Padding(12, 12, 12, 4)
			[
				SNew(STextBlock).Text(LOCTEXT("PackName", "Pack name"))
			]
			+ SVerticalBox::Slot().AutoHeight().Padding(12, 0, 12, 8)
			[
				SNew(SEditableTextBox)
				.Text(FText::FromString(*PackName))
				.OnTextChanged_Lambda([PackName](const FText& T) { *PackName = T.ToString(); })
			]
			+ SVerticalBox::Slot().FillHeight(1.f).Padding(12, 0)
			[
				SNew(SScrollBox)
				+ SScrollBox::Slot()
				[
					SNew(STextBlock).Text(FText::FromString(Listing)).AutoWrapText(true)
				]
			]
			+ SVerticalBox::Slot().AutoHeight().Padding(12, 8, 12, 4)
			[
				SNew(STextBlock)
				.Text(LOCTEXT("SendNote", "Only what other tools open goes: the files assets were imported from, or copies of meshes, textures and sounds. Sending needs an editor's token."))
				.ColorAndOpacity(FSlateColor::UseSubduedForeground())
				.AutoWrapText(true)
			]
			+ SVerticalBox::Slot().AutoHeight().HAlign(HAlign_Right).Padding(12, 4, 12, 12)
			[
				SNew(SHorizontalBox)
				+ SHorizontalBox::Slot().AutoWidth()
				[
					SNew(SButton)
					.Text(LOCTEXT("Cancel", "Cancel"))
					.OnClicked_Lambda([WeakWindow]
					{
						if (const TSharedPtr<SWindow> W = WeakWindow.Pin())
						{
							W->RequestDestroyWindow();
						}
						return FReply::Handled();
					})
				]
				+ SHorizontalBox::Slot().AutoWidth().Padding(8, 0, 0, 0)
				[
					SNew(SButton)
					.ButtonStyle(FAppStyle::Get(), "PrimaryButton")
					.Text(LOCTEXT("Send", "Send"))
					.OnClicked_Lambda([WeakWindow, Export, PackName]
					{
						if (const TSharedPtr<SWindow> W = WeakWindow.Pin())
						{
							W->RequestDestroyWindow();
						}
						Send(Export, *PackName);
						return FReply::Handled();
					})
				]
			]);
		FSlateApplication::Get().AddWindow(Window);
	}

	static void Extend()
	{
		UToolMenus* Menus = UToolMenus::Get();
		const FSlateIcon Icon(FAppStyle::GetAppStyleSetName(), "Icons.Export");
		if (UToolMenu* Menu = Menus->ExtendMenu("ContentBrowser.AssetContextMenu"))
		{
			Menu->FindOrAddSection("CommonAssetActions").AddMenuEntry("HoardSend", LOCTEXT("SendAssets", "Send to Hoard..."),
				LOCTEXT("SendAssetsTip", "Send these, and what they use, to Hoard as a new pack: the files they were imported from, or copies other tools open."),
				Icon, FToolUIAction(FToolMenuExecuteAction::CreateLambda([](const FToolMenuContext& Context)
				{
					TArray<FString> Picked;
					if (const UContentBrowserAssetContextMenuContext* Assets = Context.FindContext<UContentBrowserAssetContextMenuContext>())
					{
						for (const FAssetData& A : Assets->SelectedAssets)
						{
							Picked.AddUnique(A.PackageName.ToString());
						}
					}
					Open(Picked);
				})));
		}
		if (UToolMenu* Menu = Menus->ExtendMenu("ContentBrowser.FolderContextMenu"))
		{
			Menu->FindOrAddSection("PathViewFolderOptions").AddMenuEntry("HoardSend", LOCTEXT("SendFolders", "Send to Hoard..."),
				LOCTEXT("SendFoldersTip", "Send what's in these folders, and what it uses, to Hoard as a new pack."),
				Icon, FToolUIAction(FToolMenuExecuteAction::CreateLambda([](const FToolMenuContext& Context)
				{
					TArray<FString> Picked;
					if (const UContentBrowserFolderContext* Folders = Context.FindContext<UContentBrowserFolderContext>())
					{
						Picked = Folders->GetSelectedPackagePaths();
					}
					Open(Picked);
				})));
		}
	}

	void Register()
	{
		Startup = UToolMenus::RegisterStartupCallback(FSimpleMulticastDelegate::FDelegate::CreateStatic(&Extend));
	}

	void Unregister()
	{
		UToolMenus::UnRegisterStartupCallback(Startup);
		if (UToolMenus* Menus = UToolMenus::TryGet())
		{
			Menus->RemoveEntry("ContentBrowser.AssetContextMenu", "CommonAssetActions", "HoardSend");
			Menus->RemoveEntry("ContentBrowser.FolderContextMenu", "PathViewFolderOptions", "HoardSend");
		}
	}
}

#undef LOCTEXT_NAMESPACE
