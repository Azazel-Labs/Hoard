// The live mesh explorer's 3D world: every model a search finds, laid out on a floor in rows like a palette, streamed
// in as the camera comes near and let go again when it's far away. Loading, light, materials and clean-up are the
// model viewers' own (modelStage, modelFiles, modelScene), so a model looks the same here as in its viewer. Only the
// explorer imports this, and it loads on demand.
import * as THREE from 'three/webgpu'
import { abs, cameraPosition, float, fract, fwidth, length, max, min, mix, positionWorld, smoothstep, vec3 } from 'three/tsl'
import { RoundedBoxGeometry } from 'three/addons/geometries/RoundedBoxGeometry.js'
import { api, packFacings, thumbUrl, type Asset } from './api'
import type { SearchParams } from './types/library'
import { loadModel, sourceFor, textureLinks } from './modelFiles'
import { MAP_SLOTS } from './modelMaterials'
import { disposeObject, mergeStill } from './modelScene'
import { dressModel, standardize, studioRoom, textureManager } from './modelStage'
import { isModelAsset } from './viewers'

/** The width of one cell of the floor, and of the square each model is fitted into on it. */
const CELL = 2
const FIT = 1.5
/** The top of the slab each model stands on. */
const DECK = 0.05
/** Models are fetched this far from the camera, and kept while the budget allows. */
const LOAD_RADIUS = 24
/** Thumbnails stand in for models not loaded yet this far out; past it, only the slabs show. */
const THUMB_RADIUS = 46
/** Names show this far out. */
const LABEL_RADIUS = 18
const MAX_LABELS = 64
/** At most this many models load at once: the server converts each the first time, and the browser decodes each. */
const MAX_IN_FLIGHT = 5
const MAX_THUMBS_IN_FLIGHT = 8
/** The biggest a model stands with true size on, in meters: a landmark at its real 300 m would bury the rest. */
const TRUE_MAX = 12
/** What may stay loaded at once; the models looked at longest ago, then the farthest, go first. */
const BUDGET = { models: 160, bytes: 768 * 2 ** 20, triangles: 15_000_000 }
const MAX_THUMBS = 260
/** The most results laid out; past this, the rest of the search isn't shown. */
export const MAX_CELLS = 10_000
const PAGE = 100

const COLORS = { background: 0x1a1c21, deck: 0x2b2f37, loading: 0x354157, failed: 0x4a2d30, empty: 0x2f333b }

type CellState = 'idle' | 'loading' | 'ready' | 'failed' | 'empty'

interface Cell {
  index: number
  asset: Asset
  x: number
  z: number
  state: CellState
  /** The model, scaled and placed in its cell (see `place`), and its bounds unscaled, as it faces. */
  holder: THREE.Group | null
  natural: THREE.Box3 | null
  /** Where it stands in the world, for picking and framing. */
  box: THREE.Box3
  mixer: THREE.AnimationMixer | null
  action: THREE.AnimationAction | null
  stopMaps: (() => void) | null
  bytes: number
  triangles: number
  /** When it was last in view, for the budget. */
  seen: number
  sprite: THREE.Sprite | null
  thumb: 'none' | 'loading' | 'ready' | 'failed'
  thumbSeen: number
  wire: THREE.LineSegments | null
  /** Worked out by each pass of `schedule`. */
  dist: number
  inView: boolean
}

export interface ExplorerStats {
  /** Models up, loading, laid out, and how many the search found. */
  loaded: number
  inFlight: number
  known: number
  total: number
  /** Roughly what the loaded models hold on the GPU, in bytes, and how many triangles they draw. */
  bytes: number
  triangles: number
  /** Frames drawn in the last second (none while nothing moves). */
  fps: number
  speed: number
}

export interface ExplorerSelection {
  asset: Asset
  /** Counted from the loaded model, or what the server measured. */
  triangles: number | null
  /** Its width, height and depth in meters, once loaded. */
  size: [number, number, number] | null
  loaded: boolean
}

export interface ExplorerOptions {
  /** The highlight's color (the theme's accent). */
  accent?: string
  onStats?: (s: ExplorerStats) => void
  onSelect?: (s: ExplorerSelection | null) => void
  /** Enter (or the card's Open) on the picked model. */
  onOpen?: (a: Asset) => void
  onError?: (e: Error) => void
}

export class ExplorerStage {
  private host: HTMLElement
  private labelHost: HTMLElement
  private opts: ExplorerOptions
  private renderer: THREE.WebGPURenderer | null = null
  private scene = new THREE.Scene()
  private camera = new THREE.PerspectiveCamera(50, 1, 0.05, 400)
  private hemi = new THREE.HemisphereLight(0xdfe8ff, 0x3a3226, 0.35)
  private key = new THREE.DirectionalLight(0xffffff, 1.6)
  private envMap: THREE.Texture | null = null
  private floor: THREE.Mesh | null = null
  private decks: THREE.InstancedMesh | null = null
  private wireGeometry = new THREE.EdgesGeometry(new THREE.BoxGeometry(1.1, 1.1, 1.1))
  private wireMaterial = new THREE.LineBasicNodeMaterial({ color: 0x5b6270 })
  private highlight: THREE.Box3Helper | null = null
  private query: SearchParams
  private cells: Cell[] = []
  private cols = 8
  private total = 0
  private nextPage = 0
  private morePages = true
  private fetching = false
  private search = new AbortController()
  private inFlight = 0
  private thumbsInFlight = 0
  private thumbLoader = new THREE.TextureLoader()
  /** The cells near enough to matter, nearest (and in view) first: refreshed by each pass of `schedule`. */
  private near: Cell[] = []
  private selected: Cell | null = null
  private trueSize = false
  private animate = true
  private orbit = false
  private disposed = false
  private resizeObs: ResizeObserver | null = null
  private timer = new THREE.Timer()
  private redrawUntil = 0
  private frames = 0
  private fps = 0
  private lastFps = 0
  private lastStats = 0
  private lastSchedule = 0
  private dirty = true
  private labels: HTMLDivElement[] = []
  private frustum = new THREE.Frustum()
  private projScreen = new THREE.Matrix4()

