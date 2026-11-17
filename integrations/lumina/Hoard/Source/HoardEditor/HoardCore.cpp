#include "HoardCore.h"

#include <algorithm>
#include <charconv>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <fstream>
#include <set>
#include <sstream>
#include <system_error>

namespace Hoard
{
    // ---- Text ---------------------------------------------------------------------------------------------------

    /** The code point at I, moving I past it; a stray byte counts as U+FFFD. */
    static uint32_t Decode(std::string_view S, size_t& I)
    {
        const auto B = static_cast<unsigned char>(S[I]);
        const int Len = B < 0x80 ? 1 : (B >> 5) == 0x6 ? 2 : (B >> 4) == 0xE ? 3 : (B >> 3) == 0x1E ? 4 : 0;
        if (Len == 0 || I + Len > S.size())
        {
            ++I;
            return 0xFFFD;
        }
        uint32_t C = Len == 1 ? B : B & (0xFF >> (Len + 1));
        for (int K = 1; K < Len; ++K)
        {
            const auto Cont = static_cast<unsigned char>(S[I + K]);
            if ((Cont >> 6) != 0x2)
            {
                ++I;
                return 0xFFFD;
            }
            C = (C << 6) | (Cont & 0x3F);
        }
        I += Len;
        return C;
    }

    static void Encode(std::string& Out, uint32_t C)
    {
        if (C < 0x80)
        {
            Out += static_cast<char>(C);
        }
        else if (C < 0x800)
        {
            Out += static_cast<char>(0xC0 | (C >> 6));
            Out += static_cast<char>(0x80 | (C & 0x3F));
        }
        else if (C < 0x10000)
        {
            Out += static_cast<char>(0xE0 | (C >> 12));
            Out += static_cast<char>(0x80 | ((C >> 6) & 0x3F));
            Out += static_cast<char>(0x80 | (C & 0x3F));
        }
        else
        {
            Out += static_cast<char>(0xF0 | (C >> 18));
            Out += static_cast<char>(0x80 | ((C >> 12) & 0x3F));
            Out += static_cast<char>(0x80 | ((C >> 6) & 0x3F));
            Out += static_cast<char>(0x80 | (C & 0x3F));
        }
    }

    /** Unicode's White_Space, which is what Rust's trim takes off. */
    static bool IsSpace(uint32_t C)
    {
        return (C >= 0x09 && C <= 0x0D) || C == 0x20 || C == 0x85 || C == 0xA0 || C == 0x1680 || (C >= 0x2000 && C <= 0x200A)
            || C == 0x2028 || C == 0x2029 || C == 0x202F || C == 0x205F || C == 0x3000;
    }

    std::string Trim(std::string_view Text)
    {
        size_t Start = Text.size();
        size_t End = 0;
        for (size_t I = 0; I < Text.size();)
        {
            const size_t At = I;
            if (!IsSpace(Decode(Text, I)))
            {
                Start = std::min(Start, At);
                End = I;
            }
        }
        return Start < End ? std::string(Text.substr(Start, End - Start)) : std::string();
    }

    std::string Quote(std::string_view Text)
    {
        static const char* Hex = "0123456789abcdef";
        std::string Out = "\"";
        for (const char Ch : Text)
        {
            const auto C = static_cast<unsigned char>(Ch);
            switch (C)
            {
            case '"': Out += "\\\""; break;
            case '\\': Out += "\\\\"; break;
            case '\n': Out += "\\n"; break;
            case '\r': Out += "\\r"; break;
            case '\t': Out += "\\t"; break;
            case 0x08: Out += "\\b"; break;
            case 0x0C: Out += "\\f"; break;
            default:
                if (C < 0x20)
                {
                    Out += "\\u00";
                    Out += Hex[C >> 4];
                    Out += Hex[C & 0xF];
                }
                else
                {
                    Out += Ch;
                }
            }
        }
        return Out + "\"";
    }

    std::string Escape(std::string_view Text)
    {
        static const char* Hex = "0123456789ABCDEF";
        std::string Out;
        for (const char Ch : Text)
        {
            const auto B = static_cast<unsigned char>(Ch);
            if ((B >= 'A' && B <= 'Z') || (B >= 'a' && B <= 'z') || (B >= '0' && B <= '9') || B == '-' || B == '_' || B == '.' || B == '~')
            {
                Out += Ch;
            }
            else
            {
                Out += '%';
                Out += Hex[B >> 4];
                Out += Hex[B & 0xF];
            }
        }
        return Out;
    }

    std::filesystem::path Fs(std::string_view Utf8)
    {
        return std::filesystem::path(std::u8string(reinterpret_cast<const char8_t*>(Utf8.data()), Utf8.size()));
    }

    std::string Utf8(const std::filesystem::path& Path)
    {
        const std::u8string S = Path.generic_u8string();
        return std::string(reinterpret_cast<const char*>(S.data()), S.size());
    }

