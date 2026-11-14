// The 3D scene behind both model viewers (the side panel's preview and the full-screen one): load a model with its
// textures, light it, frame it, and the switches the full-screen viewer offers. three.js with its WebGPU renderer,
// which falls back to WebGL2 by itself. Only the viewers import this, and they load on demand, so three stays out of
// the main bundle.
import * as THREE from 'three/webgpu'
import { attribute, sRGBTransferEOTF, texture, vec3 } from 'three/tsl'
import { OrbitControls } from 'three/addons/controls/OrbitControls.js'
import { RoomEnvironment } from 'three/addons/environments/RoomEnvironment.js'
import { TGALoader } from 'three/addons/loaders/TGALoader.js'
import { mergeGeometries } from 'three/addons/utils/BufferGeometryUtils.js'
import { api, asUrl, liteUrl, packFileUrl, thumbUrl, type Asset, type AssetLink } from './api'
import { t } from './i18n'
import { applyToModel } from './materialMaps'

export type Background = 'checker' | 'dark' | 'grey' | 'light' | 'env'
export const BACKGROUNDS: Background[] = ['checker', 'dark', 'grey', 'light', 'env']

/** How the full-screen viewer shows the model: as it renders, plainly lit, one of its maps on its own, or a look that
 *  shows its shape (clay, a UV checker, each mesh in its own colour). */
export type ViewMode = 'final' | 'raw' | MapMode | 'matcap' | 'uv' | 'geometry'
export type MapMode = 'color' | 'roughness' | 'metalness' | 'normal' | 'ao'

/** Each map view: the slot it shows and the channel three reads from it. glTF packs roughness into G and metalness
 *  into B of one texture, and occlusion lives in R; maps of their own are grey, so the same channel serves. */
const MAP_VIEWS: Record<MapMode, { slot: MapSlot; channel: 'rgb' | 'r' | 'g' | 'b' }> = {
  color: { slot: 'map', channel: 'rgb' },
  roughness: { slot: 'roughnessMap', channel: 'g' },
  metalness: { slot: 'metalnessMap', channel: 'b' },
  normal: { slot: 'normalMap', channel: 'rgb' },
  ao: { slot: 'aoMap', channel: 'r' },
}
export const MAP_MODES = Object.keys(MAP_VIEWS) as MapMode[]
const isMapMode = (m: ViewMode): m is MapMode => m in MAP_VIEWS

/** What the scene information panel shows. */
export interface SceneStats {
  meshes: number
  materials: number
  /** Distinct images the materials use. */
  textures: number
  triangles: number
  vertices: number
  /** The bounding box as the model loaded: X, Y (up) and Z. */
  size: { x: number; y: number; z: number }
  /** glTF is in metres; other formats say nothing dependable about their units. */
  metres: boolean
  /** The map views worth offering: a map at least one material has. */
  maps: MapMode[]
}

export interface StageInfo {
  triangles: number
  meshes: number
  materials: number
  /** Clip names, embedded ones first, then linked clip files ("Knight@Run.fbx"). */
  clips: string[]
  /** Image files the model asked for that couldn't be found or loaded. */
  missing: string[]
  /** Textures picked by name ("Barrel_Albedo.png" for "Barrel.fbx") for a model that names none. */
  guessed: number
}

export interface StageOptions {
  /** Textures as their files are, however big. Otherwise each comes as its light preview (at most 2048 px, see
   *  `liteUrl`), which loads far faster and looks the same at viewer sizes. */
  fullTextures?: boolean
}

type Loaded = {
  object: THREE.Object3D
  animations: THREE.AnimationClip[]
  /** Material colours were written as linear values (Blender's OBJ and FBX exporters), not the sRGB the loaders
   *  assume: read as sRGB they come out far too dark, a dark green skin turning black. */
  linearColors?: boolean
}

/** What the browser decodes itself; TGA has a loader of its own below. */
const BROWSER_IMAGES = new Set(['png', 'jpg', 'jpeg', 'gif', 'webp', 'bmp', 'avif'])
/** What the server can turn into a PNG (see IMAGE_IN in hoardd's formats.rs). */
const SERVER_IMAGES = new Set(['tif', 'tiff', 'psd', 'psb', 'dds', 'exr', 'hdr', 'kra', 'ora', 'ktx2', 'basis'])
const IMAGE_EXTS = new Set([...BROWSER_IMAGES, ...SERVER_IMAGES, 'tga'])

const BLANK_PNG =
  'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII='

const extOf = (p: string) => p.split(/[\\/]/).pop()!.split('.').pop()!.toLowerCase()
const nameOf = (p: string) => p.split(/[\\/]/).pop()!.toLowerCase()
const stemOf = (p: string) => nameOf(p).replace(/\.[^.]+$/, '')

/** Where a texture's pixels come from. Light: its light preview, which the browser always reads. Full: the file
 *  itself when the browser can read it, else a PNG the server makes (cached after the first time). Anything else:
 *  the 256 px thumbnail. */
function textureUrl(t: AssetLink, packId: string, full = true): string {
  const ext = extOf(t.path)
  if (!full && IMAGE_EXTS.has(ext)) return liteUrl(t)
  if (BROWSER_IMAGES.has(ext) || ext === 'tga') return packFileUrl(packId, t.path)
  if (SERVER_IMAGES.has(ext)) return `${asUrl(t, 'png', 2048)}&download=0&wait=60`
  return thumbUrl(t)
}

/** The textures a model may ask for: its own links, plus those of its material files (OBJ → MTL → textures). */
async function textureLinks(asset: Asset): Promise<{ named: AssetLink[]; guesses: AssetLink[]; full: Asset }> {
  // The grid's search hit has no links; the detail record does.
  const full = asset.links.length ? asset : await api.asset(asset.id).catch(() => asset)
  const named = full.links.filter((l) => l.rel === 'texture' || (l.kind === 'texture' && l.rel !== 'guess'))
  const guesses = full.links.filter((l) => l.rel === 'guess')
  const materials = full.links.filter((l) => l.rel === 'material' || l.kind === 'material').slice(0, 8)
  const more = await Promise.all(materials.map((m) => api.asset(m.id).then((a) => a.links).catch(() => [])))
  for (const list of more) named.push(...list.filter((l) => l.kind === 'texture'))
  return { named, guesses, full }
}

/** Facet colours the way VisCAM and SolidView write them in a binary STL (three.js reads only Materialise's): the
 *  top bit of a facet's attribute marks it coloured, blue in the low bits. Nothing says a file does this and other
 *  tools leave junk there, so it takes two colours or more. The server reads them the same way. */
function visCamColors(buf: ArrayBuffer, geom: THREE.BufferGeometry) {
  if (buf.byteLength < 84) return
  const view = new DataView(buf)
  const n = view.getUint32(80, true)
  if (84 + n * 50 !== buf.byteLength || geom.attributes.position.count !== n * 3) return
  const attrs = new Uint16Array(n)
  const seen = new Set<number>()
  for (let i = 0; i < n; i++) {
    attrs[i] = view.getUint16(84 + i * 50 + 48, true)
    if (attrs[i] & 0x8000) seen.add(attrs[i])
  }
  if (seen.size < 2) return
  const colors = new Float32Array(n * 9)
  const c = new THREE.Color()
  for (let i = 0; i < n; i++) {
    const a = attrs[i]
    if (a & 0x8000) c.setRGB(((a >> 10) & 0x1f) / 31, ((a >> 5) & 0x1f) / 31, (a & 0x1f) / 31, THREE.SRGBColorSpace)
    else c.set(0xb8c0d0)
    for (let k = 0; k < 3; k++) c.toArray(colors, i * 9 + k * 3)
  }
  geom.setAttribute('color', new THREE.BufferAttribute(colors, 3))
}

