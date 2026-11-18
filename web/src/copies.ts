// Near copies (the same picture, sound or model at another size, rate or format): how one is described in a line.
import { humanBytes, type NearCopy } from './api'
import { formatNumber, t } from './i18n'

/** "PNG · 1.2 MB · 1024 × 1024", or "WAV · 70 KB · 22.05 kHz · 16-bit". */
export function copyFacts(n: NearCopy): string {
  const out = [n.ext.toUpperCase(), humanBytes(n.size)]
  if (n.width && n.height) out.push(`${formatNumber(n.width)} × ${formatNumber(n.height)}`)
  if (n.sample_rate) out.push(`${formatNumber(n.sample_rate / 1000, { maximumFractionDigits: 2 })} kHz`)
  if (n.bit_depth) out.push(t('asset-meta-bits', { bits: n.bit_depth }))
  return out.join(' · ')
}
