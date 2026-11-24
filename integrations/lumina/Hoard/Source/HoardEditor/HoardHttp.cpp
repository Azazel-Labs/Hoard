// The one HTTP exchange the plugin needs, since Lumina has no HTTP client of its own: WinHTTP on Windows (part of the
// OS) and libcurl elsewhere.
#include "HoardCore.h"

#include <fstream>

#ifdef _WIN32
#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#ifndef NOMINMAX
#define NOMINMAX
#endif
#include <windows.h>
#include <winhttp.h>
#else
#include <curl/curl.h>
#include <mutex>
#endif

namespace Hoard::Transport
{
#ifdef _WIN32
    static std::wstring Wide(std::string_view Utf8)
    {
        if (Utf8.empty())
        {
            return {};
        }
        const int Len = MultiByteToWideChar(CP_UTF8, 0, Utf8.data(), static_cast<int>(Utf8.size()), nullptr, 0);
        std::wstring Out(static_cast<size_t>(Len), L'\0');
        MultiByteToWideChar(CP_UTF8, 0, Utf8.data(), static_cast<int>(Utf8.size()), Out.data(), Len);
        return Out;
    }

    /** Closes a WinHTTP handle when it goes. */
    struct FHandle
    {
        HINTERNET H = nullptr;
        explicit FHandle(HINTERNET In) : H(In) {}
        ~FHandle()
        {
            if (H)
            {
                WinHttpCloseHandle(H);
            }
        }
        FHandle(const FHandle&) = delete;
        FHandle& operator=(const FHandle&) = delete;
        operator HINTERNET() const { return H; }
    };

    static std::string Why(DWORD Error)
    {
        switch (Error)
        {
        case ERROR_WINHTTP_TIMEOUT: return "it took too long";
        case ERROR_WINHTTP_CANNOT_CONNECT:
        case ERROR_WINHTTP_NAME_NOT_RESOLVED:
        case ERROR_WINHTTP_CONNECTION_ERROR: return "couldn't connect";
        case ERROR_WINHTTP_UNRECOGNIZED_SCHEME:
        case ERROR_WINHTTP_INVALID_URL: return "that isn't a web address";
        case ERROR_WINHTTP_SECURE_FAILURE: return "its certificate wasn't trusted";
        default: return "no answer";
        }
    }

