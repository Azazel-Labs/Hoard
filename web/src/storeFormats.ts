// The format wizard's workings (Settings → Store files): selectors made from a value picked in a file, and the TOML
// of a format made with the form. The syntax is the server's (crates/hoardd/src/store_formats.rs,
// docs/store-formats.md).
import { ref } from 'vue'
import type { FilePlace } from './api'
import { t } from './i18n'

/** A file someone asked to teach Hoard, from its details; Settings → Store files opens the wizard on it. */
export const teach = ref<FilePlace | null>(null)

/** The file types the wizard reads: text a store may describe its art in. */
export const TEXT_EXTS = ['json', 'txt', 'xml', 'ini', 'yaml', 'yml', 'cfg', 'nfo', 'toml', 'meta']
/** Bigger files aren't read as a store's (the server's SIDECAR_BYTES). */
export const MAX_STORE_FILE = 256 * 1024

export type FieldKey = 'title' | 'seller' | 'license' | 'url' | 'words' | 'type' | 'colors' | 'id' | 'origin' | 'size'

/** The fields the form fills in, in the order shown. The others (sizes, surfaces, maps, levels of detail) are written
 *  in TOML. Labels and hints are getters, so they're in the language of the moment. */
export const FIELDS: { key: FieldKey; readonly label: string; readonly name: string; many?: boolean; readonly hint: string }[] = [
  { key: 'title', get label() { return t('storefiles-field-title') }, get name() { return t('storefiles-field-title-name') }, get hint() { return t('storefiles-field-title-hint') } },
  { key: 'seller', get label() { return t('storefiles-field-seller') }, get name() { return t('storefiles-field-seller-name') }, get hint() { return t('storefiles-field-seller-hint') } },
  { key: 'license', get label() { return t('storefiles-field-license') }, get name() { return t('storefiles-field-license-name') }, get hint() { return t('storefiles-field-license-hint') } },
  { key: 'url', get label() { return t('storefiles-field-url') }, get name() { return t('storefiles-field-url-name') }, get hint() { return t('storefiles-field-url-hint') } },
  { key: 'words', many: true, get label() { return t('storefiles-field-words') }, get name() { return t('storefiles-field-words-name') }, get hint() { return t('storefiles-field-words-hint') } },
  { key: 'type', get label() { return t('storefiles-field-type') }, get name() { return t('storefiles-field-type-name') }, get hint() { return t('storefiles-field-type-hint') } },
  { key: 'colors', many: true, get label() { return t('storefiles-field-colors') }, get name() { return t('storefiles-field-colors-name') }, get hint() { return t('storefiles-field-colors-hint') } },
  { key: 'id', get label() { return t('storefiles-field-id') }, get name() { return t('storefiles-field-id-name') }, get hint() { return t('storefiles-field-id-hint') } },
  { key: 'origin', many: true, get label() { return t('storefiles-field-origin') }, get name() { return t('storefiles-field-origin-name') }, get hint() { return t('storefiles-field-origin-hint') } },
  { key: 'size', get label() { return t('storefiles-field-size') }, get name() { return t('storefiles-field-size-name') }, get hint() { return t('storefiles-field-size-hint') } },
]

export type JsonPath = (string | number)[]

/** Keys that often tell the items of a list apart (`meta[key=scanArea]`). */
const DISCRIMINATORS = ['key', 'type', 'name', 'id', 'kind', 'label', 'role', 'channel', 'slot']

type Obj = Record<string, unknown>
const isObj = (v: unknown): v is Obj => !!v && typeof v === 'object' && !Array.isArray(v)

/** A key as a step: characters selectors use become `?`, which matches any one character. */
function keyStep(k: string): string {
  const s = k.replace(/[.[\]()!*?~|]/g, '?').replace(/^\s|\s$/g, '?')
  return s || '*'
}

/** `[key=value]` that picks out this item of the list, if one of its keys has a value no other item has. */
function filterFor(list: unknown[], item: Obj): string {
  const items = list.filter(isObj)
  const text = (v: unknown) => (typeof v === 'string' ? v.trim().toLowerCase() : typeof v === 'number' ? String(v) : null)
  const keys = Object.keys(item)
  const order = [
    ...DISCRIMINATORS.flatMap((d) => keys.filter((k) => k.toLowerCase() === d)),
    ...keys.filter((k) => !DISCRIMINATORS.includes(k.toLowerCase())),
  ]
  for (const k of order) {
    const v = text(item[k])
    if (!v || v.length > 40 || /[|[\]()]/.test(v) || k !== k.trim() || /[[\]()=]/.test(k)) continue
    if (items.filter((x) => text(x[k]) === v).length === 1) return `[${k}=${String(item[k]).trim()}]`
  }
  return ''
}

/**
 * A selector for the value at `path` in `root`. An item of a list is picked out by a key only it has; for a field
 * that takes many values (`many`), the list the value is in is taken whole.
 */
export function jsonSelector(root: unknown, path: JsonPath, many = false): string {
  const steps: string[] = []
  const add = (filter: string) => {
    if (steps.length) steps[steps.length - 1] += filter
    else steps.push(filter)
  }
  let cur: unknown = root
  path.forEach((seg, i) => {
    if (typeof seg === 'string') {
      steps.push(keyStep(seg))
      cur = isObj(cur) ? cur[seg] : undefined
      return
    }
    const list = Array.isArray(cur) ? cur : []
    const item = list[seg]
    const last = !path.slice(i + 1).some((s) => typeof s === 'number')
    if (isObj(item) && !(many && last)) {
      const f = filterFor(list, item)
      if (f) add(f)
    } else if (Array.isArray(item)) {
      // A list in a list: step into the outer one's items, so the next filter sees the inner list's.
      add('[]')
    }
    // Otherwise the index is dropped: a key steps into each item of a list, and a field takes the first value.
    cur = item
  })
  return steps.join('.')
}

