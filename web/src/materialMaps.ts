// A material's texture maps as three.js textures, by the slot each fills on a standard material. Shared by the
// material preview and the model viewers (a model that uses a material's maps but names none). Maps arrive small
// first (the 256 px thumbnails) and then at the size asked for, each on its own, so a slow one holds up no other.
// Gloss is turned into roughness, and packed maps into the occlusion/roughness/metal layout three reads (R, G, B).
import * as THREE from 'three/webgpu'
import { asUrl, packFileUrl, RESIZABLE_AS_ITSELF, thumbUrl, type MapRole, type MaterialInfo, type MaterialMap } from './api'
import { mapPx, pickMaps } from './materials'

/** The slots a material preview fills. `ao`, `roughness` and `metalness` share one texture when the set is packed. */
export type Slot = 'color' | 'normal' | 'roughness' | 'metalness' | 'ao' | 'height' | 'bump' | 'opacity' | 'emissive'

export interface MapSet {
  textures: Partial<Record<Slot, THREE.Texture>>
  /** The normal map is DirectX style (green down): read it with the green flipped. */
  normalDx: boolean
  /** Which file fills each slot, to say so and to switch it off by role. */
  roles: Partial<Record<Slot, MapRole>>
}

/** A standard material's property for each slot. */
export const SLOT_PROP = {
  color: 'map',
  normal: 'normalMap',
  roughness: 'roughnessMap',
  metalness: 'metalnessMap',
  ao: 'aoMap',
  height: 'displacementMap',
  bump: 'bumpMap',
  opacity: 'alphaMap',
  emissive: 'emissiveMap',
} as const satisfies Record<Slot, keyof THREE.MeshStandardMaterial>

/** How a channel layout reaches three's R = occlusion, G = roughness, B = metal: the source channel of each. */
const REPACK: Partial<Record<MapRole, [number, number, number]>> = {
  mra: [2, 1, 0], // R metal, G rough, B AO
  rma: [2, 0, 1], // R rough, G metal, B AO
}

const BROWSER = new Set(['png', 'jpg', 'jpeg', 'webp', 'gif', 'bmp', 'avif'])

/** Where a map's pixels come from at about `max` pixels across: the file itself when the browser reads it and it's
 *  no bigger, else a copy the server scales (or converts to PNG) and caches. */
export function mapUrl(m: MaterialMap, packId: string, max: number): string {
  const ext = m.ext.toLowerCase()
  const px = mapPx(m)
  if (BROWSER.has(ext) && px && px <= max) return packFileUrl(packId, m.path)
  const as = BROWSER.has(ext) ? (RESIZABLE_AS_ITSELF[ext] ?? 'png') : 'png'
  return `${asUrl(m, as, max)}&download=0&wait=60`
}

/** Fetch an image, waiting out a conversion the server is still making (202). */
async function fetchImage(url: string, signal: AbortSignal): Promise<Blob> {
  for (let i = 0; i < 4; i++) {
    const res = await fetch(url, { signal })
    if (res.status === 202) continue
    if (!res.ok) throw new Error(`HTTP ${res.status}`)
    return res.blob()
  }
  throw new Error('still converting')
}

interface Want {
  max: number
  /** Image top at v = 1 (three's own shapes, OBJ, FBX); glTF has it at v = 0. */
  flipY: boolean
  /** Hears each full-size map start loading, and calls back whether it loaded: so a viewer can wait for them all. */
  track?: (name: string) => (ok: boolean) => void
}

/** Decode a map for one slot. Colour maps keep their colour profile; data maps are read as raw numbers. */
async function decode(blob: Blob, role: MapRole, want: Want): Promise<THREE.Texture> {
  const colour = role === 'base_color' || role === 'emissive'
  const bitmap = await createImageBitmap(blob, {
    imageOrientation: want.flipY ? 'flipY' : 'from-image',
    premultiplyAlpha: 'none',
    colorSpaceConversion: colour ? 'default' : 'none',
  })
  const repack = REPACK[role]
  let tex: THREE.Texture
  if (role === 'gloss' || repack) {
    const canvas = new OffscreenCanvas(bitmap.width, bitmap.height)
    const ctx = canvas.getContext('2d', { willReadFrequently: true })!
    ctx.drawImage(bitmap, 0, 0)
    bitmap.close()
    const img = ctx.getImageData(0, 0, canvas.width, canvas.height)
    const d = img.data
    if (repack) {
      const [r, g, b] = repack
      for (let i = 0; i < d.length; i += 4) {
        const px = [d[i], d[i + 1], d[i + 2]]
        d[i] = px[r]
        d[i + 1] = px[g]
        d[i + 2] = px[b]
      }
    } else {
      // Gloss is roughness turned over; three reads roughness from green.
      for (let i = 0; i < d.length; i += 4) {
        const v = 255 - d[i]
        d[i] = d[i + 1] = d[i + 2] = v
      }
    }
    ctx.putImageData(img, 0, 0)
    tex = new THREE.CanvasTexture(canvas)
  } else {
    tex = new THREE.Texture(bitmap)
  }
  // The bitmap is already the right way up.
  tex.flipY = false
  tex.colorSpace = colour ? THREE.SRGBColorSpace : THREE.NoColorSpace
  tex.wrapS = tex.wrapT = THREE.RepeatWrapping
  tex.anisotropy = 8
  tex.generateMipmaps = true
  tex.minFilter = THREE.LinearMipmapLinearFilter
  tex.needsUpdate = true
  return tex
}

