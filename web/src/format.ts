// Sizes and lengths, worded the reader's way.
import { formatNumber } from './i18n'

export function humanBytes(b: number): string {
  const units = ['B', 'KB', 'MB', 'GB', 'TB']
  let v = b
  let i = 0
  while (v >= 1024 && i < units.length - 1) {
    v /= 1024
    i++
  }
  const digits = v < 10 && i > 0 ? 1 : 0
  return `${formatNumber(v, { minimumFractionDigits: digits, maximumFractionDigits: digits, useGrouping: false })} ${units[i]}`
}

export function formatDuration(secs: number): string {
  if (secs < 10) return `${formatNumber(secs, { minimumFractionDigits: 2, maximumFractionDigits: 2 })} s`
  const m = Math.floor(secs / 60)
  const s = Math.round(secs % 60)
  return m ? `${m}:${String(s).padStart(2, '0')}` : `${s} s`
}