/** Text as a regular expression that matches it exactly. */
export function escapeRegex(s: string): string {
  return s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
}

/** Text before a value as a pattern: runs of spaces and tabs match any run, and the last one none too. */
function lead(prefix: string): string {
  const trail = /\s+$/.test(prefix)
  const parts = prefix.replace(/\s+$/, '').split(/[ \t]+/)
  const body = parts.map(escapeRegex).join('[ \\t]+')
  return body + (trail || !body ? '[ \\t]*' : '')
}

/**
 * A regular expression for the text from `start` to `end` in `line`: the line's start as written, the value as the
 * first group, and what comes after it, if anything, to end the value.
 */
export function textSelector(line: string, start: number, end: number): string {
  const prefix = line.slice(0, start)
  const value = line.slice(start, end).trim()
  const rest = line.slice(end)
  const head = '^' + lead(prefix)
  if (/^https?:\/\//i.test(value)) return `${head}(https?://\\S+)`
  const next = rest.trimStart()
  if (!next) return `${head}(.+?)[ \\t]*$`
  return `${head}(.+?)${rest !== next ? '[ \\t]*' : ''}${escapeRegex(next[0])}`
}

/** Where the value starts in a line clicked as a whole: after `name:` or `name =` if it has one. */
export function valueStart(line: string): number {
  const m = /^[^:=]{1,60}?[:=][ \t]*/.exec(line)
  return m && m[0].length < line.trimEnd().length ? m[0].length : line.length - line.trimStart().length
}

/** A way to tell a text file is the store's: its first line that isn't the title, as written. */
export function textRequire(lines: string[], title: string): string {
  const t = title.trim().toLowerCase()
  const line = lines.find((l) => l.trim() && (!t || !l.toLowerCase().includes(t)))
  if (!line) return ''
  const s = line.trim().slice(0, 48)
  return `^${line.length > line.trimStart().length ? '[ \\t]*' : ''}${escapeRegex(s)}`
}

/** A string in TOML: as written between single quotes when it can be, so a pattern's backslashes stay as they are. */
export function tomlLiteral(s: string): string {
  return /['\n\r\x00-\x08\x0b-\x1f\x7f]/.test(s) ? JSON.stringify(s) : `'${s}'`
}
const tomlString = (s: string) => JSON.stringify(s)
const list = (s: string) => s.split(',').map((x) => x.trim()).filter(Boolean)

/** A format as the form has it. */
export interface Draft {
  name: string
  syntax: 'json' | 'text'
  /** File names it reads, comma-separated. */
  files: string
  /** Ways to tell the file is the store's, one selector a line. */
  require: string
  /** Who sells everything it describes, when the file doesn't say. */
  seller: string
  /** Tags for everything it describes, comma-separated. */
  tags: string
  fields: Record<FieldKey, string>
}

export function emptyDraft(): Draft {
  return {
    name: '',
    syntax: 'json',
    files: '',
    require: '',
    seller: '',
    tags: '',
    fields: Object.fromEntries(FIELDS.map((f) => [f.key, ''])) as Record<FieldKey, string>,
  }
}

/** The draft as a format file. */
export function draftToml(d: Draft, from?: string): string {
  const out: string[] = []
  if (from) out.push(`# ${d.name.trim() || 'A store'}'s ${from}, described with Hoard's format wizard.`)
  out.push(`name = ${tomlString(d.name.trim())}`)
  if (d.seller.trim()) out.push(`seller = ${tomlString(d.seller.trim())}`)
  out.push(`files = [${list(d.files).map((f) => tomlString(f.toLowerCase())).join(', ')}]`)
  if (d.syntax === 'text') out.push('syntax = "text"')
  const req = d.require.split('\n').map((r) => r.trim()).filter(Boolean)
  if (req.length) out.push(`require = [${req.map(tomlLiteral).join(', ')}]`)
  const tags = list(d.tags)
  if (tags.length) out.push(`tags = [${tags.map(tomlString).join(', ')}]`)
  const fields = FIELDS.filter((f) => d.fields[f.key].trim())
  if (fields.length) {
    out.push('', '[fields]')
    for (const f of fields) out.push(`${f.key} = ${tomlLiteral(d.fields[f.key].trim())}`)
  }
  return out.join('\n') + '\n'
}

/** A path as a key for maps of picked values. */
export const pathKey = (p: JsonPath) => JSON.stringify(p)

/** A format's name as a file name, as the server makes it: lowercase ASCII letters and digits, other runs a dash. */
export function slug(name: string): string {
  return name.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-+|-+$/g, '')
}

/** The `name` a format's TOML gives, if it can be seen without reading the whole of it. */
export function tomlName(toml: string): string {
  const m = /^\s*name\s*=\s*(?:"((?:[^"\\]|\\.)*)"|'([^']*)')/m.exec(toml)
  if (!m) return ''
  if (m[2] !== undefined) return m[2]
  try {
    return JSON.parse(`"${m[1]}"`) as string
  } catch {
    return m[1]
  }
}
