#pragma once

#include "HoardCore.h"

#include <string>
#include <vector>

namespace Lumina::HoardImport
{
    /** Where the plugin imports to, under the project's content. */
    inline constexpr const char* ContentPath = "/Game/Content/Hoard";

    /** The content folder a file under out_dir imports into: its folders, under ContentPath. */
    std::string ContentPathFor(const std::string& Rel);

    struct FResult
    {
        /** The package path of every asset made or updated. */
        std::vector<std::string> Imported;
        /** "file: why" for each file that didn't import. */
        std::vector<std::string> Failed;
    };

    /** Import what was just downloaded that Lumina imports without questions: models, textures, sounds and fonts.
     *  A model's materials and textures come in with it. Main thread only. */
    FResult Import(const Hoard::Project& Project, const std::vector<Hoard::Placed>& Downloaded);
}
