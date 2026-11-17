#include "HoardEditorModule.h"

#include "Core/Engine/Engine.h"
#include "Core/Module/ModuleManager.h"
#include "HoardBrowserTool.h"
#include "LuminaEditor.h"
#include "Tools/UI/ImGui/ImGuiModule.h"
#include "UI/EditorUI.h"

IMPLEMENT_MODULE(Lumina::FHoardEditorModule, "HoardEditor");
LUMINA_MODULE_IMGUI();

namespace Lumina
{
    void FHoardEditorModule::StartupModule()
    {
        // The editor tears its tools down before plugins unload, so the drawer never outlives this DLL.
        if (FEditorUI* EditorUI = Global::GEditorEngine != nullptr ? static_cast<FEditorUI*>(Global::GEditorEngine->GetDevelopmentToolsUI()) : nullptr)
        {
            EditorUI->CreateDrawerTool<FHoardBrowserTool>(LE_ICON_TREASURE_CHEST, "Hoard", 0, EditorUI);
        }
    }

    void FHoardEditorModule::ShutdownModule()
    {
    }
}
