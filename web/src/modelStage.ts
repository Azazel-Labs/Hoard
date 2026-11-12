// The 3D scene behind both model viewers (the side panel's preview and the full-screen one): load a model with its
// textures, light it, frame it, and the switches the full-screen viewer offers. three.js with its WebGPU renderer,
// which falls back to WebGL2 by itself. Only the viewers import this, and they load on demand, so three stays out of
// the main bundle.
import * as THREE from 'three/webgpu'
import { OrbitControls } from 'three/addons/controls/OrbitControls.js'
import { RoomEnvironment } from 'three/addons/environments/RoomEnvironment.js'
import { TGALoader } from 'three/addons/loaders/TGALoader.js'
import { api, asUrl, packFileUrl, thumbUrl, type Asset, type AssetLink } from './api'

export type Background = 'checker' | 'dark' | 'grey' | 'light' | 'env'
export const BACKGROUNDS: Background[] = ['checker', 'dark', 'grey', 'light', 'env']

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

/** Where a texture's pixels come from: the file itself when the browser can read it, else a PNG the server makes
 *  (cached after the first time), else the 256 px thumbnail. */
function textureUrl(t: AssetLink, packId: string): string {
  const ext = extOf(t.path)
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
    case 'stl':
    case 'ply': {
      const mod = ext === 'stl' ? await import('three/addons/loaders/STLLoader.js') : await import('three/addons/loaders/PLYLoader.js')
      const Loader = 'STLLoader' in mod ? mod.STLLoader : (mod as typeof import('three/addons/loaders/PLYLoader.js')).PLYLoader
      const geom = await new Loader(manager).loadAsync(url)
      if (!geom.attributes.normal) geom.computeVertexNormals()
      const material = new THREE.MeshStandardMaterial({ color: 0xb8c0d0, roughness: 0.6, vertexColors: !!geom.attributes.color })
      return { object: new THREE.Mesh(geom, material), animations: [] }
    }
    case 'dae': {
      const { ColladaLoader } = await import('three/addons/loaders/ColladaLoader.js')
      const c = await new ColladaLoader(manager).loadAsync(url)
      if (!c) throw new Error('empty Collada file')
      return { object: c.scene, animations: c.scene.animations ?? [] }
    }
    default:
      throw new Error(`No browser loader for .${ext}`)
  }
}

function sourceFor(a: Asset | AssetLink, packId: string, derivatives: string[] = []): { url: string; ext: string } {
  if (derivatives.includes('glb')) return { url: `/api/assets/${a.id}/derivative/glb`, ext: 'glb' }
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

export class ModelStage {
  readonly host: HTMLElement
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
  playing = true
  private missing: string[] = []
  /** Textures go on loading after the model is up; this hears about the ones that fail. */
  onMissing: ((names: string[]) => void) | null = null

  constructor(host: HTMLElement) {
    this.host = host
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
    this.scene.environment = this.envMap
    this.scene.add(this.hemi)
    this.camera.add(this.key, this.key.target)
    this.key.position.set(1.2, 1.6, 0.6)
    this.key.target.position.set(0, 0, -2)
    this.scene.add(this.camera)
    this.applyLights()

    const controls = new OrbitControls(this.camera, renderer.domElement)
    controls.enableDamping = true
    controls.addEventListener('start', () => (this.glide = null))
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
      const tgaForOther = hit && extOf(hit.path) === 'tga' && extOf(path) !== 'tga'
      const out = !hit ? url : tgaForOther ? `${asUrl(hit, 'png', 2048)}&download=0&wait=60` : textureUrl(hit, asset.pack_id)
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
    manager.addHandler(/\.tga$/i, new TGALoader(manager))

    const src = sourceFor(full, full.pack_id, full.derivatives)
    const { object, animations, linearColors } = await loadModel(src.url, src.ext, manager)
    if (this.disposed) {
      disposeObject(object)
      throw new Error('closed')
    }

    let triangles = 0
    let meshes = 0
    const textured = { any: false }
    const converted = new Map<THREE.Material, AnyMaterial>()
    object.traverse((o) => {
      const mesh = o as THREE.Mesh
      if (!mesh.isMesh && !(o as THREE.Points).isPoints && !(o as THREE.Line).isLine) return
      if (mesh.isMesh) {
        meshes++
        const g = mesh.geometry
        triangles += (g.index?.count ?? g.attributes.position?.count ?? 0) / 3
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

    // A model that names no textures, next to textures named after it: try them on.
    let guessed = 0
    if (!textured.any && guesses.length) guessed = this.applyGuesses(guesses, asset.pack_id, src.ext !== 'glb' && src.ext !== 'gltf', manager)

    this.object = object
    this.scene.add(object)
    this.frameHome()

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

    return {
      triangles: Math.round(triangles),
      meshes,
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
      const url = textureUrl(link, packId)
      const t: THREE.Texture = extOf(link.path) === 'tga' ? tga.load(url) : image.load(url)
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
    for (const m of this.materials.keys()) {
      m.wireframe = on
      m.needsUpdate = true
    }
  }

  setTextures(on: boolean) {
    this.texturesOn = on
    for (const [m, maps] of this.materials) {
      for (const slot of Object.keys(maps) as MapSlot[]) m[slot] = on ? maps[slot]! : null
      m.needsUpdate = true
    }
  }

  setEnvironment(on: boolean) {
    this.envOn = on
    this.scene.environment = on ? this.envMap : null
    if (!on && this.scene.background === this.envMap) this.scene.background = null
    this.applyLights()
    for (const m of this.materials.keys()) m.needsUpdate = true
  }

  /** With the environment off, the direct lights carry everything and need to be brighter. */
  private applyLights() {
    this.hemi.intensity = this.envOn ? 0.35 : 1.6
    this.key.intensity = this.envOn ? 1.6 : 2.4
  }

  /** CSS backgrounds show through the transparent canvas; `env` draws the environment itself, blurred. */
  setBackground(bg: Background) {
    this.scene.background = bg === 'env' && this.envOn ? this.envMap : null
    this.scene.backgroundBlurriness = 0.5
    this.scene.backgroundIntensity = 0.8
  }

  setGrid(on: boolean) {
    if (this.grid) this.grid.visible = on
  }

  setAutoRotate(on: boolean) {
    if (this.controls) this.controls.autoRotate = on
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
  }

  setPlaying(on: boolean) {
    this.playing = on
    if (this.action) this.action.paused = !on
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

  private start() {
    if (!this.renderer || this.paused || this.disposed) return
    this.timer.reset()
    this.renderer.setAnimationLoop((time) => this.tick(time))
  }

  private tick(time: number) {
    this.timer.update(time)
    const dt = Math.min(this.timer.getDelta(), 0.1)
    this.mixer?.update(dt)
    if (this.glide && this.controls) {
      const g = this.glide
      g.t = Math.min(1, g.t + dt / 0.35)
      const k = 1 - Math.pow(1 - g.t, 3)
      this.camera.position.lerpVectors(g.from, g.to, k)
      this.controls.target.lerpVectors(g.fromT, g.toT, k)
      if (g.t >= 1) this.glide = null
    }
    this.controls?.update(dt)
    this.renderer?.render(this.scene, this.camera)
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
    if (this.paused) this.renderOnce()
  }

  dispose() {
    this.disposed = true
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