    bool ReadFile(const std::filesystem::path& Path, std::string& Out)
    {
        std::ifstream In(Path, std::ios::binary);
        if (!In)
        {
            return false;
        }
        std::ostringstream Buf;
        Buf << In.rdbuf();
        Out = Buf.str();
        return true;
    }

    bool WriteFile(const std::filesystem::path& Path, std::string_view Text)
    {
        std::error_code Ec;
        if (Path.has_parent_path())
        {
            std::filesystem::create_directories(Path.parent_path(), Ec);
        }
        std::ofstream Out(Path, std::ios::binary | std::ios::trunc);
        Out.write(Text.data(), static_cast<std::streamsize>(Text.size()));
        return static_cast<bool>(Out);
    }

    static std::vector<std::string> Split(std::string_view Text, char By)
    {
        std::vector<std::string> Out;
        size_t Start = 0;
        for (;;)
        {
            const size_t At = Text.find(By, Start);
            Out.emplace_back(Text.substr(Start, At == std::string_view::npos ? std::string_view::npos : At - Start));
            if (At == std::string_view::npos)
            {
                return Out;
            }
            Start = At + 1;
        }
    }

    // ---- JSON ---------------------------------------------------------------------------------------------------

    Json Json::Str(std::string_view Text)
    {
        Json J;
        J.Type = EType::String;
        J.String = Text;
        return J;
    }

    Json Json::Int(int64_t Value)
    {
        Json J;
        J.Type = EType::Number;
        J.Integer = Value;
        J.Number = static_cast<double>(Value);
        J.bInteger = true;
        return J;
    }

    Json Json::Boolean(bool Value)
    {
        Json J;
        J.Type = EType::Bool;
        J.Bool = Value;
        return J;
    }

    Json Json::Array()
    {
        Json J;
        J.Type = EType::Array;
        return J;
    }

    Json Json::Object()
    {
        Json J;
        J.Type = EType::Object;
        return J;
    }

    const Json* Json::Find(std::string_view Key) const
    {
        if (Type == EType::Object)
        {
            for (const JsonField& F : Fields)
            {
                if (F.Key == Key)
                {
                    return &F.Value;
                }
            }
        }
        return nullptr;
    }

    std::string Json::Text(std::string_view Key) const
    {
        const Json* F = Find(Key);
        return F && F->Type == EType::String ? F->String : std::string();
    }

    int64_t Json::Long(std::string_view Key) const
    {
        const Json* F = Find(Key);
        if (!F || F->Type != EType::Number)
        {
            return 0;
        }
        return F->bInteger ? F->Integer : static_cast<int64_t>(F->Number);
    }

    const std::vector<Json>& Json::List(std::string_view Key) const
    {
        static const std::vector<Json> None;
        const Json* F = Find(Key);
        return F && F->Type == EType::Array ? F->Items : None;
    }

    Json& Json::Set(std::string_view Key, Json Value)
    {
        Type = EType::Object;
        for (JsonField& F : Fields)
        {
            if (F.Key == Key)
            {
                F.Value = std::move(Value);
                return *this;
            }
        }
        Fields.push_back({std::string(Key), std::move(Value)});
        return *this;
    }

    Json& Json::Push(Json Value)
    {
        Type = EType::Array;
        Items.push_back(std::move(Value));
        return *this;
    }

    namespace
    {
        struct Parser
        {
            std::string_view S;
            size_t I = 0;
            int Depth = 0;

            void Space()
            {
                while (I < S.size() && (S[I] == ' ' || S[I] == '\t' || S[I] == '\n' || S[I] == '\r'))
                {
                    ++I;
                }
            }

            bool Lit(std::string_view Word)
            {
                if (S.substr(I, Word.size()) != Word)
                {
                    return false;
                }
                I += Word.size();
                return true;
            }

            bool Hex4(uint32_t& Out)
            {
                if (I + 4 > S.size())
                {
                    return false;
                }
                Out = 0;
                for (int K = 0; K < 4; ++K)
                {
                    const char C = S[I++];
                    Out <<= 4;
                    if (C >= '0' && C <= '9') Out |= C - '0';
                    else if (C >= 'a' && C <= 'f') Out |= C - 'a' + 10;
                    else if (C >= 'A' && C <= 'F') Out |= C - 'A' + 10;
                    else return false;
                }
                return true;
            }

