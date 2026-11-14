// A small JSON reader and writer, so the package needs nothing beyond what every Unity has. Objects come back as
// Dictionary<string, object>, arrays as List<object>, numbers as double, and true, false and null as themselves.
using System;
using System.Collections;
using System.Collections.Generic;
using System.Globalization;
using System.Text;

namespace Hoard
{
    public static class Json
    {
        public static object Parse(string text)
        {
            var r = new Reader(text ?? "");
            r.Space();
            var v = r.Value();
            r.Space();
            if (!r.End) throw new FormatException("Unexpected text after the JSON value.");
            return v;
        }

        public static string Write(object value, bool pretty = false)
        {
            var sb = new StringBuilder();
            Emit(sb, value, pretty, 0);
            return sb.ToString();
        }

        /// <summary>A field of an object as a string, or "" when it's missing or null.</summary>
        public static string Str(IDictionary<string, object> o, string key) =>
            o != null && o.TryGetValue(key, out var v) && v != null ? Convert.ToString(v, CultureInfo.InvariantCulture) : "";

        public static long Long(IDictionary<string, object> o, string key) =>
            o != null && o.TryGetValue(key, out var v) && v is double d ? (long)d : 0;

        public static bool Bool(IDictionary<string, object> o, string key) =>
            o != null && o.TryGetValue(key, out var v) && v is bool b && b;

        public static Dictionary<string, object> Obj(object v) => v as Dictionary<string, object>;

        public static List<object> List(IDictionary<string, object> o, string key) =>
            o != null && o.TryGetValue(key, out var v) && v is List<object> l ? l : new List<object>();

        static void Emit(StringBuilder sb, object v, bool pretty, int depth)
        {
            switch (v)
            {
                case null:
                    sb.Append("null");
                    break;
                case string s:
                    Quote(sb, s);
                    break;
                case bool b:
                    sb.Append(b ? "true" : "false");
                    break;
                case IDictionary<string, object> map:
                    if (map.Count == 0)
                    {
                        sb.Append("{}");
                        break;
                    }
                    sb.Append('{');
                    var first = true;
                    foreach (var kv in map)
                    {
                        if (!first) sb.Append(',');
                        first = false;
                        Indent(sb, pretty, depth + 1);
                        Quote(sb, kv.Key);
                        sb.Append(pretty ? ": " : ":");
                        Emit(sb, kv.Value, pretty, depth + 1);
                    }
                    Indent(sb, pretty, depth);
                    sb.Append('}');
                    break;
                case IEnumerable list:
                    sb.Append('[');
                    var any = false;
                    foreach (var item in list)
                    {
                        if (any) sb.Append(',');
                        any = true;
                        Indent(sb, pretty, depth + 1);
                        Emit(sb, item, pretty, depth + 1);
                    }
                    if (any) Indent(sb, pretty, depth);
                    sb.Append(']');
                    break;
                case IFormattable f:
                    sb.Append(f.ToString(v is double || v is float ? "R" : null, CultureInfo.InvariantCulture));
                    break;
                default:
                    Quote(sb, v.ToString());
                    break;
            }
        }

        static void Indent(StringBuilder sb, bool pretty, int depth)
        {
            if (pretty) sb.Append('\n').Append(' ', depth * 2);
        }

        static void Quote(StringBuilder sb, string s)
        {
            sb.Append('"');
            foreach (var c in s)
            {
                switch (c)
                {
                    case '"': sb.Append("\\\""); break;
                    case '\\': sb.Append("\\\\"); break;
                    case '\n': sb.Append("\\n"); break;
                    case '\r': sb.Append("\\r"); break;
                    case '\t': sb.Append("\\t"); break;
                    default:
                        if (c < 0x20) sb.Append("\\u").Append(((int)c).ToString("x4"));
                        else sb.Append(c);
                        break;
                }
            }
            sb.Append('"');
        }

        sealed class Reader
        {
            readonly string s;
            int i;

            public Reader(string text) => s = text;

            public bool End => i >= s.Length;

            public void Space()
            {
                while (i < s.Length && char.IsWhiteSpace(s[i])) i++;
            }

            public object Value()
            {
                if (End) throw new FormatException("JSON ended early.");
                switch (s[i])
                {
                    case '{': return Object();
                    case '[': return Array();
                    case '"': return String();
                    case 't': Word("true"); return true;
                    case 'f': Word("false"); return false;
                    case 'n': Word("null"); return null;
                    default: return Number();
                }
            }

            void Word(string w)
            {
                if (string.CompareOrdinal(s, i, w, 0, w.Length) != 0) throw new FormatException($"Expected {w} at {i}.");
                i += w.Length;
            }

            void Expect(char c)
            {
                Space();
                if (End || s[i] != c) throw new FormatException($"Expected '{c}' at {i}.");
                i++;
            }

            Dictionary<string, object> Object()
            {
                var o = new Dictionary<string, object>();
                i++;
                Space();
                if (!End && s[i] == '}')
                {
                    i++;
                    return o;
                }
                while (true)
                {
                    Space();
                    var key = String();
                    Expect(':');
                    Space();
                    o[key] = Value();
                    Space();
                    if (!End && s[i] == ',')
                    {
                        i++;
                        continue;
                    }
                    Expect('}');
                    return o;
                }
            }

            List<object> Array()
            {
                var a = new List<object>();
                i++;
                Space();
                if (!End && s[i] == ']')
                {
                    i++;
                    return a;
                }
                while (true)
                {
                    Space();
                    a.Add(Value());
                    Space();
                    if (!End && s[i] == ',')
                    {
                        i++;
                        continue;
                    }
                    Expect(']');
                    return a;
                }
            }

            string String()
            {
                if (End || s[i] != '"') throw new FormatException($"Expected a string at {i}.");
                i++;
                var sb = new StringBuilder();
                while (true)
                {
                    if (End) throw new FormatException("A string never ended.");
                    var c = s[i++];
                    if (c == '"') return sb.ToString();
                    if (c != '\\')
                    {
                        sb.Append(c);
                        continue;
                    }
                    if (End) throw new FormatException("A string never ended.");
                    var e = s[i++];
                    switch (e)
                    {
                        case 'b': sb.Append('\b'); break;
                        case 'f': sb.Append('\f'); break;
                        case 'n': sb.Append('\n'); break;
                        case 'r': sb.Append('\r'); break;
                        case 't': sb.Append('\t'); break;
                        case 'u':
                            if (i + 4 > s.Length) throw new FormatException("A \\u escape was cut short.");
                            sb.Append((char)int.Parse(s.Substring(i, 4), NumberStyles.HexNumber, CultureInfo.InvariantCulture));
                            i += 4;
                            break;
                        default: sb.Append(e); break;
                    }
                }
            }

            double Number()
            {
                var start = i;
                while (i < s.Length && "+-0123456789.eE".IndexOf(s[i]) >= 0) i++;
                if (i == start) throw new FormatException($"Unexpected '{s[i]}' at {i}.");
                return double.Parse(s.Substring(start, i - start), NumberStyles.Float, CultureInfo.InvariantCulture);
            }
        }
    }
}
