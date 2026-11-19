// The 3D scene behind both model viewers (the side panel's preview and the full-screen one): load a model with its
// textures, light it, frame it, and the switches the full-screen viewer offers. three.js with its WebGPU renderer,
// which falls back to WebGL2 by itself. Only the viewers import this, and they load on demand, so three stays out of
// the main bundle.
import * as THREE from 'three/webgpu'
import { attribute, sRGBTransferEOTF, texture, vec3 } from 'three/tsl'
import { OrbitControls } from 'three/addons/controls/OrbitControls.js'
import { RoomEnvironment } from 'three/addons/environments/RoomEnvironment.js'
import { TGALoader } from 'three/addons/loaders/TGALoader.js'
import { api, asUrl, packFacings, type Asset, type AssetLink, type Facing } from './api'
import { applyToModel } from './materialMaps'
import { BLANK_PNG, BROWSER_IMAGES, IMAGE_EXTS, TgaOrImageLoader, extOf, loadModel, nameOf, sourceFor, stemOf, textureLinks, textureUrl } from './modelFiles'
import { MAP_SLOTS, roleOf, sanitize, toStandard, type AnyMaterial, type MapSlot } from './modelMaterials'
import { PART_COLOR, disposeObject, mergeStill, snapshotView } from './modelScene'

export type Background = 'checker' | 'dark' | 'gray' | 'light' | 'env'
export const BACKGROUNDS: Background[] = ['checker', 'dark', 'gray', 'light', 'env']

/** How the full-screen viewer shows the model: as it renders, plainly lit, one of its maps on its own, or a look that
 *  shows its shape (clay, a UV checker, each mesh in its own color). */
export type ViewMode = 'final' | 'raw' | MapMode | 'matcap' | 'uv' | 'geometry'
export type MapMode = 'color' | 'roughness' | 'metalness' | 'normal' | 'ao'

/** Each map view: the slot it shows and the channel three reads from it. glTF packs roughness into G and metalness
 *  into B of one texture, and occlusion lives in R; maps of their own are gray, so the same channel serves. */
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
  /** glTF is in meters; other formats say nothing dependable about their units. */
  meters: boolean
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
  /** How the model is turned, as its pack's models are. */
  facing: Facing
  /** For an animation: the characters it fits, its pack's first, and the one it plays on (`LoadOptions.on`), or
   *  none. One from another pack (`away`) is named with its pack. */
  wearers: { id: string; name: string; away?: boolean }[]
  on: string
}

export interface LoadOptions {
  /** For an animation: the character to play it on (its id), of those it fits. The first, if not given. */
  on?: string
}

/** Clips shorter than this hold a pose ("Targeting Pose"), as the server reckons them (`animations::POSE_SECS`). */
const POSE_SECS = 0.1

