using UnrealBuildTool;

public class HoardEditor : ModuleRules
{
	public HoardEditor(ReadOnlyTargetRules Target) : base(Target)
	{
		PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;
		PrivateDependencyModuleNames.AddRange(new[]
		{
			"AssetTools", "Core", "CoreUObject", "Engine", "HTTP", "ImageWrapper", "InputCore", "Json", "Slate", "SlateCore",
			"UnrealEd", "WorkspaceMenuStructure",
		});
	}
}