            bool Str(std::string& Out)
            {
                if (I >= S.size() || S[I] != '"')
                {
                    return false;
                }
                ++I;
                while (I < S.size())
                {
                    const char C = S[I++];
                    if (C == '"')
                    {
                        return true;
                    }
                    if (static_cast<unsigned char>(C) < 0x20)
                    {
                        return false;
                    }
                    if (C != '\\')
                    {
                        Out += C;
                        continue;
                    }
                    if (I >= S.size())
                    {
                        return false;
                    }
                    switch (S[I++])
                    {
                    case '"': Out += '"'; break;
                    case '\\': Out += '\\'; break;
                    case '/': Out += '/'; break;
                    case 'b': Out += '\b'; break;
                    case 'f': Out += '\f'; break;
                    case 'n': Out += '\n'; break;
                    case 'r': Out += '\r'; break;
                    case 't': Out += '\t'; break;
                    case 'u':
                    {
                        uint32_t Cp;
                        if (!Hex4(Cp))
                        {
                            return false;
                        }
                        if (Cp >= 0xD800 && Cp < 0xDC00)
                        {
                            uint32_t Lo;
                            if (!Lit("\\u") || !Hex4(Lo) || Lo < 0xDC00 || Lo >= 0xE000)
                            {
                                return false;
                            }
                            Cp = 0x10000 + ((Cp - 0xD800) << 10) + (Lo - 0xDC00);
                        }
                        else if (Cp >= 0xDC00 && Cp < 0xE000)
                        {
                            return false;
                        }
                        Encode(Out, Cp);
                        break;
                    }
                    default:
                        return false;
                    }
                }
                return false;
            }

            bool Num(Json& Out)
            {
                const size_t Start = I;
                bool bWhole = true;
                if (I < S.size() && S[I] == '-') ++I;
                while (I < S.size() && ((S[I] >= '0' && S[I] <= '9') || S[I] == '.' || S[I] == 'e' || S[I] == 'E' || S[I] == '+' || S[I] == '-'))
                {
                    bWhole = bWhole && S[I] >= '0' && S[I] <= '9';
                    ++I;
                }
                const char* First = S.data() + Start;
                const char* Last = S.data() + I;
                if (First == Last)
                {
                    return false;
                }
                Out.Type = Json::EType::Number;
                if (bWhole)
                {
                    int64_t V = 0;
                    const auto [Ptr, Ec] = std::from_chars(First, Last, V);
                    if (Ec == std::errc() && Ptr == Last)
                    {
                        Out.Integer = V;
                        Out.Number = static_cast<double>(V);
                        Out.bInteger = true;
                        return true;
                    }
                }
                double D = 0;
                const auto [Ptr, Ec] = std::from_chars(First, Last, D);
                Out.Number = D;
                return Ec == std::errc() && Ptr == Last;
            }

            bool Value(Json& Out)
            {
                Space();
                if (I >= S.size() || ++Depth > 256)
                {
                    return false;
                }
                bool bOk = false;
                const char C = S[I];
                if (C == '{')
                {
                    ++I;
                    Out.Type = Json::EType::Object;
                    Space();
                    if (I < S.size() && S[I] == '}')
                    {
                        ++I;
                        bOk = true;
                    }
                    while (!bOk)
                    {
                        Space();
                        JsonField F;
                        if (!Str(F.Key))
                        {
                            return false;
                        }
                        Space();
                        if (I >= S.size() || S[I++] != ':' || !Value(F.Value))
                        {
                            return false;
                        }
                        Out.Fields.push_back(std::move(F));
                        Space();
                        if (I >= S.size())
                        {
                            return false;
                        }
                        const char Sep = S[I++];
                        if (Sep == '}')
                        {
                            bOk = true;
                        }
                        else if (Sep != ',')
                        {
                            return false;
                        }
                    }
                }
                else if (C == '[')
                {
                    ++I;
                    Out.Type = Json::EType::Array;
                    Space();
                    if (I < S.size() && S[I] == ']')
                    {
                        ++I;
                        bOk = true;
                    }
                    while (!bOk)
                    {
                        Json Item;
                        if (!Value(Item))
                        {
                            return false;
                        }
                        Out.Items.push_back(std::move(Item));
                        Space();
                        if (I >= S.size())
                        {
                            return false;
                        }
                        const char Sep = S[I++];
                        if (Sep == ']')
                        {
                            bOk = true;
                        }
                        else if (Sep != ',')
                        {
                            return false;
                        }
                    }
                }
                else if (C == '"')
                {
                    Out.Type = Json::EType::String;
                    bOk = Str(Out.String);
                }
                else if (Lit("true"))
                {
                    Out = Json::Boolean(true);
                    bOk = true;
                }
                else if (Lit("false"))
                {
                    Out = Json::Boolean(false);
                    bOk = true;
                }
                else if (Lit("null"))
                {
                    bOk = true;
                }
                else
                {
                    bOk = Num(Out);
                }
                --Depth;
                return bOk;
            }
        };