  // The camera: where it stands, which way it looks, and how fast it flies.
  private yaw = 0
  private pitch = -0.42
  private speed = 5
  private keys = new Set<string>()
  private pointers = new Map<number, { x: number; y: number; sx: number; sy: number; button: number; moved: boolean; mode: 'look' | 'orbit' | 'pan' }>()
  private pinch: { dist: number; mid: { x: number; y: number } } | null = null
  private pivot = new THREE.Vector3()
  private lastTap = { time: 0, x: 0, y: 0 }
  private glide: { from: THREE.Vector3; to: THREE.Vector3; fromLook: THREE.Vector3; toLook: THREE.Vector3; t: number } | null = null
  private listeners: [EventTarget, string, EventListener, AddEventListenerOptions?][] = []

  constructor(host: HTMLElement, labelHost: HTMLElement, query: SearchParams, opts: ExplorerOptions = {}) {
    this.host = host
    this.labelHost = labelHost
    this.opts = opts
    this.query = { ...query, kind: query.kind === 'animation' ? 'animation' : 'model' }
  }

  async start(): Promise<void> {
    const renderer = new THREE.WebGPURenderer({ antialias: true })
    await renderer.init()
    if (this.disposed) {
      renderer.dispose()
      throw new Error('closed')
    }
    this.renderer = renderer
    // Many models at once: a pixel ratio past 1.5 costs far more than it shows.
    renderer.setPixelRatio(Math.min(devicePixelRatio, 1.5))
    renderer.setSize(this.host.clientWidth || 1, this.host.clientHeight || 1)
    renderer.toneMapping = THREE.NeutralToneMapping
    renderer.domElement.style.touchAction = 'none'
    this.host.appendChild(renderer.domElement)

    // The viewers' light: the studio room for reflections, a sky-and-ground fill, a key light riding with the camera.
    this.envMap = studioRoom(renderer)
    this.scene.environment = this.envMap
    this.scene.background = new THREE.Color(COLORS.background)
    this.scene.fog = new THREE.Fog(COLORS.background, THUMB_RADIUS * 0.7, THUMB_RADIUS * 2.2)
    this.scene.add(this.hemi)
    this.camera.add(this.key, this.key.target)
    this.key.position.set(1.2, 1.6, 0.6)
    this.key.target.position.set(0, 0, -2)
    this.scene.add(this.camera)
    this.floor = makeFloor()
    this.scene.add(this.floor)

    this.bindInput(renderer.domElement)
    this.resizeObs = new ResizeObserver(() => this.resize())
    this.resizeObs.observe(this.host)
    this.resize()

    await this.fetchPage()
    if (this.disposed) return
    // Start at the front of the rows, a little above, looking along them.
    const width = (Math.min(this.cols, Math.max(1, this.cells.length)) - 1) * CELL
    this.camera.position.set(width / 2, 4.2, 6.5)
    this.aim()
    this.timer.reset()
    this.redraw(1000)
    renderer.setAnimationLoop((time) => this.tick(time))
  }

  // ---- The search, a page at a time ----

  private async fetchPage() {
    if (this.fetching || !this.morePages) return
    this.fetching = true
    try {
      const res = await api.search({ ...this.query, limit: PAGE, offset: this.nextPage * PAGE, facets: false }, this.search.signal)
      if (this.disposed) return
      if (this.nextPage === 0) {
        this.total = Math.min(res.total, MAX_CELLS)
        // Roughly square, so the far corner is never far: wide enough to see a row at once, not so wide that a row
        // runs off into the fog.
        this.cols = Math.min(24, Math.max(3, Math.ceil(Math.sqrt(this.total))))
        this.makeDecks(this.total)
      }
      this.nextPage++
      // Only what can be shown in 3D: a search for models may still find a format the server can't convert.
      for (const a of res.items) {
        if (!isModelAsset(a) || this.cells.length >= this.total) continue
        const i = this.cells.length
        const x = (i % this.cols) * CELL
        const z = -Math.floor(i / this.cols) * CELL
        this.cells.push({
          index: i, asset: a, x, z, state: 'idle', holder: null, natural: null,
          box: new THREE.Box3(new THREE.Vector3(x - 0.8, 0, z - 0.8), new THREE.Vector3(x + 0.8, 1.6, z + 0.8)),
          mixer: null, action: null, stopMaps: null, bytes: 0, triangles: 0, seen: 0,
          sprite: null, thumb: a.has_thumb ? 'none' : 'failed', thumbSeen: 0, wire: null, dist: Infinity, inView: false,
        })
        this.setDeck(this.cells[i])
      }
      this.morePages = res.items.length === PAGE && this.nextPage * PAGE < this.total
      // What was found, and not shown in 3D, no longer counts towards the layout.
      if (!this.morePages) this.total = this.cells.length
      if (this.decks) {
        this.decks.count = this.cells.length
        this.decks.computeBoundingSphere()
      }
      this.dirty = true
      this.redraw()
    } catch (e) {
      if (!this.disposed && (e as Error).name !== 'AbortError') this.opts.onError?.(e as Error)
      this.morePages = false
    } finally {
      this.fetching = false
    }
  }