    void Perform(const Request& In, Reply& Out)
    {
        const std::wstring Url = Wide(In.Url);
        URL_COMPONENTS Parts{};
        Parts.dwStructSize = sizeof(Parts);
        Parts.dwHostNameLength = static_cast<DWORD>(-1);
        Parts.dwUrlPathLength = static_cast<DWORD>(-1);
        Parts.dwExtraInfoLength = static_cast<DWORD>(-1);
        if (!WinHttpCrackUrl(Url.c_str(), static_cast<DWORD>(Url.size()), 0, &Parts))
        {
            Out.Error = Why(GetLastError());
            return;
        }
        const std::wstring Host(Parts.lpszHostName, Parts.dwHostNameLength);
        std::wstring Path(Parts.lpszUrlPath, Parts.dwUrlPathLength);
        Path.append(Parts.lpszExtraInfo, Parts.dwExtraInfoLength);
        if (Path.empty())
        {
            Path = L"/";
        }
        FHandle Session(WinHttpOpen(L"Hoard for Lumina", WINHTTP_ACCESS_TYPE_AUTOMATIC_PROXY, WINHTTP_NO_PROXY_NAME,
            WINHTTP_NO_PROXY_BYPASS, 0));
        if (!Session)
        {
            Out.Error = Why(GetLastError());
            return;
        }
        const int Ms = In.Timeout * 1000;
        WinHttpSetTimeouts(Session, Ms, Ms, Ms, Ms);
        FHandle Connection(WinHttpConnect(Session, Host.c_str(), Parts.nPort, 0));
        const DWORD Flags = Parts.nScheme == INTERNET_SCHEME_HTTPS ? WINHTTP_FLAG_SECURE : 0;
        FHandle Req(Connection ? WinHttpOpenRequest(Connection, Wide(In.Method).c_str(), Path.c_str(), nullptr, WINHTTP_NO_REFERER,
            WINHTTP_DEFAULT_ACCEPT_TYPES, Flags) : nullptr);
        if (!Req)
        {
            Out.Error = Why(GetLastError());
            return;
        }
        std::wstring Headers;
        for (const auto& [Name, Value] : In.Headers)
        {
            Headers += Wide(Name) + L": " + Wide(Value) + L"\r\n";
        }
        const BOOL bSent = WinHttpSendRequest(Req, Headers.empty() ? WINHTTP_NO_ADDITIONAL_HEADERS : Headers.c_str(),
            static_cast<DWORD>(-1L), In.Body.empty() ? WINHTTP_NO_REQUEST_DATA : const_cast<char*>(In.Body.data()),
            static_cast<DWORD>(In.Body.size()), static_cast<DWORD>(In.Body.size()), 0);
        if (!bSent || !WinHttpReceiveResponse(Req, nullptr))
        {
            Out.Error = Why(GetLastError());
            return;
        }
        DWORD Code = 0;
        DWORD CodeSize = sizeof(Code);
        WinHttpQueryHeaders(Req, WINHTTP_QUERY_STATUS_CODE | WINHTTP_QUERY_FLAG_NUMBER, WINHTTP_HEADER_NAME_BY_INDEX, &Code, &CodeSize,
            WINHTTP_NO_HEADER_INDEX);
        // Only a good answer goes to the file; a refusal's body is kept to say why.
        const bool bToFile = !In.DownloadTo.empty() && Code >= 200 && Code < 300;
        std::ofstream File;
        if (bToFile)
        {
            File.open(In.DownloadTo, std::ios::binary | std::ios::trunc);
            if (!File)
            {
                Out.Error = "couldn't write " + Utf8(In.DownloadTo);
                return;
            }
        }
        std::string Chunk(64 * 1024, '\0');
        for (;;)
        {
            if (In.Cancel && In.Cancel->load())
            {
                Out.Error = "canceled";
                return;
            }
            DWORD Got = 0;
            if (!WinHttpReadData(Req, Chunk.data(), static_cast<DWORD>(Chunk.size()), &Got))
            {
                Out.Error = Why(GetLastError());
                return;
            }
            if (Got == 0)
            {
                break;
            }
            if (bToFile)
            {
                File.write(Chunk.data(), Got);
                if (!File)
                {
                    Out.Error = "couldn't write " + Utf8(In.DownloadTo);
                    return;
                }
            }
            else
            {
                Out.Body.append(Chunk.data(), Got);
            }
        }
        Out.Code = static_cast<int>(Code);
    }
#else
    namespace
    {
        struct FSink
        {
            CURL* Curl = nullptr;
            const Request* In = nullptr;
            Reply* Out = nullptr;
            std::ofstream File;
            bool bDecided = false;
            bool bToFile = false;
            bool bWriteFailed = false;
        };

        size_t OnData(char* Ptr, size_t Size, size_t Count, void* User)
        {
            FSink& S = *static_cast<FSink*>(User);
            const size_t Len = Size * Count;
            if (!S.bDecided)
            {
                // Only a good answer goes to the file; a refusal's body is kept to say why.
                long Code = 0;
                curl_easy_getinfo(S.Curl, CURLINFO_RESPONSE_CODE, &Code);
                S.bDecided = true;
                S.bToFile = !S.In->DownloadTo.empty() && Code >= 200 && Code < 300;
                if (S.bToFile)
                {
                    S.File.open(S.In->DownloadTo, std::ios::binary | std::ios::trunc);
                }
            }
            if (S.bToFile)
            {
                S.File.write(Ptr, static_cast<std::streamsize>(Len));
                if (!S.File)
                {
                    S.bWriteFailed = true;
                    return 0;
                }
            }
            else
            {
                S.Out->Body.append(Ptr, Len);
            }
            return Len;
        }

        int OnProgress(void* User, curl_off_t, curl_off_t, curl_off_t, curl_off_t)
        {
            const FSink& S = *static_cast<const FSink*>(User);
            return S.In->Cancel && S.In->Cancel->load() ? 1 : 0;
        }
    }

