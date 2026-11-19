// What a 2D skeleton is to Hoard's runtime (`skeleton2d.ts`): the armatures, bones, slots, displays, meshes,
// constraints and animation timelines the readers (`dragonbones.ts`, `spine.ts`) build from each format, and the
// matrix arithmetic both sides share. Everything is in DragonBones' space: y down, angles clockwise.
// The formats' JSON is read loosely: every field may be missing, and older exports name some differently.
export type J = any

/** An affine matrix as DragonBones writes it: x' = a·x + c·y + tx, y' = b·x + d·y + ty. */
export type Mat = [a: number, b: number, c: number, d: number, tx: number, ty: number]
/** A bone's pose: position, rotation and skew in radians, scale. */
export type Pose = { x: number; y: number; rot: number; skew: number; sx: number; sy: number }

export const IDENTITY: Mat = [1, 0, 0, 1, 0, 0]
export const DEG = Math.PI / 180

/** `m`, then `p`. */
export function mul(p: Mat, m: Mat): Mat {
  return [
    m[0] * p[0] + m[1] * p[2],
    m[0] * p[1] + m[1] * p[3],
    m[2] * p[0] + m[3] * p[2],
    m[2] * p[1] + m[3] * p[3],
    p[0] * m[4] + p[2] * m[5] + p[4],
    p[1] * m[4] + p[3] * m[5] + p[5],
  ]
}

export function invert([a, b, c, d, tx, ty]: Mat): Mat {
  const det = a * d - b * c || 1e-9
  const [ia, ib, ic, id] = [d / det, -b / det, -c / det, a / det]
  return [ia, ib, ic, id, -(ia * tx + ic * ty), -(ib * tx + id * ty)]
}

export function toMatrix(p: Pose): Mat {
  return [
    Math.cos(p.rot) * p.sx,
    Math.sin(p.rot) * p.sx,
    -Math.sin(p.skew + p.rot) * p.sy,
    Math.cos(p.skew + p.rot) * p.sy,
    p.x,
    p.y,
  ]
}

/** An angle in -π..π. */
export function wrap(r: number): number {
  r = (r + Math.PI) % (Math.PI * 2)
  return r + (r > 0 ? -Math.PI : Math.PI)
}

export function num(v: unknown, fallback: number): number {
  return typeof v === 'number' && Number.isFinite(v) ? v : fallback
}

export function point(m: Mat, x: number, y: number): [number, number] {
  return [m[0] * x + m[2] * y + m[4], m[1] * x + m[3] * y + m[5]]
}

// ---- Timelines ----

export type Ease = { kind: 'linear' } | { kind: 'quad'; type: 1 | 2 | 3; amount: number } | { kind: 'curve'; c: number[] }
/** A key: where it starts and how long it lasts, in frames; `ease` null holds it until the next. */
export type Frame<T> = { start: number; dur: number; ease: Ease | null; v: T }

// ---- The skeleton ----

export type Region = { src: CanvasImageSource; sx: number; sy: number; w: number; h: number; scale: number; frame: { x: number; y: number; w: number; h: number } | null }

export type Weighted = { bones: number[]; weights: number[]; local: number[] }
export type Mesh = {
  vertices: number[]
  uvs: number[]
  triangles: number[]
  /** Per vertex, for a mesh bound to bones: which bones, how much, and where the vertex sits in each. */
  weighted: Weighted[] | null
  /** A deform moves each bone's copy of a vertex (Spine), rather than the vertex in the mesh's space (DragonBones). */
  deformPerBone: boolean
  /** Turns a deform offset (in the mesh's space) into each bone's space: the slot pose's linear part, per bone. */
  slotPose: Mat
  bonePoses: Map<number, Mat>
}

export type Display = {
  type: 'image' | 'mesh' | 'armature' | 'clip' | 'none'
  name: string
  texture: string
  matrix: Mat
  pivotX: number
  pivotY: number
  /** The size the whole picture is drawn at, when it isn't its size in the atlas. */
  size: [number, number] | null
  /** A mesh's, or a clipping polygon's, points. */
  mesh: Mesh | null
  /** For a nested armature: which armature, playing which animation. */
  animation: string
  /** For a clipping polygon: the last slot it clips. */
  clipEnd: number
}

export const NO_DISPLAY: Display = { type: 'none', name: '', texture: '', matrix: IDENTITY, pivotX: 0.5, pivotY: 0.5, size: null, mesh: null, animation: '', clipEnd: -1 }

export type BoneDef = { name: string; parent: number; length: number; origin: Pose; inheritScale: boolean; inheritRotation: boolean; inheritTranslation: boolean }
export type SlotDef = { name: string; bone: number; displayIndex: number; alpha: number; blend: GlobalCompositeOperation; displays: Display[] }
export type IkDef = { kind: 'ik'; bone: number; target: number; bendPositive: boolean; chain: number; weight: number }
/** Moves bones toward a target in world space: offsets (rotation, x, y, scale x and y, shear y) and how much of each. */
export type TransformDef = { kind: 'transform'; bones: number[]; target: number; offset: number[]; mix: number[] }
export type Constraint = IkDef | TransformDef

/** One animated part of a bone's pose: the keys, and how a value changes the pose. */
export type Channel = { fs: Frame<number[]>[]; apply: (p: Pose, v: number[]) => void }
export type SlotTimeline = { display: Frame<number>[]; alpha: Frame<number>[] }
export type DeformTimeline = { slot: number; display: string; frames: Frame<number[]>[] }
/** A constraint's mix (an IK's weight, a transform's six) and an IK's bend direction (1 or 0). */
export type ConstraintTimeline = { mix: Frame<number[]>[]; bend: Frame<number>[] }
export type Anim = {
  name: string
  /** In frames. */
  duration: number
  bones: Map<number, Channel[]>
  slots: Map<number, SlotTimeline>
  zOrder: Frame<number[]>[]
  deform: DeformTimeline[]
  constraints: Map<number, ConstraintTimeline>
}

export type ArmatureDef = {
  name: string
  fps: number
  bones: BoneDef[]
  slots: SlotDef[]
  /** In the order they're applied. */
  constraints: Constraint[]
  animations: Anim[]
  /** The animation it starts with. */
  first: string
  aabb: [number, number, number, number] | null
}

export type Skeleton = {
  armatures: ArmatureDef[]
  regions: Map<string, Region>
}

export async function getJson(url: string): Promise<J> {
  const r = await fetch(url, { credentials: 'same-origin' })
  if (!r.ok) throw new Error(`${r.status}`)
  return r.json()
}

export function loadImage(url: string): Promise<HTMLImageElement> {
  const img = new Image()
  img.decoding = 'async'
  img.src = url
  return img.decode().then(() => img)
}