  /** The slabs every model stands on, one draw for all of them. */
  private makeDecks(count: number) {
    const geometry = new RoundedBoxGeometry(CELL * 0.9, DECK, CELL * 0.9, 2, 0.04)
    geometry.translate(0, DECK / 2, 0)
    const material = new THREE.MeshStandardNodeMaterial({ roughness: 0.85, metalness: 0 })
    const decks = new THREE.InstancedMesh(geometry, material, Math.max(1, count))
    decks.count = 0
    // Set for every slot so the color buffer exists before the first draw.
    const c = new THREE.Color(COLORS.deck)
    for (let i = 0; i < Math.max(1, count); i++) decks.setColorAt(i, c)
    this.decks = decks
    this.scene.add(decks)
  }

  private setDeck(cell: Cell) {
    const decks = this.decks
    if (!decks) return
    const m = new THREE.Matrix4().makeTranslation(cell.x, 0, cell.z)
    decks.setMatrixAt(cell.index, m)
    decks.instanceMatrix.needsUpdate = true
    this.colorDeck(cell)
  }

  private colorDeck(cell: Cell) {
    const decks = this.decks
    if (!decks) return
    const hex =
      cell === this.selected
        ? new THREE.Color(COLORS.deck).lerp(this.accent(), 0.4)
        : cell.state === 'loading'
          ? COLORS.loading
          : cell.state === 'failed'
            ? COLORS.failed
            : cell.state === 'empty'
              ? COLORS.empty
              : COLORS.deck
    decks.setColorAt(cell.index, new THREE.Color(hex))
    if (decks.instanceColor) decks.instanceColor.needsUpdate = true
    this.redraw()
  }

  private accent(): THREE.Color {
    try {
      return new THREE.Color(this.opts.accent?.trim() || '#6ea8fe')
    } catch {
      return new THREE.Color('#6ea8fe')
    }
  }

  // ---- Streaming ----

  /** Work out what's near and in view, load the most wanted that aren't up, and let go of what the budget can't
   *  keep. Runs a few times a second while the camera moves, and whenever a load finishes. */
  private schedule(now: number) {
    this.lastSchedule = now
    this.dirty = false
    const cam = this.camera.position
    this.camera.updateMatrixWorld()
    this.projScreen.multiplyMatrices(this.camera.projectionMatrix, this.camera.matrixWorldInverse)
    this.frustum.setFromProjectionMatrix(this.projScreen)
    const sphere = new THREE.Sphere(new THREE.Vector3(), CELL * 0.75)
    const near: Cell[] = []
    for (const cell of this.cells) {
      const dx = cell.x - cam.x
      const dz = cell.z - cam.z
      const dy = 0.6 - cam.y
      cell.dist = Math.sqrt(dx * dx + dy * dy + dz * dz)
      if (cell.dist > THUMB_RADIUS && cell.state !== 'ready') {
        cell.inView = false
        continue
      }
      sphere.center.set(cell.x, 0.6, cell.z)
      cell.inView = this.frustum.intersectsSphere(sphere)
      if (cell.inView) {
        cell.seen = now
        cell.thumbSeen = now
      }
      near.push(cell)
    }
    // Nearest first, and what's in view well before what's behind.
    const rank = (c: Cell) => (c.inView ? c.dist : c.dist * 2.5 + 6)
    near.sort((a, b) => rank(a) - rank(b))
    this.near = near

    // Models: the most wanted first, while loads are free and the budget holds.
    for (const cell of near) {
      if (this.inFlight >= MAX_IN_FLIGHT) break
      if (cell.state !== 'idle' || cell.dist > LOAD_RADIUS) continue
      if (!this.makeRoom(cell, rank)) break
      void this.loadCell(cell)
    }

    // Thumbnails stand in for what isn't loaded; a wire box for what has none.
    for (const cell of near) {
      const wantsStandIn = cell.state !== 'ready' && cell.dist <= THUMB_RADIUS
      if (!wantsStandIn) {
        if (cell.wire) this.dropWire(cell)
        if (cell.sprite) cell.sprite.visible = false
        continue
      }
      if (cell.thumb === 'ready' && cell.sprite) cell.sprite.visible = true
      else if (cell.thumb === 'none' && this.thumbsInFlight < MAX_THUMBS_IN_FLIGHT) this.loadThumb(cell)
      if (cell.thumb === 'failed' && !cell.wire && cell.state !== 'empty') {
        cell.wire = new THREE.LineSegments(this.wireGeometry, this.wireMaterial)
        cell.wire.position.set(cell.x, DECK + 0.55, cell.z)
        this.scene.add(cell.wire)
      }
    }
    this.trimThumbs()
    // Far wire boxes go.
    for (const cell of this.cells) if (cell.wire && (cell.dist > THUMB_RADIUS || cell.state === 'ready')) this.dropWire(cell)

    // More of the search as the camera nears the end of what's laid out.
    const ahead = new THREE.Vector3()
    this.camera.getWorldDirection(ahead)
    const row = Math.max(0, -(cam.z + ahead.z * 12) / CELL)
    if (this.morePages && this.cells.length < (row + 16) * this.cols) void this.fetchPage()
  }

  /** Free the budget for a model: let go of the loaded ones seen longest ago, then the farthest, but only those
   *  wanted less than it, so two models never take turns. */
  private makeRoom(cell: Cell, rank: (c: Cell) => number): boolean {
    const over = (extra: { bytes: number; tris: number }) => {
      const s = this.usage()
      return s.models + 1 > BUDGET.models || s.bytes + extra.bytes > BUDGET.bytes || s.triangles + extra.tris > BUDGET.triangles
    }
    // A guess at what it will take, from what the server measured.
    const tris = Number(cell.asset.meta?.triangles) || 20_000
    const extra = { bytes: tris * 3 * 32 + 4 * 2 ** 20, tris }
    if (!over(extra)) return true
    const mine = rank(cell)
    const victims = this.cells
      .filter((c) => c.state === 'ready' && c !== this.selected && !c.inView && rank(c) > mine)
      .sort((a, b) => a.seen - b.seen || b.dist - a.dist)
    for (const v of victims) {
      this.unload(v)
      if (!over(extra)) return true
    }
    return !over(extra)
  }