    void Perform(const Request& In, Reply& Out)
    {
        static std::once_flag Once;
        std::call_once(Once, [] { curl_global_init(CURL_GLOBAL_DEFAULT); });
        CURL* Curl = curl_easy_init();
        if (!Curl)
        {
            Out.Error = "no answer";
            return;
        }
        FSink Sink;
        Sink.Curl = Curl;
        Sink.In = &In;
        Sink.Out = &Out;
        curl_slist* Headers = nullptr;
        for (const auto& [Name, Value] : In.Headers)
        {
            Headers = curl_slist_append(Headers, (Name + ": " + Value).c_str());
        }
        curl_easy_setopt(Curl, CURLOPT_URL, In.Url.c_str());
        curl_easy_setopt(Curl, CURLOPT_CUSTOMREQUEST, In.Method.c_str());
        curl_easy_setopt(Curl, CURLOPT_HTTPHEADER, Headers);
        curl_easy_setopt(Curl, CURLOPT_USERAGENT, "Hoard for Lumina");
        curl_easy_setopt(Curl, CURLOPT_FOLLOWLOCATION, 1L);
        curl_easy_setopt(Curl, CURLOPT_NOSIGNAL, 1L);
        curl_easy_setopt(Curl, CURLOPT_CONNECTTIMEOUT, static_cast<long>(In.Timeout));
        // Give up when nothing arrives for that long, rather than after a fixed time, so a big download can finish.
        curl_easy_setopt(Curl, CURLOPT_LOW_SPEED_LIMIT, 1L);
        curl_easy_setopt(Curl, CURLOPT_LOW_SPEED_TIME, static_cast<long>(In.Timeout));
        if (!In.Body.empty())
        {
            curl_easy_setopt(Curl, CURLOPT_POSTFIELDS, In.Body.data());
            curl_easy_setopt(Curl, CURLOPT_POSTFIELDSIZE_LARGE, static_cast<curl_off_t>(In.Body.size()));
        }
        curl_easy_setopt(Curl, CURLOPT_WRITEFUNCTION, OnData);
        curl_easy_setopt(Curl, CURLOPT_WRITEDATA, &Sink);
        curl_easy_setopt(Curl, CURLOPT_XFERINFOFUNCTION, OnProgress);
        curl_easy_setopt(Curl, CURLOPT_XFERINFODATA, &Sink);
        curl_easy_setopt(Curl, CURLOPT_NOPROGRESS, 0L);
        const CURLcode Result = curl_easy_perform(Curl);
        long Code = 0;
        curl_easy_getinfo(Curl, CURLINFO_RESPONSE_CODE, &Code);
        curl_slist_free_all(Headers);
        curl_easy_cleanup(Curl);
        if (Sink.bToFile)
        {
            Sink.File.close();
        }
        switch (Result)
        {
        case CURLE_OK:
            if (!In.DownloadTo.empty() && Code >= 200 && Code < 300 && !Sink.bDecided)
            {
                // Nothing arrived, so nothing made the file: an empty file is what was sent.
                std::ofstream(In.DownloadTo, std::ios::binary | std::ios::trunc);
            }
            Out.Code = static_cast<int>(Code);
            return;
        case CURLE_COULDNT_CONNECT:
        case CURLE_COULDNT_RESOLVE_HOST: Out.Error = "couldn't connect"; return;
        case CURLE_OPERATION_TIMEDOUT: Out.Error = "it took too long"; return;
        case CURLE_ABORTED_BY_CALLBACK: Out.Error = "canceled"; return;
        case CURLE_URL_MALFORMAT:
        case CURLE_UNSUPPORTED_PROTOCOL: Out.Error = "that isn't a web address"; return;
        case CURLE_PEER_FAILED_VERIFICATION: Out.Error = "its certificate wasn't trusted"; return;
        case CURLE_WRITE_ERROR: Out.Error = Sink.bWriteFailed ? "couldn't write " + Utf8(In.DownloadTo) : "no answer"; return;
        default: Out.Error = "no answer"; return;
        }
    }
#endif
}
