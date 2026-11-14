// Window → Hoard: search the library, see thumbnails, and add what's picked to the project with what it needs, where
// `hoard sync` would put it, pinned in hoard.toml so a later sync keeps it.
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using UnityEditor;
using UnityEngine;

namespace Hoard
{
    public sealed class HoardWindow : EditorWindow
    {
        const int Page = 40;
        const float Tile = 96;
        const float LabelHeight = 30;
        const float Gap = 6;
        const string ServerPref = "Hoard.Server";
        const string TokenPref = "Hoard.Token";
        static readonly string[] KindLabels = { "Anything", "Textures", "Models", "Materials", "Sounds", "Animations", "Fonts", "Shaders", "Data" };
        static readonly string[] KindValues = { "", "texture", "model", "material", "audio", "animation", "font", "shader", "data" };

        HoardClient client;
        string query = "";
        int kind;
        bool showSettings;
        string serverField = "";
        string tokenField = "";
        readonly List<Dictionary<string, object>> assets = new List<Dictionary<string, object>>();
        readonly Dictionary<string, Texture2D> thumbs = new Dictionary<string, Texture2D>();
        readonly HashSet<string> asked = new HashSet<string>();
        readonly HashSet<int> selected = new HashSet<int>();
        int anchor = -1;
        long total;
        /// <summary>Bumped by every new search, so a slow old one can't fill in the list.</summary>
        int generation;
        bool busy;
        string status = "";
        double searchAt = -1;
        Vector2 scroll;
        /// <summary>What Send to Hoard would send, shown until it's sent or cancelled.</summary>
        HoardExport pending;
        string packName = "";
        string sentLink;

        [MenuItem("Window/Hoard")]
        static void Open() => GetWindow<HoardWindow>("Hoard");

        static string ProjectRoot => Directory.GetParent(Application.dataPath).FullName;

        static HoardProject Project() => new HoardProject(ProjectRoot, "Assets/Hoard");

        void OnEnable()
        {
            titleContent = new GUIContent("Hoard");
            minSize = new Vector2(240, 300);
            client = new HoardClient(new UnityTransport())
            {
                Server = EditorPrefs.GetString(ServerPref, ""),
                Token = EditorPrefs.GetString(TokenPref, ""),
            };
            if (client.Server == "") client.Server = Project().Server;
            serverField = client.Server;
            tokenField = client.Token;
            showSettings = client.Server == "" || client.Token == "";
            Search(false);
        }

        void OnDisable()
        {
            generation++;
            foreach (var t in thumbs.Values)
                if (t != null) DestroyImmediate(t);
            thumbs.Clear();
            asked.Clear();
        }

        void Update()
        {
            if (searchAt > 0 && EditorApplication.timeSinceStartup >= searchAt) Search(false);
        }

        void SaveSettings()
        {
            client.Server = serverField.Trim();
            client.Token = tokenField.Trim();
            EditorPrefs.SetString(ServerPref, client.Server);
            EditorPrefs.SetString(TokenPref, client.Token);
            showSettings = false;
            GUI.FocusControl(null);
            Search(false);
        }

        async void Search(bool more)
        {
            searchAt = -1;
            var mine = ++generation;
            if (!more)
            {
                assets.Clear();
                selected.Clear();
                anchor = -1;
                scroll = Vector2.zero;
            }
            status = "Searching…";
            Repaint();
            Reply r;
            try
            {
                r = await client.Search(query, KindValues[kind], assets.Count, Page);
            }
            catch (Exception e)
            {
                r = Reply.Fail(e.Message);
            }
            if (this == null || mine != generation) return;
            if (!r.Ok)
            {
                status = r.Error;
                Repaint();
                return;
            }
            var data = Json.Obj(r.Data);
            total = Json.Long(data, "total");
            assets.AddRange(Json.List(data, "items").OfType<Dictionary<string, object>>());
            if (!more) ForgetThumbsExcept(new HashSet<string>(assets.Select(a => Json.Str(a, "id"))));
            status = total == 0 ? "Nothing matches." : $"{assets.Count} of {total}";
            Repaint();
        }

