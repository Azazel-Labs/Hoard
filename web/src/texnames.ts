// How texture maps are named in what Hoard sends (Settings → Downloads). The server does the renaming
// (crates/hoardd/src/naming.rs); this mirrors it so Settings can show an example as the pattern is typed.
import type { MapRole } from './api'
import { t } from './i18n'
import { ROLE_LABEL, ROLE_NAME } from './materials'

export interface TextureNames {
  on: boolean
  /** `{name}` the material run together, `{map}` the map's suffix, `{size}` its size when the set has several. */
  pattern: string
  /** Suffix by role. */
  maps: Record<string, string>
}

export const ROLES = Object.keys(ROLE_LABEL) as MapRole[]

/** Short suffixes, game-engine style: `T_Rock_BC`, `T_Rock_N`. */
export const SHORT: { pattern: string; maps: Record<MapRole, string> } = {
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
  },
}

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
  return null
}
