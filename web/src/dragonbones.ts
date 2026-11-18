// DragonBones skeletons (`…_ske.json`), played by Hoard's own runtime on a 2D canvas: bones (with IK), images,
// meshes (weighted or not, with their deform), nested armatures, and the animations' bone, slot, draw-order and deform
// timelines with their easing. The texture atlas is the `…_tex.json` and image beside the skeleton. The server can't
// draw these, so the browser makes their thumbnails too.
import { api, fileUrl, joinPath, packFileUrl, type Asset, type ThumbUpload } from './api'
import { coverage } from './browserThumbs'

// The skeleton's JSON is read loosely: every field may be missing, and older exports name some differently.
type J = any

/** An affine matrix as DragonBones writes it: x' = a·x + c·y + tx, y' = b·x + d·y + ty. */
type Mat = [a: number, b: number, c: number, d: number, tx: number, ty: number]
/** A bone's pose: position, rotation and skew in radians, scale. */
type Pose = { x: number; y: number; rot: number; skew: number; sx: number; sy: number }

const IDENTITY: Mat = [1, 0, 0, 1, 0, 0]
const DEG = Math.PI / 180

/** `m`, then `p`. */
function mul(p: Mat, m: Mat): Mat {
  return [
    m[0] * p[0] + m[1] * p[2],
    m[0] * p[1] + m[1] * p[3],
    m[2] * p[0] + m[3] * p[2],
    m[2] * p[1] + m[3] * p[3],
    p[0] * m[4] + p[2] * m[5] + p[4],
    p[1] * m[4] + p[3] * m[5] + p[5],
  ]
}

function invert([a, b, c, d, tx, ty]: Mat): Mat {
  const det = a * d - b * c || 1e-9
  const [ia, ib, ic, id] = [d / det, -b / det, -c / det, a / det]
  return [ia, ib, ic, id, -(ia * tx + ic * ty), -(ib * tx + id * ty)]
}

function toMatrix(p: Pose): Mat {
  return [
    Math.cos(p.rot) * p.sx,
    Math.sin(p.rot) * p.sx,
    -Math.sin(p.skew + p.rot) * p.sy,
    Math.cos(p.skew + p.rot) * p.sy,
    p.x,
    p.y,
  ]
}

/** The pose a matrix holds, the way DragonBones takes one apart. */
function fromMatrix([a, b, c, d, tx, ty]: Mat): Pose {
  const q = Math.PI / 4
  let skewX = Math.atan(-c / d) || 0
  let rot = Math.atan(b / a) || 0
  let sx = rot > -q && rot < q ? a / Math.cos(rot) : b / Math.sin(rot)
  let sy = skewX > -q && skewX < q ? d / Math.cos(skewX) : -c / Math.sin(skewX)
  if (sx < 0) [sx, rot] = [-sx, rot - Math.PI]
  if (sy < 0) [sy, skewX] = [-sy, skewX - Math.PI]
  return { x: tx, y: ty, rot, skew: skewX - rot, sx, sy }
}

/** An angle in -π..π. */
function wrap(r: number): number {
  r = (r + Math.PI) % (Math.PI * 2)
  return r + (r > 0 ? -Math.PI : Math.PI)
}

function poseOf(t: J): Pose {
  const skX = num(t?.skX, 0) * DEG
  const skY = num(t?.skY, 0) * DEG
  return { x: num(t?.x, 0), y: num(t?.y, 0), rot: skY, skew: skX - skY, sx: num(t?.scX, 1), sy: num(t?.scY, 1) }
}

function num(v: unknown, fallback: number): number {
  return typeof v === 'number' && Number.isFinite(v) ? v : fallback
}

// ---- Timelines ----

type Ease = { kind: 'linear' } | { kind: 'quad'; type: 1 | 2 | 3; amount: number } | { kind: 'curve'; c: number[] }
type Frame<T> = { start: number; dur: number; ease: Ease | null; v: T }

/** How a frame tweens into the next: `tweenEasing` absent is a step, 0 is linear, otherwise a quad ease (or a curve). */
function easeOf(f: J): Ease | null {
  if (Array.isArray(f.curve) && f.curve.length >= 4) return { kind: 'curve', c: f.curve.map((v: unknown) => num(v, 0)) }
  const e = f.tweenEasing
  if (typeof e !== 'number' || e === 100) return null
  if (e === 0) return { kind: 'linear' }
  if (e < 0) return { kind: 'quad', type: 1, amount: -e }
  if (e <= 1) return { kind: 'quad', type: 2, amount: e }
  return { kind: 'quad', type: 3, amount: e - 1 }
}

function cubic(a: number, b: number, c: number, d: number, t: number): number {
  const u = 1 - t
  return u * u * u * a + 3 * u * u * t * b + 3 * u * t * t * c + t * t * t * d
}

/** Progress `p` through an ease. A curve is bezier segments from (0,0) to (1,1): [c1x, c1y, c2x, c2y, (x, y, …)]. */
function eased(e: Ease, p: number): number {
  if (e.kind === 'linear') return p
  if (e.kind === 'quad') {
    const v = e.type === 1 ? p * p : e.type === 2 ? 1 - (1 - p) ** 2 : 0.5 * (1 - Math.cos(p * Math.PI))
    return (v - p) * e.amount + p
  }
  const c = e.c
  let at = -2
  while ((at + 6 < c.length ? c[at + 6] : 1) < p) at += 6
  const inside = at >= 0 && at + 6 < c.length
  const [x1, y1] = inside ? [c[at], c[at + 1]] : [0, 0]
  const [x4, y4] = inside ? [c[at + 6], c[at + 7]] : [1, 1]
  const [x2, y2, x3, y3] = [c[at + 2], c[at + 3], c[at + 4], c[at + 5]]
  let [lo, hi] = [0, 1]
  for (let i = 0; i < 20; i++) {
    const m = (lo + hi) / 2
    if (cubic(x1, x2, x3, x4, m) < p) lo = m
    else hi = m
  }
  return cubic(y1, y2, y3, y4, (lo + hi) / 2)
}