        void ForgetThumbsExcept(HashSet<string> keep)
        {
            foreach (var id in thumbs.Keys.Where(id => !keep.Contains(id)).ToList())
            {
                if (thumbs[id] != null) DestroyImmediate(thumbs[id]);
                thumbs.Remove(id);
            }
            asked.RemoveWhere(id => !keep.Contains(id));
        }

        async void LoadThumb(Dictionary<string, object> asset)
        {
            var id = Json.Str(asset, "id");
            asked.Add(id);
            byte[] bytes = null;
            try
            {
                bytes = await client.Thumbnail(asset);
            }
            catch (Exception e)
            {
                Debug.LogException(e);
            }
            if (this == null || bytes == null || !asked.Contains(id)) return;
            var tex = new Texture2D(2, 2) { hideFlags = HideFlags.HideAndDontSave };
            if (tex.LoadImage(bytes))
            {
                thumbs[id] = tex;
                Repaint();
            }
            else
            {
                DestroyImmediate(tex);
            }
        }

        async void Add()
        {
            if (busy || selected.Count == 0) return;
            var picked = selected.Where(i => i < assets.Count).OrderBy(i => i).Select(i => assets[i]).ToList();
            busy = true;
            status = picked.Count == 1 ? $"Adding {Json.Str(picked[0], "name")}…" : $"Adding {picked.Count} assets…";
            Repaint();
            AddResult r;
            try
            {
                r = await Project().Add(client, picked, (done, of) =>
                {
                    status = $"Downloading {done} of {of}…";
                    Repaint();
                });
            }
            catch (Exception e)
            {
                Debug.LogException(e);
                r = new AddResult { Error = e.Message };
            }
            busy = false;
            // Whatever landed, so Unity imports it.
            AssetDatabase.Refresh();
            if (this == null) return;
            if (!r.Ok)
            {
                status = r.Error;
                Repaint();
                return;
            }
            var where = Relative(r.OutDir);
            status = $"Added {r.Files} files ({r.Fetched} downloaded) to {where}.";
            if (!r.Reported) status += " The server wasn't told the project uses them.";
            if (where != "Assets" && !where.StartsWith("Assets/", StringComparison.Ordinal))
                status += " Unity only imports what's under Assets: set out_dir in hoard.toml.";
            Repaint();
        }

        [MenuItem("Assets/Send to Hoard…", false, 2000)]
        static void SendSelection() => GetWindow<HoardWindow>("Hoard").SendPaths(SelectedPaths());

        [MenuItem("Assets/Send to Hoard…", true)]
        static bool CanSendSelection() => Selection.assetGUIDs.Length > 0;

        static List<string> SelectedPaths() => Selection.assetGUIDs.Select(AssetDatabase.GUIDToAssetPath).Where(p => p != "").ToList();

        /// <summary>Work out what sending these assets and folders means, and show it before sending.</summary>
        void SendPaths(List<string> paths)
        {
            if (busy) return;
            sentLink = null;
            if (paths.Count == 0)
            {
                status = "Select assets or folders in the Project window to send them.";
                Repaint();
                return;
            }
            var export = new HoardExport(Project(), new UnityExportSource());
            try
            {
                export.Plan(paths);
            }
            catch (Exception e)
            {
                Debug.LogException(e);
                status = e.Message;
                Repaint();
                return;
            }
            if (export.Files.Count == 0)
            {
                status = $"Nothing to send: {export.WhyNothing()}.";
                Repaint();
                return;
            }
            pending = export;
            packName = $"{export.Project.Project}: {Path.GetFileNameWithoutExtension(paths[0].TrimEnd('/'))}";
            Focus();
            Repaint();
        }

        async void Send()
        {
            var export = pending;
            pending = null;
            if (export == null || busy) return;
            busy = true;
            status = $"Sending {export.Files.Count} files…";
            Repaint();
            SendResult r;
            try
            {
                r = await export.Send(client, packName);
            }
            catch (Exception e)
            {
                Debug.LogException(e);
                r = new SendResult { Error = e.Message };
            }
            busy = false;
            if (this == null) return;
            status = r.Ok ? $"Sent {r.Sent.Count} files to Hoard as “{r.Name}”." : r.Error;
            sentLink = r.Ok ? r.Link : null;
            Repaint();
        }

