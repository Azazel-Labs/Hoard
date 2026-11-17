// Everything the Hoard plugin does that isn't drawing or importing: JSON, talking to the server, and the project side of
// adding assets (hoard.toml, hoard.lock and where files go, the same as `hoard sync` does it, so a later sync finds
// everything already in place). Plain C++ with no engine types, so it builds and is tested outside the editor too.
#pragma once

#include <atomic>
#include <cstdint>
#include <filesystem>
#include <functional>
#include <map>
#include <optional>
#include <string>
#include <string_view>
#include <vector>

namespace Hoard
{
    struct JsonField;

    /** A JSON value. Objects keep their fields in order, as the server sent them or as they were set. */
    struct Json
    {
        enum class EType : uint8_t { Null, Bool, Number, String, Array, Object };

        EType Type = EType::Null;
        bool Bool = false;
        double Number = 0.0;
        /** Whole numbers are kept exactly as well, since sizes can pass what a double holds exactly. */
        int64_t Integer = 0;
        bool bInteger = false;
        std::string String;
        std::vector<Json> Items;
        std::vector<JsonField> Fields;

        static Json Str(std::string_view Text);
        static Json Int(int64_t Value);
        static Json Boolean(bool Value);
        static Json Array();
        static Json Object();

        bool IsNull() const { return Type == EType::Null; }
        bool IsObject() const { return Type == EType::Object; }

        /** A field of an object, or null when there's no such field (or this isn't an object). */
        const Json* Find(std::string_view Key) const;
        /** A string field, or empty. */
        std::string Text(std::string_view Key) const;
        /** A number field as a whole number, or 0. */
        int64_t Long(std::string_view Key) const;
        /** An array field's items, or none. */
        const std::vector<Json>& List(std::string_view Key) const;

        Json& Set(std::string_view Key, Json Value);
        Json& Push(Json Value);

        /** Parse JSON text, or nothing when it isn't JSON. */
        static std::optional<Json> Parse(std::string_view Text);
        /** Write as serde_json does: compact, or pretty with two-space indents and ": ". */
        std::string Write(bool bPretty = false) const;
    };

    struct JsonField
    {
        std::string Key;
        Json Value;
    };

    /** A JSON (and TOML) string literal, escaped as serde_json escapes it. */
    std::string Quote(std::string_view Text);
    /** Percent-encoding for a URL's path segment or query value. */
    std::string Escape(std::string_view Text);
    /** Leading and trailing whitespace off, Unicode's as well as ASCII's, as Rust's str::trim does. */
    std::string Trim(std::string_view Text);

    /** A UTF-8 path as the filesystem takes it, on Windows too. */
    std::filesystem::path Fs(std::string_view Utf8);
    /** A filesystem path as UTF-8 with forward slashes. */
    std::string Utf8(const std::filesystem::path& Path);

    bool ReadFile(const std::filesystem::path& Path, std::string& Out);
    bool WriteFile(const std::filesystem::path& Path, std::string_view Text);

    /** What came back: the parsed JSON (Data) or the raw bytes (Body), or why not (Error). */
    struct Reply
    {
        bool bOk = false;
        int Code = 0;
        std::string Body;
        Json Data;
        std::string Error;
    };

    /** Talks to a Hoard server. Every call blocks until it's answered, so the editor makes them off its main thread. */
    class Client
    {
    public:
        std::string Server;
        std::string Token;
        /** Set from another thread to give up on whatever is in flight. */
        const std::atomic<bool>* Cancel = nullptr;

        /** A path on the server, or a full URL as it is. */
        std::string Url(std::string_view Path) const;

        /** Send a request; with DownloadTo, stream the body into that file instead of memory. */
        Reply Send(std::string_view Method, std::string_view Path, std::string_view Payload = {},
            const std::filesystem::path& DownloadTo = {}) const;

        /** Send JSON (or nothing) and parse what comes back into Data. */
        Reply Call(std::string_view Method, std::string_view Path, const Json* Payload = nullptr) const;