/** Frames laid end to end from frame 0. */
function frames<T>(raw: J, value: (f: J) => T): Frame<T>[] {
  if (!Array.isArray(raw)) return []
  let start = 0
  return raw.map((f: J) => {
    const dur = Math.max(0, num(f.duration, 1))
    const out = { start, dur, ease: easeOf(f), v: value(f) }
    start += dur
    return out
  })
}

/** The timeline's value at frame `t`, tweened by `lerp`. */
function sample<T>(fs: Frame<T>[], t: number, lerp: (a: T, b: T, p: number) => T): T | undefined {
  if (!fs.length) return undefined
  let i = 0
  while (i + 1 < fs.length && fs[i + 1].start <= t) i++
  const f = fs[i]
  const next = fs[i + 1]
  if (!next || !f.ease || f.dur <= 0) return f.v
  return lerp(f.v, next.v, eased(f.ease, Math.min(1, Math.max(0, (t - f.start) / f.dur))))
}

const lerp = (a: number, b: number, p: number) => a + (b - a) * p
const lerp2 = (a: number[], b: number[], p: number) => a.map((v, i) => lerp(v, b[i], p))
const step = <T>(a: T) => a

/** Rotation keyframes go the short way round, unless `clockwise` asks for whole turns. */
function rotateFrames(raw: J[], rot: (f: J) => number, skew: (f: J) => number): Frame<number[]>[] {
  let prev = 0
  let clockwise = 0
  let first = true
  return frames(raw, (f) => {
    let r = rot(f)
    if (!first) {
      if (clockwise === 0) r = prev + wrap(r - prev)
      else {
        if (clockwise > 0 ? r >= prev : r <= prev) clockwise += clockwise > 0 ? -1 : 1
        r = r + Math.PI * 2 * clockwise
      }
    }
    clockwise = num(f.clockwise, 0)
    prev = r
    first = false
    return [r, skew(f)]
  })
}

// ---- The skeleton ----

type Region = { src: CanvasImageSource; sx: number; sy: number; w: number; h: number; scale: number; frame: { x: number; y: number; w: number; h: number } | null }

type Weighted = { bones: number[]; weights: number[]; local: number[] }
type Mesh = {
  vertices: number[]
  uvs: number[]
  triangles: number[]
  /** Per vertex, for a mesh bound to bones: which bones, how much, and where the vertex sits in each. */
  weighted: Weighted[] | null
  /** Turns a deform offset (in the mesh's space) into each bone's space: the slot pose's linear part, per bone. */
  slotPose: Mat
  bonePoses: Map<number, Mat>
}

type Display = {
  type: 'image' | 'mesh' | 'armature' | 'none'
  name: string
  texture: string
  matrix: Mat
  pivotX: number
  pivotY: number
  mesh: Mesh | null
  /** For a nested armature: which armature, playing which animation. */
  animation: string
}

type BoneDef = { name: string; parent: number; length: number; origin: Pose; inheritScale: boolean; inheritRotation: boolean; inheritTranslation: boolean }
type SlotDef = { name: string; bone: number; displayIndex: number; alpha: number; blend: GlobalCompositeOperation; displays: Display[] }
type IkDef = { bone: number; target: number; bendPositive: boolean; chain: number; weight: number }

type BoneTimeline = { t: Frame<number[]>[]; r: Frame<number[]>[]; s: Frame<number[]>[] }
type SlotTimeline = { display: Frame<number>[]; alpha: Frame<number>[] }
type DeformTimeline = { slot: number; display: string; frames: Frame<number[]>[] }
type Anim = {
  name: string
  duration: number
  bones: Map<number, BoneTimeline>
  slots: Map<number, SlotTimeline>
  zOrder: Frame<number[]>[]
  deform: DeformTimeline[]
}

type ArmatureDef = {
  name: string
  fps: number
  bones: BoneDef[]
  slots: SlotDef[]
  ik: IkDef[]
  animations: Anim[]
  /** The animation it starts with. */
  first: string
  aabb: [number, number, number, number] | null
}

export type Skeleton = {
  armatures: ArmatureDef[]
  regions: Map<string, Region>
}

const BLENDS: Record<string, GlobalCompositeOperation> = {
  add: 'lighter',
  multiply: 'multiply',
  screen: 'screen',
  overlay: 'overlay',
  darken: 'darken',
  lighten: 'lighten',
  difference: 'difference',
  hardlight: 'hard-light',
}

function parseMesh(d: J, bones: number): Mesh {
  const list = (v: J) => (Array.isArray(v) ? v.map((x: unknown) => num(x, 0)) : [])
  const vertices = list(d.vertices)
  const slotPose: Mat = Array.isArray(d.slotPose) && d.slotPose.length >= 6 ? (list(d.slotPose).slice(0, 6) as Mat) : IDENTITY
  const bonePoses = new Map<number, Mat>()
  const bp = list(d.bonePose)
  for (let i = 0; i + 6 < bp.length; i += 7) bonePoses.set(bp[i], bp.slice(i + 1, i + 7) as Mat)
  let weighted: Weighted[] | null = null
  const w = list(d.weights)
  if (w.length) {
    weighted = []
    const inverse = new Map([...bonePoses].map(([b, m]) => [b, invert(m)]))
    let at = 0
    for (let v = 0; v * 2 < vertices.length && at < w.length; v++) {
      const n = w[at++]
      const [x, y] = point(slotPose, vertices[v * 2], vertices[v * 2 + 1])
      const out: Weighted = { bones: [], weights: [], local: [] }
      for (let k = 0; k < n; k++) {
        const b = w[at++]
        const weight = w[at++]
        if (b < 0 || b >= bones) continue
        const [lx, ly] = point(inverse.get(b) ?? IDENTITY, x, y)
        out.bones.push(b)
        out.weights.push(weight)
        out.local.push(lx, ly)
      }
      weighted.push(out)
    }
  }
  return { vertices, uvs: list(d.uvs), triangles: list(d.triangles), weighted, slotPose, bonePoses }
}

