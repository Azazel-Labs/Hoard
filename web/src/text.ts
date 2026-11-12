// Reading text assets for the text viewers: fetch (all or the first part), decode, and colour lines for display.
// A small hand-rolled highlighter instead of a library: it only has to make structure visible (comments, strings,
// keys, tags) across a few dozen formats, and a full grammar engine would outweigh the rest of the viewer.
import { fileUrl, type Asset } from './api'
import './textTokens.css'

export interface TextFile {
  text: string
  /** Bytes in the whole file. */
  total: number
  /** Only the first part was fetched. */
  truncated: boolean
  /** NUL bytes: not a text file, whatever its extension says. */
  binary: boolean
}

/** Fetch the file's text, only its first `limit` bytes when given (a Range request; the server honours them). */
export async function fetchText(asset: Asset, limit?: number): Promise<TextFile> {
  const headers: HeadersInit = limit ? { Range: `bytes=0-${limit - 1}` } : {}
  const res = await fetch(fileUrl(asset), { headers })
  if (!res.ok) throw new Error(`HTTP ${res.status}`)
  const bytes = new Uint8Array(await res.arrayBuffer())
  const range = res.headers.get('content-range')?.match(/\/(\d+)$/)
  const total = range ? Number(range[1]) : res.status === 206 ? asset.size : bytes.length
  const truncated = bytes.length < total
  return { ...decode(bytes, truncated), total, truncated }
}

function decode(bytes: Uint8Array, truncated: boolean): { text: string; binary: boolean } {
  // Windows tools still write UTF-16 now and then (with a byte-order mark); NULs are expected there.
  let encoding = 'utf-8'
  if (bytes[0] === 0xff && bytes[1] === 0xfe) encoding = 'utf-16le'
  else if (bytes[0] === 0xfe && bytes[1] === 0xff) encoding = 'utf-16be'
  if (encoding === 'utf-8') {
    const head = bytes.subarray(0, 8192)
    if (head.includes(0)) return { text: '', binary: true }
  }
  let text = new TextDecoder(encoding).decode(bytes)
  // A cut through a multi-byte character leaves a replacement mark at the very end.
  if (truncated) text = text.replace(/�+$/, '')
  return { text: text.replace(/\r\n?/g, '\n'), binary: false }
}

export type Lang = 'json' | 'xml' | 'yaml' | 'ini' | 'csv' | 'markdown' | 'c' | 'hash' | 'lua' | 'plain'

const LANG_BY_EXT: Record<string, Lang> = {}
const assign = (lang: Lang, exts: string) => exts.split(' ').forEach((e) => (LANG_BY_EXT[e] = lang))
assign('json', 'json jsonc json5 ldtk gltf')
assign('xml', 'xml tmx tsx tx plist html htm svg dae')
assign('yaml', 'yaml yml prefab unity asset mat controller overridecontroller physicmaterial mask meta anim')
assign('ini', 'ini cfg conf properties env toml tscn tres godot import fnt atlas mtl')
assign('csv', 'csv tsv')
assign('markdown', 'md markdown')
assign('c', 'glsl hlsl wgsl shader vert frag comp geom tesc tese gdshader usf ush metal cginc fx fxh compute js ts cs c h cpp hpp')
assign('hash', 'py sh gd cmake')
assign('lua', 'lua')

export function langOf(ext: string): Lang {
  return LANG_BY_EXT[ext.toLowerCase()] ?? 'plain'
}

export const LANG_LABEL: Record<Lang, string> = {
  json: 'JSON', xml: 'XML', yaml: 'YAML', ini: 'Config', csv: 'Table', markdown: 'Markdown', c: 'Code',
  hash: 'Script', lua: 'Lua', plain: 'Text',
}

/** Re-indent JSON two spaces deep without parsing it, so it works on a file cut short or one too big to parse, and
 *  keeps numbers exactly as written. Not for JSON with comments. */
export function reflowJson(src: string): string {
  const out: string[] = []
  let indent = 0
  const nl = () => out.push('\n', '  '.repeat(indent))
  for (let i = 0; i < src.length; i++) {
    const ch = src[i]
    if (ch === '"') {
      let j = i + 1
      while (j < src.length && src[j] !== '"') j += src[j] === '\\' ? 2 : 1
      out.push(src.slice(i, j + 1))
      i = j
    } else if (ch === '{' || ch === '[') {
      let j = i + 1
      while (j < src.length && /\s/.test(src[j])) j++
      if (src[j] === (ch === '{' ? '}' : ']')) {
        out.push(ch, src[j])
        i = j
      } else {
        out.push(ch)
        indent++
        nl()
      }
    } else if (ch === '}' || ch === ']') {
      indent = Math.max(0, indent - 1)
      nl()
      out.push(ch)
    } else if (ch === ',') {
      out.push(',')
      nl()
    } else if (ch === ':') {
      out.push(': ')
    } else if (!/\s/.test(ch)) {
      out.push(ch)
    }
  }
  return out.join('')
}

