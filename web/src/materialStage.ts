// The 3D scene behind both material previews (the side panel's and the full-screen one): a material made from texture
// maps on a sphere, cube or plane, lit by the same studio room and key light as the model viewers. Only the previews
// import this, and they load on demand, so three stays out of the main bundle.
import * as THREE from 'three/webgpu'
import { OrbitControls } from 'three/addons/controls/OrbitControls.js'
import { RoomEnvironment } from 'three/addons/environments/RoomEnvironment.js'
import type { MapRole, MaterialInfo } from './api'
import { loadMaps, SLOT_PROP, type MapSet, type Slot } from './materialMaps'
import type { Background } from './modelStage'

export type Shape = 'sphere' | 'cube' | 'plane'
export const SHAPES: Shape[] = ['sphere', 'cube', 'plane']

export class MaterialStage {
  readonly host: HTMLElement
  private renderer: THREE.WebGPURenderer | null = null
  private scene = new THREE.Scene()
  private camera = new THREE.PerspectiveCamera(35, 1, 0.01, 100)
  private controls: OrbitControls | null = null
  private hemi = new THREE.HemisphereLight(0xdfe8ff, 0x3a3226, 1)
  private key = new THREE.DirectionalLight(0xffffff, 2)
  private envMap: THREE.Texture | null = null
  private material = new THREE.MeshStandardMaterial({ color: 0xffffff, roughness: 1, metalness: 0 })
  private mesh: THREE.Mesh | null = null
  private shape: Shape = 'sphere'
  private set: MapSet | null = null
  private off = new Set<MapRole>()
  private repeat = 1
  private displacement = 0.03
  private stop: (() => void) | null = null
  private resizeObs: ResizeObserver | null = null
  private timer = new THREE.Timer()
  private glide: { from: THREE.Vector3; to: THREE.Vector3; fromT: THREE.Vector3; toT: THREE.Vector3; t: number } | null = null
  private paused = false
  private disposed = false
  private envOn = true
  /** Someone has orbited or zoomed since the last reset: a resize leaves the view alone. */
  private moved = false
  /** A map arrived (or a better one replaced a thumbnail): which roles are on the material now. */
  onMaps: ((roles: MapRole[]) => void) | null = null

  constructor(host: HTMLElement) {
    this.host = host
  }

  /** Set up the renderer and start loading the maps. Resolves once the first frame can draw; maps keep arriving. */
  async load(info: MaterialInfo, packId: string, max: number): Promise<void> {
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
    renderer.toneMapping = THREE.NeutralToneMapping
    renderer.toneMappingExposure = 1
    el.appendChild(renderer.domElement)

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
    controls.minDistance = 1.2
    controls.maxDistance = 20
    controls.addEventListener('start', () => {
      this.glide = null
      this.moved = true
    })
    this.controls = controls

    this.setShape(this.shape)
    this.camera.position.copy(this.home())
    controls.target.set(0, 0, 0)
    controls.update()

    this.stop = loadMaps(info, packId, { max, flipY: true }, (set, replaced) => {
      this.set = set
      this.applyMaps()
      for (const t of replaced) t.dispose()
      this.onMaps?.(this.activeRoles())
    })

    this.resizeObs = new ResizeObserver(() => this.resize())
    this.resizeObs.observe(el)
    this.resize()
    this.start()
  }

  /** Roles whose maps are on the material now, switched off or not. */
  activeRoles(): MapRole[] {
    if (!this.set) return []
    const roles = new Set<MapRole>()
    for (const slot of Object.keys(this.set.textures) as Slot[]) {
      const r = this.set.roles[slot]
      if (r) roles.add(r)
    }
    return [...roles]
  }

  private applyMaps() {
    const m = this.material as THREE.MeshStandardMaterial & Record<string, unknown>
    const set = this.set
    let changed = false
    for (const slot of Object.keys(SLOT_PROP) as Slot[]) {
      const prop = SLOT_PROP[slot]
      const role = set?.roles[slot]
      let tex = (role && !this.off.has(role) && set?.textures[slot]) || null
      // Displacement moves the shape's vertices, finer detail than that is lost: with no normal map to carry it,
      // height is a bump map besides.
      if (slot === 'bump' && !tex && set && !set.textures.normal && set.textures.height && !this.off.has('height')) {
        tex = set.textures.height
      }
      if (tex) {
        tex.repeat.set(this.repeat, this.repeat)
        tex.updateMatrix()
      }
      if ((m[prop] ?? null) !== tex) {
        if (!m[prop] !== !tex) changed = true
        m[prop] = tex
      }
    }
    m.normalScale.set(1, set?.normalDx ? -1 : 1)
    m.displacementScale = this.displacement
    m.displacementBias = -this.displacement / 2
    // With no roughness map a plain surface reads best a little rough; with no metal map, not metal.
    m.roughness = m.roughnessMap ? 1 : 0.6
    m.metalness = m.metalnessMap ? 1 : 0
    m.emissive.set(m.emissiveMap ? 0xffffff : 0x000000)
    m.alphaTest = m.alphaMap ? 0.5 : 0
    m.side = m.alphaMap || this.shape === 'plane' ? THREE.DoubleSide : THREE.FrontSide
    if (changed) m.needsUpdate = true
    if (this.paused) this.renderOnce()
  }

