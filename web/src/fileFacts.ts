// The facts a file states or was measured to have, worded for the detail panel.
import { formatDuration, type Asset } from './api'
import { formatNumber, t } from './i18n'
import { findKey, keyName } from './keys'
import { resolutions, sizeLabel } from './materials'

/** A number with one decimal, the reader's way (44.1). */
const oneDecimal = (n: number) => formatNumber(n, { minimumFractionDigits: 1, maximumFractionDigits: 1 })
/** Labels for metadata the file names by key. */
const TEXT_FIELD_LABEL: Record<string, () => string> = {
  title: () => t('asset-meta-title'),
  artist: () => t('asset-meta-artist'),
  album: () => t('asset-meta-album'),
  genre: () => t('asset-meta-genre'),
  generator: () => t('asset-meta-generator'),
}
const COUNT_LABEL: Record<string, () => string> = {
  meshes: () => t('asset-meta-meshes'),
  materials: () => t('asset-meta-materials'),
  animations: () => t('asset-meta-animations'),
  skins: () => t('asset-meta-skins'),
}

/** A sound's measured envelope, as `meta.envelope` names it. */
const ENVELOPE: Record<string, () => string> = {
  hit: () => t('asset-meta-envelope-hit'),
  swell: () => t('asset-meta-envelope-swell'),
  sustained: () => t('asset-meta-envelope-sustained'),
}

/** What the file says of itself and what was measured of it, as [label, value, tag?] rows; a row's tag (its artist,
 *  its genre) finds everything else with that fact. */
