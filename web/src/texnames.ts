// How texture maps are named in what Hoard sends (Settings → Downloads). The server does the renaming
// (crates/hoardd/src/naming.rs); this mirrors it so Settings can show an example as the pattern is typed.
import type { MapRole } from './api'
import { t } from './i18n'
import { ROLE_LABEL, ROLE_NAME } from './materials'

export interface TextureNames {
  on: boolean
  /** `{name}` the material run together, `{map}` the map's suffix, `{size}` its size when the set has several. */
  pattern: string
  /** Suffix by role, and by glTF slot ([`SLOTS`]). */
  maps: Record<string, string>
  /** Endings of your own: a texture that isn't a map whose name ends in `from` goes out with `to` instead. */
  endings: Ending[]
}

export interface Ending { from: string; to: string }

export const ROLES = Object.keys(ROLE_LABEL) as MapRole[]

/** The slots a glTF material fills with a texture beyond the kinds of map, in words. What fills one inside a model
 * is named after it in GLB downloads. Getters, so they're in the language of the moment. */
export const SLOT_LABEL: Readonly<Record<string, string>> = {
  get metallic_roughness() { return t('texnames-slot-metallic-roughness') },
  get specular_color() { return t('texnames-slot-specular-color') },
  get specular_glossiness() { return t('texnames-slot-specular-glossiness') },
  get clearcoat() { return t('texnames-slot-clearcoat') },
  get clearcoat_roughness() { return t('texnames-slot-clearcoat-roughness') },
  get clearcoat_normal() { return t('texnames-slot-clearcoat-normal') },
  get sheen_color() { return t('texnames-slot-sheen-color') },
  get sheen_roughness() { return t('texnames-slot-sheen-roughness') },
  get transmission() { return t('texnames-slot-transmission') },
  get thickness() { return t('texnames-slot-thickness') },
  get diffuse_transmission() { return t('texnames-slot-diffuse-transmission') },
  get diffuse_transmission_color() { return t('texnames-slot-diffuse-transmission-color') },
  get iridescence() { return t('texnames-slot-iridescence') },
  get iridescence_thickness() { return t('texnames-slot-iridescence-thickness') },
  get anisotropy() { return t('texnames-slot-anisotropy') },
}

export const SLOTS = Object.keys(SLOT_LABEL)

/** Short suffixes, game-engine style: `T_Rock_BC`, `T_Rock_N`. */
export const SHORT: { pattern: string; maps: Record<string, string> } = {
  pattern: 'T_{name}_{map}',
  maps: {
    base_color: 'BC',
    normal: 'N',
    normal_dx: 'N_DX',
    roughness: 'R',
    gloss: 'G',
    metalness: 'M',
    ao: 'AO',
    orm: 'ORM',
    mra: 'MRA',
    rma: 'RMA',
    height: 'H',
    bump: 'B',
    cavity: 'C',
    specular: 'S',
    opacity: 'O',
    emissive: 'E',
    translucency: 'T',
    metallic_roughness: 'MR',
    specular_color: 'SC',
    specular_glossiness: 'SG',
    clearcoat: 'CC',
    clearcoat_roughness: 'CCR',
    clearcoat_normal: 'CCN',
    sheen_color: 'SHC',
    sheen_roughness: 'SHR',
    transmission: 'TR',
    thickness: 'TH',
    diffuse_transmission: 'DT',
    diffuse_transmission_color: 'DTC',
    iridescence: 'IR',
    iridescence_thickness: 'IRT',
    anisotropy: 'AN',
  },
}

/** The endings worth saving: a row left empty isn't one. */
export const usedEndings = (endings: Ending[]) => endings.filter((e) => e.from.trim() || e.to.trim())

/** "Stand Wood Old 05" -> "StandWoodOld05". */
export const runTogether = (name: string) =>
  name
    .split(/[^\p{L}\p{N}]+/u)
    .filter(Boolean)
    .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
    .join('')

const tidy = (s: string) => s.replace(/([_\-. ])[_\-. ]+/g, '$1').replace(/^[_\-. ]+|[_\-. ]+$/g, '')

/** A map's file name without extension, as the server would give it. */
export function stem(n: TextureNames, material: string, role: MapRole, size?: string): string {
  const out = n.pattern.replaceAll('{name}', runTogether(material)).replaceAll('{map}', (n.maps[role] ?? role).trim())
  if (size) return out.includes('{size}') ? out.replaceAll('{size}', size) : `${out}_${size}`
  return out.includes('{size}') ? tidy(out.replaceAll('{size}', '')) : out
}

const BAD = /[<>:"/\\|?*\p{Cc}]/u

/** What's wrong with the convention, in the server's words (in the reader's language), or null. */
export function problem(n: TextureNames): string | null {
  if (!n.pattern.includes('{map}')) return t('texnames-needs-map')
  const rest = n.pattern.replaceAll('{name}', '').replaceAll('{map}', '').replaceAll('{size}', '')
  if (/[{}]/.test(rest)) return t('texnames-placeholders')
  if (BAD.test(rest)) return t('texnames-bad-characters')
  for (const r of ROLES) {
    const s = n.maps[r] ?? ''
    if (!s.trim()) return t('texnames-no-suffix', { role: ROLE_NAME[r] })
    if (BAD.test(s)) return t('texnames-bad-suffix', { role: ROLE_NAME[r] })
  }
  for (const k of SLOTS) {
    const s = n.maps[k] ?? ''
    if (!s.trim()) return t('texnames-slot-no-suffix', { slot: SLOT_LABEL[k] })
    if (BAD.test(s)) return t('texnames-slot-bad-suffix', { slot: SLOT_LABEL[k] })
  }
  const seen = new Set<string>()
  for (const e of usedEndings(n.endings)) {
    const from = e.from.trim()
    if (!from) return t('texnames-ending-missing')
    if (BAD.test(from) || BAD.test(e.to.trim())) return t('texnames-ending-bad')
    if (seen.has(from.toLowerCase())) return t('texnames-ending-twice', { ending: from })
    seen.add(from.toLowerCase())
  }
  return null
}