export interface StageOptions {
  /** Textures as their files are, however big. Otherwise each comes as its light preview (at most 2048 px, see
   *  `liteUrl`), which loads far faster and looks the same at viewer sizes. */
  fullTextures?: boolean
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
  /** The model, inside a group that turns it as its pack's models face (`setFacing`). */
  private object: THREE.Object3D | null = null
  private facing: Facing = { turn: 0, tilt: 0 }
  private mixer: THREE.AnimationMixer | null = null
  private clips = new Map<string, THREE.AnimationClip>()
  private action: THREE.AnimationAction | null = null
  /** An animation played on a character, rather than a character playing its own clips. */
  private wearing = false
  /** An animation (on a character or its own mesh): its thumbnail shows it mid-clip. */
  private animation = false
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
  private meters = false
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
  async load(asset: Asset, opts: LoadOptions = {}): Promise<StageInfo> {
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
    // Neutral tone mapping keeps base colors as painted (ACES would shift and desaturate them), while highlights
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

    // Every file a loader asks for passes through here (the server's GLBs carry their textures; a Collada file read
    // here names its own): textures are matched to the pack's files by name, as they often carry the artist's absolute
    // path, or point at a PSD where the pack ships a PNG.
    let { named, guesses, full } = await textureLinks(asset)
    // An animation plays on a character it fits: the character is what's drawn, and only the animation's clips play.
    // Those its pack ships it for come first, then any other with its skeleton, in any pack.
    let clipLinks = full.links.filter((l) => l.rel === 'animation')
    this.animation = full.kind === 'animation'
    const wearers =
      full.kind === 'animation'
        ? [
            ...full.linked_from.filter((l) => l.rel === 'animation' && l.kind === 'model'),
            ...(full.fits ?? []).map((f) =>
              f.pack_id === full.pack_id ? { id: f.id, name: f.name } : { id: f.id, name: `${f.name} · ${f.pack_name}`, away: true },
            ),
          ]
        : []
    const wearer = wearers.find((w) => w.id === opts.on) ?? wearers[0]
    const body = wearer && (await api.asset(wearer.id).catch(() => null))
    if (this.disposed) throw new Error('closed')
    if (body) {
      const { id, name, path, kind, has_thumb } = full
      const own: AssetLink = { id, name, path, kind, rel: 'animation', has_thumb }
      ;({ named, guesses, full } = await textureLinks(body))
      clipLinks = [own]
      this.wearing = true
    }
    const byName = new Map<string, AssetLink>()
    const byStem = new Map<string, AssetLink>()
    for (const t of [...named, ...guesses]) {
      if (!byName.has(nameOf(t.path))) byName.set(nameOf(t.path), t)
      // A name match beats a stem match, and among stems the browser-readable file wins.
      const prev = byStem.get(stemOf(t.path))
      if (!prev || (!BROWSER_IMAGES.has(extOf(prev.path)) && BROWSER_IMAGES.has(extOf(t.path)))) byStem.set(stemOf(t.path), t)
    }
    const byPath = new Map([...named, ...guesses].map((t) => [t.path.toLowerCase(), t]))
    const packFiles = `/api/packs/${full.pack_id}/files/`
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
          : textureUrl(hit, full.pack_id, this.fullTextures)
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
    const { object, animations, missing } = await loadModel(src.url, src.ext, manager)
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
        // Meshes share materials: convert each once, so they stay shared.
        let s = converted.get(m)
        if (!s) {
          s = toStandard(m) as AnyMaterial
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
    if (!animations.length && !clipLinks.length) this.meshes = mergeStill(object, this.meshes)

    // A model that names no textures, next to textures named after it: try them on.
    let guessed = 0
    this.flipY = src.ext === 'dae'
    const onMaps = () => {
      // A view built from the maps that were there shows the new ones only once built again.
      if (this.views) this.buildViews()
      this.redraw()
      this.onTexturesChanged?.()
    }
    // The material's maps count as the model's textures: a thumbnail waits for them, and one that fails is missing.
    const track = (name: string) => {
      this.pending++
      return (ok: boolean) => {
        this.pending = Math.max(0, this.pending - 1)
        this.redraw()
        if (!ok && !this.missing.includes(name)) {
          this.missing.push(name)
          this.onMissing?.([...this.missing])
        }
        this.checkSettled()
      }
    }
    // Materials that go in the model's slots (Unity's, by an FBX's material names): each on the slots it names
    // that have no color map of their own.
    const slotted = (full.materials ?? []).filter((m) => m.slots?.length)
    // Or a material made from maps that share its name: its maps by role.
    const material = textured.any || slotted.length ? undefined : full.materials?.[0]
    if (slotted.length) {
      const stops: (() => void)[] = []
      for (const info of slotted) {
        const wear = new Map(
          [...this.materials].filter(([m]) => info.slots!.includes(m.name) && !(m as THREE.MeshStandardMaterial).map),
        )
        if (!wear.size) continue
        const tint = info.look?.base_color
        for (const m of wear.keys()) {
          const lit = m as THREE.MeshStandardMaterial
          if (tint) lit.color?.setRGB(tint[0], tint[1], tint[2], THREE.LinearSRGBColorSpace)
        }
        if (!info.maps.length) continue
        const applied = applyToModel(info, full.pack_id, this.flipY, wear, onMaps, track)
        stops.push(applied.stop)
        guessed = Math.max(guessed, applied.count)
      }
      this.stopMaps = () => stops.forEach((stop) => stop())
    } else if (material) {
      const applied = applyToModel(material, full.pack_id, this.flipY, this.materials, onMaps, track)
      this.stopMaps = applied.stop
      guessed = applied.count
    } else if (!textured.any && guesses.length && !this.colored()) {
      guessed = this.applyGuesses(guesses, full.pack_id, this.flipY, manager)
    }

    for (const name of missing ?? []) if (!this.missing.includes(name)) this.missing.push(name)
    if (missing?.length) this.onMissing?.([...this.missing])

    const turner = new THREE.Group()
    turner.add(object)
    this.object = turner
    this.scene.add(turner)
    this.turn(packFacings.get(full.pack_id) ?? full.facing ?? { turn: 0, tilt: 0 })
    this.frameHome()
    this.counts = { triangles: Math.round(triangles), vertices }
    this.bounds().box.getSize(this.size)
    this.meters = ['glb', 'gltf'].includes(extOf(full.path))

    // Clips embedded in the file plus linked clip files ("Knight@Run.fbx"); a character wearing an animation plays
    // only that.
    if (!this.wearing) animations.forEach((c, i) => this.clips.set(c.name || `clip ${i + 1}`, c))
    // Clip files only lend their animation: don't fetch their textures.
    const clipManager = new THREE.LoadingManager()
    clipManager.setURLModifier((url) => (IMAGE_EXTS.has(extOf(url.split(/[?#]/)[0])) ? BLANK_PNG : url))
    for (const link of clipLinks) {
      try {
        const s = sourceFor(link, full.pack_id)
        const loaded = await loadModel(s.url, s.ext, clipManager)
        const label = link.path.split('/').pop()!.replace(/\.[^.]+$/, '').split('@').pop()!
        // A pose saved beside the clip (Blender's "Targeting Pose") isn't one to play.
        const takes = loaded.animations.filter((c) => c.duration >= POSE_SECS)
        const list = takes.length ? takes : loaded.animations
        list.forEach((c, i) => this.clips.set(list.length > 1 ? `${label} ${i + 1}` : label, c))
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
      facing: { ...this.facing },
      wearers,
      on: body ? body.id : '',
    }
  }

  private turn(facing: Facing) {
    this.facing = { turn: facing.turn, tilt: facing.tilt }
    this.object!.rotation.set((facing.tilt * Math.PI) / 180, (facing.turn * Math.PI) / 180, 0, 'YXZ')
    this.object!.updateMatrixWorld(true)
  }

  /** Turn the model (to try a facing before giving it to the pack) and look at it afresh from the front. */
  setFacing(facing: Facing) {
    if (!this.object) return
    this.turn(facing)
    const grid = this.grid?.visible ?? false
    this.frameHome()
    if (this.grid) this.grid.visible = grid
    this.bounds().box.getSize(this.size)
    this.redraw()
    if (this.paused) this.renderOnce()
  }

  private track(m: AnyMaterial) {
    if (this.materials.has(m)) return
    const maps: Partial<Record<MapSlot, THREE.Texture | null>> = {}
    for (const slot of MAP_SLOTS) if (m[slot]) maps[slot] = m[slot]
    this.materials.set(m, maps)
  }

  /** Whether the model paints its surfaces with colors of its own (Kenney's kits: tire gray, paint red), so a
   *  texture found by name (most likely a 2D sprite of it) would only cover them up: two colors besides white, or
   *  one that isn't a gray (a white body with orange trim). */
  private colored(): boolean {
    const colors = new Set<number>()
    let hue = false
    for (const m of this.materials.keys()) {
      const c = m.color
      if (!c || c.getHex() === 0xffffff) continue
      colors.add(c.getHex())
      hue ||= Math.max(c.r, c.g, c.b) - Math.min(c.r, c.g, c.b) > 0.05
    }
    return colors.size > 1 || hue
  }

  private applyGuesses(guesses: AssetLink[], packId: string, flipY: boolean, manager: THREE.LoadingManager): number {
    const roles = new Map<MapSlot, AssetLink>()
    const unknown: AssetLink[] = []
    for (const g of guesses) {
      const role = roleOf(g.path)
      if (role === null) unknown.push(g)
      else if (role !== 'skip' && !roles.has(role)) roles.set(role, g)
    }
    // No texture says it's the color: the first one whose name doesn't say otherwise ("Barrel.png") is.
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
      // A painted color map stands for the whole surface: a gray base color would only darken it, and an
      // emissive map shows only through a lit emissive color.
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
      meters: this.meters,
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
   *  the geometry view, where each mesh gets a color of its own. Points and lines keep theirs. */
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
      // Unlit and full-bright, a mesh without the map in flat gray. Color maps are decoded from sRGB as they're
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
      // Hues a golden angle apart, so neighbors in the file differ however many meshes there are.
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

  /** An 8 × 8 checker of colored, labeled cells (A1 to H8): stretching shows in the squares, seams where the
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

  /** An animation's character posed halfway through the clip, so a grid of thumbnails isn't a grid of the same
   *  rest pose; the returned function puts it back. */
  private midClip(): (() => void) | null {
    const clip = this.action?.getClip() ?? [...this.clips.values()].find((c) => c.duration >= POSE_SECS)
    if (!this.animation || !this.mixer || !clip) return null
    const mixer = this.mixer
    const fresh = !this.action
    const action = this.action ?? mixer.clipAction(clip).play()
    const time = action.time
    action.time = clip.duration / 2
    mixer.update(0)
    return () => {
      if (fresh) action.stop()
      else action.time = time
      mixer.update(0)
    }
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
    const posed = this.midClip()
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
      posed?.()
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