/** Which file goes in which slot, by role, with the fallbacks: gloss for roughness, cavity for occlusion, a packed
 *  map for whichever of occlusion, roughness and metal has no map of its own, bump only without a normal map. */
function plan(picked: Map<MapRole, MaterialMap>): { slots: Slot[]; map: MaterialMap; role: MapRole }[] {
  const jobs: { slots: Slot[]; map: MaterialMap; role: MapRole }[] = []
  const add = (slots: Slot[], role: MapRole) => {
    const map = picked.get(role)
    if (map && slots.length) jobs.push({ slots, map, role })
    return !!map
  }
  add(['color'], 'base_color')
  if (!add(['normal'], 'normal')) add(['normal'], 'normal_dx')
  const rough = add(['roughness'], 'roughness') || add(['roughness'], 'gloss')
  const metal = add(['metalness'], 'metalness')
  const ao = add(['ao'], 'ao') || add(['ao'], 'cavity')
  const packedSlots = (): Slot[] => [...(ao ? [] : ['ao' as const]), ...(rough ? [] : ['roughness' as const]), ...(metal ? [] : ['metalness' as const])]
  for (const role of ['orm', 'mra', 'rma'] as const) if (picked.has(role)) { add(packedSlots(), role); break }
  add(['height'], 'height')
  if (!picked.has('normal') && !picked.has('normal_dx')) add(['bump'], 'bump')
  add(['opacity'], 'opacity')
  add(['emissive'], 'emissive')
  return jobs
}

/** Load a material's maps, calling `onChange` each time one arrives (thumbnail first, then full size). Returns a
 *  function that stops loading; the textures handed over are the caller's to dispose once replaced. */
export function loadMaps(
  info: MaterialInfo,
  packId: string,
  want: Want,
  onChange: (set: MapSet, replaced: THREE.Texture[]) => void,
): () => void {
  const abort = new AbortController()
  const jobs = plan(pickMaps(info, want.max))
  const set: MapSet = { textures: {}, normalDx: jobs.some((j) => j.role === 'normal_dx'), roles: {} }
  for (const j of jobs) for (const s of j.slots) set.roles[s] = j.role
  const done = new Set<string>()

  const put = (slots: Slot[], tex: THREE.Texture) => {
    const replaced = new Set<THREE.Texture>()
    for (const s of slots) {
      const old = set.textures[s]
      if (old && old !== tex) replaced.add(old)
      set.textures[s] = tex
    }
    // A texture shared by several slots (a packed map) is only gone when no slot holds it.
    const still = new Set(Object.values(set.textures))
    onChange(set, [...replaced].filter((t) => !still.has(t)))
  }

  for (const j of jobs) {
    const key = j.map.id
    const full = mapUrl(j.map, packId, want.max)
    const small = j.map.has_thumb && mapPx(j.map) > 256 ? thumbUrl(j.map) : ''
    const load = async (url: string, isFull: boolean) => {
      const end = isFull ? want.track?.(j.map.name) : undefined
      let ok = false
      try {
        const tex = await decode(await fetchImage(url, abort.signal), j.role, want)
        if (abort.signal.aborted || (!isFull && done.has(key))) {
          tex.dispose()
          return
        }
        if (isFull) done.add(key)
        put(j.slots, tex)
        ok = true
      } catch {
        // A thumbnail that fails is no loss; a full map that fails leaves its thumbnail, if any.
      } finally {
        end?.(ok || abort.signal.aborted)
      }
    }
    if (small) void load(small, false)
    void load(full, true)
  }
  return () => abort.abort()
}

/** Put a material's maps on a model's materials: for a model that names no textures but uses a material's maps.
 *  `tracked` is what the model viewer remembers of each material's maps for its "textures off" switch; a slot the
 *  switch has emptied stays empty, with the new texture remembered for when it comes back. Returns how many kinds
 *  of map there are, and a function that stops loading. `onChange` hears each time maps arrive. */
export function applyToModel(
  info: MaterialInfo,
  packId: string,
  flipY: boolean,
  tracked: Map<THREE.Material, Partial<Record<string, THREE.Texture | null>>>,
  onChange?: () => void,
  track?: Want['track'],
): { count: number; stop: () => void } {
  const roles = new Set<Slot>()
  const stop = loadMaps(info, packId, { max: 2048, flipY, track }, (set, replaced) => {
    for (const [m, maps] of tracked) {
      const mat = m as THREE.MeshStandardMaterial & Record<string, unknown>
      for (const [slot, tex] of Object.entries(set.textures) as [Slot, THREE.Texture][]) {
        const prop = SLOT_PROP[slot]
        if (slot === 'height' || slot === 'bump') continue // A model's own shape, not the swatch's.
        if (prop !== 'map' && !(prop in mat)) continue
        const shown = mat[prop] === (maps[prop] ?? null)
        maps[prop] = tex
        if (shown) {
          if (!mat[prop]) mat.needsUpdate = true
          mat[prop] = tex
        }
      }
      if (set.normalDx && mat.normalScale) mat.normalScale.set(1, -1)
      mat.color?.set(0xffffff)
      if (set.textures.emissive && mat.emissive) mat.emissive.set(0xffffff)
      if (set.textures.opacity) {
        mat.alphaTest = 0.5
        mat.needsUpdate = true
      }
    }
    for (const t of replaced) t.dispose()
    onChange?.()
  })
  for (const j of plan(pickMaps(info, 2048))) for (const s of j.slots) if (s !== 'height' && s !== 'bump') roles.add(s)
  return { count: roles.size, stop }
}
