using LuminaBuildTool.Configuration;

public class HoardEditor : LuminaModuleRules
{
    public HoardEditor(TargetInfo Target)
        : base(Target)
    {
        BinaryType = ModuleBinaryType.SharedLibrary;
        HostType = ModuleHostType.Editor;

        PublicIncludePaths.Add(".");

        PublicDependencyModuleNames.AddRange(new[] { "Runtime" });

        PrivateDependencyModuleNames.AddRange(new[]
        {
            "Editor",
            "ImGui",
            "RPMalloc",
        });

        // The core is plain C++ that also builds outside the editor, and the HTTP file pulls in the OS's headers,
        // so each keeps its own translation unit.
        bUseUnityBuild = false;

        // Lumina has no HTTP client: WinHTTP is part of Windows, and libcurl is on every Linux desktop.
        PublicSystemLibraries.Add(Target.Platform == BuildPlatform.Windows64 ? "winhttp" : "curl");
    }
}
