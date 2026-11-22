// How the search box reads what's typed, as the server does (`words` in db/search.rs): for marking it up as it's
// typed, and for the chips of what a search leaves out.
import { ref } from 'vue'
import { api } from './api'

/** One piece of the search text, by where it is in it. */
export interface Piece {
  start: number
  end: number
  /** `word` a word or "a phrase"; `or` either side; `space` what's between. */
  kind: 'word' | 'or' | 'space'
  /** After a `-`: left out. */
  not?: boolean
  /** `pack:…`: a word of the pack's name. */
  pack?: boolean
  /** A tag the library has, as typed (`unreal`, `style:sci-fi`, or `forest` for `biome:forest`). */
  tag?: boolean
  /** The word itself, without its marks (`-`, `pack:`, quotes). */
  text?: string
}

/** Split search text into its pieces; `isTag` says which words are tags. Every character is in exactly one piece. */
export function pieces(q: string, isTag: (word: string) => boolean = () => false): Piece[] {
  const out: Piece[] = []
  let i = 0
  const space = (start: number) => {
    if (i > start) out.push({ start, end: i, kind: 'space' })
  }
  while (i < q.length) {
    const s = i
    while (i < q.length && /\s/.test(q[i])) i++
    space(s)
    if (i >= q.length) break
    const start = i
    if (q[i] === '|') {
      i++
      out.push({ start, end: i, kind: 'or' })
      continue
    }
    const not = q[i] === '-' && i + 1 < q.length && !/\s/.test(q[i + 1])
    if (not) i++
    const pack = q.slice(i, i + 5).toLowerCase() === 'pack:'
    if (pack) i += 5
    let text: string
    if (q[i] === '"') {
      const close = q.indexOf('"', i + 1)
      text = q.slice(i + 1, close < 0 ? q.length : close)
      i = close < 0 ? q.length : close + 1
    } else {
      const from = i
      while (i < q.length && !/[\s"|]/.test(q[i])) i++
      text = q.slice(from, i)
    }
    if (!not && !pack && text.toLowerCase() === 'or' && q[start] !== '"') {
      out.push({ start, end: i, kind: 'or' })
      continue
    }
    // Marks with nothing after them (a lone `-` is a word; `pack:` with no name) count for nothing, as on the server.
    const counts = /[\p{L}\p{N}]/u.test(text)
    out.push({ start, end: i, kind: 'word', text, ...(counts && not && { not }), ...(counts && pack && { pack }), ...(counts && !pack && isTag(text) && { tag: true }) })
  }
  return out
}

/** Whether any piece is more than a plain word, so marking the text up shows something. */
export const marked = (ps: Piece[]) => ps.some((p) => p.kind === 'or' || p.not || p.pack || p.tag)

/** The search text without the piece at `start`, and the space before it. */
export function withoutPiece(q: string, p: Piece): string {
  return (q.slice(0, p.start).replace(/\s+$/, '') + ' ' + q.slice(p.end).replace(/^\s+/, '')).trim()
}

/** How a word is written in the search box: in quotes when it has a space or a quote mark in it. */
export function asWord(text: string): string {
  return /[\s"|]/.test(text) ? `"${text.replaceAll('"', '')}"` : text
}

/** The search text leaving `text` out too, unless it does already; and without it as a word to find. */
export function leaveOut(q: string, text: string, pack = false): string {
  const word = (pack ? 'pack:' : '') + asWord(text)
  const lower = text.toLowerCase()
  const ps = pieces(q)
  if (ps.some((p) => p.not && !!p.pack === pack && p.text?.toLowerCase() === lower)) return q
  let rest = q
  for (const p of [...ps].reverse()) {
    if (p.kind === 'word' && !p.not && !!p.pack === pack && p.text?.toLowerCase() === lower) rest = withoutPiece(rest, p)
  }
  return `${rest.trim()} -${word}`.trim()
}

/** The library's tags, lowercased, with the part after the colon of each too: fetched once, when first wanted. */
const tagWords = ref<Set<string> | null>(null)
let fetching = false
export function knownTags(): Set<string> | null {
  if (!tagWords.value && !fetching) {
    fetching = true
    api
      .allTags('', 5000)
      .then((tags) => {
        const words = new Set<string>()
        for (const { value } of tags) {
          const v = value.toLowerCase()
          words.add(v)
          const colon = v.indexOf(':')
          if (colon > 0) words.add(v.slice(colon + 1))
        }
        tagWords.value = words
      })
      .catch(() => {
        fetching = false
      })
  }
  return tagWords.value
}
