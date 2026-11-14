#include "HoardClient.h"

#include "Containers/Ticker.h"
#include "HAL/FileManager.h"
#include "HAL/PlatformProcess.h"
#include "HAL/PlatformTime.h"
#include "HoardJson.h"
#include "HttpManager.h"
#include "HttpModule.h"
#include "Interfaces/IHttpRequest.h"
#include "Interfaces/IHttpResponse.h"

TSharedPtr<FJsonObject> FHoardReply::Object() const
{
	return HoardJson::Obj(Data);
}

static FHoardReply Fail(const FString& Error, int32 Code = 0)
{
	FHoardReply R;
	R.Error = Error;
	R.Code = Code;
	return R;
}

FString FHoardClient::Url(const FString& Path) const
{
	if (Path.StartsWith(TEXT("http://")) || Path.StartsWith(TEXT("https://")))
	{
		return Path;
	}
	FString Base = Server.TrimStartAndEnd();
	while (Base.EndsWith(TEXT("/")))
	{
		Base.LeftChopInline(1);
	}
	if (!Base.Contains(TEXT("://")))
	{
		Base = TEXT("http://") + Base;
	}
	return Base + Path;
}

FString FHoardClient::Why(int32 Code, const FString& Body)
{
	const FString Msg = HoardJson::Str(HoardJson::Obj(HoardJson::Parse(Body)), TEXT("error"));
	switch (Code)
	{
	case 401:
		return TEXT("The server wants a token: make one in Hoard (Settings > Access tokens) and paste it here.");
	case 403:
		return TEXT("This token can't do that.") + (Msg.IsEmpty() ? FString() : TEXT(" ") + Msg);
	default:
		return FString::Printf(TEXT("The server said %d"), Code) + (Msg.IsEmpty() ? FString(TEXT(".")) : TEXT(": ") + Msg);
	}
}

static FString Unreachable(EHttpFailureReason Reason)
{
	switch (Reason)
	{
	case EHttpFailureReason::ConnectionError: return TEXT("couldn't connect");
	case EHttpFailureReason::TimedOut: return TEXT("it took too long");
	case EHttpFailureReason::Cancelled: return TEXT("cancelled");
	default: return TEXT("no answer");
	}
}

void FHoardClient::Send(const FString& Path, const FString& Verb, const FString& Payload, FHoardDone Done, const FString& DownloadTo) const
{
	TArray<uint8> Content;
	if (!Payload.IsEmpty())
	{
		const FTCHARToUTF8 Utf8(*Payload);
		Content.Append(reinterpret_cast<const uint8*>(Utf8.Get()), Utf8.Length());
	}
	Request(Path, Verb, Payload.IsEmpty() ? FString() : FString(TEXT("application/json")), MoveTemp(Content), DownloadTo.IsEmpty() ? 30.f : 0.f,
		MoveTemp(Done), DownloadTo);
}

void FHoardClient::Upload(const FString& Path, TArray<uint8>&& Body, const FString& ContentType, FHoardDone Done) const
{
	// Long enough for a big pack on a slow link; the server answers once it has it all.
	Request(Path, TEXT("POST"), ContentType, MoveTemp(Body), 3600.f, MoveTemp(Done), FString());
}

