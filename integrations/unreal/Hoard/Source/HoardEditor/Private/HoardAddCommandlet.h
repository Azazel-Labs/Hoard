// Adds assets without the editor's window, the way the Hoard tab does, for tests:
//   UnrealEditor-Cmd Game.uproject -run=HoardAdd -unattended -nullrhi
// with HOARD_URL, HOARD_TOKEN, HOARD_QUERY and HOARD_KIND (and HOARD_PROJECT for another folder than the project's) set.
// It searches, fetches and decodes the thumbnails, adds everything found, imports it, and prints RESULT {json}.
#pragma once

#include "Commandlets/Commandlet.h"

#include "HoardAddCommandlet.generated.h"

UCLASS()
class UHoardAddCommandlet : public UCommandlet
{
	GENERATED_BODY()

public:
	UHoardAddCommandlet();

	virtual int32 Main(const FString& Params) override;
};