        /** A float as serde_json writes one (ryu's shortest form): 0.5, 2.0, 1e-7, 1.5e20. */
        std::string Float(double D)
        {
            if (!std::isfinite(D))
            {
                return "null";
            }
            char Buf[64];
            const auto [End, Ec] = std::to_chars(Buf, Buf + sizeof(Buf), D, std::chars_format::scientific);
            const std::string Sci(Buf, End);
            const size_t E = Sci.find('e');
            std::string Digits;
            bool bNegative = false;
            for (size_t K = 0; K < E; ++K)
            {
                if (Sci[K] == '-') bNegative = true;
                else if (Sci[K] != '.') Digits += Sci[K];
            }
            const int Exp = std::atoi(Sci.c_str() + E + 1);
            const int Len = static_cast<int>(Digits.size());
            // Where the decimal point falls, counting from the first digit.
            const int Point = Exp + 1;
            std::string Out = bNegative ? "-" : "";
            if (Point >= Len && Point <= 16)
            {
                Out += Digits + std::string(Point - Len, '0') + ".0";
            }
            else if (Point > 0 && Point <= 16)
            {
                Out += Digits.substr(0, Point) + "." + Digits.substr(Point);
            }
            else if (Point > -5 && Point <= 0)
            {
                Out += "0." + std::string(-Point, '0') + Digits;
            }
            else
            {
                Out += Len == 1 ? Digits : Digits.substr(0, 1) + "." + Digits.substr(1);
                Out += "e" + std::to_string(Exp);
            }
            return Out;
        }

        void Newline(std::string& Out, bool bPretty, int Depth)
        {
            if (bPretty)
            {
                Out += '\n';
                Out.append(static_cast<size_t>(Depth) * 2, ' ');
            }
        }

        void WriteValue(std::string& Out, const Json& V, bool bPretty, int Depth)
        {
            switch (V.Type)
            {
            case Json::EType::Null: Out += "null"; break;
            case Json::EType::Bool: Out += V.Bool ? "true" : "false"; break;
            case Json::EType::Number: Out += V.bInteger ? std::to_string(V.Integer) : Float(V.Number); break;
            case Json::EType::String: Out += Quote(V.String); break;
            case Json::EType::Array:
                Out += '[';
                for (size_t K = 0; K < V.Items.size(); ++K)
                {
                    if (K > 0) Out += ',';
                    Newline(Out, bPretty, Depth + 1);
                    WriteValue(Out, V.Items[K], bPretty, Depth + 1);
                }
                if (!V.Items.empty()) Newline(Out, bPretty, Depth);
                Out += ']';
                break;
            case Json::EType::Object:
                Out += '{';
                for (size_t K = 0; K < V.Fields.size(); ++K)
                {
                    if (K > 0) Out += ',';
                    Newline(Out, bPretty, Depth + 1);
                    Out += Quote(V.Fields[K].Key);
                    Out += bPretty ? ": " : ":";
                    WriteValue(Out, V.Fields[K].Value, bPretty, Depth + 1);
                }
                if (!V.Fields.empty()) Newline(Out, bPretty, Depth);
                Out += '}';
                break;
            }
        }
    }

    std::optional<Json> Json::Parse(std::string_view Text)
    {
        if (Text.substr(0, 3) == "\xEF\xBB\xBF")
        {
            Text.remove_prefix(3);
        }
        Parser P{Text};
        Json Out;
        if (!P.Value(Out))
        {
            return std::nullopt;
        }
        P.Space();
        if (P.I != Text.size())
        {
            return std::nullopt;
        }
        return Out;
    }

    std::string Json::Write(bool bPretty) const
    {
        std::string Out;
        WriteValue(Out, *this, bPretty, 0);
        return Out;
    }

    // ---- Client -------------------------------------------------------------------------------------------------

    std::string Client::Url(std::string_view Path) const
    {
        if (Path.starts_with("http://") || Path.starts_with("https://"))
        {
            return std::string(Path);
        }
        std::string Base = Trim(Server);
        while (!Base.empty() && Base.back() == '/')
        {
            Base.pop_back();
        }
        if (Base.find("://") == std::string::npos)
        {
            Base = "http://" + Base;
        }
        return Base + std::string(Path);
    }

    std::string Client::Why(int Code, std::string_view Body)
    {
        const std::optional<Json> Parsed = Json::Parse(Body);
        const std::string Msg = Parsed ? Parsed->Text("error") : std::string();
        switch (Code)
        {
        case 401:
            return "The server wants a token: make one in Hoard (Settings > Access tokens) and paste it here.";
        case 403:
            return std::string("This token can't do that.") + (Msg.empty() ? "" : " " + Msg);
        default:
            return "The server said " + std::to_string(Code) + (Msg.empty() ? std::string(".") : ": " + Msg);
        }
    }

    Reply Client::Send(std::string_view Method, std::string_view Path, std::string_view Payload,
        const std::filesystem::path& DownloadTo) const
    {
        Reply R;
        if (Trim(Server).empty())
        {
            R.Error = "No server address: set it in the Hoard tab's settings.";
            return R;
        }
        Transport::Request Req;
        Req.Method = Method;
        Req.Url = Url(Path);
        Req.Headers.emplace_back("Accept", "application/json");
        const std::string Bearer = Trim(Token);
        if (!Bearer.empty())
        {
            Req.Headers.emplace_back("Authorization", "Bearer " + Bearer);
        }
        if (!Payload.empty())
        {
            Req.Headers.emplace_back("Content-Type", "application/json");
            Req.Body = Payload;
        }
        Req.DownloadTo = DownloadTo;
        Req.Timeout = DownloadTo.empty() ? 30 : 120;
        Req.Cancel = Cancel;
        Transport::Perform(Req, R);
        if (R.Code <= 0)
        {
            R.Error = "Couldn't talk to " + Server + ": " + (R.Error.empty() ? "no answer" : R.Error) + ".";
            return R;
        }
        if (R.Code < 200 || R.Code >= 300)
        {
            R.Error = Why(R.Code, R.Body);
            return R;
        }
        R.bOk = true;
        return R;
    }

