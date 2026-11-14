// Sends a project's own assets to Hoard without the editor's window, the way Send to Hoard does, for tests:
//   UnrealEditor-Cmd Game.uproject -run=HoardSend -unattended -nullrhi
// with HOARD_URL, HOARD_TOKEN, HOARD_EXPORT (packages and folders, a line each, like /Game/Props) and HOARD_PACK_NAME
// set. It prints RESULT {json}. With HOARD_IMPORT (files, a line each) it imports those into HOARD_IMPORT_TO instead,
// to give a test project assets of its own.
#pragma once

#include "Commandlets/Commandlet.h"

#include "HoardSendCommandlet.generated.h"

UCLASS()
class UHoardSendCommandlet : public UCommandlet
{
	GENERATED_BODY()

public:
	UHoardSendCommandlet();

	virtual int32 Main(const FString& Params) override;
};
