// Materials made from texture maps (Rock_BaseColor.png, Rock_Normal.png, … become one "Rock" material): what each
// map is for, in words, and which of a set's sizes to show. No three.js here; the grid and the detail panel use it.
import type { Asset, MapRole, MaterialInfo, MaterialMap } from './api'
import { t } from './i18n'

/** Each kind of map in words, in the current language. Getters, so `ROLE_LABEL[role]` reads it at render time. */
export const ROLE_LABEL: Readonly<Record<MapRole, string>> = {
  get base_color() { return t('material-role-base-color') },
  get normal() { return t('material-role-normal') },
  get normal_dx() { return t('material-role-normal-dx') },
  get roughness() { return t('material-role-roughness') },
  get gloss() { return t('material-role-gloss') },
  get metalness() { return t('material-role-metalness') },
  get ao() { return t('material-role-ao') },
  get orm() { return t('material-role-orm') },
  get mra() { return t('material-role-mra') },
  get rma() { return t('material-role-rma') },
  get height() { return t('material-role-height') },
  get bump() { return t('material-role-bump') },
  get cavity() { return t('material-role-cavity') },
  get specular() { return t('material-role-specular') },
  get opacity() { return t('material-role-opacity') },
  get emissive() { return t('material-role-emissive') },
  get translucency() { return t('material-role-translucency') },
}

/** Maps the preview has no slot for: they're listed, but don't change how it looks. */
/** Each kind of map as it reads mid-sentence (“Show the normal map”), in the current language. */
export const ROLE_NAME: Readonly<Record<MapRole, string>> = {
  get base_color() { return t('material-role-base-color-name') },
  get normal() { return t('material-role-normal-name') },
  get normal_dx() { return t('material-role-normal-dx-name') },
  get roughness() { return t('material-role-roughness-name') },
  get gloss() { return t('material-role-gloss-name') },
  get metalness() { return t('material-role-metalness-name') },
  get ao() { return t('material-role-ao-name') },
  get orm() { return t('material-role-orm-name') },
  get mra() { return t('material-role-mra-name') },
  get rma() { return t('material-role-rma-name') },
  get height() { return t('material-role-height-name') },
  get bump() { return t('material-role-bump-name') },
  get cavity() { return t('material-role-cavity-name') },
  get specular() { return t('material-role-specular-name') },
  get opacity() { return t('material-role-opacity-name') },
  get emissive() { return t('material-role-emissive-name') },
  get translucency() { return t('material-role-translucency-name') },
}

export const UNUSED_ROLES = new Set<MapRole>(['specular', 'translucency'])

export function isMaterialAsset(a: Asset): boolean {
  return a.kind === 'material' && typeof a.meta.material_maps === 'number'
}

/** A size as people say it: "2K", or "512 px" for a set smaller than that. */
export const sizeLabel = (r: string) => (/^\d+$/.test(r) ? `${r} px` : r)

/** "8 maps · 1K–4K" for a tile. The server lists the sizes largest first. */
export function materialSummary(a: Asset): string {
  const n = Number(a.meta.material_maps) || 0
  const res = Array.isArray(a.meta.resolutions) ? (a.meta.resolutions as string[]) : []
  const parts = [t('material-maps', { count: n })]
  if (res.length) {
    const lo = res[res.length - 1]
    const hi = res[0]
    parts.push(res.length === 1 ? sizeLabel(hi) : /^\d+$/.test(hi) ? `${lo}–${hi} px` : `${lo}–${hi}`)
  }
  return parts.join(' · ')
}

/** The longest side of a map in pixels, when known. */
export const mapPx = (m: MaterialMap) => Math.max(m.width ?? 0, m.height ?? 0)

/** One map per role: of a set that comes in several sizes, the smallest that's at least `want` pixels across
 *  (else the largest there is). A file the browser reads itself wins a tie. */
export function pickMaps(info: MaterialInfo, want: number): Map<MapRole, MaterialMap> {
  const out = new Map<MapRole, MaterialMap>()
  for (const m of info.maps) {
    const prev = out.get(m.role)
    if (!prev || better(m, prev, want)) out.set(m.role, m)
  }
  return out
}

const READABLE = new Set(['png', 'jpg', 'jpeg', 'webp'])

function better(a: MaterialMap, b: MaterialMap, want: number): boolean {
  const pa = mapPx(a)
  const pb = mapPx(b)
  if (pa !== pb) {
    const okA = pa >= want
    const okB = pb >= want
    if (okA !== okB) return okA
    return okA ? pa < pb : pa > pb
  }
  return READABLE.has(a.ext.toLowerCase()) && !READABLE.has(b.ext.toLowerCase())
}

/** The distinct sizes of a set, largest first ("4K", "2K"). */
export function resolutions(info: MaterialInfo): string[] {
  const seen = new Map<string, number>()
  for (const m of info.maps) if (m.resolution && !seen.has(m.resolution)) seen.set(m.resolution, mapPx(m))
  return [...seen.entries()].sort((a, b) => b[1] - a[1]).map(([r]) => r)
}