function point(m: Mat, x: number, y: number): [number, number] {
  return [m[0] * x + m[2] * y + m[4], m[1] * x + m[3] * y + m[5]]
}

function parseDisplay(d: J, bones: number): Display {
  const type = d?.type ?? 'image'
  const name = String(d?.name ?? '')
  const base: Display = {
    type: 'none',
    name,
    texture: String(d?.path ?? name),
    matrix: d?.transform ? toMatrix(poseOf(d.transform)) : IDENTITY,
    pivotX: num(d?.pivot?.x, 0.5),
    pivotY: num(d?.pivot?.y, 0.5),
    mesh: null,
    animation: '',
  }
  if (type === 'image') return { ...base, type: 'image' }
  if (type === 'mesh' && Array.isArray(d.vertices)) return { ...base, type: 'mesh', mesh: parseMesh(d, bones) }
  if (type === 'armature') {
    const action = (Array.isArray(d.actions) ? d.actions : []).find((a: J) => a?.gotoAndPlay || a?.name)
    return { ...base, type: 'armature', animation: String(action?.gotoAndPlay ?? action?.name ?? '') }
  }
  return base
}

function parseAnimation(raw: J, boneIndex: Map<string, number>, slotIndex: Map<string, number>): Anim {
  const bones = new Map<number, BoneTimeline>()
  for (const b of Array.isArray(raw.bone) ? raw.bone : []) {
    const i = boneIndex.get(b?.name)
    if (i === undefined) continue
    let tl: BoneTimeline
    if (Array.isArray(b.frame) && !b.translateFrame && !b.rotateFrame && !b.scaleFrame) {
      // Before 5.5, one timeline of whole transforms.
      const tf = (f: J) => f.transform ?? {}
      tl = {
        t: frames(b.frame, (f) => [num(tf(f).x, 0), num(tf(f).y, 0)]),
        r: rotateFrames(b.frame, (f) => num(tf(f).skY, 0) * DEG, (f) => (num(tf(f).skX, 0) - num(tf(f).skY, 0)) * DEG),
        s: frames(b.frame, (f) => [num(tf(f).scX, 1), num(tf(f).scY, 1)]),
      }
    } else {
      tl = {
        t: frames(b.translateFrame, (f) => [num(f.x, 0), num(f.y, 0)]),
        r: rotateFrames(b.rotateFrame ?? [], (f) => num(f.rotate, 0) * DEG, (f) => num(f.skew, 0) * DEG),
        s: frames(b.scaleFrame, (f) => [num(f.x, 1), num(f.y, 1)]),
      }
    }
    bones.set(i, tl)
  }
  const slots = new Map<number, SlotTimeline>()
  for (const s of Array.isArray(raw.slot) ? raw.slot : []) {
    const i = slotIndex.get(s?.name)
    if (i === undefined) continue
    const alpha = (c: J) => num(c?.aM, 100) / 100
    const old = Array.isArray(s.frame) && !s.displayFrame && !s.colorFrame
    slots.set(i, {
      display: old ? frames(s.frame, (f) => num(f.displayIndex, 0)) : frames(s.displayFrame, (f) => num(f.value, 0)),
      alpha: old ? frames(s.frame, (f) => alpha(f.color)) : frames(s.colorFrame, (f) => alpha(f.value)),
    })
  }
  const deform: DeformTimeline[] = []
  for (const f of [...(Array.isArray(raw.ffd) ? raw.ffd : []), ...(Array.isArray(raw.deform) ? raw.deform : [])]) {
    const slot = slotIndex.get(f?.slot)
    if (slot === undefined) continue
    deform.push({
      slot,
      display: String(f.name ?? ''),
      frames: frames(f.frame, (k) => {
        const out: number[] = []
        const offset = Math.max(0, Math.floor(num(k.offset, 0)))
        const v = Array.isArray(k.vertices) ? k.vertices : []
        for (let j = 0; j < v.length; j++) out[offset + j] = num(v[j], 0)
        for (let j = 0; j < offset; j++) out[j] = 0
        return out
      }),
    })
  }
  return {
    name: String(raw.name ?? ''),
    duration: Math.max(0, num(raw.duration, 0)),
    bones,
    slots,
    zOrder: frames(raw.zOrder?.frame, (f) => (Array.isArray(f.zOrder) ? f.zOrder.map((v: unknown) => num(v, 0)) : [])),
    deform,
  }
}