        /** A page of search results: Data is {items, total}. */
        Reply Search(std::string_view Query, std::string_view Kind, int Offset, int Limit) const;

        /** An asset's thumbnail, as PNG or JPEG bytes, or nothing. */
        std::string Thumbnail(const Json& Asset) const;

        /** Why the server refused, in words. */
        static std::string Why(int Code, std::string_view Body);
    };

    /** The server and token saved for this user (not the project's: the token is yours, and the project is shared). */
    struct Settings
    {
        std::string Server;
        std::string Token;

        static std::filesystem::path File();
        static Settings Load();
        bool Save() const;
    };

    struct LockEntry
    {
        std::string Id;
        std::string Hash;
        int64_t Size = 0;
    };

    /** hoard.lock's files, by path. std::string orders by bytes, as `hoard sync` writes them. */
    using Lock = std::map<std::string, LockEntry>;

    /** A manifest entry and where it goes under out_dir. */
    struct Placed
    {
        std::string Rel;
        Json Entry;
    };

    /** What adding came to: how many files the assets have, which were downloaded, and whether the server heard what
     *  the project uses. */
    struct AddResult
    {
        bool bOk = false;
        std::string Error;
        int Files = 0;
        bool bReported = false;
        std::string OutDir;
        std::vector<Placed> Downloaded;
    };

    class Project
    {
    public:
        /** Root is the folder holding hoard.toml (the project's). */
        Project(std::string_view InRoot, std::string_view DefaultOutDir);

        std::string Root;
        std::string Server;
        std::string Name;
        std::string OutDir;
        std::string Layout = "category";
        bool bSidecars = false;

        std::filesystem::path TomlPath() const { return Fs(Root) / "hoard.toml"; }
        std::filesystem::path LockPath() const { return Fs(Root) / "hoard.lock"; }
        std::filesystem::path OutPath() const;

        static std::string Sanitize(std::string_view Segment);
        static std::string FileName(const Json& Entry);
        std::string FolderFor(const Json& Entry) const;
        std::string DestFor(const Json& Entry) const;
        std::string BundleDest(const Json& BundleRoot, std::string_view BundlePath) const;

        /** Where each file of a manifest goes under out_dir, as `hoard sync` places them. Taken is what hoard.lock
         *  already holds, so a different asset at the same path gets a hash in its name instead. */
        std::vector<Placed> Place(const std::vector<Json>& Entries, const Lock& Taken) const;

        Lock ReadLock() const;
        bool WriteLock(const Lock& Files) const;

        /** Pin these assets in hoard.toml (making it if there's none), so `hoard sync` keeps them. */
        bool Pin(const std::vector<std::string>& Ids, const std::vector<std::string>& Names, std::string_view DefaultServer) const;

        /** Add assets to the project with what they need: resolve them, download what isn't here yet, pin them in
         *  hoard.toml, update hoard.lock and tell the server what the project now uses. Progress gets (done, total)
         *  downloads. */
        AddResult Add(const Client& With, const std::vector<Json>& Assets, const std::function<void(int, int)>& Progress = {}) const;

    private:
        void ReadConfig();
    };

    namespace Transport
    {
        struct Request
        {
            std::string Method;
            std::string Url;
            std::vector<std::pair<std::string, std::string>> Headers;
            std::string Body;
            /** Seconds to wait for each step (connecting, the next bytes), not for the whole exchange, so a big
             *  download can take as long as it needs while it keeps moving. */
            int Timeout = 30;
            /** Where the body goes as it arrives, instead of Reply::Body. */
            std::filesystem::path DownloadTo;
            const std::atomic<bool>* Cancel = nullptr;
        };

        /** One HTTP exchange: WinHTTP on Windows, libcurl elsewhere. Code is 0 when there was no answer, with Error
         *  saying why. */
        void Perform(const Request& In, Reply& Out);
    }
}
