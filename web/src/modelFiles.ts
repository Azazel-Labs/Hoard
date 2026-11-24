// Reading a model and its textures for the model viewers: where each comes from (the server's GLB of the model,
// a texture as the browser reads it or as a PNG the server makes), the links that say which textures a model asks
// for, and the loaders that read them.
import * as THREE from 'three/webgpu'
import { TGALoader } from 'three/addons/loaders/TGALoader.js'
import { api, asUrl, liteUrl, packFileUrl, thumbUrl, type Asset, type AssetLink } from './api'
import { t } from './i18n'

type Loaded = {
  object: THREE.Object3D
  animations: THREE.AnimationClip[]
  /** Image files the model names that its pack doesn't have. */
  missing?: string[]
}

/** What the browser decodes itself; TGA has a loader of its own below. */
export const BROWSER_IMAGES = new Set(['png', 'jpg', 'jpeg', 'gif', 'webp', 'bmp', 'avif'])
/** What the server can turn into a PNG (see IMAGE_IN in hoardd's formats module). */
const SERVER_IMAGES = new Set(['tif', 'tiff', 'psd', 'psb', 'dds', 'exr', 'hdr', 'kra', 'ora', 'ktx', 'ktx2', 'basis', 'pvr', 'vtf'])
export const IMAGE_EXTS = new Set([...BROWSER_IMAGES, ...SERVER_IMAGES, 'tga'])

export const BLANK_PNG =
  'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII='

export const extOf = (p: string) => p.split(/[\\/]/).pop()!.split('.').pop()!.toLowerCase()
export const nameOf = (p: string) => p.split(/[\\/]/).pop()!.toLowerCase()
export const stemOf = (p: string) => nameOf(p).replace(/\.[^.]+$/, '')

/** Where a texture's pixels come from. Light: its light preview, which the browser always reads. Full: the file
 *  itself when the browser can read it, else a PNG the server makes (cached after the first time). Anything else:
 *  the 256 px thumbnail. */
export function textureUrl(t: AssetLink, packId: string, full = true): string {
  const ext = extOf(t.path)
  if (!full && IMAGE_EXTS.has(ext)) return liteUrl(t)
  if (BROWSER_IMAGES.has(ext) || ext === 'tga') return packFileUrl(packId, t.path)
  if (SERVER_IMAGES.has(ext)) return `${asUrl(t, 'png', 2048)}&download=0&wait=60`
  return thumbUrl(t)
}

/** The textures a model may ask for: its own links, plus those of its material files (OBJ → MTL → textures). */
export async function textureLinks(asset: Asset): Promise<{ named: AssetLink[]; guesses: AssetLink[]; full: Asset }> {
  // The grid's search hit has no links; the detail record does.
  const full = asset.links.length ? asset : await api.asset(asset.id).catch(() => asset)
  const named = full.links.filter((l) => l.rel === 'texture' || (l.kind === 'texture' && l.rel !== 'guess'))
  const guesses = full.links.filter((l) => l.rel === 'guess')
  // Material files (an MTL, with its maps) before Hoard's materials, which may be only colors (an MTL's swatches).
  const materials = full.links
    .filter((l) => l.rel === 'material' || l.kind === 'material')
    .sort((a, b) => Number(extOf(a.path) === 'material') - Number(extOf(b.path) === 'material'))
    .slice(0, 8)
  const more = await Promise.all(materials.map((m) => api.asset(m.id).then((a) => a.links).catch(() => [])))
  for (const list of more) named.push(...list.filter((l) => l.kind === 'texture'))
  return { named, guesses, full }
}

/** The server's GLBs are plain glTF: nothing compressed, every texture a JPEG or PNG. */
async function gltfLoader(manager: THREE.LoadingManager) {
  const { GLTFLoader } = await import('three/addons/loaders/GLTFLoader.js')
  return new GLTFLoader(manager)
}

export async function loadModel(url: string, ext: string, manager: THREE.LoadingManager): Promise<Loaded> {
  switch (ext) {
    case 'server-glb': {
      // A conversion still running answers 202 after waiting a while: ask again, for up to ten minutes or so.
      const loader = await gltfLoader(manager)
      for (let tries = 0; tries < 20; tries++) {
        const res = await fetch(url)
        if (res.status === 202) continue
        if (!res.ok) {
          const body = (await res.json().catch(() => ({}))) as { error?: string }
          throw new Error(body.error ?? `HTTP ${res.status}`)
        }
        const g = await loader.parseAsync(await res.arrayBuffer(), '')
        const extras = g.asset?.extras as { missing_textures?: string[] } | undefined
        return { object: g.scene, animations: g.animations, missing: extras?.missing_textures ?? [] }
      }
      throw new Error(t('model-error-converting'))
    }
    default:
      throw new Error(t('model-error-no-loader', { ext }))
  }
}

/** Every model is shown from the GLB the server makes of it (`view_glb` in hoardd's meshes.rs): polysmelt reads the
 *  file, decodes any compression and re-encodes the textures as JPEG or PNG, so the browser reads nothing but plain
 *  GLB. A model only Blender reads comes from Blender's GLB of it. */
export function sourceFor(a: Pick<Asset | AssetLink, 'id'>): { url: string; ext: string } {
  return { url: `/api/assets/${a.id}/as/view?download=0&wait=30`, ext: 'server-glb' }
}

/** The loader for files asked for as `.tga`. Loaders pick it by the name they ask for, before the URL is
 *  rewritten; the rewritten URL may be a light preview (JPEG or WebP) rather than a TGA, which the image loader reads. */
export class TgaOrImageLoader extends THREE.Loader<THREE.Texture> {
  private tga: TGALoader
  private image: THREE.TextureLoader
  constructor(manager: THREE.LoadingManager) {
    super(manager)
    this.tga = new TGALoader(manager)
    this.image = new THREE.TextureLoader(manager)
  }
  load(url: string, onLoad?: (t: THREE.Texture) => void, onProgress?: (e: ProgressEvent) => void, onError?: (e: unknown) => void) {
    // A rewritten URL comes through the modifier unchanged a second time, so the inner loader may resolve it again.
    const resolved = this.manager.resolveURL((this.path || '') + url)
    const loader = extOf(resolved.split(/[?#]/)[0]) === 'tga' ? this.tga : this.image
    loader.setPath('')
    return loader.load(resolved, onLoad, onProgress, onError)
  }
}