// --- Highlighting -------------------------------------------------------------------------------------------------
// Each line becomes HTML: spans with tk-* classes (textTokens.css) around the parts worth telling apart. A block
// comment can run across lines, so that state is carried from one line to the next.

type Rule = { re: RegExp; cls: string; start?: boolean }
interface Grammar {
  rules: Rule[]
  block?: { open: string; close: string }
  words?: { re: RegExp; keywords: Set<string>; types: Set<string> }
}

const r = (src: string, cls: string, start = false): Rule => ({ re: new RegExp(src, 'y'), cls, start })
const STR2 = r(String.raw`"(?:[^"\\]|\\.)*"?`, 's')
const STR1 = r(String.raw`'(?:[^'\\]|\\.)*'?`, 's')
// Not the digits inside a name ("Bone01", "x1").
const NUM = r(String.raw`(?<![\w.])-?(?:0x[\da-f]+|\d[\d_]*(?:\.\d*)?(?:e[+-]?\d+)?|\.\d+(?:e[+-]?\d+)?)[a-z]*\b`, 'n')
const words = (s: string) => new Set(s.split(' '))

const C_KEYWORDS = words(
  'if else for while do switch case default break continue return discard struct uniform varying attribute in out ' +
    'inout const static layout precision highp mediump lowp cbuffer tbuffer register packoffset groupshared ' +
    'shared fn let var loop true false null this new class public private protected internal void using ' +
    'namespace import export function typeof instanceof function override virtual sealed abstract readonly ' +
    'Shader Properties SubShader Pass Tags CGPROGRAM ENDCG HLSLPROGRAM ENDHLSL Fallback CustomEditor ' +
    'shader_type render_mode instance',
)
const C_TYPES = words(
  'float float2 float3 float4 float2x2 float3x3 float4x4 half half2 half3 half4 fixed fixed2 fixed3 fixed4 int ' +
    'int2 int3 int4 uint uint2 uint3 uint4 bool vec2 vec3 vec4 ivec2 ivec3 ivec4 uvec2 uvec3 uvec4 bvec2 bvec3 ' +
    'bvec4 mat2 mat3 mat4 mat3x3 mat4x4 sampler sampler2D sampler3D samplerCube SamplerState Texture2D Texture3D ' +
    'TextureCube texture_2d f32 i32 u32 vec2f vec3f vec4f mat4x4f double char string object byte long short ' +
    'Vector2 Vector3 Vector4 Color',
)

const GRAMMARS: Record<Exclude<Lang, 'csv' | 'plain'>, Grammar> = {
  json: {
    rules: [r(String.raw`"(?:[^"\\]|\\.)*"(?=\s*:)`, 'key'), STR2, NUM, r(String.raw`\b(?:true|false|null)\b`, 'n'), r(String.raw`//.*`, 'c')],
    block: { open: '/*', close: '*/' },
  },
  xml: {
    rules: [
      r(String.raw`</?[\w:.-]+`, 'k'), r(String.raw`/?>`, 'k'), r(String.raw`<[?!][\w-]*`, 'k'), r(String.raw`\?>`, 'k'),
      r(String.raw`[\w:.-]+(?=\s*=)`, 'key'), STR2, STR1, r(String.raw`&#?\w+;`, 'n'),
    ],
    block: { open: '<!--', close: '-->' },
  },
  yaml: {
    rules: [
      r(String.raw`#.*`, 'c'), r(String.raw`---.*|\.\.\.$`, 'k', true), r(String.raw`(?:-\s+)?[^\s#'"][^:#]*?:(?=\s|$)`, 'key', true),
      r(String.raw`(?:-\s+)?"[^"]*"\s*:(?=\s|$)`, 'key', true), STR2, STR1, r(String.raw`[&*][\w-]+|![\w!/:.-]*`, 't'),
      NUM, r(String.raw`\b(?:true|false|null|yes|no|on|off|~)\b`, 'n'),
    ],
  },
  ini: {
    rules: [
      r(String.raw`[;#].*`, 'c', true), r(String.raw`//.*`, 'c', true), r(String.raw`\[.*\]`, 'k', true),
      r(String.raw`[\w.\-/ "'@$]+?(?=\s*[=:])`, 'key', true), STR2, STR1, NUM, r(String.raw`\b(?:true|false|null|nil|yes|no|on|off)\b`, 'n'),
      // Godot's resources: ExtResource("1_abc"), Vector3(0, 1, 0).
      r(String.raw`\b[A-Z]\w*(?=\()`, 't'),
    ],
  },
  markdown: {
    rules: [
      r(String.raw`#{1,6}\s.*`, 'k', true), r('```.*', 'c', true), r(String.raw`>.*`, 'c', true),
      r(String.raw`(?:[-*+]|\d+\.)\s`, 'key', true), r('`[^`]*`', 's'), r(String.raw`\*\*[^*]+\*\*|__[^_]+__`, 'key'),
      r(String.raw`!?\[[^\]]*\]\([^)]*\)`, 'n'),
    ],
  },
  c: {
    rules: [r(String.raw`//.*`, 'c'), r(String.raw`#\s*\w+`, 'k', true), STR2, STR1, NUM],
    block: { open: '/*', close: '*/' },
    words: { re: /[A-Za-z_]\w*/y, keywords: C_KEYWORDS, types: C_TYPES },
  },
  hash: {
    rules: [r(String.raw`#.*`, 'c'), r(String.raw`"""[^]*?(?:"""|$)`, 's'), STR2, STR1, NUM],
    words: {
      re: /[A-Za-z_]\w*/y,
      keywords: words('def class return if elif else for while in not and or import from as with try except finally raise pass break continue lambda None True False self func var const extends signal export onready match fi then do done esac case function local echo set'),
      types: words('int float bool str list dict String Array Dictionary Vector2 Vector3 Node'),
    },
  },
  lua: {
    rules: [r(String.raw`--(?!\[\[).*`, 'c'), STR2, STR1, NUM],
    block: { open: '--[[', close: ']]' },
    words: {
      re: /[A-Za-z_]\w*/y,
      keywords: words('and break do else elseif end false for function goto if in local nil not or repeat return then true until while self'),
      types: new Set(),
    },
  },
}