    Reply Client::Call(std::string_view Method, std::string_view Path, const Json* Payload) const
    {
        Reply R = Send(Method, Path, Payload ? Payload->Write() : std::string());
        if (!R.bOk)
        {
            return R;
        }
        if (R.Body.empty())
        {
            R.Data = Json::Object();
            return R;
        }
        std::optional<Json> Parsed = Json::Parse(R.Body);
        if (!Parsed)
        {
            R.bOk = false;
            R.Error = "The server's answer wasn't JSON.";
            return R;
        }
        R.Data = std::move(*Parsed);
        return R;
    }

    Reply Client::Search(std::string_view Query, std::string_view Kind, int Offset, int Limit) const
    {
        std::string Qs = "limit=" + std::to_string(Limit) + "&offset=" + std::to_string(Offset) + "&hide_supporting=true";
        const std::string Q = Trim(Query);
        if (!Q.empty())
        {
            Qs += "&q=" + Escape(Q);
        }
        if (!Kind.empty())
        {
            Qs += "&kind=" + Escape(Kind);
        }
        return Call("GET", "/api/assets?" + Qs);
    }

    std::string Client::Thumbnail(const Json& Asset) const
    {
        std::string Path = "/api/assets/" + Escape(Asset.Text("id")) + "/thumb";
        if (const Json* Rev = Asset.Find("thumb_rev"))
        {
            if (Rev->Type == Json::EType::String)
            {
                Path += "?v=" + Escape(Rev->String);
            }
            else if (Rev->Type == Json::EType::Number)
            {
                Path += "?v=" + Escape(Rev->Write());
            }
        }
        const Reply R = Send("GET", Path);
        return R.bOk ? R.Body : std::string();
    }

    // ---- Settings -----------------------------------------------------------------------------------------------

    std::filesystem::path Settings::File()
    {
#ifdef _WIN32
        wchar_t* AppData = nullptr;
        size_t Len = 0;
        if (_wdupenv_s(&AppData, &Len, L"APPDATA") == 0 && AppData)
        {
            const std::filesystem::path Dir(AppData);
            std::free(AppData);
            if (!Dir.empty())
            {
                return Dir / "Hoard" / "lumina.json";
            }
        }
#else
        if (const char* Xdg = std::getenv("XDG_CONFIG_HOME"); Xdg && *Xdg)
        {
            return std::filesystem::path(Xdg) / "hoard" / "lumina.json";
        }
        if (const char* Home = std::getenv("HOME"); Home && *Home)
        {
            return std::filesystem::path(Home) / ".config" / "hoard" / "lumina.json";
        }
#endif
        return {};
    }

    Settings Settings::Load()
    {
        Settings S;
        std::string Text;
        const std::filesystem::path Path = File();
        if (!Path.empty() && ReadFile(Path, Text))
        {
            if (const std::optional<Json> Doc = Json::Parse(Text))
            {
                S.Server = Doc->Text("server");
                S.Token = Doc->Text("token");
            }
        }
        return S;
    }

    bool Settings::Save() const
    {
        const std::filesystem::path Path = File();
        if (Path.empty())
        {
            return false;
        }
        Json Doc = Json::Object();
        Doc.Set("server", Json::Str(Server));
        Doc.Set("token", Json::Str(Token));
        return WriteFile(Path, Doc.Write(true) + "\n");
    }

    // ---- Project ------------------------------------------------------------------------------------------------

    /** A path's folders and name, leaving out empty ones and "..", each made safe. */
    static std::vector<std::string> Segments(std::string_view Path)
    {
        std::vector<std::string> Out;
        for (const std::string& P : Split(Path, '/'))
        {
            if (!P.empty() && P != "..")
            {
                Out.push_back(Project::Sanitize(P));
            }
        }
        return Out;
    }

    /** The non-empty parts, joined with /. */
    static std::string Join(const std::vector<std::string>& Parts)
    {
        std::string Out;
        for (const std::string& P : Parts)
        {
            if (!P.empty())
            {
                Out += Out.empty() ? P : "/" + P;
            }
        }
        return Out;
    }

    Project::Project(std::string_view InRoot, std::string_view DefaultOutDir) : Root(InRoot), OutDir(DefaultOutDir)
    {
        std::replace(Root.begin(), Root.end(), '\\', '/');
        while (Root.size() > 1 && Root.back() == '/')
        {
            Root.pop_back();
        }
        const size_t Slash = Root.rfind('/');
        Name = Slash == std::string::npos ? Root : Root.substr(Slash + 1);
        ReadConfig();
    }