export function fileFacts(asset: Asset): [string, string, string?][] {
  const m = asset.meta
  const material = asset.material ?? null
  const factTag = (key: string) => asset.tags.find((x) => x.tag.startsWith(`${key}:`))?.tag
  const rows: [string, string, string?][] = []
  const num = (k: string) => (typeof m[k] === 'number' ? (m[k] as number) : undefined)
  if (num('width') !== undefined) rows.push([t('asset-meta-size'), `${m.width} × ${m.height} px`])
  if (num('duration_secs') !== undefined) rows.push([t('asset-meta-length'), formatDuration(num('duration_secs')!)])
  if (num('sample_rate') !== undefined) rows.push([t('asset-meta-sample-rate'), `${oneDecimal(num('sample_rate')! / 1000)} kHz`])
  if (num('channels') !== undefined) {
    const ch = num('channels') === 1 ? t('asset-meta-mono') : num('channels') === 2 ? t('asset-meta-stereo') : String(m.channels)
    rows.push([t('asset-meta-channels'), ch])
  }
  if (num('bit_depth') !== undefined) rows.push([t('asset-meta-bit-depth'), t('asset-meta-bits', { bits: num('bit_depth')! })])
  if (num('peak_dbfs') !== undefined) {
    const peak = oneDecimal(num('peak_dbfs')!)
    const rms = oneDecimal(num('rms_dbfs')!)
    rows.push([t('asset-meta-level'), m.clipped ? t('asset-meta-level-clips', { peak, rms }) : t('asset-meta-level-value', { peak, rms })])
  }
  if (num('loop_start') !== undefined && num('sample_rate')) {
    const sr = num('sample_rate')!
    rows.push([t('asset-meta-loop'), t('asset-meta-loop-value', { start: formatDuration(num('loop_start')! / sr), end: formatDuration(num('loop_end')! / sr) })])
  } else if (m.one_shot === true) rows.push([t('asset-meta-playback'), t('asset-meta-one-shot')])
  else if (m.loops_cleanly === true) rows.push([t('asset-meta-playback'), t('asset-meta-loops-cleanly')])
  // What was measured of how it sounds.
  if (num('attack_ms') !== undefined) {
    const time = (ms: number) => (ms < 1000 ? `${formatNumber(ms)} ms` : `${oneDecimal(ms / 1000)} s`)
    const shape = ENVELOPE[m.envelope as string]?.()
    const times = t('asset-meta-envelope-times', { attack: time(num('attack_ms')!), tail: time(num('tail_ms') ?? 0) })
    rows.push([t('asset-meta-envelope'), shape ? `${shape}, ${times}` : times])
  }
  if (num('brightness_hz') !== undefined) {
    const noise = Math.round((num('noisiness') ?? 0) * 100)
    rows.push([t('asset-meta-tone'), t('asset-meta-tone-value', { khz: oneDecimal(num('brightness_hz')! / 1000), noise })])
  }
  const sweep = num('sweep_octaves')
  if (sweep !== undefined && Math.abs(sweep) >= 0.5) {
    const octaves = Math.abs(sweep)
    rows.push([t('asset-meta-pitch'), sweep > 0 ? t('asset-meta-pitch-rises', { octaves }) : t('asset-meta-pitch-falls', { octaves })])
  }
  if (num('tempo_bpm') !== undefined) rows.push([t('asset-meta-tempo'), `${m.tempo_bpm} BPM`, factTag('bpm')])
  else if (num('measured_bpm') !== undefined) {
    const [bpm, alt] = [num('measured_bpm')!, num('measured_bpm_alt')]
    rows.push([t('asset-meta-tempo'), alt === undefined ? t('asset-meta-tempo-heard', { bpm }) : t('asset-meta-tempo-heard-or', { bpm, alt }), factTag('bpm')])
  }
  const key = findKey(String(m.key ?? m.measured_key ?? ''))
  if (key) {
    const value = t('asset-meta-key-value', { key: keyName(key), camelot: key.camelot })
    rows.push([t('asset-meta-key'), m.key ? value : t('asset-meta-key-heard', { key: value }), factTag('key')])
  }
  if (typeof m.root_note === 'string') rows.push([t('asset-meta-root-note'), m.root_note as string])
  if (num('cue_points')) rows.push([t('asset-meta-cue-markers'), String(m.cue_points)])
  if (typeof m.ucs_category === 'string') {
    rows.push(['UCS', [m.ucs_category, m.ucs_subcategory].filter(Boolean).join(' / ') + (m.ucs_catid ? ` (${m.ucs_catid})` : '')])
  }
  if (typeof m.library === 'string') rows.push([t('asset-meta-library'), m.library as string])
  if (Array.isArray(m.keywords) && m.keywords.length) rows.push([t('asset-meta-keywords'), (m.keywords as string[]).join(', ')])
  for (const k of ['title', 'artist', 'album', 'genre', 'generator']) {
    if (typeof m[k] === 'string') rows.push([TEXT_FIELD_LABEL[k](), m[k] as string, factTag(k)])
  }
  if (typeof m.blender_version === 'string') rows.push([t('asset-meta-saved-with'), `Blender ${m.blender_version}`])
  if (typeof m.color_mode === 'string') rows.push([t('asset-meta-color-mode'), (m.color_mode as string).toUpperCase()])
  if (num('layers') !== undefined) rows.push([t('asset-meta-layers'), String(m.layers)])
  if (m.hdr === true) rows.push([t('asset-meta-range'), t('asset-meta-hdr')])
  if (m.large_document === true) rows.push([t('asset-meta-format'), t('asset-meta-large-document')])
  const ai = m.ai as { made?: string; by?: string; how?: string } | undefined
  if (ai?.made) {
    const made = ai.made === 'edited' ? t('asset-meta-ai-edited') : t('asset-meta-ai-made')
    // The server names where it read this with one of a few fixed English phrases.
    const how = ai.how === 'generator settings' ? t('asset-meta-ai-how-generator')
      : ai.how === 'IPTC digital source type' ? t('asset-meta-ai-how-iptc')
      : ai.how ?? t('asset-meta-ai-how-metadata')
    rows.push([t('asset-meta-ai'), ai.by ? t('asset-meta-ai-by-says', { made, by: ai.by, how }) : t('asset-meta-ai-says', { made, how })])
  } else if (m.content_credentials === true) rows.push([t('asset-meta-provenance'), t('asset-meta-c2pa')])
  if (typeof m.compression === 'string') rows.push([t('asset-meta-compression'), m.compression as string])
  if (typeof m.note === 'string') rows.push([t('asset-meta-note'), m.note as string])
  if (material) {
    rows.push([t('asset-meta-maps'), String(material.maps.length)])
    const sizes = resolutions(material)
    if (sizes.length) rows.push([t('asset-meta-sizes', { count: sizes.length }), sizes.map(sizeLabel).join(', ')])
  }
  if (num('triangles') !== undefined) rows.push([t('asset-meta-triangles'), formatNumber(num('triangles')!)])
  if (num('vertices') !== undefined) rows.push([t('asset-meta-vertices'), formatNumber(num('vertices')!)])
  for (const k of ['meshes', 'materials', 'animations', 'skins']) {
    if (num(k)) rows.push([COUNT_LABEL[k](), String(m[k])])
  }
  if (typeof m.ue_class === 'string') {
    const cls = m.ue_class as string
    rows.push([t('asset-meta-unreal-asset'), typeof m.ue_engine === 'string' ? t('asset-meta-unreal-saved-with', { class: cls, engine: m.ue_engine }) : cls])
    if (m.ue_cooked === true) rows.push([t('asset-meta-package'), t('asset-meta-cooked')])
  }
  if (m.tileable === true) rows.push([t('asset-meta-tiling'), t('asset-meta-seamless')])
  if (typeof m.analysis_error === 'string') rows.push([t('asset-meta-note'), t('asset-meta-analysis-error', { error: m.analysis_error })])
  return rows
}