  private usage() {
    let models = 0
    let bytes = 0
    let triangles = 0
    for (const c of this.cells) {
      if (c.state !== 'ready' && c.state !== 'loading') continue
      models++
      bytes += c.bytes
      triangles += c.triangles
    }
    return { models, bytes, triangles }
  }

  /** Load one model the way its viewer does: the server's GLB, its textures found by name, its materials made
   *  standard, the textures it doesn't name put on, turned as its pack's models face. */
  private async loadCell(cell: Cell) {
    cell.state = 'loading'
    this.inFlight++
    this.colorDeck(cell)
    try {
      const { named, guesses, full } = await textureLinks(cell.asset)
      if (this.disposed) return
      const redraw = () => this.redraw()
      const manager = textureManager(full, named, guesses, false, { end: redraw })
      const src = sourceFor(full)
      const { object, animations } = await loadModel(src.url, src.ext, manager)
      if (this.disposed) {
        disposeObject(object)
        return
      }
      const made = standardize(object)
      if (!made.meshes.length) {
        // A clip with no mesh of its own: its thumbnail stays.
        disposeObject(object)
        cell.state = 'empty'
        return
      }
      if (!animations.length) mergeStill(object, made.meshes)
      const dressed = dressModel(full, guesses, made.materials, made.textured, {
        fullTextures: false,
        manager,
        onMaps: redraw,
        track: () => redraw,
      })
      const facing = packFacings.get(full.pack_id) ?? full.facing ?? { turn: 0, tilt: 0 }
      const turner = new THREE.Group()
      turner.add(object)
      turner.rotation.set((facing.tilt * Math.PI) / 180, (facing.turn * Math.PI) / 180, 0, 'YXZ')
      const holder = new THREE.Group()
      holder.add(turner)
      holder.updateMatrixWorld(true)
      const natural = new THREE.Box3().setFromObject(holder, true)
      if (natural.isEmpty() || !Number.isFinite(natural.min.x)) natural.set(new THREE.Vector3(-0.5, 0, -0.5), new THREE.Vector3(0.5, 1, 0.5))
      cell.holder = holder
      cell.natural = natural
      cell.stopMaps = dressed.stop
      cell.triangles = Math.round(made.triangles)
      cell.bytes = estimateBytes(holder)
      if (animations.length) {
        // An idle clip first, as the viewer plays: a looping "Death" is a poor first look.
        const clip = [...animations].sort((a, b) => Number(/idle/i.test(b.name)) - Number(/idle/i.test(a.name)))[0]
        cell.mixer = new THREE.AnimationMixer(object)
        cell.action = cell.mixer.clipAction(clip)
        cell.action.play()
        cell.action.paused = !this.animate
      }
      this.place(cell)
      // Build its shaders before it shows, so it doesn't stall the frame it first appears in.
      try {
        await this.renderer?.compileAsync(holder, this.camera, this.scene)
      } catch {
        // Built on first draw instead.
      }
      if (this.disposed) {
        this.release(cell)
        return
      }
      this.scene.add(holder)
      cell.state = 'ready'
      if (cell.sprite) cell.sprite.visible = false
      if (cell === this.selected) this.select(cell)
    } catch (e) {
      cell.state = 'failed'
      console.warn(`explorer: ${cell.asset.name}:`, e)
    } finally {
      this.inFlight--
      if (!this.disposed) {
        this.colorDeck(cell)
        this.dirty = true
        this.redraw()
      }
    }
  }

  /** Scale a loaded model to the cell's footprint (or as big as it is, in meters, with true size on) and stand it on
   *  its slab, centered. */
  private place(cell: Cell) {
    const { holder, natural } = cell
    if (!holder || !natural) return
    const size = natural.getSize(new THREE.Vector3())
    const center = natural.getCenter(new THREE.Vector3())
    const biggest = Math.max(size.x, size.y, size.z, 1e-6)
    const s = this.trueSize ? Math.min(1, TRUE_MAX / biggest) : FIT / biggest
    holder.scale.setScalar(s)
    holder.position.set(cell.x - center.x * s, DECK - natural.min.y * s, cell.z - center.z * s)
    holder.updateMatrixWorld(true)
    cell.box.copy(natural).applyMatrix4(holder.matrixWorld)
  }

  private unload(cell: Cell) {
    if (cell.state !== 'ready') return
    this.release(cell)
    cell.state = 'idle'
    cell.box.set(new THREE.Vector3(cell.x - 0.8, 0, cell.z - 0.8), new THREE.Vector3(cell.x + 0.8, 1.6, cell.z + 0.8))
    if (cell.sprite) cell.sprite.visible = true
    this.colorDeck(cell)
  }

  private release(cell: Cell) {
    cell.stopMaps?.()
    cell.mixer?.stopAllAction()
    if (cell.holder) {
      cell.mixer?.uncacheRoot(cell.holder.children[0]?.children[0] ?? cell.holder)
      cell.holder.removeFromParent()
      disposeObject(cell.holder)
    }
    Object.assign(cell, { holder: null, natural: null, mixer: null, action: null, stopMaps: null, bytes: 0, triangles: 0 })
  }

  private loadThumb(cell: Cell) {
    cell.thumb = 'loading'
    this.thumbsInFlight++
    this.thumbLoader.load(
      thumbUrl(cell.asset),
      (tex) => {
        this.thumbsInFlight--
        if (this.disposed) return tex.dispose()
        tex.colorSpace = THREE.SRGBColorSpace
        const material = new THREE.SpriteNodeMaterial({ map: tex, transparent: true, alphaTest: 0.02, depthWrite: true })
        const sprite = new THREE.Sprite(material)
        sprite.scale.set(FIT, FIT, 1)
        sprite.position.set(cell.x, DECK + FIT / 2 + 0.05, cell.z)
        sprite.visible = cell.state !== 'ready'
        cell.sprite = sprite
        cell.thumb = 'ready'
        this.scene.add(sprite)
        this.dirty = true
        this.redraw()
      },
      undefined,
      () => {
        this.thumbsInFlight--
        cell.thumb = 'failed'
        this.dirty = true
      },
    )
  }

