// The Hoard tab, under Window.
#include "Framework/Application/SlateApplication.h"
#include "Framework/Docking/TabManager.h"
#include "Misc/CoreMisc.h"
#include "Modules/ModuleManager.h"
#include "SHoardBrowser.h"
#include "Styling/AppStyle.h"
#include "Widgets/Docking/SDockTab.h"
#include "WorkspaceMenuStructure.h"
#include "WorkspaceMenuStructureModule.h"

#define LOCTEXT_NAMESPACE "Hoard"

static const FName HoardTab(TEXT("Hoard"));

class FHoardEditorModule : public IModuleInterface
{
public:
	virtual void StartupModule() override
	{
		if (IsRunningCommandlet())
		{
			return;
		}
		FGlobalTabmanager::Get()
			->RegisterNomadTabSpawner(HoardTab, FOnSpawnTab::CreateLambda([](const FSpawnTabArgs&)
			{
				return SNew(SDockTab).TabRole(ETabRole::NomadTab)[SNew(SHoardBrowser)];
			}))
			.SetDisplayName(LOCTEXT("TabTitle", "Hoard"))
			.SetTooltipText(LOCTEXT("TabTip", "Search your Hoard library and add assets, with what they need, to this project."))
			.SetGroup(WorkspaceMenu::GetMenuStructure().GetLevelEditorCategory())
			.SetIcon(FSlateIcon(FAppStyle::GetAppStyleSetName(), "ContentBrowser.TabIcon"));
	}

	virtual void ShutdownModule() override
	{
		if (FSlateApplication::IsInitialized())
		{
			FGlobalTabmanager::Get()->UnregisterNomadTabSpawner(HoardTab);
		}
	}
};

IMPLEMENT_MODULE(FHoardEditorModule, HoardEditor)

#undef LOCTEXT_NAMESPACE