function parseArmature(raw: J, fps: number): ArmatureDef {
  const rawBones: J[] = Array.isArray(raw.bone) ? raw.bone : []
  const boneIndex = new Map<string, number>(rawBones.map((b, i) => [String(b?.name), i]))
  const bones: BoneDef[] = rawBones.map((b) => ({
    name: String(b?.name ?? ''),
    parent: b?.parent != null ? (boneIndex.get(String(b.parent)) ?? -1) : -1,
    length: num(b?.length, 0),
    origin: poseOf(b?.transform),
    inheritScale: b?.inheritScale !== false,
    inheritRotation: b?.inheritRotation !== false,
    inheritTranslation: b?.inheritTranslation !== false,
  }))
  const skins: J[] = Array.isArray(raw.skin) ? raw.skin : []
  const skin = skins.find((s) => !s?.name || s.name === 'default') ?? skins[0]
  const skinSlots = new Map<string, J[]>()
  for (const s of skin?.slot ?? []) skinSlots.set(String(s?.name), Array.isArray(s?.display) ? s.display : [])
  const rawSlots: J[] = Array.isArray(raw.slot) ? raw.slot : []
  const slots: SlotDef[] = rawSlots.map((s) => ({
    name: String(s?.name ?? ''),
    bone: boneIndex.get(String(s?.parent)) ?? 0,
    displayIndex: num(s?.displayIndex, 0),
    alpha: num(s?.color?.aM, 100) / 100,
    blend: BLENDS[String(s?.blendMode)] ?? 'source-over',
    displays: (skinSlots.get(String(s?.name)) ?? []).map((d) => parseDisplay(d, bones.length)),
  }))
  const slotIndex = new Map<string, number>(slots.map((s, i) => [s.name, i]))
  const ik: IkDef[] = (Array.isArray(raw.ik) ? raw.ik : [])
    .map((k: J) => ({
      bone: boneIndex.get(String(k?.bone)) ?? -1,
      target: boneIndex.get(String(k?.target)) ?? -1,
      bendPositive: k?.bendPositive !== false,
      chain: num(k?.chain, 0),
      weight: num(k?.weight, 1),
    }))
    .filter((k: IkDef) => k.bone >= 0 && k.target >= 0)
  const animations = (Array.isArray(raw.animation) ? raw.animation : []).map((a: J) => parseAnimation(a, boneIndex, slotIndex))
  const action = (Array.isArray(raw.defaultActions) ? raw.defaultActions : []).find((a: J) => a?.gotoAndPlay)
  const first = animations.some((a: Anim) => a.name === action?.gotoAndPlay) ? String(action.gotoAndPlay) : (animations[0]?.name ?? '')
  const box = raw.aabb
  return {
    name: String(raw.name ?? ''),
    fps: num(raw.frameRate, fps) || 24,
    bones,
    slots,
    ik,
    animations,
    first,
    aabb: box && num(box.width, 0) > 0 && num(box.height, 0) > 0 ? [num(box.x, 0), num(box.y, 0), box.width, box.height] : null,
  }
}

/** The pieces of an atlas, by name. A piece stored turned (clockwise) gets a canvas of its own, turned back. */
function addRegions(out: Map<string, Region>, atlas: J, image: HTMLImageElement) {
  const scale = 1 / (num(atlas.scale, 1) || 1)
  for (const t of Array.isArray(atlas.SubTexture) ? atlas.SubTexture : []) {
    const name = String(t?.name ?? '')
    if (!name || out.has(name)) continue
    const [x, y, w, h] = [num(t.x, 0), num(t.y, 0), num(t.width, 0), num(t.height, 0)]
    if (w <= 0 || h <= 0) continue
    const frame = num(t.frameWidth, 0) > 0 && num(t.frameHeight, 0) > 0 ? { x: num(t.frameX, 0), y: num(t.frameY, 0), w: t.frameWidth, h: t.frameHeight } : null
    if (t.rotated === true) {
      // Stored w×h on the sheet; the piece itself is h×w.
      const c = document.createElement('canvas')
      c.width = h
      c.height = w
      const ctx = c.getContext('2d')
      if (!ctx) continue
      ctx.translate(0, w)
      ctx.rotate(-Math.PI / 2)
      ctx.drawImage(image, x, y, w, h, 0, 0, w, h)
      out.set(name, { src: c, sx: 0, sy: 0, w: h, h: w, scale, frame })
    } else {
      out.set(name, { src: image, sx: x, sy: y, w, h, scale, frame })
    }
  }
}

async function getJson(url: string): Promise<J> {
  const r = await fetch(url, { credentials: 'same-origin' })
  if (!r.ok) throw new Error(`${r.status}`)
  return r.json()
}

function loadImage(url: string): Promise<HTMLImageElement> {
  const img = new Image()
  img.decoding = 'async'
  img.src = url
  return img.decode().then(() => img)
}

/** A DragonBones skeleton, read from its JSON. */
export function readSkeleton(ske: J): Skeleton {
  const fps = num(ske?.frameRate, 24)
  const armatures = (Array.isArray(ske?.armature) ? ske.armature : []).map((a: J) => parseArmature(a, fps))
  if (!armatures.length) throw new Error('no armature')
  return { armatures, regions: new Map() }
}

/** The skeleton and the atlas pieces it's drawn with: the `_tex.json` it links to, or the one named after it. */
export async function dragonBonesData(a: Asset): Promise<Skeleton> {
  const skeleton = readSkeleton(await getJson(fileUrl(a)))
  const dir = a.path.includes('/') ? a.path.slice(0, a.path.lastIndexOf('/')) : ''
  const atlases = new Set((a.links ?? []).filter((l) => /_tex\.json$/i.test(l.path)).map((l) => l.path))
  const stem = a.path.match(/^(.*)_ske\.json$/i)?.[1]
  if (stem) atlases.add(`${stem}_tex.json`)
  for (const path of atlases) {
    try {
      const atlas = await getJson(packFileUrl(a.pack_id, path))
      const atlasDir = path.includes('/') ? path.slice(0, path.lastIndexOf('/')) : dir
      const imagePath = typeof atlas.imagePath === 'string' ? joinPath(atlasDir, atlas.imagePath) : path.replace(/\.json$/i, '.png')
      addRegions(skeleton.regions, atlas, await loadImage(packFileUrl(a.pack_id, imagePath)))
    } catch {
      // Drawn without that atlas's pieces.
    }
  }
  if (!skeleton.regions.size) throw new Error('no atlas')
  return skeleton
}