  /** Keep at most `MAX_THUMBS` thumbnails: those seen longest ago go, and come back from the browser's cache. */
  private trimThumbs() {
    const held = this.cells.filter((c) => c.sprite)
    if (held.length <= MAX_THUMBS) return
    held.sort((a, b) => a.thumbSeen - b.thumbSeen || b.dist - a.dist)
    for (const cell of held.slice(0, held.length - MAX_THUMBS)) {
      const sprite = cell.sprite!
      sprite.removeFromParent()
      ;(sprite.material as THREE.SpriteNodeMaterial).map?.dispose()
      sprite.material.dispose()
      cell.sprite = null
      cell.thumb = 'none'
    }
  }

  private dropWire(cell: Cell) {
    cell.wire?.removeFromParent()
    cell.wire = null
  }

  // ---- Switches ----

  /** Every model at its own size, in meters, rather than fitted to its cell (up to TRUE_MAX). */
  setTrueSize(on: boolean) {
    this.trueSize = on
    for (const cell of this.cells) this.place(cell)
    if (this.selected) this.select(this.selected)
    this.redraw()
  }

  setAnimate(on: boolean) {
    this.animate = on
    for (const cell of this.cells) if (cell.action) cell.action.paused = !on
    this.redraw()
  }

  /** Left-drag orbits what's picked (or the floor ahead) rather than looking around. */
  setOrbit(on: boolean) {
    this.orbit = on
  }

  // ---- Picking ----

  private pick(clientX: number, clientY: number): Cell | null {
    const rect = this.host.getBoundingClientRect()
    const ndc = new THREE.Vector2(((clientX - rect.left) / rect.width) * 2 - 1, -((clientY - rect.top) / rect.height) * 2 + 1)
    const ray = new THREE.Raycaster()
    ray.setFromCamera(ndc, this.camera)
    // The nearest cell whose model (or its stand-in's space) the ray passes through.
    let best: Cell | null = null
    let bestDist = Infinity
    const hit = new THREE.Vector3()
    for (const cell of this.cells) {
      if (!ray.ray.intersectBox(cell.box, hit)) continue
      const d = hit.distanceTo(ray.ray.origin)
      if (d < bestDist) [best, bestDist] = [cell, d]
    }
    if (best) return best
    // Or the cell under the floor point it reaches.
    const ground = new THREE.Plane(new THREE.Vector3(0, 1, 0), 0)
    if (!ray.ray.intersectPlane(ground, hit)) return null
    const col = Math.round(hit.x / CELL)
    const row = Math.round(-hit.z / CELL)
    if (col < 0 || col >= this.cols || row < 0) return null
    return this.cells[row * this.cols + col] ?? null
  }

  select(cell: Cell | null) {
    const prev = this.selected
    this.selected = cell
    if (prev && prev !== cell) this.colorDeck(prev)
    this.highlight?.removeFromParent()
    this.highlight?.dispose()
    this.highlight = null
    if (cell) {
      this.colorDeck(cell)
      const box = cell.box.clone().expandByScalar(0.04)
      this.highlight = new THREE.Box3Helper(box, this.accent())
      this.scene.add(this.highlight)
      const meta = Number(cell.asset.meta?.triangles)
      this.opts.onSelect?.({
        asset: cell.asset,
        triangles: cell.state === 'ready' ? cell.triangles : Number.isFinite(meta) && meta > 0 ? meta : null,
        size: cell.natural ? (cell.natural.getSize(new THREE.Vector3()).toArray() as [number, number, number]) : null,
        loaded: cell.state === 'ready',
      })
    } else {
      this.opts.onSelect?.(null)
    }
    this.redraw()
  }

  /** Glide to look at the picked model from a little above. */
  frameSelected() {
    const cell = this.selected
    if (!cell) return
    const sphere = cell.box.getBoundingSphere(new THREE.Sphere())
    const radius = Math.max(sphere.radius, 0.3)
    const vfov = (this.camera.fov * Math.PI) / 180
    const hfov = 2 * Math.atan(Math.tan(vfov / 2) * this.camera.aspect)
    const dist = (radius / Math.sin(Math.min(vfov, hfov) / 2)) * 1.25
    const dir = this.camera.position.clone().sub(sphere.center).normalize()
    if (dir.y < 0.3) dir.setY(0.35).normalize()
    const look = this.camera.position.clone().add(this.forward().multiplyScalar(sphere.center.distanceTo(this.camera.position)))
    this.glide = { from: this.camera.position.clone(), to: sphere.center.clone().addScaledVector(dir, dist), fromLook: look, toLook: sphere.center.clone(), t: 0 }
    this.redraw()
  }

  openSelected() {
    if (this.selected) this.opts.onOpen?.(this.selected.asset)
  }

  // ---- The camera ----

  private aim() {
    this.pitch = THREE.MathUtils.clamp(this.pitch, -Math.PI / 2 + 0.01, Math.PI / 2 - 0.01)
    this.camera.rotation.set(this.pitch, this.yaw, 0, 'YXZ')
  }

  private lookAt(target: THREE.Vector3) {
    const d = target.clone().sub(this.camera.position)
    if (d.lengthSq() < 1e-9) return
    d.normalize()
    this.yaw = Math.atan2(-d.x, -d.z)
    this.pitch = Math.asin(THREE.MathUtils.clamp(d.y, -1, 1))
    this.aim()
  }

