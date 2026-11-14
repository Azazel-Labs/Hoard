using UnrealBuildTool;

public class HoardEditor : ModuleRules
{
	public HoardEditor(ReadOnlyTargetRules Target) : base(Target)
	{
		PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;
		PrivateDependencyModuleNames.AddRange(new[]
		{
			"AssetRegistry", "AssetTools", "ContentBrowser", "ContentBrowserData", "Core", "CoreUObject", "Engine", "HTTP", "ImageWrapper",
			"InputCore", "Json", "Slate", "SlateCore", "ToolMenus", "UnrealEd", "WorkspaceMenuStructure",
		});
	}
}