const esc = (s: string) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')
const span = (cls: string, s: string) => `<span class="tk-${cls}">${esc(s)}</span>`

/** One line of CSV: each column its own colour, so a row can be followed across. Quoted fields may hold the
 *  delimiter; a field running over a line break just colours on as the next column. */
function csvLine(line: string, delim: string): string {
  const out: string[] = []
  const field = new RegExp(`"(?:[^"]|"")*"?|[^${delim === '\t' ? '\\t' : delim}]*`, 'y')
  let col = 0
  let pos = 0
  while (pos <= line.length) {
    field.lastIndex = pos
    const m = field.exec(line)![0]
    out.push(m ? span(`col${col % 5}`, m) : '')
    pos += m.length
    if (pos >= line.length) break
    out.push(span('p', line[pos]))
    pos++
    col++
  }
  return out.join('')
}

/** Lines as HTML, coloured for the language. */
export function highlightLines(text: string, lang: Lang, ext = ''): string[] {
  const lines = text.split('\n')
  if (lang === 'plain') return lines.map(esc)
  if (lang === 'csv') {
    const delim = ext === 'tsv' || (!lines[0].includes(',') && lines[0].includes('\t')) ? '\t' : lines[0].includes(';') && !lines[0].includes(',') ? ';' : ','
    return lines.map((l) => csvLine(l, delim))
  }
  const g = GRAMMARS[lang]
  let inBlock = false
  return lines.map((line) => {
    // Very long lines (minified data) are shown as they are: colouring them costs more than it helps.
    if (line.length > 20000) return esc(line)
    const out: string[] = []
    let plain = ''
    const flush = () => {
      if (plain) out.push(esc(plain))
      plain = ''
    }
    let pos = 0
    if (inBlock) {
      const end = line.indexOf(g.block!.close)
      if (end < 0) return span('c', line)
      pos = end + g.block!.close.length
      out.push(span('c', line.slice(0, pos)))
      inBlock = false
    }
    const lead = line.length - line.trimStart().length
    outer: while (pos < line.length) {
      if (g.block && line.startsWith(g.block.open, pos)) {
        flush()
        const end = line.indexOf(g.block.close, pos + g.block.open.length)
        if (end < 0) {
          out.push(span('c', line.slice(pos)))
          inBlock = true
          break
        }
        out.push(span('c', line.slice(pos, end + g.block.close.length)))
        pos = end + g.block.close.length
        continue
      }
      for (const rule of g.rules) {
        if (rule.start && pos !== lead) continue
        rule.re.lastIndex = pos
        const m = rule.re.exec(line)
        if (m && m[0]) {
          flush()
          out.push(span(rule.cls, m[0]))
          pos += m[0].length
          continue outer
        }
      }
      if (g.words) {
        g.words.re.lastIndex = pos
        const m = g.words.re.exec(line)
        if (m) {
          const w = m[0]
          if (g.words.keywords.has(w)) {
            flush()
            out.push(span('k', w))
          } else if (g.words.types.has(w)) {
            flush()
            out.push(span('t', w))
          } else plain += w
          pos += w.length
          continue
        }
      }
      plain += line[pos++]
    }
    flush()
    return out.join('')
  })
}