  private forward() {
    return new THREE.Vector3(0, 0, -1).applyEuler(new THREE.Euler(this.pitch, this.yaw, 0, 'YXZ'))
  }

  private keepAbove() {
    if (this.camera.position.y < 0.15) this.camera.position.y = 0.15
  }

  /** WASD and the arrows fly where the camera looks, Q and E straight down and up; Shift goes faster. */
  private fly(dt: number): boolean {
    const k = this.keys
    let f = 0
    let r = 0
    let u = 0
    if (k.has('KeyW') || k.has('ArrowUp')) f++
    if (k.has('KeyS') || k.has('ArrowDown')) f--
    if (k.has('KeyD') || k.has('ArrowRight')) r++
    if (k.has('KeyA') || k.has('ArrowLeft')) r--
    if (k.has('KeyE') || k.has('PageUp')) u++
    if (k.has('KeyQ') || k.has('PageDown')) u--
    if (!f && !r && !u) return false
    const fast = k.has('ShiftLeft') || k.has('ShiftRight') ? 3 : 1
    const step = this.speed * fast * dt
    const fwd = this.forward()
    const right = new THREE.Vector3(1, 0, 0).applyAxisAngle(new THREE.Vector3(0, 1, 0), this.yaw)
    this.camera.position.addScaledVector(fwd, f * step).addScaledVector(right, r * step)
    this.camera.position.y += u * step
    this.keepAbove()
    this.glide = null
    return true
  }

  keyDown(code: string): boolean {
    if (/^(Key[WASDQE]|Arrow(Up|Down|Left|Right)|Page(Up|Down)|Shift(Left|Right))$/.test(code)) {
      this.keys.add(code)
      this.redraw()
      return true
    }
    return false
  }

  keyUp(code: string) {
    this.keys.delete(code)
  }

  /** Let go of every key (the window lost focus). */
  releaseKeys() {
    this.keys.clear()
  }

  /** Change the flying speed by a number of steps. */
  nudgeSpeed(steps: number) {
    this.speed = THREE.MathUtils.clamp(this.speed * Math.pow(1.25, steps), 0.5, 80)
    this.emitStats(performance.now(), true)
  }

  /** Where Alt-drag and orbit mode turn about: the picked model when near, else the floor straight ahead. */
  private choosePivot() {
    if (this.selected && this.selected.box.getCenter(new THREE.Vector3()).distanceTo(this.camera.position) < 40) {
      this.selected.box.getCenter(this.pivot)
      return
    }
    const ray = new THREE.Ray(this.camera.position.clone(), this.forward())
    const hit = ray.intersectPlane(new THREE.Plane(new THREE.Vector3(0, 1, 0), 0), new THREE.Vector3())
    this.pivot.copy(hit && hit.distanceTo(this.camera.position) < 60 ? hit : this.camera.position.clone().addScaledVector(this.forward(), 8))
  }

  private orbitBy(dx: number, dy: number) {
    const offset = this.camera.position.clone().sub(this.pivot)
    const s = new THREE.Spherical().setFromVector3(offset)
    s.theta -= dx * 0.005
    s.phi = THREE.MathUtils.clamp(s.phi - dy * 0.005, 0.05, Math.PI / 2 - 0.02)
    this.camera.position.copy(this.pivot).add(new THREE.Vector3().setFromSpherical(s))
    this.keepAbove()
    this.lookAt(this.pivot)
  }

  private panBy(dx: number, dy: number) {
    const k = 0.0025 * Math.max(2, this.speed)
    const right = new THREE.Vector3(1, 0, 0).applyQuaternion(this.camera.quaternion)
    const up = new THREE.Vector3(0, 1, 0).applyQuaternion(this.camera.quaternion)
    this.camera.position.addScaledVector(right, -dx * k).addScaledVector(up, dy * k)
    this.keepAbove()
  }

  private dolly(amount: number) {
    if (this.orbit) {
      const offset = this.camera.position.clone().sub(this.pivot)
      offset.multiplyScalar(Math.pow(1.0015, amount))
      if (offset.length() > 0.3) this.camera.position.copy(this.pivot).add(offset)
    } else {
      this.camera.position.addScaledVector(this.forward(), -amount * 0.004 * Math.max(2, this.speed))
    }
    this.keepAbove()
  }

  private on<K extends keyof HTMLElementEventMap>(el: EventTarget, type: K, fn: (e: HTMLElementEventMap[K]) => void, opts?: AddEventListenerOptions) {
    el.addEventListener(type, fn as EventListener, opts)
    this.listeners.push([el, type, fn as EventListener, opts])
  }

