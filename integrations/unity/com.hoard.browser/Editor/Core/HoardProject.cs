// The project side of adding assets: hoard.toml, hoard.lock and where files go, the same as `hoard sync` does it, so a
// later sync finds everything already in place.
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Hoard
{
    /// <summary>What adding came to: how many files the assets have, how many were downloaded, and whether the server
    /// heard what the project uses.</summary>
    public sealed class AddResult
    {
        public bool Ok;
        public string Error;
        public int Files;
        public int Fetched;
        public bool Reported;
        public string OutDir;
    }

    public sealed class HoardProject
    {
        /// <summary>The folder holding hoard.toml (the project's root).</summary>
        public readonly string Root;
        public string Server = "";
        public string Project;
        public string OutDir;
        public string Layout = "category";
        public bool Sidecars;

        public HoardProject(string root, string defaultOutDir)
        {
            Root = root.Replace('\\', '/').TrimEnd('/');
            Project = Path.GetFileName(Root);
            OutDir = defaultOutDir;
            ReadConfig();
        }

        public string TomlPath => Root + "/hoard.toml";
        public string LockPath => Root + "/hoard.lock";
        public string OutPath => Root + "/" + (OutDir.StartsWith("./") ? OutDir.Substring(2) : OutDir).TrimEnd('/');

        /// <summary>The settings `hoard sync` reads from the top of hoard.toml (before its first [table]).</summary>
        void ReadConfig()
        {
            if (!File.Exists(TomlPath)) return;
            foreach (var line in File.ReadAllLines(TomlPath))
            {
                var l = line.Trim();
                if (l.StartsWith("[")) break;
                var eq = l.IndexOf('=');
                if (l.StartsWith("#") || eq < 0) continue;
                var value = Value(l.Substring(eq + 1));
                switch (l.Substring(0, eq).Trim())
                {
                    case "server": Server = value; break;
                    case "project": Project = value; break;
                    case "out_dir": OutDir = value; break;
                    case "layout": Layout = value; break;
                    case "sidecars": Sidecars = value == "true"; break;
                }
            }
        }

        /// <summary>A TOML string's contents, or a bare value (true, 3) without any comment after it.</summary>
        static string Value(string raw)
        {
            var s = raw.Trim();
            if (s.StartsWith("\"") || s.StartsWith("'"))
            {
                var end = s.IndexOf(s[0], 1);
                return end > 0 ? s.Substring(1, end - 1) : s.Substring(1);
            }
            var hash = s.IndexOf('#');
            return (hash >= 0 ? s.Substring(0, hash) : s).Trim();
        }

        public static string Sanitize(string s)
        {
            var sb = new StringBuilder();
            foreach (var c in s)
                sb.Append(c < 32 || (c >= 127 && c < 160) || "<>:\"/\\|?*".IndexOf(c) >= 0 ? '_' : c);
            var o = sb.ToString().Trim().Trim('.');
            return o == "" ? "_" : o;
        }

        static IEnumerable<string> Segments(string path) =>
            path.Split('/').Where(s => s != "" && s != "..").Select(Sanitize);

        static string Str(IDictionary<string, object> e, string key) => Json.Str(e, key);

        public static string FileName(IDictionary<string, object> e)
        {
            var name = Str(e, "file_name");
            if (name == "")
            {
                var path = Str(e, "path");
                name = path.Contains("/") ? path.Substring(path.LastIndexOf('/') + 1) : path;
            }
            return Sanitize(name);
        }

        public string FolderFor(IDictionary<string, object> e)
        {
            switch (Layout)
            {
                case "pack":
                    var path = Str(e, "path");
                    var dir = path.Contains("/") ? path.Substring(0, path.LastIndexOf('/')) : "";
                    return string.Join("/", new[] { Sanitize(Str(e, "pack_name")) }.Concat(Segments(dir)));
                case "flat":
                    return "";
                default:
                    var category = Str(e, "category");
                    return string.Join("/", (category == "" ? "misc" : category).Split('/').Select(Sanitize));
            }
        }

        public string DestFor(IDictionary<string, object> e)
        {
            var folder = FolderFor(e);
            return folder == "" ? FileName(e) : folder + "/" + FileName(e);
        }

        /// <summary>A bundle member: the root's folder, then a folder named after the root, then its path in the bundle.</summary>
        public string BundleDest(IDictionary<string, object> bundleRoot, string bundlePath)
        {
            var rootFile = FileName(bundleRoot);
            var dot = rootFile.LastIndexOf('.');
            var stem = dot >= 0 ? rootFile.Substring(0, dot) : rootFile;
            return string.Join("/", new[] { FolderFor(bundleRoot), stem, string.Join("/", Segments(bundlePath)) }.Where(p => p != ""));
        }

        /// <summary>Where each file of a manifest goes under out_dir, as `hoard sync` places them. <paramref name="taken"/>
        /// is what hoard.lock already holds, so a different asset at the same path gets a hash in its name instead.</summary>
        public Dictionary<string, Dictionary<string, object>> Place(List<object> entries, Dictionary<string, LockEntry> taken)
        {
            var all = entries.OfType<Dictionary<string, object>>().ToList();
            var roots = new Dictionary<string, Dictionary<string, object>>();
            var inBundles = new HashSet<string>();
            foreach (var e in all)
            {
                var bundle = Str(e, "bundle");
                if (bundle == "") continue;
                if (bundle == Str(e, "id")) roots[bundle] = e;
                else inBundles.Add(Str(e, "id"));
            }
            var wanted = new Dictionary<string, Dictionary<string, object>>();
            foreach (var e in all)
            {
                var bundle = Str(e, "bundle");
                if (bundle == "" && inBundles.Contains(Str(e, "id"))) continue;
                var bundlePath = Str(e, "bundle_path");
                var rel = bundle != "" && bundlePath != "" && roots.TryGetValue(bundle, out var root) ? BundleDest(root, bundlePath) : DestFor(e);
                string thereId = null, thereHash = null;
                if (wanted.TryGetValue(rel, out var w))
                {
                    thereId = Str(w, "id");
                    thereHash = Str(w, "hash");
                }
                else if (taken.TryGetValue(rel, out var t))
                {
                    thereId = t.Id;
                    thereHash = t.Hash;
                }
                if (thereId != null && thereHash != Str(e, "hash") && thereId != Str(e, "id"))
                {
                    var dot = rel.LastIndexOf('.');
                    var hash8 = Str(e, "hash").Substring(0, Math.Min(8, Str(e, "hash").Length));
                    rel = dot > rel.LastIndexOf('/') ? rel.Substring(0, dot) + "." + hash8 + rel.Substring(dot) : rel + "." + hash8;
                }
                if (!wanted.ContainsKey(rel)) wanted[rel] = e;
            }
            return wanted;
        }

        public sealed class LockEntry
        {
            public string Id;
            public string Hash;
            public long Size;
        }

        public Dictionary<string, LockEntry> ReadLock()
        {
            var files = new Dictionary<string, LockEntry>();
            if (!File.Exists(LockPath)) return files;
            try
            {
                var data = Json.Obj(Json.Parse(File.ReadAllText(LockPath)));
                if (data != null && data.TryGetValue("files", out var f) && f is Dictionary<string, object> map)
                {
                    foreach (var kv in map)
                    {
                        var o = Json.Obj(kv.Value);
                        files[kv.Key] = new LockEntry { Id = Str(o, "id"), Hash = Str(o, "hash"), Size = Json.Long(o, "size") };
                    }
                }
            }
            catch (FormatException)
            {
            }
            return files;
        }

        public void WriteLock(Dictionary<string, LockEntry> files)
        {
            var map = new SortedDictionary<string, object>(StringComparer.Ordinal);
            foreach (var kv in files)
                map[kv.Key] = new Dictionary<string, object> { ["id"] = kv.Value.Id, ["hash"] = kv.Value.Hash, ["size"] = kv.Value.Size };
            var doc = new Dictionary<string, object> { ["version"] = 1, ["files"] = map };
            Write(LockPath, Json.Write(doc, true) + "\n");
        }

        /// <summary>Pin these assets in hoard.toml (making it if there's none), so `hoard sync` keeps them.</summary>
        public void Pin(IList<string> ids, IList<string> names, string defaultServer)
        {
            var text = File.Exists(TomlPath) ? File.ReadAllText(TomlPath) : "";
            if (text == "")
            {
                text = "# What this project takes from Hoard. `hoard sync` fetches it; the editor plugin adds to it.\n";
                if (defaultServer != "") text += $"server = {Json.Write(defaultServer)}\n";
                text += $"out_dir = {Json.Write(OutDir)}\n";
            }
            var fresh = new List<string>();
            var labels = new List<string>();
            for (var i = 0; i < ids.Count; i++)
            {
                if (text.Contains($"\"{ids[i]}\"")) continue;
                fresh.Add(Json.Write(ids[i]));
                labels.Add(names[i].Replace('\n', ' '));
            }
            if (fresh.Count == 0) return;
            if (!text.EndsWith("\n")) text += "\n";
            text += $"\n[[select]]\nids = [{string.Join(", ", fresh)}]  # {string.Join(", ", labels)}\n";
            Write(TomlPath, text);
        }

        static void Write(string path, string text) => File.WriteAllText(path, text, new UTF8Encoding(false));

        static long SizeOf(string path) => File.Exists(path) ? new FileInfo(path).Length : -1;

        /// <summary>Add assets to the project with what they need: resolve them, download what isn't here yet, pin them
        /// in hoard.toml, update hoard.lock and tell the server what the project now uses. <paramref name="progress"/> is
        /// called with (done, total).</summary>
        public async Task<AddResult> Add(HoardClient client, IList<Dictionary<string, object>> assets, Action<int, int> progress = null)
        {
            var ids = assets.Select(a => Str(a, "id")).ToList();
            var r = await client.Json("/api/manifest", "POST",
                new Dictionary<string, object> { ["ids"] = ids, ["include_deps"] = true });
            if (!r.Ok) return new AddResult { Error = r.Error };
            var lockFiles = ReadLock();
            var wanted = Place(Json.List(Json.Obj(r.Data), "entries"), lockFiles);
            var todo = wanted.Where(kv =>
            {
                var path = OutPath + "/" + kv.Key;
                return !(lockFiles.TryGetValue(kv.Key, out var had) && had.Hash == Str(kv.Value, "hash") && SizeOf(path) == Json.Long(kv.Value, "size"));
            }).Select(kv => kv.Key).ToList();
            var done = 0;
            foreach (var rel in todo)
            {
                var e = wanted[rel];
                var path = OutPath + "/" + rel;
                Directory.CreateDirectory(Path.GetDirectoryName(path));
                // Unity leaves alone names ending in ~, so it never imports a file half downloaded.
                var partial = path + ".hoard-partial~";
                var got = await client.Send(Str(e, "url"), "GET", null, partial);
                if (!got.Ok || SizeOf(partial) != Json.Long(e, "size"))
                {
                    if (File.Exists(partial)) File.Delete(partial);
                    return new AddResult { Error = $"Couldn't download {rel}: {got.Error ?? "it came out the wrong size"}" };
                }
                if (File.Exists(path)) File.Delete(path);
                File.Move(partial, path);
                done++;
                progress?.Invoke(done, todo.Count);
            }
            if (Sidecars)
            {
                foreach (var kv in wanted)
                {
                    var e = kv.Value;
                    var side = new Dictionary<string, object>
                    {
                        ["id"] = Str(e, "id"), ["hash"] = Str(e, "hash"), ["name"] = Str(e, "name"), ["kind"] = Str(e, "kind"),
                        ["category"] = e.TryGetValue("category", out var c) ? c : null, ["tags"] = Json.List(e, "tags"),
                        ["pack"] = Str(e, "pack_name"), ["source_path"] = Str(e, "path"),
                        ["meta"] = e.TryGetValue("meta", out var m) ? m : new Dictionary<string, object>(),
                    };
                    Write(OutPath + "/" + kv.Key + ".hoard.json", Json.Write(side, true));
                }
            }
            Pin(ids, assets.Select(a => Str(a, "name")).ToList(), client.Server);
            foreach (var kv in wanted)
                lockFiles[kv.Key] = new LockEntry { Id = Str(kv.Value, "id"), Hash = Str(kv.Value, "hash"), Size = Json.Long(kv.Value, "size") };
            WriteLock(lockFiles);
            // So the library knows what this project uses. The files are in place either way.
            var used = lockFiles.Select(kv => (object)new Dictionary<string, object> { ["id"] = kv.Value.Id, ["path"] = kv.Key }).ToList();
            var told = await client.Json("/api/projects/" + Uri.EscapeDataString(Project), "PUT",
                new Dictionary<string, object> { ["assets"] = used });
            return new AddResult { Ok = true, Files = wanted.Count, Fetched = done, Reported = told.Ok, OutDir = OutPath };
        }
    }
}