// ---- Playing ----

type Bounds = [minX: number, minY: number, maxX: number, maxY: number]

/** One armature playing one animation. */
class Armature {
  readonly worlds: Mat[]
  private display: number[]
  private alpha: number[]
  private deform: (number[] | null)[]
  private order: number[]
  private children = new Map<string, Armature>()
  anim: Anim | null = null
  readonly skeleton: Skeleton
  readonly def: ArmatureDef
  private depth: number

  constructor(skeleton: Skeleton, def: ArmatureDef, animation: string, depth = 0) {
    this.skeleton = skeleton
    this.def = def
    this.depth = depth
    this.worlds = def.bones.map(() => IDENTITY)
    this.display = def.slots.map((s) => s.displayIndex)
    this.alpha = def.slots.map((s) => s.alpha)
    this.deform = def.slots.map(() => null)
    this.order = def.slots.map((_, i) => i)
    this.play(animation)
  }

  play(name: string) {
    this.anim = this.def.animations.find((a) => a.name === name) ?? this.def.animations.find((a) => a.name === this.def.first) ?? null
  }

  /** Frames in the animation (at least one). */
  get frames(): number {
    return Math.max(1, Math.round(this.anim?.duration ?? 0))
  }

  /** Pose everything at `secs` into the animation (looping). */
  pose(secs: number) {
    const def = this.def
    const anim = this.anim
    const t = anim && anim.duration > 0 ? (((secs * def.fps) % anim.duration) + anim.duration) % anim.duration : 0
    const locals = def.bones.map((b, i) => {
      const tl = anim?.bones.get(i)
      const o = b.origin
      if (!tl) return o
      const [x, y] = sample(tl.t, t, lerp2) ?? [0, 0]
      const [rot, skew] = sample(tl.r, t, lerp2) ?? [0, 0]
      const [sx, sy] = sample(tl.s, t, lerp2) ?? [1, 1]
      return { x: o.x + x, y: o.y + y, rot: o.rot + rot, skew: o.skew + skew, sx: o.sx * sx, sy: o.sy * sy }
    })
    const done = def.bones.map(() => false)
    const world = (i: number): Mat => {
      if (done[i]) return this.worlds[i]
      done[i] = true
      this.worlds[i] = this.boneWorld(i, locals[i], def.bones[i].parent >= 0 ? world(def.bones[i].parent) : null)
      return this.worlds[i]
    }
    def.bones.forEach((_, i) => world(i))
    for (const ik of def.ik) this.solve(ik, locals)
    def.slots.forEach((s, i) => {
      const tl = anim?.slots.get(i)
      this.display[i] = tl ? (sample(tl.display, t, step) ?? s.displayIndex) : s.displayIndex
      this.alpha[i] = tl ? (sample(tl.alpha, t, lerp) ?? s.alpha) : s.alpha
      this.deform[i] = null
    })
    for (const d of anim?.deform ?? []) {
      const shown = def.slots[d.slot].displays[this.display[d.slot]]
      if (shown?.name === d.display) this.deform[d.slot] = sample(d.frames, t, lerp2) ?? null
    }
    this.order = this.drawOrder(anim ? sample(anim.zOrder, t, step) : undefined)
    if (this.depth < 4) {
      def.slots.forEach((s, i) => {
        const d = s.displays[this.display[i]]
        if (d?.type === 'armature') this.child(i, d)?.pose(secs)
      })
    }
  }

  /** A bone's world matrix from its local pose and its parent's, the way DragonBones works it out. */
  private boneWorld(i: number, local: Pose, parent: Mat | null): Mat {
    const b = this.def.bones[i]
    if (!parent) return toMatrix(local)
    if (b.inheritScale) {
      let pose = local
      if (!b.inheritRotation) pose = { ...local, rot: local.rot - fromMatrix(parent).rot }
      const m = mul(parent, toMatrix(pose))
      if (!b.inheritTranslation) [m[4], m[5]] = [local.x, local.y]
      return m
    }
    const pose = { ...local }
    if (b.inheritTranslation) [pose.x, pose.y] = point(parent, local.x, local.y)
    if (b.inheritRotation) {
      const p = fromMatrix(parent)
      let rot = local.rot + p.rot + (p.sx < 0 ? Math.PI : 0)
      if (parent[0] * parent[3] - parent[1] * parent[2] < 0) {
        rot -= local.rot * 2
        pose.skew += Math.PI
      }
      pose.rot = rot
    }
    return toMatrix(pose)
  }

