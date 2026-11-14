// Sizes people type and read: "100 GB", "512mb", "1.5 TB". The server keeps limits in MiB; a unit here is binary
// (1 GB = 1024 MB), the way humanBytes shows sizes everywhere else. A bare number is gigabytes, and the field
// rewrites it with its unit once it's entered, so nobody is left guessing what "100" meant.
import { formatNumber, language } from './i18n'

const UNITS: Record<string, number> = { k: 1 / 1024, m: 1, g: 1024, t: 1024 * 1024 }

/** "100 GB" -> 102400 (MiB). Null when it isn't a size. */
export function parseMiB(text: string): number | null {
  const m = plainNumber(text.trim()).match(/^(\d+(?:\.\d+)?|\.\d+)\s*([kmgt])?(?:i?b)?$/i)
  if (!m) return null
  const n = Number(m[1]) * UNITS[(m[2] ?? 'g').toLowerCase()]
  return Number.isFinite(n) && n > 0 ? Math.max(1, Math.round(n)) : null
}

/** The number in `text` written with a `.` decimal point and no grouping, whichever way the portal's language
 *  writes numbers (formatMiB writes 1.5 GB as "1,5 GB" in German, and that has to read back as 1.5). */
function plainNumber(text: string): string {
  const parts = new Intl.NumberFormat(language()).formatToParts(12345.6)
  const decimal = parts.find((p) => p.type === 'decimal')?.value ?? '.'
  if (decimal === '.') return text.replace(/,/g, '')
  // A grouping mark only counts as one when three digits follow; "1.5" typed the English way stays 1.5.
  const grouped = text.replace(/[.,'\u2019\s\u00a0\u202f](?=\d{3}(?!\d))/g, (c) => (c === decimal ? c : ''))
  return grouped.replace(decimal, '.')
}

/** 102400 -> "100 GB", 1536 -> "1.5 GB", 300 -> "300 MB". */
export function formatMiB(mib: number): string {
  const units: [string, number][] = [['TB', 1024 * 1024], ['GB', 1024], ['MB', 1]]
  for (const [unit, size] of units) {
    if (mib >= size) {
      const v = mib / size
      // Up to two decimals, without trailing zeros: 1.5 GB, 1.25 TB, 16 GB.
      return `${formatNumber(Number(v.toFixed(2)))} ${unit}`
    }
  }
  return `${mib} MB`
}