        void SendPanel()
        {
            using (new EditorGUILayout.VerticalScope(EditorStyles.helpBox))
            {
                EditorGUILayout.LabelField("Send to Hoard", EditorStyles.boldLabel);
                packName = EditorGUILayout.TextField("Pack name", packName);
                var lines = pending.Files.Take(12).Select(f => "  " + f.Rel + (f.IsCopy ? "  (a copy)" : ""));
                var more = pending.Files.Count > 12 ? $"\n  and {pending.Files.Count - 12} more" : "";
                EditorGUILayout.LabelField($"{pending.Summary()}:\n{string.Join("\n", lines)}{more}", EditorStyles.wordWrappedMiniLabel);
                using (new EditorGUILayout.HorizontalScope())
                {
                    GUILayout.FlexibleSpace();
                    if (GUILayout.Button("Cancel", GUILayout.Width(70))) pending = null;
                    using (new EditorGUI.DisabledScope(busy))
                        if (GUILayout.Button("Send", GUILayout.Width(70))) Send();
                }
            }
        }

        static string Relative(string path)
        {
            var root = ProjectRoot.Replace('\\', '/').TrimEnd('/') + "/";
            return path.StartsWith(root, StringComparison.OrdinalIgnoreCase) ? path.Substring(root.Length) : path;
        }

        void OnGUI()
        {
            Toolbar();
            if (showSettings) Settings();
            if (pending != null) SendPanel();
            Grid();
            Footer();
        }

        void Toolbar()
        {
            using (new EditorGUILayout.HorizontalScope(EditorStyles.toolbar))
            {
                var e = Event.current;
                if (e.type == EventType.KeyDown && (e.keyCode == KeyCode.Return || e.keyCode == KeyCode.KeypadEnter) && GUI.GetNameOfFocusedControl() == "HoardSearch")
                {
                    Search(false);
                    e.Use();
                }
                EditorGUI.BeginChangeCheck();
                GUI.SetNextControlName("HoardSearch");
                query = EditorGUILayout.TextField(query, EditorStyles.toolbarSearchField, GUILayout.MinWidth(80));
                if (EditorGUI.EndChangeCheck()) searchAt = EditorApplication.timeSinceStartup + 0.35;
                EditorGUI.BeginChangeCheck();
                kind = EditorGUILayout.Popup(kind, KindLabels, EditorStyles.toolbarPopup, GUILayout.Width(88));
                if (EditorGUI.EndChangeCheck()) Search(false);
                showSettings = GUILayout.Toggle(showSettings, new GUIContent("Settings", "The server's address and your token"), EditorStyles.toolbarButton, GUILayout.Width(62));
            }
        }

        void Settings()
        {
            using (new EditorGUILayout.VerticalScope(EditorStyles.helpBox))
            {
                serverField = EditorGUILayout.TextField(new GUIContent("Server", "Like http://nas.local:4627"), serverField);
                tokenField = EditorGUILayout.PasswordField(new GUIContent("Token", "Make one in Hoard: Settings → Access tokens"), tokenField);
                EditorGUILayout.LabelField("A token that can only look is enough to add; sending needs an editor's. Both are kept in this computer's editor preferences, not the project.", EditorStyles.wordWrappedMiniLabel);
                if (GUILayout.Button("Save and search")) SaveSettings();
            }
        }

        void Grid()
        {
            var width = position.width - 18;
            var cols = Mathf.Max(1, (int)((width + Gap) / (Tile + Gap)));
            scroll = EditorGUILayout.BeginScrollView(scroll, GUILayout.ExpandHeight(true));
            for (var row = 0; row * cols < assets.Count; row++)
            {
                using (new EditorGUILayout.HorizontalScope())
                {
                    for (var c = 0; c < cols; c++)
                    {
                        var i = row * cols + c;
                        if (i >= assets.Count) break;
                        var rect = GUILayoutUtility.GetRect(Tile, Tile + LabelHeight, GUILayout.Width(Tile), GUILayout.Height(Tile + LabelHeight));
                        Cell(rect, i);
                        GUILayout.Space(Gap);
                    }
                }
                GUILayout.Space(Gap);
            }
            EditorGUILayout.EndScrollView();
        }

