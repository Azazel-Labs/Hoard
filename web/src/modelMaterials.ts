// A loaded model's materials made fit to show: the texture slots a material carries, old Phong and Lambert
// materials remade as PBR ones, the mix-ups that make a model invisible or black undone, and what a texture's name
// says it is.
import * as THREE from 'three/webgpu'
import { stemOf } from './modelFiles'

/** Texture slots a material can carry; the "textures off" switch empties them and puts them back. */
export const MAP_SLOTS = [
  'map', 'normalMap', 'bumpMap', 'emissiveMap', 'aoMap', 'roughnessMap', 'metalnessMap', 'alphaMap', 'lightMap',
  'specularMap', 'displacementMap',
] as const
export type MapSlot = (typeof MAP_SLOTS)[number]
export type AnyMaterial = THREE.Material & Partial<Record<MapSlot, THREE.Texture | null>> & {
  color?: THREE.Color; wireframe?: boolean; vertexColors?: boolean
}

/** FBX and OBJ files arrive with Phong or Lambert materials, which only see direct lights: no environment, so they
 *  read flat and dark in a viewer. Remake them as standard PBR materials (same color, maps and transparency) so the
 *  environment lights them like everything else. */
export function toStandard(m: THREE.Material): THREE.Material {
  const old = m as THREE.MeshPhongMaterial & THREE.MeshLambertMaterial
  if (!old.isMeshPhongMaterial && !old.isMeshLambertMaterial) return m
  const shininess = old.isMeshPhongMaterial ? old.shininess : 0
  const s = new THREE.MeshStandardMaterial({
    name: old.name,
    color: old.color,
    map: old.map,
    normalMap: old.normalMap,
    normalScale: old.normalScale,
    bumpMap: old.bumpMap,
    bumpScale: old.bumpScale,
    emissive: old.emissive,
    emissiveMap: old.emissiveMap,
    emissiveIntensity: old.emissiveIntensity,
    alphaMap: old.alphaMap,
    aoMap: old.aoMap,
    aoMapIntensity: old.aoMapIntensity,
    lightMap: old.lightMap,
    displacementMap: old.displacementMap,
    displacementScale: old.displacementScale,
    opacity: old.opacity,
    transparent: old.transparent,
    alphaTest: old.alphaTest,
    side: old.side,
    vertexColors: old.vertexColors,
    flatShading: old.flatShading,
    // Phong shininess runs 0..~100+; glossy surfaces get a tighter highlight, but nothing turns into a mirror.
    roughness: Math.min(1, Math.max(0.35, 1 - Math.sqrt(Math.max(0, shininess) / 120))),
    metalness: 0,
  })
  old.dispose()
  return s
}

/** Fix what commonly makes an imported model invisible or black in a viewer. */
export function sanitize(m: AnyMaterial, geometry: THREE.BufferGeometry | undefined) {
  // Exporters disagree on what FBX's TransparencyFactor and MTL's `d`/`Tr` mean; a fully transparent material with
  // no alpha texture is almost always that mix-up, not an invisible object.
  if (m.transparent && m.opacity < 0.02 && !m.alphaMap) {
    m.opacity = 1
    m.transparent = false
  }
  // Vertex colors that are all black (an empty color layer) multiply the whole model to black.
  const color = geometry?.attributes.color
  if (m.vertexColors && color) {
    let max = 0
    const step = Math.max(1, Math.floor(color.count / 2000))
    for (let i = 0; i < color.count && max < 0.02; i += step) max = Math.max(max, color.getX(i), color.getY(i), color.getZ(i))
    if (max < 0.02) m.vertexColors = false
  }
  for (const slot of ['map', 'emissiveMap'] as const) {
    const t = m[slot]
    if (t) t.colorSpace = THREE.SRGBColorSpace
  }
}

/** What a texture's name says it is ("Rock_Albedo", "rock_n", "Rock-Normal"); `skip` for maps a viewer can't use
 *  as they are (height, packed ORM, gloss), null when the name doesn't say. */
export function roleOf(name: string): MapSlot | 'skip' | null {
  const n = stemOf(name)
  // A map-type suffix after a separator: "_n", "-ao", ".bc".
  const end = (s: string) => new RegExp(`(^|[-_. ])(${s})$`).test(n)
  if (/normal|nrm/.test(n) || end('n|nor|norm')) return 'normalMap'
  if (/emissi|emit|glow/.test(n) || end('e')) return 'emissiveMap'
  if (/rough/.test(n) || end('r')) return 'roughnessMap'
  if (/metal/.test(n) || end('m|mt')) return 'metalnessMap'
  if (/occlusion/.test(n) || end('ao|occ')) return 'aoMap'
  if (/height|disp|bump|mask|opacity|alpha|spec|gloss|smooth/.test(n) || end('h|orm|arm|mra|rma|s|g')) return 'skip'
  if (/albedo|base.?colou?r|diffuse/.test(n) || end('diff|dif|col|color|color|d|bc|a|c')) return 'map'
  return null
}