  private bindInput(canvas: HTMLCanvasElement) {
    this.on(canvas, 'contextmenu', (e) => e.preventDefault())
    this.on(canvas, 'pointerdown', (e) => {
      canvas.setPointerCapture(e.pointerId)
      this.glide = null
      const touch = e.pointerType === 'touch'
      const mode = e.button === 1 ? 'pan' : e.altKey || (this.orbit && e.button === 0 && !touch) ? 'orbit' : 'look'
      if (mode === 'orbit') this.choosePivot()
      this.pointers.set(e.pointerId, { x: e.clientX, y: e.clientY, sx: e.clientX, sy: e.clientY, button: e.button, moved: false, mode })
      if (this.pointers.size === 2) {
        const [a, b] = [...this.pointers.values()]
        this.pinch = { dist: Math.hypot(a.x - b.x, a.y - b.y), mid: { x: (a.x + b.x) / 2, y: (a.y + b.y) / 2 } }
      }
      if (e.button === 1) e.preventDefault()
    })
    this.on(canvas, 'pointermove', (e) => {
      const p = this.pointers.get(e.pointerId)
      if (!p) return
      const dx = e.clientX - p.x
      const dy = e.clientY - p.y
      p.x = e.clientX
      p.y = e.clientY
      if (Math.hypot(p.x - p.sx, p.y - p.sy) > 4) p.moved = true
      if (!p.moved) return
      if (this.pointers.size >= 2 && this.pinch) {
        // Two fingers: pinch to move closer, drag both to pan.
        const [a, b] = [...this.pointers.values()]
        const dist = Math.hypot(a.x - b.x, a.y - b.y)
        const mid = { x: (a.x + b.x) / 2, y: (a.y + b.y) / 2 }
        this.dolly(-(dist - this.pinch.dist) * 3)
        this.panBy(mid.x - this.pinch.mid.x, mid.y - this.pinch.mid.y)
        this.pinch = { dist, mid }
      } else if (p.mode === 'pan') {
        this.panBy(dx, dy)
      } else if (p.mode === 'orbit') {
        this.orbitBy(dx, dy)
      } else {
        this.yaw -= dx * 0.0035
        this.pitch -= dy * 0.0035
        this.aim()
      }
      this.redraw(150)
    })
    const up = (e: PointerEvent) => {
      const p = this.pointers.get(e.pointerId)
      this.pointers.delete(e.pointerId)
      if (this.pointers.size < 2) this.pinch = null
      if (!p || p.moved || e.type === 'pointercancel') return
      if (p.button !== 0) return
      const cell = this.pick(e.clientX, e.clientY)
      if (e.pointerType === 'touch') {
        // A second tap on the same spot soon after frames it.
        const now = performance.now()
        const double = now - this.lastTap.time < 320 && Math.hypot(e.clientX - this.lastTap.x, e.clientY - this.lastTap.y) < 24
        this.lastTap = { time: double ? 0 : now, x: e.clientX, y: e.clientY }
        this.select(cell)
        if (double && cell) this.frameSelected()
        return
      }
      this.select(cell)
    }
    this.on(canvas, 'pointerup', up)
    this.on(canvas, 'pointercancel', up)
    this.on(canvas, 'dblclick', (e) => {
      const cell = this.pick(e.clientX, e.clientY)
      if (!cell) return
      this.select(cell)
      this.frameSelected()
    })
    this.on(
      canvas,
      'wheel',
      (e) => {
        e.preventDefault()
        const delta = e.deltaMode === 1 ? e.deltaY * 16 : e.deltaY
        // With the right button held, the wheel sets the flying speed; otherwise it moves along the view.
        const looking = [...this.pointers.values()].some((p) => p.button === 2)
        if (looking) this.nudgeSpeed(-Math.sign(delta))
        else {
          if (this.orbit) this.choosePivot()
          this.dolly(delta)
          this.glide = null
        }
        this.redraw(150)
      },
      { passive: false },
    )
  }

  // ---- Drawing ----

  private redraw(ms = 400) {
    this.redrawUntil = Math.max(this.redrawUntil, performance.now() + ms)
  }

  private tick(time: number) {
    this.timer.update(time)
    const dt = Math.min(this.timer.getDelta(), 0.1)
    const now = performance.now()
    let moved = this.fly(dt)
    if (this.glide) {
      const g = this.glide
      g.t = Math.min(1, g.t + dt / 0.35)
      const k = 1 - Math.pow(1 - g.t, 3)
      this.camera.position.lerpVectors(g.from, g.to, k)
      this.lookAt(new THREE.Vector3().lerpVectors(g.fromLook, g.toLook, k))
      if (g.t >= 1) this.glide = null
      moved = true
    }
    if (moved) this.redraw(150)
    if ((moved || this.dirty) && now - this.lastSchedule > 100) this.schedule(now)
    else if (now - this.lastSchedule > 600) this.schedule(now)

    // Only what's near and in view plays: a clip far off isn't worth a frame.
    let animating = false
    if (this.animate) {
      for (const cell of this.near) {
        if (!cell.action || !cell.inView || cell.dist > LOAD_RADIUS) continue
        cell.mixer!.update(dt)
        animating = true
      }
    }

    if (animating || now < this.redrawUntil) {
      // The floor follows the camera a cell at a time, so its edge is never reached.
      if (this.floor) this.floor.position.set(Math.round(this.camera.position.x / CELL) * CELL, 0, Math.round(this.camera.position.z / CELL) * CELL)
      this.renderer?.render(this.scene, this.camera)
      this.placeLabels()
      this.frames++
    }
    if (now - this.lastFps >= 1000) {
      this.fps = Math.round((this.frames * 1000) / (now - this.lastFps))
      this.frames = 0
      this.lastFps = now
    }
    this.emitStats(now)
  }

  private emitStats(now: number, force = false) {
    if (!force && now - this.lastStats < 250) return
    this.lastStats = now
    const u = this.usage()
    this.opts.onStats?.({
      loaded: this.cells.filter((c) => c.state === 'ready').length,
      inFlight: this.inFlight,
      known: this.cells.length,
      total: Math.max(this.total, this.cells.length),
      bytes: u.bytes,
      triangles: u.triangles,
      fps: this.fps,
      speed: this.speed,
    })
  }

