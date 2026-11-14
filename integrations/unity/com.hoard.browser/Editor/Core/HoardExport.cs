// Sending a project's own assets to Hoard as a new pack. Only what other tools can open goes: the file an asset was
// imported from (with the files a model names, like an OBJ's materials and their pictures), or, for a mesh or texture
// made in Unity, a copy as OBJ or PNG. Prefabs, materials, scenes, scripts and whatever else only Unity reads aren't
// sent, but what they use is. Nothing that came from Hoard goes back. The editor answers what an asset uses and how it
// was imported, through an IExportSource.
using System;
using System.Collections.Generic;
using System.Globalization;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Hoard
{
    /// <summary>A copy of something made in the engine, in a format other tools open.</summary>
    public sealed class ExportCopy
    {
        /// <summary>With its dot, like ".png".</summary>
        public string Extension;
        public byte[] Bytes;
    }

    /// <summary>What the editor knows about the project's assets. Paths are project-relative, like "Assets/Props/crate.fbx".</summary>
    public interface IExportSource
    {
        /// <summary>The assets in a folder and the folders under it.</summary>
        IEnumerable<string> InFolder(string folder);

        /// <summary>These assets and everything they use, all the way down.</summary>
        IEnumerable<string> Dependencies(IList<string> assets);

        /// <summary>Whether the asset was imported from a file other tools open: a model, picture, sound, font or video.</summary>
        bool IsImported(string asset);

        /// <summary>A copy other tools open of something made in the engine, or null when there's no such thing.</summary>
        ExportCopy Copy(string asset);
    }

    /// <summary>One file of the pack: a file on disk (Path), or a copy made for sending (Bytes).</summary>
    public sealed class ExportFile
    {
        /// <summary>Where it goes in the pack.</summary>
        public string Rel;
        public string Path;
        public byte[] Bytes;
        /// <summary>The asset it stands for, project-relative.</summary>
        public string From;
        public bool IsCopy => Bytes != null;
    }

    public sealed class SendResult
    {
        public bool Ok;
        public string Error;
        public string Pack;
        public string Name;
        public string Link;
        public List<string> Sent = new List<string>();
    }

    public sealed class HoardExport
    {
        /// <summary>Formats imported from a file other engines read too, whatever imported them.</summary>
        public static readonly HashSet<string> SourceExtensions = new HashSet<string>(StringComparer.OrdinalIgnoreCase)
        {
            ".png", ".jpg", ".jpeg", ".webp", ".tga", ".bmp", ".exr", ".hdr", ".svg", ".tif", ".tiff", ".psd", ".dds", ".ktx",
            ".ktx2", ".gltf", ".glb", ".fbx", ".obj", ".mtl", ".dae", ".blend", ".3ds", ".max", ".ma", ".mb", ".ply", ".stl",
            ".usd", ".usda", ".usdc", ".usdz", ".wav", ".ogg", ".mp3", ".flac", ".aif", ".aiff", ".ttf", ".otf", ".woff",
            ".woff2", ".fnt", ".ogv", ".mp4", ".webm", ".mov",
        };

        /// <summary>Code, which only Unity builds.</summary>
        static readonly HashSet<string> Code = new HashSet<string>(StringComparer.OrdinalIgnoreCase)
        {
            ".cs", ".asmdef", ".asmref", ".dll", ".rsp",
        };

        public readonly HoardProject Project;
        /// <summary>What will go, in the order found.</summary>
        public readonly List<ExportFile> Files = new List<ExportFile>();
        /// <summary>Left out because Hoard gave them to this project.</summary>
        public readonly List<string> FromHoard = new List<string>();
        /// <summary>Left out because only Unity opens them (what they use was still looked at).</summary>
        public readonly List<string> EngineOnly = new List<string>();

        readonly IExportSource source;
        readonly HashSet<string> seen = new HashSet<string>(StringComparer.OrdinalIgnoreCase);
        readonly HashSet<string> rels = new HashSet<string>(StringComparer.OrdinalIgnoreCase);
        readonly HashSet<string> paths = new HashSet<string>(StringComparer.OrdinalIgnoreCase);
        readonly HashSet<string> locked = new HashSet<string>(StringComparer.OrdinalIgnoreCase);
        readonly string outRel;

        public HoardExport(HoardProject project, IExportSource source)
        {
            Project = project;
            this.source = source;
            outRel = Relative(Full(project.OutPath));
            foreach (var key in project.ReadLock().Keys) locked.Add(Relative(Full(project.OutPath + "/" + key)));
        }

        static string Full(string path) => System.IO.Path.GetFullPath(path).Replace('\\', '/').TrimEnd('/');

        string Abs(string rel) => Full(Project.Root + "/" + rel);

        /// <summary>Project-relative when it's inside the project, else as it is.</summary>
        string Relative(string abs)
        {
            var root = Full(Project.Root) + "/";
            return abs.StartsWith(root, StringComparison.OrdinalIgnoreCase) ? abs.Substring(root.Length) : abs;
        }

        bool IsFromHoard(string rel) =>
            rel.Equals(outRel, StringComparison.OrdinalIgnoreCase) || rel.StartsWith(outRel + "/", StringComparison.OrdinalIgnoreCase) || locked.Contains(rel);

        /// <summary>Work out what sending these assets and folders means. Call once, then look at Files before Send.</summary>
        public void Plan(IEnumerable<string> picked)
        {
            var assets = new List<string>();
            foreach (var p in picked)
            {
                var rel = p.Replace('\\', '/').TrimEnd('/');
                if (IsFromHoard(rel))
                {
                    if (seen.Add(rel)) FromHoard.Add(rel);
                }
                else if (Directory.Exists(Abs(rel)))
                {
                    assets.AddRange(source.InFolder(rel));
                }
                else
                {
                    assets.Add(rel);
                }
            }
            foreach (var a in source.Dependencies(assets)) Visit(a.Replace('\\', '/'));
        }

        void Visit(string rel)
        {
            if (!seen.Add(rel) || !rel.StartsWith("Assets/", StringComparison.OrdinalIgnoreCase)) return;
            if (IsFromHoard(rel))
            {
                FromHoard.Add(rel);
                return;
            }
            var abs = Abs(rel);
            if (!File.Exists(abs)) return;
            var ext = System.IO.Path.GetExtension(rel);
            var name = rel.Substring("Assets/".Length);
            if (Code.Contains(ext))
            {
                EngineOnly.Add(rel);
                return;
            }
            // A glTF's buffer goes with the glTF.
            if (ext.Equals(".bin", StringComparison.OrdinalIgnoreCase)) return;
            if (source.IsImported(rel) || SourceExtensions.Contains(ext))
            {
                Add(name, abs, null, rel);
                foreach (var c in Companions(abs))
                {
                    var cRel = Relative(c);
                    if (IsFromHoard(cRel)) continue;
                    seen.Add(cRel);
                    // Beside the model in the pack, as it found them, when they're outside Assets.
                    var cName = cRel.StartsWith("Assets/", StringComparison.OrdinalIgnoreCase)
                        ? cRel.Substring("Assets/".Length)
                        : Dir(name) + System.IO.Path.GetFileName(c);
                    Add(cName, c, null, cRel);
                }
                return;
            }
            var copy = source.Copy(rel);
            if (copy != null && copy.Bytes != null && copy.Bytes.Length > 0)
                Add(name.Substring(0, name.Length - ext.Length) + copy.Extension, null, copy.Bytes, rel);
            else
                EngineOnly.Add(rel);
        }

        static string Dir(string rel)
        {
            var slash = rel.LastIndexOf('/');
            return slash >= 0 ? rel.Substring(0, slash + 1) : "";
        }

        void Add(string rel, string abs, byte[] bytes, string from)
        {
            if (abs != null && !paths.Add(abs)) return;
            var r = rel;
            for (var n = 2; !rels.Add(r); n++)
            {
                var ext = System.IO.Path.GetExtension(rel);
                r = $"{rel.Substring(0, rel.Length - ext.Length)} {n}{ext}";
            }
            Files.Add(new ExportFile { Rel = r, Path = abs, Bytes = bytes, From = from });
        }

        /// <summary>The files a model names that it can't be opened without: an OBJ's material libraries and their
        /// pictures, a glTF's buffers and images. Full paths, only those that exist.</summary>
        public static List<string> Companions(string path)
        {
            var output = new List<string>();
            var dir = System.IO.Path.GetDirectoryName(path) ?? "";
            void Take(string p)
            {
                var f = Full(p);
                if (File.Exists(f) && !output.Contains(f)) output.Add(f);
            }
            switch (System.IO.Path.GetExtension(path).ToLowerInvariant())
            {
                case ".obj":
                    foreach (var line in File.ReadAllLines(path))
                    {
                        var l = line.Trim();
                        if (!l.StartsWith("mtllib ", StringComparison.Ordinal)) continue;
                        var mtl = Full(System.IO.Path.Combine(dir, l.Substring(7).Trim()));
                        if (!File.Exists(mtl) || output.Contains(mtl)) continue;
                        output.Add(mtl);
                        foreach (var m in MtlMaps(mtl)) Take(m);
                    }
                    break;
                case ".mtl":
                    foreach (var m in MtlMaps(path)) Take(m);
                    break;
                case ".gltf":
                    try
                    {
                        var data = Json.Obj(Json.Parse(File.ReadAllText(path)));
                        if (data == null) break;
                        foreach (var key in new[] { "buffers", "images" })
                        {
                            foreach (var item in Json.List(data, key).OfType<Dictionary<string, object>>())
                            {
                                var uri = Json.Str(item, "uri");
                                if (uri != "" && !uri.StartsWith("data:", StringComparison.Ordinal))
                                    Take(System.IO.Path.Combine(dir, Uri.UnescapeDataString(uri)));
                            }
                        }
                    }
                    catch (FormatException)
                    {
                    }
                    break;
            }
            return output;
        }

        /// <summary>The pictures a material library uses: the last word of each map line, after any options.</summary>
        static IEnumerable<string> MtlMaps(string mtl)
        {
            var dir = System.IO.Path.GetDirectoryName(mtl) ?? "";
            foreach (var line in File.ReadAllLines(mtl))
            {
                var words = line.Trim().Split(new[] { ' ', '\t' }, StringSplitOptions.RemoveEmptyEntries);
                if (words.Length < 2) continue;
                var key = words[0].ToLowerInvariant();
                if (key.StartsWith("map_", StringComparison.Ordinal) || key == "bump" || key == "disp" || key == "decal" || key == "refl" || key == "norm")
                    yield return System.IO.Path.Combine(dir, words[words.Length - 1].Replace('\\', '/'));
            }
        }

        /// <summary>A mesh as Wavefront OBJ: positions (x, y, z), normals and UVs (u, v) per vertex, either of those
        /// empty when the mesh has none, and a triangle list per submesh. Unity is left-handed and OBJ right-handed, so x
        /// is mirrored and the winding turned around, as Unity does importing one.</summary>
        public static byte[] Obj(string name, float[] positions, float[] normals, float[] uvs, IList<int[]> submeshes)
        {
            var c = CultureInfo.InvariantCulture;
            var sb = new StringBuilder();
            sb.Append("# Made in Unity, sent to Hoard\n");
            sb.Append("o ").Append(name).Append('\n');
            var count = positions.Length / 3;
            for (var i = 0; i < count; i++)
                sb.Append(string.Format(c, "v {0:R} {1:R} {2:R}\n", -positions[i * 3], positions[i * 3 + 1], positions[i * 3 + 2]));
            var hasUv = uvs != null && uvs.Length >= count * 2;
            if (hasUv)
                for (var i = 0; i < count; i++)
                    sb.Append(string.Format(c, "vt {0:R} {1:R}\n", uvs[i * 2], uvs[i * 2 + 1]));
            var hasNormal = normals != null && normals.Length >= count * 3;
            if (hasNormal)
                for (var i = 0; i < count; i++)
                    sb.Append(string.Format(c, "vn {0:R} {1:R} {2:R}\n", -normals[i * 3], normals[i * 3 + 1], normals[i * 3 + 2]));
            string Corner(int v)
            {
                var n = (v + 1).ToString(c);
                if (hasUv && hasNormal) return $"{n}/{n}/{n}";
                if (hasUv) return $"{n}/{n}";
                return hasNormal ? $"{n}//{n}" : n;
            }
            for (var s = 0; s < submeshes.Count; s++)
            {
                var tris = submeshes[s];
                if (submeshes.Count > 1) sb.Append("g ").Append(name).Append('_').Append(s.ToString(c)).Append('\n');
                for (var t = 0; t + 2 < tris.Length; t += 3)
                    sb.Append("f ").Append(Corner(tris[t])).Append(' ').Append(Corner(tris[t + 2])).Append(' ').Append(Corner(tris[t + 1])).Append('\n');
            }
            return Encoding.UTF8.GetBytes(sb.ToString());
        }

        /// <summary>Upload Files as a new pack called <paramref name="packName"/>, in one request.</summary>
        public async Task<SendResult> Send(HoardClient client, string packName)
        {
            if (Files.Count == 0) return new SendResult { Error = $"Nothing here to send: {WhyNothing()}." };
            var boundary = "hoard" + Guid.NewGuid().ToString("N");
            using var body = new MemoryStream();
            void Text(string s)
            {
                var b = Encoding.UTF8.GetBytes(s);
                body.Write(b, 0, b.Length);
            }
            void Field(string field, string value) => Text($"--{boundary}\r\nContent-Disposition: form-data; name=\"{field}\"\r\n\r\n{value}\r\n");
            Field("name", packName.Trim() != "" ? packName.Trim() : "From " + Project.Project);
            Field("source", "Unity project: " + Project.Project);
            foreach (var f in Files)
            {
                Field("relpath", f.Rel);
                var file = System.IO.Path.GetFileName(f.Rel).Replace('"', '_');
                Text($"--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"{file}\"\r\nContent-Type: application/octet-stream\r\n\r\n");
                var bytes = f.Bytes ?? File.ReadAllBytes(f.Path);
                body.Write(bytes, 0, bytes.Length);
                Text("\r\n");
            }
            Text($"--{boundary}--\r\n");
            var r = await client.Upload("/api/packs", body.ToArray(), "multipart/form-data; boundary=" + boundary);
            if (!r.Ok)
            {
                return new SendResult
                {
                    Error = r.Code == 403 ? "Sending needs a token that can add to the library (an editor's or an admin's); this one can only look." : r.Error,
                };
            }
            Dictionary<string, object> pack;
            try
            {
                pack = Json.Obj(Json.Parse(Encoding.UTF8.GetString(r.Body)));
            }
            catch (FormatException)
            {
                pack = null;
            }
            if (pack == null || Json.Str(pack, "id") == "") return new SendResult { Error = "The server's answer made no sense." };
            var id = Json.Str(pack, "id");
            var name = Json.Str(pack, "name");
            return new SendResult
            {
                Ok = true, Pack = id, Name = name, Link = client.Url($"/?pack={Uri.EscapeDataString(id)}&pack_name={Uri.EscapeDataString(name)}"),
                Sent = Files.Select(f => f.Rel).ToList(),
            };
        }

        public string WhyNothing()
        {
            if (FromHoard.Count > 0 && EngineOnly.Count == 0) return "it all came from Hoard";
            if (EngineOnly.Count > 0) return "only Unity opens what's picked, and it uses nothing else";
            return "nothing is picked";
        }

        /// <summary>"3 files; left out: 2 that came from Hoard, 1 only Unity opens".</summary>
        public string Summary()
        {
            var s = Files.Count == 1 ? "1 file" : $"{Files.Count} files";
            var left = new List<string>();
            if (FromHoard.Count > 0) left.Add($"{FromHoard.Count} that came from Hoard");
            if (EngineOnly.Count > 0) left.Add($"{EngineOnly.Count} only Unity opens");
            return left.Count > 0 ? s + "; left out: " + string.Join(", ", left) : s;
        }
    }
}