async function loadModel(url: string, ext: string, manager: THREE.LoadingManager): Promise<Loaded> {
  switch (ext) {
    case 'glb':
    case 'gltf': {
      const { GLTFLoader } = await import('three/addons/loaders/GLTFLoader.js')
      const g = await new GLTFLoader(manager).loadAsync(url)
      return { object: g.scene, animations: g.animations }
    }
    case 'fbx': {
      const { FBXLoader } = await import('three/addons/loaders/FBXLoader.js')
      const res = await fetch(url)
      if (!res.ok) throw new Error(`HTTP ${res.status}`)
      const buf = await res.arrayBuffer()
      const o = new FBXLoader(manager).parse(buf, url.slice(0, url.lastIndexOf('/') + 1))
      // The creator is named in the header, near the start of the file.
      const head = new TextDecoder('latin1').decode(buf.slice(0, 65536))
      return { object: o, animations: o.animations, linearColors: head.includes('Blender') }
    }
    case 'obj': {
      const [{ OBJLoader }, { MTLLoader }] = await Promise.all([
        import('three/addons/loaders/OBJLoader.js'),
        import('three/addons/loaders/MTLLoader.js'),
      ])
      const res = await fetch(url)
      if (!res.ok) throw new Error(`HTTP ${res.status}`)
      const text = await res.text()
      const loader = new OBJLoader(manager)
      const mtl = text.match(/^mtllib\s+(.+)$/m)?.[1]?.trim()
      if (mtl) {
        try {
          const base = url.slice(0, url.lastIndexOf('/') + 1)
          const materials = await new MTLLoader(manager).setPath(base).loadAsync(mtl)
          materials.preload()
          loader.setMaterials(materials)
        } catch {
          // Missing MTL: fall back to the default material.
        }
      }
      return { object: loader.parse(text), animations: [], linearColors: /^# Blender/m.test(text.slice(0, 500)) }
    }
    case 'stl': {
      const { STLLoader } = await import('three/addons/loaders/STLLoader.js')
      const res = await fetch(url)
      if (!res.ok) throw new Error(`HTTP ${res.status}`)
      const buf = await res.arrayBuffer()
      const geom = new STLLoader(manager).parse(buf)
      // Some exporters leave the facet normals zero (they're optional): work them out, as the server does.
      const normals = geom.attributes.normal?.array
      if (!normals || normals.some((_, i) => i % 3 === 0 && !normals[i] && !normals[i + 1] && !normals[i + 2])) geom.computeVertexNormals()
      if (!geom.attributes.color) visCamColors(buf, geom)
      const material = new THREE.MeshStandardMaterial({ color: 0xb8c0d0, roughness: 0.6, vertexColors: !!geom.attributes.color })
      if (geom.attributes.color) material.color.set(0xffffff)
      return { object: new THREE.Mesh(geom, material), animations: [] }
    }
    case 'ply': {
      const { PLYLoader } = await import('three/addons/loaders/PLYLoader.js')
      const geom = await new PLYLoader(manager).loadAsync(url)
      if (!geom.attributes.normal) geom.computeVertexNormals()
      const material = new THREE.MeshStandardMaterial({ color: 0xb8c0d0, roughness: 0.6, vertexColors: !!geom.attributes.color })
      if (geom.attributes.color) material.color.set(0xffffff) // the file's own colours, not tinted gray
      return { object: new THREE.Mesh(geom, material), animations: [] }
    }
    case 'server-glb': {
      // The server reads 3MF (slicers' filament colours and painting included) and USD, and hands them over as a GLB. A
      // conversion still running answers 202 after waiting a while: ask again, for up to ten minutes or so.
      const { GLTFLoader } = await import('three/addons/loaders/GLTFLoader.js')
      for (let tries = 0; tries < 20; tries++) {
        const res = await fetch(url)
        if (res.status === 202) continue
        if (!res.ok) {
          const body = (await res.json().catch(() => ({}))) as { error?: string }
          throw new Error(body.error ?? `HTTP ${res.status}`)
        }
        const g = await new GLTFLoader(manager).parseAsync(await res.arrayBuffer(), '')
        return { object: g.scene, animations: g.animations }
      }
      throw new Error(t('model-error-converting'))
    }
    case 'dae': {
      const { ColladaLoader } = await import('three/addons/loaders/ColladaLoader.js')
      const c = await new ColladaLoader(manager).loadAsync(url)
      if (!c) throw new Error(t('model-error-empty-collada'))
      return { object: c.scene, animations: c.scene.animations ?? [] }
    }
    default:
      throw new Error(t('model-error-no-loader', { ext }))
  }
}

/** Formats the browser has no loader for, which the server reads and hands over as a GLB. */
const SERVER_GLB = new Set(['3mf', 'usd', 'usda', 'usdc', 'usdz'])

function sourceFor(a: Asset | AssetLink, packId: string, derivatives: string[] = []): { url: string; ext: string } {
  if (derivatives.includes('glb')) return { url: `/api/assets/${a.id}/derivative/glb`, ext: 'glb' }
  if (SERVER_GLB.has(extOf(a.path))) return { url: `/api/assets/${a.id}/as/glb?download=0&wait=30`, ext: 'server-glb' }
  return { url: packFileUrl(packId, a.path), ext: extOf(a.path) }
}

/** Texture slots a material can carry; the "textures off" switch empties them and puts them back. */
const MAP_SLOTS = [
  'map', 'normalMap', 'bumpMap', 'emissiveMap', 'aoMap', 'roughnessMap', 'metalnessMap', 'alphaMap', 'lightMap',
  'specularMap', 'displacementMap',
] as const
type MapSlot = (typeof MAP_SLOTS)[number]
type AnyMaterial = THREE.Material & Partial<Record<MapSlot, THREE.Texture | null>> & {
  color?: THREE.Color; wireframe?: boolean; vertexColors?: boolean
}

/** FBX and OBJ files arrive with Phong or Lambert materials, which only see direct lights: no environment, so they
 *  read flat and dark in a viewer. Remake them as standard PBR materials (same colour, maps and transparency) so the
 *  environment lights them like everything else. */
function toStandard(m: THREE.Material): THREE.Material {
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
function sanitize(m: AnyMaterial, geometry: THREE.BufferGeometry | undefined) {
  // Exporters disagree on what FBX's TransparencyFactor and MTL's `d`/`Tr` mean; a fully transparent material with
  // no alpha texture is almost always that mix-up, not an invisible object.
  if (m.transparent && m.opacity < 0.02 && !m.alphaMap) {
    m.opacity = 1
    m.transparent = false
  }
  // Vertex colours that are all black (an empty colour layer) multiply the whole model to black.
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
function roleOf(name: string): MapSlot | 'skip' | null {
  const n = stemOf(name)
  // A map-type suffix after a separator: "_n", "-ao", ".bc".
  const end = (s: string) => new RegExp(`(^|[-_. ])(${s})$`).test(n)
  if (/normal|nrm/.test(n) || end('n|nor|norm')) return 'normalMap'
  if (/emissi|emit|glow/.test(n) || end('e')) return 'emissiveMap'
  if (/rough/.test(n) || end('r')) return 'roughnessMap'
  if (/metal/.test(n) || end('m|mt')) return 'metalnessMap'
  if (/occlusion/.test(n) || end('ao|occ')) return 'aoMap'
  if (/height|disp|bump|mask|opacity|alpha|spec|gloss|smooth/.test(n) || end('h|orm|arm|mra|rma|s|g')) return 'skip'
  if (/albedo|base.?colou?r|diffuse/.test(n) || end('diff|dif|col|color|colour|d|bc|a|c')) return 'map'
  return null
}

/** The loader for files asked for as `.tga`. Loaders pick it by the name they ask for, before the URL is
 *  rewritten; the rewritten URL may be a light preview (JPEG or WebP) rather than a TGA, which the image loader reads. */
class TgaOrImageLoader extends THREE.Loader<THREE.Texture> {
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

export class ModelStage {
  readonly host: HTMLElement
  /** See `StageOptions.fullTextures`. */
  readonly fullTextures: boolean
  /** Files the loaders are still fetching; textures go on loading after the model is up. */
  private pending = 0
  private loaded = false
  private settledWaiters: (() => void)[] = []
  private renderer: THREE.WebGPURenderer | null = null
  private scene = new THREE.Scene()
  private camera = new THREE.PerspectiveCamera(35, 1, 0.01, 1000)
  private controls: OrbitControls | null = null
  private hemi = new THREE.HemisphereLight(0xdfe8ff, 0x3a3226, 1)
  private key = new THREE.DirectionalLight(0xffffff, 2)
  private envMap: THREE.Texture | null = null
  private grid: THREE.GridHelper | null = null
  private object: THREE.Object3D | null = null
  private mixer: THREE.AnimationMixer | null = null
  private clips = new Map<string, THREE.AnimationClip>()
  private action: THREE.AnimationAction | null = null
  private resizeObs: ResizeObserver | null = null
  private timer = new THREE.Timer()
  private materials = new Map<AnyMaterial, Partial<Record<MapSlot, THREE.Texture | null>>>()
  private home = { position: new THREE.Vector3(), target: new THREE.Vector3() }
  private glide: { from: THREE.Vector3; to: THREE.Vector3; fromT: THREE.Vector3; toT: THREE.Vector3; t: number } | null = null
  private paused = false
  private disposed = false
  private envOn = true
  private texturesOn = true
  private wireframeOn = false
  private bg: Background = 'checker'
  private mode: ViewMode = 'final'
  /** The meshes and the materials a view mode put on them in place of their own, which go back afterwards. */
  private views: { meshes: Map<THREE.Mesh, { own: THREE.Material | THREE.Material[]; view: THREE.Material | THREE.Material[] }>; made: AnyMaterial[] } | null = null
  /** Made on first use: the clay and checker pictures. */
  private matcap: THREE.Texture | null = null
  private checker: THREE.Texture | null = null
  /** The meshes drawn: the file's, with the still ones that share a material merged into one (see `mergeStill`). */
  private meshes: THREE.Mesh[] = []
  /** How many meshes the file has, before merging. */
  private meshCount = 0
  /** Frames are drawn only when something changed, and for a moment after, until this time (`redraw`). */
  private redrawUntil = 0
  private counts = { triangles: 0, vertices: 0 }
  private size = new THREE.Vector3()
  private metres = false
  /** Image top at v = 1 (OBJ, FBX), as three's textures assume; glTF has it at v = 0. */
  private flipY = true
  playing = true
  private missing: string[] = []
  private stopMaps: (() => void) | null = null
  /** Textures go on loading after the model is up; this hears about the ones that fail. */
  onMissing: ((names: string[]) => void) | null = null
  /** Hears when a material's maps arrive late (a model wearing a material's maps), to count them again. */
  onTexturesChanged: (() => void) | null = null

  constructor(host: HTMLElement, options: StageOptions = {}) {
    this.host = host
    this.fullTextures = !!options.fullTextures
  }

  /** Resolves once the model is up and every texture it asked for has loaded or failed. */
  texturesSettled(): Promise<void> {
    if (this.loaded && !this.pending) return Promise.resolve()
    return new Promise((resolve) => this.settledWaiters.push(resolve))
  }

  private checkSettled() {
    if (!this.loaded || this.pending) return
    for (const w of this.settledWaiters.splice(0)) w()
  }

  /** Image files the model asked for that couldn't be found or loaded, so far. */
  get missingTextures(): string[] {
    return [...this.missing]
  }

  /** Set up the renderer and load the asset. Resolves once the first frame can draw. */
  async load(asset: Asset): Promise<StageInfo> {
    const el = this.host
    const renderer = new THREE.WebGPURenderer({ antialias: true, alpha: true })
    await renderer.init()
    if (this.disposed) {
      renderer.dispose()
      throw new Error('closed')
    }
    this.renderer = renderer
    renderer.setPixelRatio(Math.min(devicePixelRatio, 2))
    renderer.setSize(el.clientWidth || 1, el.clientHeight || 1)
    // Neutral tone mapping keeps base colours as painted (ACES would shift and desaturate them), while highlights
    // from the environment still roll off instead of clipping.
    renderer.toneMapping = THREE.NeutralToneMapping
    renderer.toneMappingExposure = 1
    el.appendChild(renderer.domElement)

    // A soft studio room for image-based light: PBR materials (glTF's especially, which are often metallic) look
    // black or flat under direct lights alone. A key light rides with the camera so the side you look at is lit.
    const pmrem = new THREE.PMREMGenerator(renderer)
    const room = new RoomEnvironment()
    this.envMap = pmrem.fromScene(room, 0.04).texture
    room.dispose()
    pmrem.dispose()
    this.scene.add(this.hemi)
    this.camera.add(this.key, this.key.target)
    this.key.position.set(1.2, 1.6, 0.6)
    this.key.target.position.set(0, 0, -2)
    this.scene.add(this.camera)
    this.applyLook()

    const controls = new OrbitControls(this.camera, renderer.domElement)
    controls.enableDamping = true
    controls.addEventListener('start', () => (this.glide = null))
    // Moved by the pointer, or still gliding to a stop, or spinning.
    controls.addEventListener('change', () => this.redraw(100))
    this.controls = controls

    // Every file the loaders ask for passes through here: textures are matched to the pack's files by name (FBX
    // and MTL files often carry the artist's absolute path, or point at a PSD where the pack ships a PNG).
    const { named, guesses, full } = await textureLinks(asset)
    const byName = new Map<string, AssetLink>()
    const byStem = new Map<string, AssetLink>()
    for (const t of [...named, ...guesses]) {
      if (!byName.has(nameOf(t.path))) byName.set(nameOf(t.path), t)
      // A name match beats a stem match, and among stems the browser-readable file wins.
      const prev = byStem.get(stemOf(t.path))
      if (!prev || (!BROWSER_IMAGES.has(extOf(prev.path)) && BROWSER_IMAGES.has(extOf(t.path)))) byStem.set(stemOf(t.path), t)
    }
    const byPath = new Map([...named, ...guesses].map((t) => [t.path.toLowerCase(), t]))
    const packFiles = `/api/packs/${asset.pack_id}/files/`
    /** The file name each rewritten URL was asked for by, to name what failed. */
    const askedAs = new Map<string, string>()
    const manager = new THREE.LoadingManager()
    manager.setURLModifier((url) => {
      if (/^(blob|data):/.test(url)) return url
      let path = url.split(/[?#]/)[0]
      try {
        path = decodeURIComponent(path)
      } catch {
        // Not percent-encoded after all.
      }
      if (!IMAGE_EXTS.has(extOf(path))) return url
      // The exact file first (glTF's relative paths), then by name, then by name without the extension.
      const inPack = path.startsWith(packFiles) ? path.slice(packFiles.length).replace(/\\/g, '/') : ''
      const hit = byPath.get(inPack.toLowerCase()) ?? byName.get(nameOf(path)) ?? byStem.get(stemOf(path))
      // The loader was picked by the extension asked for: a TGA standing in for a PNG goes through the server.
      // (Light previews are never TGA.)
      const tgaForOther = this.fullTextures && hit && extOf(hit.path) === 'tga' && extOf(path) !== 'tga'
      const out = !hit
        ? url
        : tgaForOther
          ? `${asUrl(hit, 'png', 2048)}&download=0&wait=60`
          : textureUrl(hit, asset.pack_id, this.fullTextures)
      askedAs.set(out, path.split(/[\\/]/).pop()!)
      return out
    })
    manager.onError = (url) => {
      const name = askedAs.get(url)
      if (name && !this.missing.includes(name)) {
        this.missing.push(name)
        this.onMissing?.([...this.missing])
      }
    }
    manager.addHandler(/\.tga$/i, new TgaOrImageLoader(manager))
    const itemStart = manager.itemStart.bind(manager)
    const itemEnd = manager.itemEnd.bind(manager)
    manager.itemStart = (url) => {
      this.pending++
      itemStart(url)
    }
    manager.itemEnd = (url) => {
      itemEnd(url)
      this.pending = Math.max(0, this.pending - 1)
      this.redraw()
      this.checkSettled()
    }

    const src = sourceFor(full, full.pack_id, full.derivatives)
    const { object, animations, linearColors } = await loadModel(src.url, src.ext, manager)
    if (this.disposed) {
      disposeObject(object)
      throw new Error('closed')
    }

    let triangles = 0
    let vertices = 0
    const textured = { any: false }
    const converted = new Map<THREE.Material, AnyMaterial>()
    object.traverse((o) => {
      const mesh = o as THREE.Mesh
      if (!mesh.isMesh && !(o as THREE.Points).isPoints && !(o as THREE.Line).isLine) return
      if (mesh.isMesh) {
        this.meshes.push(mesh)
        const g = mesh.geometry
        triangles += (g.index?.count ?? g.attributes.position?.count ?? 0) / 3
        vertices += g.attributes.position?.count ?? 0
      }
      const list = (Array.isArray(mesh.material) ? mesh.material : [mesh.material]).map((m) => {
        // Meshes share materials: convert each once, so they stay shared and the colour fix isn't applied twice.
        let s = converted.get(m)
        if (!s) {
          s = toStandard(m) as AnyMaterial
          if (linearColors) {
            s.color?.convertLinearToSRGB()
            ;(s as THREE.MeshStandardMaterial).emissive?.convertLinearToSRGB()
          }
          converted.set(m, s)
        }
        sanitize(s, mesh.geometry)
        if (s.map) textured.any = true
        return s
      })
      mesh.material = Array.isArray(mesh.material) ? list : list[0]
      for (const m of list) this.track(m)
    })
    this.meshCount = this.meshes.length
    // Nothing moves in a model without animation, so its meshes can be drawn together.
    if (!animations.length && !full.links.some((l) => l.rel === 'animation')) this.meshes = mergeStill(object, this.meshes)

    // A model that names no textures, next to textures named after it: try them on.
    let guessed = 0
    this.flipY = src.ext !== 'glb' && src.ext !== 'gltf'
    // Or a material made from maps that share its name: its maps by role.
    const material = textured.any ? undefined : full.materials?.[0]
    if (material) {
      const applied = applyToModel(
        material,
        asset.pack_id,
        this.flipY,
        this.materials,
        () => {
          // A view built from the maps that were there shows the new ones only once built again.
          if (this.views) this.buildViews()
          this.redraw()
          this.onTexturesChanged?.()
        },
        // The material's maps count as the model's textures: a thumbnail waits for them, and one that fails is missing.
        (name) => {
          this.pending++
          return (ok) => {
            this.pending = Math.max(0, this.pending - 1)
            this.redraw()
            if (!ok && !this.missing.includes(name)) {
              this.missing.push(name)
              this.onMissing?.([...this.missing])
            }
            this.checkSettled()
          }
        },
      )
      this.stopMaps = applied.stop
      guessed = applied.count
    } else if (!textured.any && guesses.length) guessed = this.applyGuesses(guesses, asset.pack_id, this.flipY, manager)

    this.object = object
    this.scene.add(object)
    this.frameHome()
    this.counts = { triangles: Math.round(triangles), vertices }
    this.bounds().box.getSize(this.size)
    this.metres = ['glb', 'gltf'].includes(extOf(full.path))

    // Clips embedded in the file plus linked clip files ("Knight@Run.fbx").
    animations.forEach((c, i) => this.clips.set(c.name || `clip ${i + 1}`, c))
    // Clip files only lend their animation: don't fetch their textures.
    const clipManager = new THREE.LoadingManager()
    clipManager.setURLModifier((url) => (IMAGE_EXTS.has(extOf(url.split(/[?#]/)[0])) ? BLANK_PNG : url))
    for (const link of full.links.filter((l) => l.rel === 'animation')) {
      try {
        const s = sourceFor(link, full.pack_id)
        const loaded = await loadModel(s.url, s.ext, clipManager)
        const label = link.path.split('/').pop()!.replace(/\.[^.]+$/, '').split('@').pop()!
        loaded.animations.forEach((c, i) => this.clips.set(loaded.animations.length > 1 ? `${label} ${i + 1}` : label, c))
        disposeObject(loaded.object)
      } catch {
        // A clip that won't load shouldn't break the preview.
      }
      if (this.disposed) throw new Error('closed')
    }
    if (this.clips.size) this.mixer = new THREE.AnimationMixer(object)

    this.resizeObs = new ResizeObserver(() => this.resize())
    this.resizeObs.observe(el)
    this.resize()
    this.start()
    this.loaded = true
    this.checkSettled()

    return {
      triangles: this.counts.triangles,
      meshes: this.meshCount,
      materials: this.materials.size,
      // An idle clip first, since the first is what plays: a character pack's clips come alphabetically, and a
      // looping "Death" or "Victory" is a poor first look.
      clips: [...this.clips.keys()].sort((a, b) => Number(/idle/i.test(b)) - Number(/idle/i.test(a))),
      missing: [...this.missing],
      guessed,
    }
  }

  private track(m: AnyMaterial) {
    if (this.materials.has(m)) return
    const maps: Partial<Record<MapSlot, THREE.Texture | null>> = {}
    for (const slot of MAP_SLOTS) if (m[slot]) maps[slot] = m[slot]
    this.materials.set(m, maps)
  }

  private applyGuesses(guesses: AssetLink[], packId: string, flipY: boolean, manager: THREE.LoadingManager): number {
    const roles = new Map<MapSlot, AssetLink>()
    const unknown: AssetLink[] = []
    for (const g of guesses) {
      const role = roleOf(g.path)
      if (role === null) unknown.push(g)
      else if (role !== 'skip' && !roles.has(role)) roles.set(role, g)
    }
    // No texture says it's the colour: the first one whose name doesn't say otherwise ("Barrel.png") is.
    if (!roles.has('map') && unknown.length) roles.set('map', unknown[0])
    if (!roles.has('map')) return 0
    const image = new THREE.TextureLoader(manager)
    const tga = new TGALoader(manager)
    const textures = new Map<MapSlot, THREE.Texture>()
    for (const [role, link] of roles) {
      const url = textureUrl(link, packId, this.fullTextures)
      const t: THREE.Texture = this.fullTextures && extOf(link.path) === 'tga' ? tga.load(url) : image.load(url)
      // TGA comes back as pixel data the loader has already turned the right way up.
      if (!(t as THREE.DataTexture).isDataTexture) t.flipY = flipY
      if (role === 'map' || role === 'emissiveMap') t.colorSpace = THREE.SRGBColorSpace
      t.wrapS = t.wrapT = THREE.RepeatWrapping
      textures.set(role, t)
    }
    for (const [m, maps] of this.materials) {
      for (const [slot, t] of textures) {
        if (slot !== 'map' && !(slot in m)) continue
        m[slot] = t
        maps[slot] = t
      }
      // A painted colour map stands for the whole surface: a grey base colour would only darken it, and an
      // emissive map shows only through a lit emissive colour.
      m.color?.set(0xffffff)
      if (textures.has('emissiveMap') && 'emissive' in m) (m as THREE.MeshStandardMaterial).emissive.set(0xffffff)
      m.needsUpdate = true
    }
    return textures.size
  }

  /** Bounding sphere of the model as it stands, for framing. */
  private bounds() {
    const box = new THREE.Box3().setFromObject(this.object!, true)
    const sphere = box.getBoundingSphere(new THREE.Sphere())
    if (!Number.isFinite(sphere.radius) || sphere.radius <= 0) sphere.set(new THREE.Vector3(), 1)
    return { box, sphere }
  }

  private distanceFor(radius: number) {
    // Fit the sphere in whichever field of view is narrower, with a little margin.
    const vfov = (this.camera.fov * Math.PI) / 180
    const hfov = 2 * Math.atan(Math.tan(vfov / 2) * this.camera.aspect)
    return (radius / Math.sin(Math.min(vfov, hfov) / 2)) * 1.08
  }

  /** The starting view: a 3/4 angle from the front, above. */
  private frameHome() {
    const { box, sphere } = this.bounds()
    const dist = this.distanceFor(sphere.radius)
    this.home.target.copy(sphere.center)
    this.home.position.copy(sphere.center).add(new THREE.Vector3(0.75, 0.5, 1).normalize().multiplyScalar(dist))
    this.camera.near = sphere.radius / 100
    this.camera.far = sphere.radius * 200
    this.camera.updateProjectionMatrix()
    this.camera.position.copy(this.home.position)
    this.controls!.target.copy(this.home.target)
    this.controls!.minDistance = sphere.radius * 0.05
    this.controls!.maxDistance = sphere.radius * 50
    this.controls!.update()
    // The grid sits on the floor under the model, a little wider than it.
    const size = Math.max(box.max.x - box.min.x, box.max.z - box.min.z) * 2 || 2
    this.grid?.removeFromParent()
    this.grid?.dispose()
    this.grid = new THREE.GridHelper(size, 20, 0x888888, 0x555555)
    this.grid.position.set(sphere.center.x, box.min.y, sphere.center.z)
    const gm = this.grid.material as THREE.Material
    gm.transparent = true
    gm.opacity = 0.45
    this.grid.visible = false
    this.scene.add(this.grid)
  }

  private glideTo(position: THREE.Vector3, target: THREE.Vector3) {
    this.glide = {
      from: this.camera.position.clone(),
      to: position.clone(),
      fromT: this.controls!.target.clone(),
      toT: target.clone(),
      t: 0,
    }
    this.redraw()
    if (this.paused) this.renderOnce()
  }

  /** Fit the whole model in view from where the camera looks now. */
  frame() {
    if (!this.object || !this.controls) return
    const { sphere } = this.bounds()
    const dir = this.camera.position.clone().sub(this.controls.target).normalize()
    if (!dir.lengthSq()) dir.set(0, 0, 1)
    this.glideTo(sphere.center.clone().add(dir.multiplyScalar(this.distanceFor(sphere.radius))), sphere.center)
  }

  /** Back to the starting view. */
  reset() {
    if (!this.object) return
    this.glideTo(this.home.position, this.home.target)
  }

  setWireframe(on: boolean) {
    this.wireframeOn = on
    for (const m of [...this.materials.keys(), ...(this.views?.made ?? [])]) {
      m.wireframe = on
      m.needsUpdate = true
    }
    this.redraw()
  }

  /** Show the model another way; the wireframe and grid stay as they are on top. */
  setMode(mode: ViewMode) {
    this.mode = mode
    this.buildViews()
    this.applyLook()
    if (this.paused) this.renderOnce()
  }

  /** Counts for the scene information panel. Textures that failed to load don't count once loading is over. */
  sceneStats(): SceneStats {
    const settled = this.loaded && !this.pending
    const sources = new Set<object>()
    const maps = new Set<MapMode>()
    for (const tracked of this.materials.values()) {
      for (const [slot, t] of Object.entries(tracked) as [MapSlot, THREE.Texture | null][]) {
        if (!t || (settled && !t.image)) continue
        sources.add(t.source)
        for (const mode of MAP_MODES) if (MAP_VIEWS[mode].slot === slot) maps.add(mode)
      }
    }
    return {
      meshes: this.meshCount,
      materials: this.materials.size,
      textures: sources.size,
      ...this.counts,
      size: { x: this.size.x, y: this.size.y, z: this.size.z },
      metres: this.metres,
      maps: MAP_MODES.filter((m) => maps.has(m)),
    }
  }

  /** Put each mesh's own materials back, and free the ones the view made. */
  private dropViews() {
    if (!this.views) return
    for (const [mesh, { own }] of this.views.meshes) mesh.material = own
    for (const m of this.views.made) m.dispose()
    this.views = null
  }

  /** Swap between the view's materials and the meshes' own, keeping both (the thumbnail is taken in its own look). */
  private showViews(on: boolean) {
    for (const [mesh, { own, view }] of this.views?.meshes ?? []) mesh.material = on ? view : own
  }

  /** Make the materials the mode shows the meshes in. Meshes that share a material share its stand-in, except in
   *  the geometry view, where each mesh gets a colour of its own. Points and lines keep theirs. */
  private buildViews() {
    this.dropViews()
    const mode = this.mode
    if (mode === 'final' || mode === 'raw') return
    const views: NonNullable<typeof this.views> = { meshes: new Map(), made: [] }
    const shared = new Map<string, AnyMaterial>()
    this.meshes.forEach((mesh, i) => {
      const own = mesh.material
      const hasUv = !!mesh.geometry.attributes.uv
      const list = (Array.isArray(own) ? own : [own]).map((m) => {
        const key = mode === 'geometry' ? `${i}:${m.uuid}` : `${m.uuid}:${hasUv}`
        let v = shared.get(key)
        if (!v) {
          v = this.viewMaterial(mode, m as AnyMaterial, hasUv, i, !!mesh.geometry.attributes[PART_COLOR])
          shared.set(key, v)
          views.made.push(v)
        }
        return v
      })
      const view = Array.isArray(own) ? list : list[0]
      views.meshes.set(mesh, { own, view })
      mesh.material = view
    })
    this.views = views
    this.redraw()
  }

  /** The stand-in for one material in a view mode, from the maps it had when it loaded (whatever "textures off" says). */
  private viewMaterial(mode: Exclude<ViewMode, 'final' | 'raw'>, own: AnyMaterial, hasUv: boolean, index: number, parts: boolean): AnyMaterial {
    const maps = this.materials.get(own) ?? {}
    let m: THREE.NodeMaterial
    if (isMapMode(mode)) {
      // Unlit and full-bright, a mesh without the map in flat grey. Colour maps are decoded from sRGB as they're
      // read and encoded again on the way out; data maps are numbers, so they're read as if they were sRGB too, to
      // show each pixel as the file has it.
      m = new THREE.MeshBasicNodeMaterial({ color: 0x808080 })
      const { slot, channel } = MAP_VIEWS[mode]
      const t = maps[slot]
      if (t) {
        const s = texture(t)
        const c = channel === 'rgb' ? s.rgb : vec3(channel === 'r' ? s.r : channel === 'g' ? s.g : s.b)
        m.colorNode = t.colorSpace === THREE.SRGBColorSpace ? c : (sRGBTransferEOTF(c) as typeof c)
      }
    } else if (mode === 'matcap') {
      // Clay, with the normal map's detail pressed in.
      const std = own as THREE.MeshStandardMaterial
      m = new THREE.MeshMatcapNodeMaterial({ matcap: this.matcapTexture(), normalMap: maps.normalMap ?? null })
      if (std.normalScale) (m as THREE.MeshMatcapNodeMaterial).normalScale.copy(std.normalScale)
    } else if (mode === 'uv') {
      m = hasUv
        ? new THREE.MeshStandardNodeMaterial({ map: this.checkerTexture(), roughness: 0.85, metalness: 0 })
        : new THREE.MeshStandardNodeMaterial({ color: 0x808080, roughness: 0.85, metalness: 0 })
    } else {
      // Hues a golden angle apart, so neighbours in the file differ however many meshes there are.
      const color = new THREE.Color().setHSL((index * 0.618034) % 1, 0.6, 0.55)
      m = new THREE.MeshStandardNodeMaterial({ color, roughness: 0.6, metalness: 0 })
      // Meshes merged into one still show as the parts they were.
      if (parts) m.colorNode = attribute(PART_COLOR, 'vec3')
    }
    m.side = own.side
    ;(m as AnyMaterial).wireframe = this.wireframeOn
    // Cut-outs (leaves, fences) keep their holes, so the shape reads the same in every view.
    if (own.alphaTest > 0) {
      const cut = maps.alphaMap ? texture(maps.alphaMap).g : maps.map ? texture(maps.map).a : null
      if (cut) {
        m.opacityNode = cut
        m.alphaTest = own.alphaTest
      }
    }
    return m as AnyMaterial
  }

  /** A clay ball lit from the upper left, drawn pixel by pixel: the matcap view's only light. */
  private matcapTexture(): THREE.Texture {
    if (this.matcap) return this.matcap
    const S = 256
    const canvas = document.createElement('canvas')
    canvas.width = canvas.height = S
    const ctx = canvas.getContext('2d')!
    const img = ctx.createImageData(S, S)
    const L = new THREE.Vector3(-0.45, 0.6, 0.66).normalize()
    // A dim fill from the lower right, so faces turned from the key light still show their shape.
    const F = new THREE.Vector3(0.6, -0.25, 0.5).normalize()
    const H = L.clone().add(new THREE.Vector3(0, 0, 1)).normalize()
    const n = new THREE.Vector3()
    for (let y = 0; y < S; y++) {
      for (let x = 0; x < S; x++) {
        n.set(((x + 0.5) / S) * 2 - 1, 1 - ((y + 0.5) / S) * 2, 0)
        // Past the rim, the rim's own shade, so edges sampled at the very border don't pick up black.
        const r2 = Math.min(n.x * n.x + n.y * n.y, 1)
        if (r2 >= 1) n.normalize()
        n.z = Math.sqrt(1 - r2)
        const diffuse = Math.max(0, n.dot(L))
        const spec = Math.pow(Math.max(0, n.dot(H)), 40) * 0.18
        const rim = Math.pow(1 - n.z, 3) * 0.12
        const shade = 0.3 + 0.62 * diffuse + 0.2 * Math.max(0, n.dot(F))
        const i = (y * S + x) * 4
        ;[0.76, 0.72, 0.68].forEach((base, c) => (img.data[i + c] = Math.min(255, (base * shade + spec + rim) * 255)))
        img.data[i + 3] = 255
      }
    }
    ctx.putImageData(img, 0, 0)
    const t = new THREE.CanvasTexture(canvas)
    t.colorSpace = THREE.SRGBColorSpace
    this.matcap = t
    return t
  }

  /** An 8 × 8 checker of coloured, labelled cells (A1 to H8): stretching shows in the squares, seams where the
   *  labels jump, flipped islands in mirrored letters. */
  private checkerTexture(): THREE.Texture {
    if (this.checker) return this.checker
    const S = 1024
    const N = 8
    const cell = S / N
    const canvas = document.createElement('canvas')
    canvas.width = canvas.height = S
    const ctx = canvas.getContext('2d')!
    ctx.textAlign = 'center'
    ctx.textBaseline = 'middle'
    ctx.font = `600 ${cell * 0.36}px system-ui, sans-serif`
    for (let row = 0; row < N; row++) {
      for (let col = 0; col < N; col++) {
        const light = (row + col) % 2 === 0
        ctx.fillStyle = `hsl(${(col * 360) / N} 60% ${light ? 70 : 42}%)`
        ctx.fillRect(col * cell, row * cell, cell, cell)
        // A small square in one corner of each cell shows which way round it is.
        ctx.fillStyle = light ? 'rgb(0 0 0 / 0.35)' : 'rgb(255 255 255 / 0.45)'
        ctx.fillRect(col * cell + cell * 0.08, row * cell + cell * 0.08, cell * 0.14, cell * 0.14)
        ctx.fillStyle = light ? '#1b1c20' : '#ffffff'
        // Row 1 at the bottom of the picture.
        ctx.fillText(`${String.fromCharCode(65 + col)}${N - row}`, col * cell + cell / 2, row * cell + cell / 2)
      }
    }
    const t = new THREE.CanvasTexture(canvas)
    t.colorSpace = THREE.SRGBColorSpace
    t.wrapS = t.wrapT = THREE.RepeatWrapping
    t.anisotropy = 8
    t.flipY = this.flipY
    this.checker = t
    return t
  }

  setTextures(on: boolean) {
    this.texturesOn = on
    for (const [m, maps] of this.materials) {
      for (const slot of Object.keys(maps) as MapSlot[]) m[slot] = on ? maps[slot]! : null
      m.needsUpdate = true
    }
    this.redraw()
  }

  setEnvironment(on: boolean) {
    this.envOn = on
    this.applyLook()
  }

  /** The light, tone mapping and background a mode wants. Final render is the switches as they are; raw render the
   *  model's own materials under plain, even light with nothing added; the map and matcap views are unlit, shown as
   *  their pixels are; the UV checker and geometry views take the studio light, so their shapes read. */
  private applyLook(mode = this.mode) {
    const studio = mode === 'final' ? this.envOn : mode === 'uv' || mode === 'geometry'
    const environment = studio ? this.envMap : null
    if (this.scene.environment !== environment) {
      this.scene.environment = environment
      // Materials are built for the light they're lit by.
      for (const m of [...this.materials.keys(), ...(this.views?.made ?? [])]) m.needsUpdate = true
    }
    this.scene.background = this.bg === 'env' && (mode !== 'final' || this.envOn) ? this.envMap : null
    if (this.renderer) {
      this.renderer.toneMapping = mode === 'final' || mode === 'uv' || mode === 'geometry' ? THREE.NeutralToneMapping : THREE.NoToneMapping
    }
    // With the environment off, the direct lights carry everything and need to be brighter.
    const raw = mode === 'raw'
    this.hemi.color.set(raw ? 0xffffff : 0xdfe8ff)
    this.hemi.groundColor.set(raw ? 0xa8a8a8 : 0x3a3226)
    this.hemi.intensity = raw ? 2.4 : studio ? 0.35 : 1.6
    this.key.intensity = raw ? 0.9 : studio ? 1.6 : 2.4
    this.redraw()
  }

  /** CSS backgrounds show through the transparent canvas; `env` draws the environment itself, blurred. */
  setBackground(bg: Background) {
    this.bg = bg
    this.scene.backgroundBlurriness = 0.5
    this.scene.backgroundIntensity = 0.8
    this.applyLook()
  }

  setGrid(on: boolean) {
    if (this.grid) this.grid.visible = on
    this.redraw()
  }

  setAutoRotate(on: boolean) {
    if (this.controls) this.controls.autoRotate = on
    this.redraw()
  }

  /** Play a clip by name ('' stops). */
  playClip(name: string) {
    if (!this.mixer) return
    this.mixer.stopAllAction()
    this.action = null
    const clip = this.clips.get(name)
    if (clip) {
      this.action = this.mixer.clipAction(clip)
      this.action.paused = !this.playing
      this.action.play()
    }
    this.redraw()
  }

  setPlaying(on: boolean) {
    this.playing = on
    if (this.action) this.action.paused = !on
    this.redraw()
  }

  /** Stop drawing while hidden (the full-screen viewer covers the side panel's). */
  setPaused(on: boolean) {
    this.paused = on
    if (on) this.renderer?.setAnimationLoop(null)
    else this.start()
  }

  get texturesEnabled() {
    return this.texturesOn
  }

  /** The model as its thumbnail: a square PNG taken from the angle the server draws thumbnails from (`camera` in
   *  hoard-render), framed as tightly, in the final render lit by the studio room and the key light, on a clear
   *  background, with textures on, wireframe and grid off. The view on screen is put back afterwards. */
  async snapshot(size = 256): Promise<Blob> {
    const renderer = this.renderer
    const object = this.object
    if (!renderer || !object) throw new Error('no model loaded')
    const view = snapshotView(object, new THREE.Vector3(0.75, 0.55, 1))
    const cam = this.camera
    const saved = {
      position: cam.position.clone(), up: cam.up.clone(), quaternion: cam.quaternion.clone(),
      fov: cam.fov, aspect: cam.aspect, near: cam.near, far: cam.far,
      size: renderer.getSize(new THREE.Vector2()), ratio: renderer.getPixelRatio(),
      background: this.scene.background, grid: this.grid?.visible ?? false, env: this.envOn, textures: this.texturesOn,
      wireframe: this.wireframeOn,
    }
    this.showViews(false)
    if (!this.envOn) this.setEnvironment(true)
    if (!this.texturesOn) this.setTextures(true)
    if (saved.wireframe) this.setWireframe(false)
    this.applyLook('final')
    this.scene.background = null
    if (this.grid) this.grid.visible = false
    cam.up.copy(view.up)
    cam.position.copy(view.eye)
    cam.lookAt(view.target)
    cam.fov = 35
    cam.aspect = 1
    cam.near = view.near
    cam.far = view.far
    cam.updateProjectionMatrix()
    // Drawn at twice the size and scaled down, for smooth edges. Everything from here to the copy happens in one go:
    // the canvas shows only what's drawn last, and the picture is read before the browser puts it on screen.
    const out = document.createElement('canvas')
    out.width = out.height = size
    try {
      renderer.setPixelRatio(1)
      renderer.setSize(size * 2, size * 2, false)
      renderer.render(this.scene, cam)
      const ctx = out.getContext('2d')!
      ctx.imageSmoothingQuality = 'high'
      ctx.drawImage(renderer.domElement, 0, 0, size, size)
    } finally {
      cam.position.copy(saved.position)
      cam.up.copy(saved.up)
      cam.quaternion.copy(saved.quaternion)
      Object.assign(cam, { fov: saved.fov, aspect: saved.aspect, near: saved.near, far: saved.far })
      cam.updateProjectionMatrix()
      renderer.setPixelRatio(saved.ratio)
      renderer.setSize(saved.size.x, saved.size.y, false)
      this.scene.background = saved.background
      if (this.grid) this.grid.visible = saved.grid
      if (!saved.env) this.setEnvironment(false)
      if (!saved.textures) this.setTextures(false)
      if (saved.wireframe) this.setWireframe(true)
      this.showViews(true)
      this.applyLook()
      this.renderOnce()
    }
    return new Promise((resolve, reject) =>
      out.toBlob((b) => (b ? resolve(b) : reject(new Error("the picture couldn't be made"))), 'image/png'),
    )
  }

  private start() {
    if (!this.renderer || this.paused || this.disposed) return
    this.timer.reset()
    this.redraw(1000)
    this.renderer.setAnimationLoop((time) => this.tick(time))
  }

  /** Draw the frames from now until `ms` from now. A change is drawn for a moment longer than it takes: a texture or
   *  material reaches the GPU a frame or two after it's asked for. */
  private redraw(ms = 500) {
    this.redrawUntil = Math.max(this.redrawUntil, performance.now() + ms)
  }

  /** Each frame moves what moves; it's drawn only if something did, or changed a moment ago. A still model costs
   *  nothing to look at. */
  private tick(time: number) {
    this.timer.update(time)
    const dt = Math.min(this.timer.getDelta(), 0.1)
    const animating = !!this.action?.isRunning()
    if (animating) this.mixer?.update(dt)
    if (this.glide) this.redraw(100)
    if (this.glide && this.controls) {
      const g = this.glide
      g.t = Math.min(1, g.t + dt / 0.35)
      const k = 1 - Math.pow(1 - g.t, 3)
      this.camera.position.lerpVectors(g.from, g.to, k)
      this.controls.target.lerpVectors(g.fromT, g.toT, k)
      if (g.t >= 1) this.glide = null
    }
    this.controls?.update(dt)
    if (animating || this.pending || performance.now() < this.redrawUntil) this.renderer?.render(this.scene, this.camera)
  }

  private renderOnce() {
    this.controls?.update()
    this.renderer?.render(this.scene, this.camera)
  }

  private resize() {
    const el = this.host
    if (!this.renderer || !el.clientWidth || !el.clientHeight) return
    this.renderer.setSize(el.clientWidth, el.clientHeight)
    this.camera.aspect = el.clientWidth / el.clientHeight
    this.camera.updateProjectionMatrix()
    this.redraw()
    if (this.paused) this.renderOnce()
  }

  dispose() {
    this.disposed = true
    this.stopMaps?.()
    this.dropViews()
    this.matcap?.dispose()
    this.checker?.dispose()
    this.resizeObs?.disconnect()
    this.mixer?.stopAllAction()
    this.controls?.dispose()
    if (this.object) disposeObject(this.object)
    this.grid?.dispose()
    this.envMap?.dispose()
    if (this.renderer) {
      this.renderer.setAnimationLoop(null)
      this.renderer.domElement.remove()
      this.renderer.dispose()
      this.renderer = null
    }
  }
}

/** Where the thumbnail camera stands: looking from `dir`, close enough that the model's silhouette from there fills
 *  90% of the frame. The same sums as `camera` in hoard-render, so a browser's thumbnail and the server's match. */
function snapshotView(object: THREE.Object3D, dir: THREE.Vector3) {
  const FILL = 0.9
  const fov = (35 * Math.PI) / 180
  dir = dir.clone().normalize()
  object.updateWorldMatrix(true, true)
  const box = new THREE.Box3().setFromObject(object, true)
  const center = box.getCenter(new THREE.Vector3())
  const radius = Math.max(box.getSize(new THREE.Vector3()).length() / 2, 1e-6)
  const up0 = Math.abs(dir.y) > 0.95 ? new THREE.Vector3(0, 0, -1) : new THREE.Vector3(0, 1, 0)
  const right = up0.clone().cross(dir).normalize()
  const up = dir.clone().cross(right)
  // Every vertex where it stands now (skinned and morphed), about the box centre, in the camera's axes. Big meshes
  // are sampled: the frame's margin covers what a sample misses.
  const local: number[] = []
  const v = new THREE.Vector3()
  let total = 0
  object.traverse((o) => {
    const g = (o as THREE.Mesh).geometry
    if ((o as THREE.Mesh).isMesh && g?.attributes.position) total += g.attributes.position.count
  })
  const step = Math.max(1, Math.ceil(total / 200_000))
  object.traverse((o) => {
    const mesh = o as THREE.Mesh
    const pos = mesh.isMesh ? mesh.geometry?.attributes.position : undefined
    if (!pos) return
    for (let i = 0; i < pos.count; i += step) {
      mesh.getVertexPosition(i, v).applyMatrix4(mesh.matrixWorld).sub(center)
      local.push(v.dot(right), v.dot(up), v.dot(dir))
    }
  })
  if (!local.length) local.push(0, 0, 0)
  let [lx, ly, lz, hx, hy, hz] = [Infinity, Infinity, Infinity, -Infinity, -Infinity, -Infinity]
  for (let i = 0; i < local.length; i += 3) {
    lx = Math.min(lx, local[i]); hx = Math.max(hx, local[i])
    ly = Math.min(ly, local[i + 1]); hy = Math.max(hy, local[i + 1])
    lz = Math.min(lz, local[i + 2]); hz = Math.max(hz, local[i + 2])
  }
  const mx = (lx + hx) / 2
  const my = (ly + hy) / 2
  const target = center.clone().addScaledVector(right, mx).addScaledVector(up, my)
  const t = Math.tan(fov / 2) * FILL
  let dist = hz + radius * 0.02
  for (let i = 0; i < local.length; i += 3) {
    const x = local[i] - mx
    const y = local[i + 1] - my
    dist = Math.max(dist, local[i + 2] + Math.max(Math.abs(x), Math.abs(y)) / t)
  }
  return {
    eye: target.clone().addScaledVector(dir, dist),
    target,
    up,
    near: Math.max(dist - hz, radius * 0.001) * 0.5,
    far: dist - lz + radius,
  }
}

/** The attribute a merged mesh keeps each part's colour in, for the geometry view. */
const PART_COLOR = 'partColor'

/** Merge the meshes that never move apart and look the same (one material, the same attributes) into one mesh
 *  each: a model of thousands of parts otherwise takes thousands of draws a frame, which is what makes a big CAD
 *  model slow to turn, not its triangles. Each part's vertices are put where it stands, and its colour in the
 *  geometry view goes in `PART_COLOR`. Returns the meshes left to draw. */
function mergeStill(root: THREE.Object3D, meshes: THREE.Mesh[]): THREE.Mesh[] {
  root.updateWorldMatrix(true, true)
  // Parts are put where they stand in the root, which keeps its own transform.
  const toRoot = root.matrixWorld.clone().invert()
  const rootDet = root.matrixWorld.determinant()
  const groups = new Map<string, { mesh: THREE.Mesh; index: number }[]>()
  meshes.forEach((mesh, index) => {
    const g = mesh.geometry
    const a = g.attributes
    if ((mesh as THREE.SkinnedMesh).isSkinnedMesh || (mesh as THREE.InstancedMesh).isInstancedMesh || Array.isArray(mesh.material)) return
    // Quantized positions would be clamped to their range once moved.
    if (Object.keys(g.morphAttributes).length || !(a.position?.array instanceof Float32Array) || !mesh.visible) return
    // Mirrored parts are drawn with their faces turned the other way: they'd merge inside out.
    if (mesh.matrixWorld.determinant() * rootDet < 0) return
    const shape = Object.keys(a).sort().map((n) => `${n}:${a[n].itemSize}:${a[n].normalized}:${a[n].array.constructor.name}`)
    const key = `${(mesh.material as THREE.Material).uuid}|${!!g.index}|${mesh.renderOrder}|${shape.join(',')}`
    let list = groups.get(key)
    if (!list) groups.set(key, (list = []))
    list.push({ mesh, index })
  })
  const color = new THREE.Color()
  const merged = new Set<THREE.Mesh>()
  const out: THREE.Mesh[] = []
  for (const list of groups.values()) {
    if (list.length < 2) continue
    const parts = list.map(({ mesh, index }) => {
      const g = mesh.geometry.clone().applyMatrix4(toRoot.clone().multiply(mesh.matrixWorld))
      // Hues a golden angle apart, as the geometry view gives meshes of their own.
      color.setHSL((index * 0.618034) % 1, 0.6, 0.55)
      const n = g.attributes.position.count
      const c = new Float32Array(n * 3)
      for (let i = 0; i < n * 3; i += 3) {
        c[i] = color.r
        c[i + 1] = color.g
        c[i + 2] = color.b
      }
      g.setAttribute(PART_COLOR, new THREE.BufferAttribute(c, 3))
      return g
    })
    const geometry = mergeGeometries(parts)
    for (const g of parts) g.dispose()
    if (!geometry) continue
    const first = list[0].mesh
    const mesh = new THREE.Mesh(geometry, first.material)
    mesh.name = first.name
    mesh.renderOrder = first.renderOrder
    root.add(mesh)
    out.push(mesh)
    // Each part's node stays, empty, for what hangs from it.
    for (const { mesh: m } of list) {
      merged.add(m)
      const node = new THREE.Group()
      node.name = m.name
      node.position.copy(m.position)
      node.quaternion.copy(m.quaternion)
      node.scale.copy(m.scale)
      for (const child of [...m.children]) node.add(child)
      m.parent?.add(node)
      m.removeFromParent()
      m.geometry.dispose()
    }
  }
  if (!merged.size) return meshes
  return [...meshes.filter((m) => !merged.has(m)), ...out]
}

/** Free a loaded model's GPU resources: geometry, materials and every texture they hold. */
function disposeObject(root: THREE.Object3D) {
  root.traverse((o) => {
    const mesh = o as THREE.Mesh
    mesh.geometry?.dispose()
    const mats = mesh.material ? (Array.isArray(mesh.material) ? mesh.material : [mesh.material]) : []
    for (const m of mats as AnyMaterial[]) {
      for (const slot of MAP_SLOTS) m[slot]?.dispose()
      m.dispose()
    }
  })
}