  /** Names over the nearest models in view, in the page rather than the scene so they stay crisp. */
  private placeLabels() {
    const w = this.host.clientWidth
    const h = this.host.clientHeight
    this.camera.updateMatrixWorld()
    this.projScreen.multiplyMatrices(this.camera.projectionMatrix, this.camera.matrixWorldInverse)
    this.frustum.setFromProjectionMatrix(this.projScreen)
    const p = new THREE.Vector3()
    let used = 0
    for (const cell of this.near) {
      if (used >= MAX_LABELS) break
      const d = cell.box.getCenter(p).distanceTo(this.camera.position)
      if (d > LABEL_RADIUS) continue
      p.set(cell.x, Math.max(cell.box.max.y, 0.3) + 0.12, cell.z)
      if (!this.frustum.containsPoint(p)) continue
      p.project(this.camera)
      if (p.z > 1) continue
      let el = this.labels[used]
      if (!el) {
        el = document.createElement('div')
        el.className = 'label'
        this.labelHost.appendChild(el)
        this.labels.push(el)
      }
      if (el.textContent !== cell.asset.name) el.textContent = cell.asset.name
      const x = (p.x * 0.5 + 0.5) * w
      const y = (-p.y * 0.5 + 0.5) * h
      el.style.transform = `translate(${x.toFixed(1)}px, ${y.toFixed(1)}px) translate(-50%, -100%)`
      el.style.opacity = String(Math.min(1, (LABEL_RADIUS - d) / 5))
      el.classList.toggle('on', cell === this.selected)
      el.hidden = false
      used++
    }
    for (let i = used; i < this.labels.length; i++) this.labels[i].hidden = true
  }

  private resize() {
    const el = this.host
    if (!this.renderer || !el.clientWidth || !el.clientHeight) return
    this.renderer.setSize(el.clientWidth, el.clientHeight)
    this.camera.aspect = el.clientWidth / el.clientHeight
    this.camera.updateProjectionMatrix()
    this.dirty = true
    this.redraw()
  }

  dispose() {
    this.disposed = true
    this.search.abort()
    for (const [el, type, fn, opts] of this.listeners) el.removeEventListener(type, fn, opts)
    this.resizeObs?.disconnect()
    for (const cell of this.cells) {
      this.release(cell)
      if (cell.sprite) {
        ;(cell.sprite.material as THREE.SpriteNodeMaterial).map?.dispose()
        cell.sprite.material.dispose()
      }
    }
    this.cells = []
    for (const l of this.labels) l.remove()
    this.highlight?.dispose()
    this.wireGeometry.dispose()
    this.wireMaterial.dispose()
    if (this.decks) {
      this.decks.geometry.dispose()
      ;(this.decks.material as THREE.Material).dispose()
      this.decks.dispose()
    }
    if (this.floor) {
      this.floor.geometry.dispose()
      ;(this.floor.material as THREE.Material).dispose()
    }
    this.envMap?.dispose()
    if (this.renderer) {
      this.renderer.setAnimationLoop(null)
      this.renderer.domElement.remove()
      this.renderer.dispose()
      this.renderer = null
    }
  }
}

/** A floor that seems to go on for ever: a large plane drawn with grid lines at each cell's edge and finer ones
 *  between, crisp at any distance (each line as wide as a pixel), fading out with distance into the fog. It moves
 *  with the camera, a whole cell at a time, so the lines stay put. */
function makeFloor(): THREE.Mesh {
  const geometry = new THREE.PlaneGeometry(THUMB_RADIUS * 5, THUMB_RADIUS * 5)
  geometry.rotateX(-Math.PI / 2)
  const material = new THREE.MeshBasicNodeMaterial({ transparent: true, depthWrite: false })
  material.fog = false
  const p = positionWorld.xz
  const lines = (size: number, offset: number) => {
    const c = p.add(offset).div(size)
    const g = abs(fract(c.sub(0.5)).sub(0.5)).div(fwidth(c))
    return float(1).sub(min(min(g.x, g.y), float(1)))
  }
  const major = lines(CELL, CELL / 2)
  const minor = lines(CELL / 4, 0)
  const dist = length(p.sub(cameraPosition.xz))
  const fade = float(1).sub(smoothstep(float(THUMB_RADIUS * 0.3), float(THUMB_RADIUS * 1.9), dist))
  const near = float(1).sub(smoothstep(float(4), float(22), dist))
  // The rows start at the floor's red and blue axes, as an editor's do.
  const axisX = float(1).sub(min(abs(p.y.sub(CELL / 2)).div(fwidth(p.y)).mul(0.6), float(1)))
  const axisZ = float(1).sub(min(abs(p.x.add(CELL / 2)).div(fwidth(p.x)).mul(0.6), float(1)))
  const base = vec3(0.42, 0.45, 0.52)
  const tinted = mix(mix(base, vec3(0.85, 0.3, 0.3), axisX), vec3(0.3, 0.45, 0.9), axisZ)
  material.colorNode = tinted
  material.opacityNode = max(max(major.mul(0.5), minor.mul(0.16).mul(near)), max(axisX, axisZ).mul(0.85)).mul(fade)
  const floor = new THREE.Mesh(geometry, material)
  floor.renderOrder = -1
  floor.frustumCulled = false
  return floor
}

/** Roughly what a model holds on the GPU: its vertex and index buffers, and each texture with its mipmaps. */
function estimateBytes(root: THREE.Object3D): number {
  let bytes = 0
  const geometries = new Set<THREE.BufferGeometry>()
  const textures = new Set<THREE.Texture>()
  root.traverse((o) => {
    const mesh = o as THREE.Mesh
    if (mesh.geometry && !geometries.has(mesh.geometry)) {
      geometries.add(mesh.geometry)
      for (const a of Object.values(mesh.geometry.attributes)) bytes += (a as THREE.BufferAttribute).array?.byteLength ?? 0
      bytes += mesh.geometry.index?.array.byteLength ?? 0
    }
    const list = mesh.material ? (Array.isArray(mesh.material) ? mesh.material : [mesh.material]) : []
    for (const m of list) {
      for (const slot of MAP_SLOTS) {
        const t = (m as unknown as Record<string, THREE.Texture | null>)[slot]
        if (t) textures.add(t)
      }
    }
  })
  for (const t of textures) {
    const img = t.image as { width?: number; height?: number } | undefined
    bytes += (img?.width ?? 512) * (img?.height ?? 512) * 4 * 1.33
  }
  return bytes
}
