// What the AssetDatabase would answer, read from the project's files instead: an asset uses the assets whose GUIDs its
// YAML names, and was imported from a file other tools open when its .meta names one of the importers that do it.
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;
using Hoard;

sealed class FileSource : IExportSource
{
    static readonly Regex Guid = new Regex(@"guid: ([0-9a-f]{32})");
    static readonly string[] Importers = { "ModelImporter:", "TextureImporter:", "AudioImporter:", "TrueTypeFontImporter:", "VideoClipImporter:" };

    readonly string root;
    Dictionary<string, string> byGuid;

    public FileSource(string root) => this.root = root.Replace('\\', '/').TrimEnd('/');

    string Rel(string abs) => abs.Replace('\\', '/').Substring(root.Length + 1);

    public IEnumerable<string> InFolder(string folder) =>
        Directory.EnumerateFiles(Path.Combine(root, folder), "*", SearchOption.AllDirectories)
            .Where(f => !f.EndsWith(".meta", StringComparison.Ordinal) && !f.EndsWith("~", StringComparison.Ordinal) && !Path.GetFileName(f).StartsWith("."))
            .Select(Rel)
            .OrderBy(f => f, StringComparer.Ordinal);

    public IEnumerable<string> Dependencies(IList<string> assets)
    {
        byGuid ??= Directory.EnumerateFiles(Path.Combine(root, "Assets"), "*.meta", SearchOption.AllDirectories)
            .Select(m => (path: Rel(m.Substring(0, m.Length - 5)), guid: Guid.Match(File.ReadAllText(m)).Groups[1].Value))
            .Where(m => m.guid != "")
            .GroupBy(m => m.guid)
            .ToDictionary(g => g.Key, g => g.First().path);
        var seen = new HashSet<string>();
        var queue = new Queue<string>(assets);
        while (queue.Count > 0)
        {
            var a = queue.Dequeue();
            if (!seen.Add(a)) continue;
            yield return a;
            var file = Path.Combine(root, a);
            if (!File.Exists(file)) continue;
            using (var head = new StreamReader(file))
                if (head.ReadLine()?.StartsWith("%YAML") != true) continue;
            foreach (Match m in Guid.Matches(File.ReadAllText(file)))
                if (byGuid.TryGetValue(m.Groups[1].Value, out var used)) queue.Enqueue(used);
        }
    }

    public bool IsImported(string asset)
    {
        var meta = Path.Combine(root, asset + ".meta");
        return File.Exists(meta) && File.ReadLines(meta).Any(l => Importers.Contains(l.Trim()));
    }

    // Copies are made from what Unity loaded, which isn't here.
    public ExportCopy Copy(string asset) => null;
}
