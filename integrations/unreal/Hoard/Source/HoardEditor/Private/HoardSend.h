// Send to Hoard in the editor: on the Content Browser's right-click menus for assets and folders, and the Hoard tab's
// Send… button. It works out what will go, shows it with a name for the pack, sends it, and links to the new pack.
#pragma once

#include "CoreMinimal.h"

class FHoardClient;

namespace HoardSend
{
	/** Put Send to Hoard… on the Content Browser's menus. */
	void Register();
	void Unregister();

	/** What's picked in the Content Browser: assets as packages (/Game/Props/Crate) and folders (/Game/Props). */
	TArray<FString> Selection();

	/** Work out what sending these means and ask before sending it. */
	void Open(const TArray<FString>& Picked);

	/** A client for the server and token in the editor's settings (or hoard.toml's server). */
	TSharedRef<FHoardClient> SettingsClient();
}
