#pragma once

#include "Core/Module/ModuleInterface.h"

namespace Lumina
{
    // Puts the Hoard drawer in the editor: search a Hoard server and add assets to the project.
    class HOARDEDITOR_API FHoardEditorModule : public IModuleInterface
    {
    public:

        void StartupModule() override;
        void ShutdownModule() override;
    };
}