    std::filesystem::path Project::OutPath() const
    {
        std::string Dir = OutDir.starts_with("./") ? OutDir.substr(2) : OutDir;
        while (!Dir.empty() && (Dir.back() == '/' || Dir.back() == '\\'))
        {
            Dir.pop_back();
        }
        const std::filesystem::path P = Fs(Dir);
        return P.is_relative() ? Fs(Root) / P : P;
    }

    /** A TOML string's contents, or a bare value (true, 3) without any comment after it. */
    static std::string TomlValue(std::string_view Raw)
    {
        const std::string S = Trim(Raw);
        if (!S.empty() && (S[0] == '"' || S[0] == '\''))
        {
            const size_t End = S.find(S[0], 1);
            return End == std::string::npos ? S.substr(1) : S.substr(1, End - 1);
        }
        return Trim(std::string_view(S).substr(0, S.find('#')));
    }

    void Project::ReadConfig()
    {
        std::string Text;
        if (!ReadFile(TomlPath(), Text))
        {
            return;
        }
        // The settings `hoard sync` reads from the top of hoard.toml, before its first [table].
        for (std::string Line : Split(Text, '\n'))
        {
            const std::string L = Trim(Line);
            if (L.starts_with("["))
            {
                break;
            }
            const size_t Eq = L.find('=');
            if (L.starts_with("#") || Eq == std::string::npos)
            {
                continue;
            }
            const std::string Key = Trim(std::string_view(L).substr(0, Eq));
            const std::string V = TomlValue(std::string_view(L).substr(Eq + 1));
            if (Key == "server") Server = V;
            else if (Key == "project") Name = V;
            else if (Key == "out_dir") OutDir = V;
            else if (Key == "layout") Layout = V;
            else if (Key == "sidecars") bSidecars = V == "true";
        }
    }

    std::string Project::Sanitize(std::string_view Segment)
    {
        static const std::string_view Bad = "<>:\"/\\|?*";
        std::string Cleaned;
        for (size_t I = 0; I < Segment.size();)
        {
            const size_t At = I;
            const uint32_t C = Decode(Segment, I);
            const bool bControl = C < 0x20 || (C >= 0x7F && C < 0xA0);
            if (bControl || (C < 0x80 && Bad.find(static_cast<char>(C)) != std::string_view::npos))
            {
                Cleaned += '_';
            }
            else
            {
                Cleaned.append(Segment.substr(At, I - At));
            }
        }
        std::string Out = Trim(Cleaned);
        const size_t First = Out.find_first_not_of('.');
        Out = First == std::string::npos ? std::string() : Out.substr(First, Out.find_last_not_of('.') - First + 1);
        return Out.empty() ? "_" : Out;
    }

    std::string Project::FileName(const Json& Entry)
    {
        std::string Name = Entry.Text("file_name");
        if (Name.empty())
        {
            const std::string Path = Entry.Text("path");
            const size_t Slash = Path.rfind('/');
            Name = Slash == std::string::npos ? Path : Path.substr(Slash + 1);
        }
        return Sanitize(Name);
    }

    std::string Project::FolderFor(const Json& Entry) const
    {
        if (Layout == "flat")
        {
            return {};
        }
        if (Layout == "pack")
        {
            const std::string Path = Entry.Text("path");
            const size_t Slash = Path.rfind('/');
            std::vector<std::string> Parts = {Sanitize(Entry.Text("pack_name"))};
            for (std::string& S : Segments(Slash == std::string::npos ? std::string() : Path.substr(0, Slash)))
            {
                Parts.push_back(std::move(S));
            }
            return Join(Parts);
        }
        const std::string Category = Entry.Text("category");
        std::string Out;
        for (const std::string& P : Split(Category.empty() ? "misc" : Category, '/'))
        {
            Out += (Out.empty() ? "" : "/") + Sanitize(P);
        }
        return Out;
    }

    std::string Project::DestFor(const Json& Entry) const
    {
        const std::string Folder = FolderFor(Entry);
        return Folder.empty() ? FileName(Entry) : Folder + "/" + FileName(Entry);
    }

    std::string Project::BundleDest(const Json& BundleRoot, std::string_view BundlePath) const
    {
        const std::string RootFile = FileName(BundleRoot);
        const size_t Dot = RootFile.rfind('.');
        const std::string Stem = Dot == std::string::npos ? RootFile : RootFile.substr(0, Dot);
        return Join({FolderFor(BundleRoot), Stem, Join(Segments(BundlePath))});
    }

