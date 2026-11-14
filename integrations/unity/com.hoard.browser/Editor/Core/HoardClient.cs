// Talks to a Hoard server: searches, thumbnails, manifests, downloads and usage reports. A token that can only look
// is enough for all of it. The requests themselves go through an ITransport: UnityWebRequest in the editor.
using System;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;

namespace Hoard
{
    /// <summary>One HTTP request. With DownloadTo set, the body goes into that file instead of memory.</summary>
    public sealed class HttpCall
    {
        public string Url;
        public string Method = "GET";
        public Dictionary<string, string> Headers = new Dictionary<string, string>();
        public byte[] Body;
        public string DownloadTo;
        /// <summary>Seconds to wait for the whole thing, or 0 for as long as it takes (downloads).</summary>
        public int TimeoutSeconds;
    }

    /// <summary>The status and body that came back, or Error when nothing did (no connection, a timeout, a full disk).</summary>
    public sealed class HttpAnswer
    {
        public int Code;
        public byte[] Body = Array.Empty<byte>();
        public string Error;
    }

    public interface ITransport
    {
        Task<HttpAnswer> Send(HttpCall call);
    }

    /// <summary>What came back: the parsed JSON (Data) or the raw bytes (Body), or why not (Error).</summary>
    public sealed class Reply
    {
        public bool Ok;
        public int Code;
        public object Data;
        public byte[] Body = Array.Empty<byte>();
        public string Error;

        public static Reply Fail(string error, int code = 0) => new Reply { Ok = false, Error = error, Code = code };
    }

    public sealed class HoardClient
    {
        readonly ITransport transport;
        public string Server = "";
        public string Token = "";

        public HoardClient(ITransport transport) => this.transport = transport;

        public string Url(string path)
        {
            if (path.StartsWith("http://", StringComparison.OrdinalIgnoreCase) || path.StartsWith("https://", StringComparison.OrdinalIgnoreCase))
                return path;
            var server = Server.Trim().TrimEnd('/');
            if (!server.Contains("://")) server = "http://" + server;
            return server + path;
        }

        /// <summary>Send a request; with <paramref name="downloadTo"/>, stream the body into that file instead.</summary>
        public async Task<Reply> Send(string path, string method = "GET", object payload = null, string downloadTo = null)
        {
            if (Server.Trim() == "") return Reply.Fail("No server address: set it in the Hoard window's settings.");
            var call = new HttpCall { Url = Url(path), Method = method, DownloadTo = downloadTo, TimeoutSeconds = downloadTo == null ? 30 : 0 };
            call.Headers["Accept"] = "application/json";
            if (Token.Trim() != "") call.Headers["Authorization"] = "Bearer " + Token.Trim();
            if (payload != null)
            {
                call.Headers["Content-Type"] = "application/json";
                call.Body = Encoding.UTF8.GetBytes(Hoard.Json.Write(payload));
            }
            var a = await transport.Send(call);
            if (a.Error != null) return Reply.Fail($"Couldn't talk to {Server}: {a.Error}");
            if (a.Code < 200 || a.Code >= 300) return Reply.Fail(Why(a.Code, Encoding.UTF8.GetString(a.Body ?? Array.Empty<byte>())), a.Code);
            return new Reply { Ok = true, Code = a.Code, Body = a.Body ?? Array.Empty<byte>() };
        }

        static string Why(int code, string body)
        {
            var msg = "";
            try
            {
                msg = Hoard.Json.Str(Hoard.Json.Obj(Hoard.Json.Parse(body)), "error");
            }
            catch (FormatException)
            {
            }
            switch (code)
            {
                case 401:
                    return "The server wants a token: make one in Hoard (Settings → Access tokens) and paste it here.";
                case 403:
                    return "This token can't do that." + (msg != "" ? " " + msg : "");
                default:
                    return $"The server said {code}" + (msg != "" ? ": " + msg : ".");
            }
        }

        /// <summary>Send JSON (or nothing) and parse what comes back into <see cref="Reply.Data"/>.</summary>
        public async Task<Reply> Json(string path, string method = "GET", object payload = null)
        {
            var r = await Send(path, method, payload);
            if (!r.Ok) return r;
            try
            {
                r.Data = r.Body.Length > 0 ? Hoard.Json.Parse(Encoding.UTF8.GetString(r.Body)) : new Dictionary<string, object>();
            }
            catch (FormatException e)
            {
                return Reply.Fail($"The server's answer wasn't JSON ({e.Message}).", r.Code);
            }
            return r;
        }

        /// <summary>A page of search results: Data is {items, total}.</summary>
        public Task<Reply> Search(string q, string kind, int offset, int limit = 40)
        {
            var qs = $"limit={limit}&offset={offset}&hide_supporting=true";
            if (!string.IsNullOrWhiteSpace(q)) qs += "&q=" + Uri.EscapeDataString(q.Trim());
            if (!string.IsNullOrEmpty(kind)) qs += "&kind=" + Uri.EscapeDataString(kind);
            return Json("/api/assets?" + qs);
        }

        /// <summary>An asset's thumbnail, as PNG or JPEG bytes, or null.</summary>
        public async Task<byte[]> Thumbnail(IDictionary<string, object> asset)
        {
            var path = $"/api/assets/{Hoard.Json.Str(asset, "id")}/thumb";
            var rev = Hoard.Json.Str(asset, "thumb_rev");
            if (rev != "") path += "?v=" + Uri.EscapeDataString(rev);
            var r = await Send(path);
            return r.Ok && r.Body.Length > 0 ? r.Body : null;
        }
    }
}