void FHoardClient::Request(const FString& Path, const FString& Verb, const FString& ContentType, TArray<uint8>&& Content, float Timeout, FHoardDone Done,
	const FString& DownloadTo) const
{
	if (Server.TrimStartAndEnd().IsEmpty())
	{
		Done(Fail(TEXT("No server address: set it in the Hoard tab's settings.")));
		return;
	}
	const TSharedRef<IHttpRequest, ESPMode::ThreadSafe> Req = FHttpModule::Get().CreateRequest();
	Req->SetURL(Url(Path));
	Req->SetVerb(Verb);
	Req->SetHeader(TEXT("Accept"), TEXT("application/json"));
	const FString Bearer = Token.TrimStartAndEnd();
	if (!Bearer.IsEmpty())
	{
		Req->SetHeader(TEXT("Authorization"), TEXT("Bearer ") + Bearer);
	}
	if (!ContentType.IsEmpty())
	{
		Req->SetHeader(TEXT("Content-Type"), ContentType);
		Req->SetContent(MoveTemp(Content));
	}
	if (Timeout > 0.f)
	{
		Req->SetTimeout(Timeout);
	}
	TSharedPtr<FArchive> Writer;
	if (!DownloadTo.IsEmpty())
	{
		// Straight to disk as it arrives, so a big file never sits in memory.
		Writer = MakeShareable(IFileManager::Get().CreateFileWriter(*DownloadTo));
		if (!Writer.IsValid())
		{
			Done(Fail(TEXT("Couldn't write ") + DownloadTo));
			return;
		}
		Req->SetResponseBodyReceiveStreamDelegateV2(FHttpRequestStreamDelegateV2::CreateLambda([Writer](void* Ptr, int64& Length)
		{
			Writer->Serialize(Ptr, Length);
			if (Writer->IsError())
			{
				Length = 0;
			}
		}));
	}
	Req->OnProcessRequestComplete().BindLambda([Server = Server, Writer, Done](FHttpRequestPtr Request, FHttpResponsePtr Response, bool bConnected)
	{
		if (Writer.IsValid())
		{
			Writer->Close();
		}
		if (!bConnected || !Response.IsValid() || Response->GetResponseCode() <= 0)
		{
			Done(Fail(FString::Printf(TEXT("Couldn't talk to %s: %s."), *Server, *Unreachable(Request.IsValid() ? Request->GetFailureReason() : EHttpFailureReason::Other))));
			return;
		}
		const int32 Code = Response->GetResponseCode();
		if (Code < 200 || Code >= 300)
		{
			Done(Fail(Why(Code, Response->GetContentAsString()), Code));
			return;
		}
		FHoardReply R;
		R.bOk = true;
		R.Code = Code;
		if (!Writer.IsValid())
		{
			R.Body = Response->GetContent();
		}
		Done(R);
	});
	Req->ProcessRequest();
}

void FHoardClient::Json(const FString& Path, const FString& Verb, const TSharedPtr<FJsonValue>& Payload, FHoardDone Done) const
{
	Send(Path, Verb, Payload.IsValid() ? HoardJson::Write(Payload) : FString(), [Done](const FHoardReply& In)
	{
		if (!In.bOk)
		{
			Done(In);
			return;
		}
		FHoardReply R = In;
		R.Data = R.Body.Num() > 0 ? HoardJson::Parse(HoardJson::Utf8(R.Body)) : MakeShared<FJsonValueObject>(MakeShared<FJsonObject>());
		if (!R.Data.IsValid())
		{
			R.bOk = false;
			R.Error = TEXT("The server's answer wasn't JSON.");
		}
		Done(R);
	});
}

void FHoardClient::Search(const FString& Query, const FString& Kind, int32 Offset, int32 Limit, FHoardDone Done) const
{
	FString Qs = FString::Printf(TEXT("limit=%d&offset=%d&hide_supporting=true"), Limit, Offset);
	const FString Q = Query.TrimStartAndEnd();
	if (!Q.IsEmpty())
	{
		Qs += TEXT("&q=") + HoardJson::Escape(Q);
	}
	if (!Kind.IsEmpty())
	{
		Qs += TEXT("&kind=") + HoardJson::Escape(Kind);
	}
	Json(TEXT("/api/assets?") + Qs, TEXT("GET"), nullptr, MoveTemp(Done));
}

void FHoardClient::Thumbnail(const TSharedPtr<FJsonObject>& Asset, TFunction<void(const TArray<uint8>&)> Done) const
{
	FString Path = FString::Printf(TEXT("/api/assets/%s/thumb"), *HoardJson::Escape(HoardJson::Str(Asset, TEXT("id"))));
	const TSharedPtr<FJsonValue> Rev = Asset.IsValid() ? Asset->TryGetField(TEXT("thumb_rev")) : nullptr;
	if (Rev.IsValid() && (Rev->Type == EJson::String || Rev->Type == EJson::Number))
	{
		Path += TEXT("?v=") + HoardJson::Escape(HoardJson::Write(Rev).Replace(TEXT("\""), TEXT("")));
	}
	Send(Path, TEXT("GET"), FString(), [Done](const FHoardReply& R)
	{
		Done(R.bOk ? R.Body : TArray<uint8>());
	});
}

void HoardWait(const bool& bDone, double Seconds)
{
	const double Until = FPlatformTime::Seconds() + Seconds;
	double Last = FPlatformTime::Seconds();
	while (!bDone && FPlatformTime::Seconds() < Until)
	{
		const double Now = FPlatformTime::Seconds();
		const float Delta = static_cast<float>(Now - Last);
		Last = Now;
		FHttpModule::Get().GetHttpManager().Tick(Delta);
		FTSTicker::GetCoreTicker().Tick(Delta);
		FPlatformProcess::Sleep(0.005f);
	}
}
