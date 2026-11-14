#include "HoardJson.h"

#include "Serialization/JsonReader.h"
#include "Serialization/JsonSerializer.h"

namespace HoardJson
{
	TSharedPtr<FJsonValue> Parse(const FString& Text)
	{
		TSharedPtr<FJsonValue> Out;
		if (!FJsonSerializer::Deserialize(TJsonReaderFactory<>::Create(Text), Out))
		{
			return nullptr;
		}
		return Out;
	}

	FString Utf8(const TArray<uint8>& Bytes)
	{
		const FUTF8ToTCHAR Text(reinterpret_cast<const ANSICHAR*>(Bytes.GetData()), Bytes.Num());
		return FString::ConstructFromPtrSize(Text.Get(), Text.Length());
	}

	FString Quote(const FString& Text)
	{
		FString Out = TEXT("\"");
		for (const TCHAR C : Text)
		{
			switch (C)
			{
			case TEXT('"'): Out += TEXT("\\\""); break;
			case TEXT('\\'): Out += TEXT("\\\\"); break;
			case TEXT('\n'): Out += TEXT("\\n"); break;
			case TEXT('\r'): Out += TEXT("\\r"); break;
			case TEXT('\t'): Out += TEXT("\\t"); break;
			case 0x08: Out += TEXT("\\b"); break;
			case 0x0C: Out += TEXT("\\f"); break;
			default:
				if (C < 0x20)
				{
					Out += FString::Printf(TEXT("\\u%04x"), static_cast<uint32>(C));
				}
				else
				{
					Out.AppendChar(C);
				}
			}
		}
		Out += TEXT("\"");
		return Out;
	}

	FString Escape(const FString& Text)
	{
		const FTCHARToUTF8 Bytes(*Text);
		FString Out;
		for (int32 I = 0; I < Bytes.Length(); ++I)
		{
			const uint8 B = static_cast<uint8>(Bytes.Get()[I]);
			if ((B >= 'A' && B <= 'Z') || (B >= 'a' && B <= 'z') || (B >= '0' && B <= '9') || B == '-' || B == '_' || B == '.' || B == '~')
			{
				Out.AppendChar(static_cast<TCHAR>(B));
			}
			else
			{
				Out += FString::Printf(TEXT("%%%02X"), B);
			}
		}
		return Out;
	}

	/** The shortest form that reads back as the same number, like serde_json's; whole numbers without a fraction. */
	static FString Number(double D)
	{
		if (!FMath::IsFinite(D))
		{
			return TEXT("null");
		}
		if (D == FMath::FloorToDouble(D) && FMath::Abs(D) < 9007199254740992.0)
		{
			return FString::Printf(TEXT("%lld"), static_cast<long long>(D));
		}
		for (int32 Digits = 15; Digits < 17; ++Digits)
		{
			const FString S = FString::Printf(TEXT("%.*g"), Digits, D);
			if (FCString::Atod(*S) == D)
			{
				return S;
			}
		}
		return FString::Printf(TEXT("%.17g"), D);
	}

	static void Indent(FString& Out, bool bPretty, int32 Depth)
	{
		if (bPretty)
		{
			Out += TEXT("\n");
			for (int32 I = 0; I < Depth; ++I)
			{
				Out += TEXT("  ");
			}
		}
	}

	static void WriteValue(FString& Out, const TSharedPtr<FJsonValue>& Value, bool bPretty, int32 Depth)
	{
		if (!Value.IsValid())
		{
			Out += TEXT("null");
			return;
		}
		switch (Value->Type)
		{
		case EJson::Boolean:
			Out += Value->AsBool() ? TEXT("true") : TEXT("false");
			break;
		case EJson::Number:
			Out += Number(Value->AsNumber());
			break;
		case EJson::String:
			Out += Quote(Value->AsString());
			break;
		case EJson::Array:
		{
			const TArray<TSharedPtr<FJsonValue>>& Items = Value->AsArray();
			Out += TEXT("[");
			for (int32 I = 0; I < Items.Num(); ++I)
			{
				Out += I > 0 ? TEXT(",") : TEXT("");
				Indent(Out, bPretty, Depth + 1);
				WriteValue(Out, Items[I], bPretty, Depth + 1);
			}
			if (Items.Num() > 0)
			{
				Indent(Out, bPretty, Depth);
			}
			Out += TEXT("]");
			break;
		}
		case EJson::Object:
		{
			const TSharedPtr<FJsonObject> Object = Value->AsObject();
			Out += TEXT("{");
			int32 I = 0;
			for (const TPair<FString, TSharedPtr<FJsonValue>>& Field : Object->Values)
			{
				Out += I++ > 0 ? TEXT(",") : TEXT("");
				Indent(Out, bPretty, Depth + 1);
				Out += Quote(Field.Key);
				Out += bPretty ? TEXT(": ") : TEXT(":");
				WriteValue(Out, Field.Value, bPretty, Depth + 1);
			}
			if (I > 0)
			{
				Indent(Out, bPretty, Depth);
			}
			Out += TEXT("}");
			break;
		}
		default:
			Out += TEXT("null");
		}
	}

	FString Write(const TSharedPtr<FJsonValue>& Value, bool bPretty)
	{
		FString Out;
		WriteValue(Out, Value, bPretty, 0);
		return Out;
	}

	FString Str(const TSharedPtr<FJsonObject>& Object, const TCHAR* Key)
	{
		FString Out;
		if (Object.IsValid())
		{
			const TSharedPtr<FJsonValue> Field = Object->TryGetField(Key);
			if (Field.IsValid() && Field->Type == EJson::String)
			{
				Out = Field->AsString();
			}
		}
		return Out;
	}

	int64 Long(const TSharedPtr<FJsonObject>& Object, const TCHAR* Key)
	{
		if (Object.IsValid())
		{
			const TSharedPtr<FJsonValue> Field = Object->TryGetField(Key);
			if (Field.IsValid() && Field->Type == EJson::Number)
			{
				return static_cast<int64>(Field->AsNumber());
			}
		}
		return 0;
	}

	TArray<TSharedPtr<FJsonValue>> List(const TSharedPtr<FJsonObject>& Object, const TCHAR* Key)
	{
		if (Object.IsValid())
		{
			const TSharedPtr<FJsonValue> Field = Object->TryGetField(Key);
			if (Field.IsValid() && Field->Type == EJson::Array)
			{
				return Field->AsArray();
			}
		}
		return {};
	}

	TSharedPtr<FJsonObject> Obj(const TSharedPtr<FJsonValue>& Value)
	{
		return Value.IsValid() && Value->Type == EJson::Object ? Value->AsObject() : nullptr;
	}
}