  /** Aim one bone, or bend two, at the IK target; then the bones below them follow. */
  private solve(ik: IkDef, locals: Pose[]) {
    const bones = this.def.bones
    const target = this.worlds[ik.target]
    const [tx, ty] = [target[4], target[5]]
    const bone = ik.bone
    const parent = bones[bone].parent
    if (ik.chain === 0 || parent < 0) {
      const g = fromMatrix(this.worlds[bone])
      let r = Math.atan2(ty - g.y, tx - g.x)
      if (g.sx < 0) r += Math.PI
      g.rot += wrap(r - g.rot) * ik.weight
      this.worlds[bone] = toMatrix(g)
      this.refresh(bone, locals)
      return
    }
    const m = this.worlds[bone]
    const pg = fromMatrix(this.worlds[parent])
    const g = fromMatrix(m)
    const len = bones[bone].length
    const [lx, ly] = [m[0] * len, m[1] * len]
    const lLL = lx * lx + ly * ly
    const lL = Math.sqrt(lLL)
    let [dx, dy] = [g.x - pg.x, g.y - pg.y]
    const lPP = dx * dx + dy * dy
    const lP = Math.sqrt(lPP)
    const rawRot = g.rot
    const rawParentRot = pg.rot
    const rawA = Math.atan2(dy, dx)
    ;[dx, dy] = [tx - pg.x, ty - pg.y]
    const lTT = dx * dx + dy * dy
    const lT = Math.sqrt(lTT)
    let a: number
    if (lL + lP <= lT || lT + lL <= lP || lT + lP <= lL) {
      a = Math.atan2(ty - pg.y, tx - pg.x)
      if (!(lL + lP <= lT) && lP < lL) a += Math.PI
    } else {
      const h = (lPP - lLL + lTT) / (2 * lTT)
      const r = Math.sqrt(Math.max(0, lPP - h * h * lTT)) / lT
      const [hx, hy] = [pg.x + dx * h, pg.y + dy * h]
      const [rx, ry] = [-dy * r, dx * r]
      const gp = bones[parent].parent
      const pm = gp >= 0 ? this.worlds[gp] : IDENTITY
      const flipped = pm[0] * pm[3] - pm[1] * pm[2] < 0
      const [x, y] = flipped !== ik.bendPositive ? [hx - rx, hy - ry] : [hx + rx, hy + ry]
      a = Math.atan2(y - pg.y, x - pg.x)
    }
    const dr = wrap(a - rawA)
    pg.rot = rawParentRot + dr * ik.weight
    this.worlds[parent] = toMatrix(pg)
    const ca = rawA + dr * ik.weight
    g.x = pg.x + Math.cos(ca) * lP
    g.y = pg.y + Math.sin(ca) * lP
    let rb = Math.atan2(ty - g.y, tx - g.x)
    if (g.sx < 0) rb += Math.PI
    g.rot = pg.rot + rawRot - rawParentRot + wrap(rb - dr - rawRot) * ik.weight
    this.worlds[bone] = toMatrix(g)
    this.refresh(parent, locals, bone)
  }

  /** Work out again every bone below `root` (except `keep`, already placed). */
  private refresh(root: number, locals: Pose[], keep = -1) {
    const bones = this.def.bones
    const below = (i: number): boolean => {
      for (let p = bones[i].parent; p >= 0; p = bones[p].parent) if (p === root) return true
      return false
    }
    const redo = bones.map((_, i) => i !== keep && below(i))
    const done = bones.map((_, i) => !redo[i])
    const world = (i: number): Mat => {
      if (done[i]) return this.worlds[i]
      done[i] = true
      this.worlds[i] = this.boneWorld(i, locals[i], world(bones[i].parent))
      return this.worlds[i]
    }
    bones.forEach((_, i) => world(i))
  }

  /** The draw order: the slots' own, with a `zOrder` frame's moves (slot index, offset pairs) applied. */
  private drawOrder(moves: number[] | undefined): number[] {
    const n = this.def.slots.length
    if (!moves?.length) return this.def.slots.map((_, i) => i)
    const order = new Array<number>(n).fill(-1)
    const unchanged: number[] = []
    let at = 0
    for (let i = 0; i + 1 < moves.length; i += 2) {
      const [slot, offset] = [moves[i], moves[i + 1]]
      while (at < slot && at < n) unchanged.push(at++)
      if (at >= n) break
      const to = at + offset
      if (to >= 0 && to < n && order[to] === -1) order[to] = at
      else unchanged.push(at)
      at++
    }
    while (at < n) unchanged.push(at++)
    let u = unchanged.length
    for (let i = n - 1; i >= 0; i--) if (order[i] === -1) order[i] = unchanged[--u] ?? i
    return order
  }

  private child(slot: number, d: Display): Armature | null {
    const key = `${slot}:${d.name}`
    let c = this.children.get(key)
    if (!c) {
      const def = this.skeleton.armatures.find((a) => a.name === d.name)
      if (!def || def === this.def) return null
      c = new Armature(this.skeleton, def, d.animation, this.depth + 1)
      this.children.set(key, c)
    }
    return c
  }

  /** The slot's matrix: its bone's, with the display's own offset. */
  private slotMatrix(i: number, d: Display): Mat {
    return mul(this.worlds[this.def.slots[i].bone], d.matrix)
  }

  /** A mesh's vertices in the armature's space. */
  private meshPoints(i: number, d: Display, mesh: Mesh): number[] {
    const deform = this.deform[i]
    const out: number[] = []
    if (mesh.weighted) {
      const linear = (m: Mat): Mat => [m[0], m[1], m[2], m[3], 0, 0]
      mesh.weighted.forEach((v, k) => {
        let dx = deform?.[k * 2] ?? 0
        let dy = deform?.[k * 2 + 1] ?? 0
        if (dx || dy) [dx, dy] = point(linear(mesh.slotPose), dx, dy)
        let [x, y] = [0, 0]
        v.bones.forEach((b, j) => {
          let [lx, ly] = [v.local[j * 2], v.local[j * 2 + 1]]
          if (dx || dy) {
            const [ox, oy] = point(linear(invert(mesh.bonePoses.get(b) ?? IDENTITY)), dx, dy)
            lx += ox
            ly += oy
          }
          const [wx, wy] = point(this.worlds[b], lx, ly)
          x += wx * v.weights[j]
          y += wy * v.weights[j]
        })
        out.push(x, y)
      })
      return out
    }
    const m = this.slotMatrix(i, d)
    for (let k = 0; k + 1 < mesh.vertices.length; k += 2) {
      out.push(...point(m, mesh.vertices[k] + (deform?.[k] ?? 0), mesh.vertices[k + 1] + (deform?.[k + 1] ?? 0)))
    }
    return out
  }