  setShape(shape: Shape) {
    this.shape = shape
    if (this.mesh) {
      this.mesh.removeFromParent()
      this.mesh.geometry.dispose()
    }
    // Enough vertices for a height map to move; a cube's faces each carry the whole map, like a sample swatch.
    const geom =
      shape === 'sphere'
        ? new THREE.SphereGeometry(1, 256, 128)
        : shape === 'cube'
          ? new THREE.BoxGeometry(1.5, 1.5, 1.5, 128, 128, 128)
          : new THREE.PlaneGeometry(2.4, 2.4, 256, 256)
    this.mesh = new THREE.Mesh(geom, this.material)
    // A sphere's UVs run twice as far around as top to bottom: repeat across twice, so texels stay square.
    if (shape === 'sphere') {
      const uv = geom.attributes.uv
      for (let i = 0; i < uv.count; i++) uv.setX(i, uv.getX(i) * 2)
      uv.needsUpdate = true
    }
    this.scene.add(this.mesh)
    this.material.needsUpdate = true
    this.applyMaps()
  }

  /** Times the map repeats across the shape. */
  setRepeat(n: number) {
    this.repeat = n
    this.applyMaps()
  }

  /** How far the height map pushes the surface out, in units of the sphere's radius. */
  setDisplacement(v: number) {
    this.displacement = v
    this.applyMaps()
  }

  /** Switch one kind of map off (or back on), to see what it does. */
  setRole(role: MapRole, on: boolean) {
    if (on) this.off.delete(role)
    else this.off.add(role)
    this.applyMaps()
  }

  setEnvironment(on: boolean) {
    this.envOn = on
    this.scene.environment = on ? this.envMap : null
    if (!on && this.scene.background === this.envMap) this.scene.background = null
    this.applyLights()
    this.material.needsUpdate = true
  }

  private applyLights() {
    this.hemi.intensity = this.envOn ? 0.35 : 1.6
    this.key.intensity = this.envOn ? 1.6 : 2.4
  }

  setBackground(bg: Background) {
    this.scene.background = bg === 'env' && this.envOn ? this.envMap : null
    this.scene.backgroundBlurriness = 0.5
    this.scene.backgroundIntensity = 0.8
  }

  setAutoRotate(on: boolean) {
    if (this.controls) this.controls.autoRotate = on
  }

  /** Back to the starting view. */
  reset() {
    if (!this.controls) return
    this.moved = false
    this.glide = { from: this.camera.position.clone(), to: this.home(), fromT: this.controls.target.clone(), toT: new THREE.Vector3(), t: 0 }
    if (this.paused) this.renderOnce()
  }

  /** Stop drawing while hidden (the full-screen viewer covers the side panel's). */
  setPaused(on: boolean) {
    this.paused = on
    if (on) this.renderer?.setAnimationLoop(null)
    else this.start()
  }

  private start() {
    if (!this.renderer || this.paused || this.disposed) return
    this.timer.reset()
    this.renderer.setAnimationLoop((time) => this.tick(time))
  }

  private tick(time: number) {
    this.timer.update(time)
    const dt = Math.min(this.timer.getDelta(), 0.1)
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

  /** The starting view, moved back on a tall, narrow screen so the shape still fits across. */
  private home(): THREE.Vector3 {
    const halfV = THREE.MathUtils.degToRad(this.camera.fov / 2)
    const halfH = Math.atan(Math.tan(halfV) * this.camera.aspect)
    const across = 1.12 / Math.sin(Math.min(halfV, halfH))
    return HOME.clone().setLength(Math.max(HOME.length(), across))
  }

  private resize() {
    const el = this.host
    if (!this.renderer || !el.clientWidth || !el.clientHeight) return
    this.renderer.setSize(el.clientWidth, el.clientHeight)
    this.camera.aspect = el.clientWidth / el.clientHeight
    this.camera.updateProjectionMatrix()
    if (!this.moved && !this.glide) this.camera.position.copy(this.home())
    if (this.paused) this.renderOnce()
  }

  dispose() {
    this.disposed = true
    this.stop?.()
    this.resizeObs?.disconnect()
    this.controls?.dispose()
    this.mesh?.geometry.dispose()
    for (const t of new Set(Object.values(this.set?.textures ?? {}))) t.dispose()
    this.material.dispose()
    this.envMap?.dispose()
    if (this.renderer) {
      this.renderer.setAnimationLoop(null)
      this.renderer.domElement.remove()
      this.renderer.dispose()
      this.renderer = null
    }
  }
}

/** The starting view: a little above, close enough that the material fills the frame. */
const HOME = new THREE.Vector3(0, 0.9, 3.6)