        void Cell(Rect rect, int i)
        {
            var a = assets[i];
            var id = Json.Str(a, "id");
            var e = Event.current;
            if (e.type == EventType.MouseDown && e.button == 0 && rect.Contains(e.mousePosition))
            {
                Pick(i, e);
                if (e.clickCount == 2) Add();
                e.Use();
            }
            if (e.type != EventType.Repaint) return;
            if (selected.Contains(i)) EditorGUI.DrawRect(rect, new Color(0.24f, 0.48f, 0.9f, 0.45f));
            var image = new Rect(rect.x + 2, rect.y + 2, rect.width - 4, Tile - 4);
            if (thumbs.TryGetValue(id, out var tex) && tex != null)
            {
                GUI.DrawTexture(image, tex, ScaleMode.ScaleToFit);
            }
            else
            {
                EditorGUI.DrawRect(image, new Color(0.5f, 0.5f, 0.5f, 0.15f));
                GUI.Label(image, Json.Str(a, "kind"), Centered);
                if (Json.Bool(a, "has_thumb") && !asked.Contains(id)) LoadThumb(a);
            }
            var category = Json.Str(a, "category");
            var tip = $"{Json.Str(a, "name")}\n{Json.Str(a, "pack_name")} · {(category != "" ? category : Json.Str(a, "kind"))} · {EditorUtility.FormatBytes(Json.Long(a, "size"))}";
            GUI.Label(new Rect(rect.x, rect.y + Tile, rect.width, LabelHeight), new GUIContent(Json.Str(a, "name"), tip), NameStyle);
        }

        void Pick(int i, Event e)
        {
            if (e.shift && anchor >= 0)
            {
                if (!(e.control || e.command)) selected.Clear();
                for (var j = Math.Min(anchor, i); j <= Math.Max(anchor, i); j++) selected.Add(j);
                return;
            }
            if (e.control || e.command)
            {
                if (!selected.Remove(i)) selected.Add(i);
            }
            else if (!(selected.Contains(i) && e.clickCount == 2))
            {
                selected.Clear();
                selected.Add(i);
            }
            anchor = i;
            Repaint();
        }

        void Footer()
        {
            EditorGUILayout.LabelField(status, EditorStyles.wordWrappedMiniLabel);
            if (sentLink != null && GUILayout.Button("Open it in Hoard", EditorStyles.linkLabel)) Application.OpenURL(sentLink);
            using (new EditorGUILayout.HorizontalScope())
            {
                if (assets.Count < total && GUILayout.Button("More", GUILayout.Width(60))) Search(true);
                GUILayout.FlexibleSpace();
                using (new EditorGUI.DisabledScope(busy))
                {
                    var send = new GUIContent("Send…", "Send what's selected in the Project window to Hoard as a new pack: the files it was made from, or copies other engines open");
                    if (GUILayout.Button(send, GUILayout.Width(60))) SendPaths(SelectedPaths());
                }
                using (new EditorGUI.DisabledScope(busy || selected.Count == 0))
                {
                    var label = new GUIContent(selected.Count > 1 ? $"Add {selected.Count} to project" : "Add to project",
                        "Download with what it needs, and pin it in hoard.toml so hoard sync keeps it");
                    if (GUILayout.Button(label, GUILayout.MinWidth(110))) Add();
                }
            }
        }

        static GUIStyle centeredStyle;
        static GUIStyle Centered => centeredStyle ??= new GUIStyle(EditorStyles.centeredGreyMiniLabel) { wordWrap = true };

        static GUIStyle nameStyle;
        static GUIStyle NameStyle => nameStyle ??= new GUIStyle(EditorStyles.miniLabel)
        {
            alignment = TextAnchor.UpperCenter,
            wordWrap = true,
            clipping = TextClipping.Clip,
        };
    }
}