  /** Draw it with `view` (armature space to canvas pixels). */
  draw(ctx: CanvasRenderingContext2D, view: Mat, alpha = 1) {
    for (const i of this.order) {
      const s = this.def.slots[i]
      const d = s.displays[this.display[i]]
      const a = alpha * this.alpha[i]
      if (!d || a <= 0) continue
      ctx.globalAlpha = Math.min(1, a)
      ctx.globalCompositeOperation = s.blend
      if (d.type === 'armature') {
        this.child(i, d)?.draw(ctx, mul(view, this.slotMatrix(i, d)), a)
        continue
      }
      const region = this.skeleton.regions.get(d.texture)
      if (!region) continue
      if (d.type === 'image') {
        const m = mul(view, this.slotMatrix(i, d))
        ctx.setTransform(m[0], m[1], m[2], m[3], m[4], m[5])
        const [x, y, w, h] = imageRect(d, region)
        ctx.drawImage(region.src, region.sx, region.sy, region.w, region.h, x, y, w, h)
      } else if (d.type === 'mesh' && d.mesh) {
        drawMesh(ctx, region, d.mesh, this.meshPoints(i, d, d.mesh), view)
      }
    }
    ctx.setTransform(1, 0, 0, 1, 0, 0)
    ctx.globalAlpha = 1
    ctx.globalCompositeOperation = 'source-over'
  }

  /** What it covers as posed, in `m`'s space. */
  bounds(m: Mat = IDENTITY, into: Bounds = [Infinity, Infinity, -Infinity, -Infinity]): Bounds {
    const add = (x: number, y: number) => {
      const [px, py] = point(m, x, y)
      into[0] = Math.min(into[0], px)
      into[1] = Math.min(into[1], py)
      into[2] = Math.max(into[2], px)
      into[3] = Math.max(into[3], py)
    }
    for (const i of this.order) {
      const d = this.def.slots[i].displays[this.display[i]]
      if (!d || this.alpha[i] <= 0) continue
      if (d.type === 'armature') {
        this.child(i, d)?.bounds(mul(m, this.slotMatrix(i, d)), into)
        continue
      }
      const region = this.skeleton.regions.get(d.texture)
      if (!region) continue
      if (d.type === 'image') {
        const sm = this.slotMatrix(i, d)
        const [x, y, w, h] = imageRect(d, region)
        for (const [cx, cy] of [[x, y], [x + w, y], [x, y + h], [x + w, y + h]]) add(...point(sm, cx, cy))
      } else if (d.mesh) {
        const p = this.meshPoints(i, d, d.mesh)
        for (let k = 0; k + 1 < p.length; k += 2) add(p[k], p[k + 1])
      }
    }
    return into
  }
}

/** Where an image display's piece goes in the slot's space: centred on its pivot, its trimmed edges put back. */
function imageRect(d: Display, r: Region): [number, number, number, number] {
  const fw = r.frame ? r.frame.w : r.w
  const fh = r.frame ? r.frame.h : r.h
  const fx = r.frame ? r.frame.x : 0
  const fy = r.frame ? r.frame.y : 0
  return [-(d.pivotX * fw + fx) * r.scale, -(d.pivotY * fh + fy) * r.scale, r.w * r.scale, r.h * r.scale]
}

/** A textured mesh, one triangle at a time: each is clipped and the piece drawn through the affine that maps it. */
function drawMesh(ctx: CanvasRenderingContext2D, r: Region, mesh: Mesh, points: number[], view: Mat) {
  const screen: number[] = []
  for (let k = 0; k + 1 < points.length; k += 2) screen.push(...point(view, points[k], points[k + 1]))
  const tri = mesh.triangles
  for (let t = 0; t + 2 < tri.length; t += 3) {
    const [i0, i1, i2] = [tri[t], tri[t + 1], tri[t + 2]]
    const [x0, y0, x1, y1, x2, y2] = [screen[i0 * 2], screen[i0 * 2 + 1], screen[i1 * 2], screen[i1 * 2 + 1], screen[i2 * 2], screen[i2 * 2 + 1]]
    if ([x0, y0, x1, y1, x2, y2].some((v) => v === undefined || !Number.isFinite(v))) continue
    const u = (i: number) => r.sx + mesh.uvs[i * 2] * r.w
    const v = (i: number) => r.sy + mesh.uvs[i * 2 + 1] * r.h
    const [u0, v0, u1, v1, u2, v2] = [u(i0), v(i0), u(i1), v(i1), u(i2), v(i2)]
    const [du1, dv1, du2, dv2] = [u1 - u0, v1 - v0, u2 - u0, v2 - v0]
    const det = du1 * dv2 - du2 * dv1
    if (!det) continue
    const [dx1, dy1, dx2, dy2] = [x1 - x0, y1 - y0, x2 - x0, y2 - y0]
    const a = (dx1 * dv2 - dx2 * dv1) / det
    const c = (du1 * dx2 - du2 * dx1) / det
    const b = (dy1 * dv2 - dy2 * dv1) / det
    const d = (du1 * dy2 - du2 * dy1) / det
    // Grown half a pixel about its middle, so neighbours meet without a hairline between them.
    const [mx, my] = [(x0 + x1 + x2) / 3, (y0 + y1 + y2) / 3]
    const grow = (x: number, y: number): [number, number] => {
      const len = Math.hypot(x - mx, y - my) || 1
      return [x + ((x - mx) / len) * 0.5, y + ((y - my) / len) * 0.5]
    }
    ctx.save()
    ctx.setTransform(1, 0, 0, 1, 0, 0)
    ctx.beginPath()
    ctx.moveTo(...grow(x0, y0))
    ctx.lineTo(...grow(x1, y1))
    ctx.lineTo(...grow(x2, y2))
    ctx.closePath()
    ctx.clip()
    ctx.setTransform(a, b, c, d, x0 - a * u0 - c * v0, y0 - b * u0 - d * v0)
    ctx.drawImage(r.src, r.sx, r.sy, r.w, r.h, r.sx, r.sy, r.w, r.h)
    ctx.restore()
  }
}

