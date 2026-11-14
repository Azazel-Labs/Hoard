// Searches HOARD_URL (with HOARD_TOKEN) for HOARD_QUERY, adds every match to the Unity project at HOARD_PROJECT the way
// the Hoard window does, and prints the outcome as one `RESULT {json}` line. With HOARD_EXPORT, it sends the project's own
// assets instead, as Assets → Send to Hoard does. Run by web/e2e/engines.spec.ts.
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Hoard;

static class Program
{
    static async Task<int> Main()
    {
        var client = new HoardClient(new HttpClientTransport()) { Server = Env("HOARD_URL"), Token = Env("HOARD_TOKEN") };
        if (Env("HOARD_EXPORT") != "") return await Export(client);
        var found = await client.Search(Env("HOARD_QUERY"), Env("HOARD_KIND"), 0);
        if (!found.Ok) return Done(new Dictionary<string, object> { ["ok"] = false, ["error"] = found.Error });
        var items = Json.List(Json.Obj(found.Data), "items").OfType<Dictionary<string, object>>().ToList();
        var thumbs = 0;
        foreach (var a in items)
        {
            // What Unity's Texture2D.LoadImage reads: PNG or JPEG.
            var b = Json.Bool(a, "has_thumb") ? await client.Thumbnail(a) : null;
            if (b != null && b.Length > 3 && ((b[0] == 0x89 && b[1] == 0x50) || (b[0] == 0xFF && b[1] == 0xD8))) thumbs++;
        }
        var project = new HoardProject(Env("HOARD_PROJECT"), "Assets/Hoard");
        var r = await project.Add(client, items);
        return Done(new Dictionary<string, object>
        {
            ["ok"] = r.Ok, ["error"] = r.Error, ["files"] = r.Files, ["fetched"] = r.Fetched, ["reported"] = r.Reported,
            ["matched"] = items.Count, ["thumbnails"] = thumbs,
        });
    }

    /// <summary>Sends the assets and folders HOARD_EXPORT lists (a line each) as a new pack called HOARD_PACK_NAME, the way
    /// Assets → Send to Hoard does.</summary>
    static async Task<int> Export(HoardClient client)
    {
        var project = new HoardProject(Env("HOARD_PROJECT"), "Assets/Hoard");
        var export = new HoardExport(project, new FileSource(project.Root));
        export.Plan(Env("HOARD_EXPORT").Split('\n').Select(l => l.Trim()).Where(l => l != ""));
        var r = await export.Send(client, Env("HOARD_PACK_NAME"));
        return Done(new Dictionary<string, object>
        {
            ["ok"] = r.Ok, ["error"] = r.Error, ["pack"] = r.Pack, ["link"] = r.Link, ["sent"] = r.Sent,
            ["planned"] = export.Files.Select(f => f.Rel).ToList(), ["from_hoard"] = export.FromHoard.Count, ["engine_only"] = export.EngineOnly.Count,
        });
    }

    static string Env(string name) => Environment.GetEnvironmentVariable(name) ?? "";

    static int Done(Dictionary<string, object> r)
    {
        Console.WriteLine("RESULT " + Json.Write(r));
        return Json.Bool(r, "ok") ? 0 : 1;
    }
}