    std::vector<Placed> Project::Place(const std::vector<Json>& Entries, const Lock& Taken) const
    {
        std::map<std::string, const Json*> Roots;
        std::set<std::string> InBundles;
        for (const Json& E : Entries)
        {
            const std::string Bundle = E.Text("bundle");
            if (Bundle.empty())
            {
                continue;
            }
            if (Bundle == E.Text("id"))
            {
                Roots.emplace(Bundle, &E);
            }
            else
            {
                InBundles.insert(E.Text("id"));
            }
        }
        std::vector<Placed> Wanted;
        std::map<std::string, size_t> Index;
        for (const Json& E : Entries)
        {
            if (!E.IsObject())
            {
                continue;
            }
            const std::string Id = E.Text("id");
            const std::string Hash = E.Text("hash");
            const std::string Bundle = E.Text("bundle");
            if (Bundle.empty() && InBundles.contains(Id))
            {
                continue;
            }
            const std::string BundlePath = E.Text("bundle_path");
            const auto RootAt = Bundle.empty() || BundlePath.empty() ? Roots.end() : Roots.find(Bundle);
            std::string Rel = RootAt != Roots.end() ? BundleDest(*RootAt->second, BundlePath) : DestFor(E);
            std::string ThereId, ThereHash;
            bool bThere = false;
            if (const auto At = Index.find(Rel); At != Index.end())
            {
                ThereId = Wanted[At->second].Entry.Text("id");
                ThereHash = Wanted[At->second].Entry.Text("hash");
                bThere = true;
            }
            else if (const auto Had = Taken.find(Rel); Had != Taken.end())
            {
                ThereId = Had->second.Id;
                ThereHash = Had->second.Hash;
                bThere = true;
            }
            if (bThere && ThereHash != Hash && ThereId != Id)
            {
                const size_t Dot = Rel.rfind('.');
                const size_t Slash = Rel.rfind('/');
                const std::string Short = Hash.substr(0, 8);
                const bool bExt = Dot != std::string::npos && (Slash == std::string::npos || Dot > Slash);
                Rel = bExt ? Rel.substr(0, Dot) + "." + Short + Rel.substr(Dot) : Rel + "." + Short;
            }
            if (!Index.contains(Rel))
            {
                Index.emplace(Rel, Wanted.size());
                Wanted.push_back({Rel, E});
            }
        }
        return Wanted;
    }

    Lock Project::ReadLock() const
    {
        Lock Files;
        std::string Text;
        if (!ReadFile(LockPath(), Text))
        {
            return Files;
        }
        const std::optional<Json> Doc = Json::Parse(Text);
        const Json* Map = Doc ? Doc->Find("files") : nullptr;
        if (Map && Map->IsObject())
        {
            for (const JsonField& F : Map->Fields)
            {
                Files[F.Key] = {F.Value.Text("id"), F.Value.Text("hash"), F.Value.Long("size")};
            }
        }
        return Files;
    }

    bool Project::WriteLock(const Lock& Files) const
    {
        // Written out by hand to match `hoard sync`'s lock to the byte: serde_json's pretty printing, paths in byte
        // order (which std::map<std::string> already keeps).
        std::string Text = "{\n  \"version\": 1,\n  \"files\": {";
        bool bFirst = true;
        for (const auto& [Path, E] : Files)
        {
            Text += bFirst ? "\n" : ",\n";
            bFirst = false;
            Text += "    " + Quote(Path) + ": {\n      \"id\": " + Quote(E.Id) + ",\n      \"hash\": " + Quote(E.Hash)
                + ",\n      \"size\": " + std::to_string(E.Size) + "\n    }";
        }
        Text += Files.empty() ? "}\n}\n" : "\n  }\n}\n";
        return WriteFile(LockPath(), Text);
    }

    bool Project::Pin(const std::vector<std::string>& Ids, const std::vector<std::string>& Names, std::string_view DefaultServer) const
    {
        std::string Text;
        ReadFile(TomlPath(), Text);
        if (Text.empty())
        {
            Text = "# What this project takes from Hoard. `hoard sync` fetches it; the editor plugin adds to it.\n";
            if (!DefaultServer.empty())
            {
                Text += "server = " + Quote(DefaultServer) + "\n";
            }
            Text += "out_dir = " + Quote(OutDir) + "\n";
        }
        std::string Fresh, Labels;
        bool bAny = false;
        for (size_t K = 0; K < Ids.size(); ++K)
        {
            if (Text.find("\"" + Ids[K] + "\"") != std::string::npos)
            {
                continue;
            }
            std::string Label = K < Names.size() ? Names[K] : std::string();
            std::replace(Label.begin(), Label.end(), '\n', ' ');
            Fresh += (bAny ? ", " : "") + Quote(Ids[K]);
            Labels += (bAny ? ", " : "") + Label;
            bAny = true;
        }
        if (!bAny)
        {
            return true;
        }
        if (Text.back() != '\n')
        {
            Text += '\n';
        }
        Text += "\n[[select]]\nids = [" + Fresh + "]  # " + Labels + "\n";
        return WriteFile(TomlPath(), Text);
    }