/** The view that fits `b` into a `w`×`h` canvas with a margin. */
function fit(b: Bounds, w: number, h: number): Mat {
  const bw = b[2] - b[0]
  const bh = b[3] - b[1]
  if (!(bw > 0 && bh > 0)) return [1, 0, 0, 1, w / 2, h / 2]
  const s = Math.min(w / bw, h / bh) * 0.92
  return [s, 0, 0, s, w / 2 - s * (b[0] + bw / 2), h / 2 - s * (b[1] + bh / 2)]
}

/** What the animation covers over its length (a few poses sampled), or the armature's own box. */
function animationBounds(arm: Armature): Bounds {
  const b: Bounds = [Infinity, Infinity, -Infinity, -Infinity]
  const n = arm.frames
  for (let k = 0; k < 12; k++) {
    arm.pose(((n * k) / 12) / arm.def.fps)
    arm.bounds(IDENTITY, b)
  }
  if (Number.isFinite(b[0])) return b
  const box = arm.def.aabb
  return box ? [box[0], box[1], box[0] + box[2], box[1] + box[3]] : [-100, -100, 100, 100]
}

/** The armatures and their animations, for a picker. */
export function choices(s: Skeleton): { armature: string; animations: string[] }[] {
  return s.armatures.map((a) => ({ armature: a.name, animations: a.animations.map((x) => x.name) }))
}

export type Player = {
  /** Show `animation` of `armature` (the first armature and its starting animation when not given). */
  show(armature?: string, animation?: string): void
  setPlaying(on: boolean): void
  /** The frame shown, and how many there are. */
  readonly frame: number
  readonly total: number
  goTo(frame: number): void
  onFrame: ((frame: number) => void) | null
  destroy(): void
}

/** Play the skeleton on `canvas`, which is kept at its laid-out size (times the device's pixel ratio). */
export function mountDragonBones(canvas: HTMLCanvasElement, skeleton: Skeleton, playing: boolean): Player {
  const ctx = canvas.getContext('2d')
  if (!ctx) throw new Error('no 2D canvas')
  let arm = new Armature(skeleton, skeleton.armatures[0], skeleton.armatures[0].first)
  let box = animationBounds(arm)
  let secs = 0
  let last = performance.now()
  let raf = 0
  let on = playing
  const draw = () => {
    const dpr = window.devicePixelRatio || 1
    const w = Math.max(1, Math.round(canvas.clientWidth * dpr))
    const h = Math.max(1, Math.round(canvas.clientHeight * dpr))
    if (canvas.width !== w || canvas.height !== h) [canvas.width, canvas.height] = [w, h]
    ctx.setTransform(1, 0, 0, 1, 0, 0)
    ctx.clearRect(0, 0, w, h)
    arm.pose(secs)
    arm.draw(ctx, fit(box, w, h))
  }
  const frameNow = () => Math.floor(secs * arm.def.fps + 1e-6) % arm.frames
  const tick = (now: number) => {
    if (on) secs += Math.min(0.25, (now - last) / 1000)
    last = now
    draw()
    if (on) player.onFrame?.(frameNow())
    raf = requestAnimationFrame(tick)
  }
  const player: Player = {
    show(armature, animation) {
      const def = skeleton.armatures.find((a) => a.name === armature) ?? skeleton.armatures[0]
      arm = new Armature(skeleton, def, animation ?? def.first)
      box = animationBounds(arm)
      secs = 0
      player.onFrame?.(0)
    },
    setPlaying(v) {
      on = v
      last = performance.now()
    },
    get frame() {
      return frameNow()
    },
    get total() {
      return arm.frames
    },
    goTo(frame) {
      secs = (((frame % arm.frames) + arm.frames) % arm.frames) / arm.def.fps
      player.onFrame?.(frameNow())
    },
    onFrame: null,
    destroy() {
      cancelAnimationFrame(raf)
    },
  }
  raf = requestAnimationFrame(tick)
  draw()
  return player
}

/**
 * Draw the skeleton 256 px across, at whichever of a few poses of its starting animation covers the most, and send it as
 * the asset's thumbnail.
 */
export async function drawDragonBonesThumb(a: Asset, skeleton?: Skeleton): Promise<ThumbUpload | null> {
  skeleton ??= await dragonBonesData(a)
  const arm = new Armature(skeleton, skeleton.armatures[0], skeleton.armatures[0].first)
  const box = animationBounds(arm)
  const [bw, bh] = [box[2] - box[0], box[3] - box[1]]
  const s = 256 / Math.max(bw, bh, 1)
  const canvas = document.createElement('canvas')
  canvas.width = Math.max(1, Math.round(bw * s))
  canvas.height = Math.max(1, Math.round(bh * s))
  const ctx = canvas.getContext('2d')
  if (!ctx) return null
  const view = fit(box, canvas.width, canvas.height)
  const n = arm.frames
  const at = (f: number) => Math.min(n - 1, Math.floor(n * f)) / arm.def.fps
  let best = { secs: 0, cover: -1 }
  for (const f of [0, 0.5, 0.25, 0.75]) {
    ctx.clearRect(0, 0, canvas.width, canvas.height)
    arm.pose(at(f))
    arm.draw(ctx, view)
    const cover = coverage(canvas)
    if (cover > best.cover) best = { secs: at(f), cover }
  }
  ctx.clearRect(0, 0, canvas.width, canvas.height)
  arm.pose(best.secs)
  arm.draw(ctx, view)
  if (best.cover <= 0) return null
  const png = await new Promise<Blob | null>((resolve) => canvas.toBlob(resolve, 'image/png'))
  return png ? api.putThumb(a.id, png, 'browser') : null
}