    AddResult Project::Add(const Client& With, const std::vector<Json>& Assets, const std::function<void(int, int)>& Progress) const
    {
        AddResult Result;
        Json Ids = Json::Array();
        for (const Json& A : Assets)
        {
            Ids.Push(Json::Str(A.Text("id")));
        }
        Json Payload = Json::Object();
        Payload.Set("ids", std::move(Ids));
        Payload.Set("include_deps", Json::Boolean(true));
        const Reply Plan = With.Call("POST", "/api/manifest", &Payload);
        if (!Plan.bOk)
        {
            Result.Error = Plan.Error;
            return Result;
        }
        Lock Files = ReadLock();
        const std::vector<Placed> Wanted = Place(Plan.Data.List("entries"), Files);
        const std::filesystem::path Out = OutPath();
        std::vector<const Placed*> Todo;
        for (const Placed& W : Wanted)
        {
            const auto Had = Files.find(W.Rel);
            std::error_code Ec;
            const uintmax_t Size = std::filesystem::file_size(Out / Fs(W.Rel), Ec);
            const bool bHere = Had != Files.end() && Had->second.Hash == W.Entry.Text("hash") && !Ec
                && static_cast<int64_t>(Size) == W.Entry.Long("size");
            if (!bHere)
            {
                Todo.push_back(&W);
            }
        }
        for (size_t K = 0; K < Todo.size(); ++K)
        {
            const Placed& W = *Todo[K];
            const std::filesystem::path Path = Out / Fs(W.Rel);
            std::filesystem::path Partial = Path;
            Partial += ".hoard-partial";
            std::error_code Ec;
            std::filesystem::create_directories(Path.parent_path(), Ec);
            const Reply Got = With.Send("GET", W.Entry.Text("url"), {}, Partial);
            const uintmax_t Size = std::filesystem::file_size(Partial, Ec);
            if (!Got.bOk || Ec || static_cast<int64_t>(Size) != W.Entry.Long("size"))
            {
                std::filesystem::remove(Partial, Ec);
                Result.Error = "Couldn't download " + W.Rel + ": " + (Got.bOk ? std::string("it came out the wrong size.") : Got.Error);
                return Result;
            }
            std::filesystem::rename(Partial, Path, Ec);
            if (Ec)
            {
                std::filesystem::remove(Partial, Ec);
                Result.Error = "Couldn't put " + Utf8(Path) + " in place.";
                return Result;
            }
            Result.Downloaded.push_back(W);
            if (Progress)
            {
                Progress(static_cast<int>(K + 1), static_cast<int>(Todo.size()));
            }
        }
        if (bSidecars)
        {
            for (const Placed& W : Wanted)
            {
                const Json& E = W.Entry;
                Json Side = Json::Object();
                Side.Set("id", Json::Str(E.Text("id")));
                Side.Set("hash", Json::Str(E.Text("hash")));
                Side.Set("name", Json::Str(E.Text("name")));
                Side.Set("kind", Json::Str(E.Text("kind")));
                const Json* Category = E.Find("category");
                Side.Set("category", Category ? *Category : Json());
                Json Tags = Json::Array();
                for (const Json& T : E.List("tags"))
                {
                    Tags.Push(T);
                }
                Side.Set("tags", std::move(Tags));
                Side.Set("pack", Json::Str(E.Text("pack_name")));
                Side.Set("source_path", Json::Str(E.Text("path")));
                const Json* Meta = E.Find("meta");
                Side.Set("meta", Meta ? *Meta : Json::Object());
                std::filesystem::path SidePath = Out / Fs(W.Rel);
                SidePath += ".hoard.json";
                WriteFile(SidePath, Side.Write(true));
            }
        }
        std::vector<std::string> PinIds, PinNames;
        for (const Json& A : Assets)
        {
            PinIds.push_back(A.Text("id"));
            PinNames.push_back(A.Text("name"));
        }
        if (!Pin(PinIds, PinNames, With.Server))
        {
            Result.Error = "Couldn't write " + Utf8(TomlPath()) + ".";
            return Result;
        }
        for (const Placed& W : Wanted)
        {
            Files[W.Rel] = {W.Entry.Text("id"), W.Entry.Text("hash"), W.Entry.Long("size")};
        }
        if (!WriteLock(Files))
        {
            Result.Error = "Couldn't write " + Utf8(LockPath()) + ".";
            return Result;
        }
        Result.bOk = true;
        Result.Files = static_cast<int>(Wanted.size());
        Result.OutDir = Utf8(Out);
        // So the library knows what this project uses. The files are in place either way.
        Json Used = Json::Array();
        for (const auto& [Path, E] : Files)
        {
            Json U = Json::Object();
            U.Set("id", Json::Str(E.Id));
            U.Set("path", Json::Str(Path));
            Used.Push(std::move(U));
        }
        Json Report = Json::Object();
        Report.Set("assets", std::move(Used));
        Result.bReported = With.Call("PUT", "/api/projects/" + Escape(Name), &Report).bOk;
        return Result;
    }
}
