// Hoard's own runtime for 2D skeletons, the kind DragonBones and Spine make: bones (with IK and transform constraints),
// images, meshes (weighted or not, with their deform), clipping, nested armatures, and animations' bone, slot,
// draw-order, deform and constraint timelines with their easing, drawn on a 2D canvas. The readers (`dragonbones.ts`,
// `spine.ts`) turn each format into the armatures here, in DragonBones' space: y down, angles clockwise. The server
// can't draw these, so the browser makes their thumbnails too.
import { api, type Asset, type ThumbUpload } from './api'
import { coverage } from './browserThumbs'

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

/** The timeline's value at frame `t`, tweened by `lerp`. */
function sample<T>(fs: Frame<T>[], t: number, lerp: (a: T, b: T, p: number) => T): T | undefined {
  if (!fs.length) return undefined
  let i = 0
  while (i + 1 < fs.length && fs[i + 1].start <= t) i++
  const f = fs[i]
  const next = fs[i + 1]
  if (!next || !f.ease || f.dur <= 0 || t < f.start) return f.v
  return lerp(f.v, next.v, eased(f.ease, Math.min(1, Math.max(0, (t - f.start) / f.dur))))
}

const lerp = (a: number, b: number, p: number) => a + (b - a) * p
const lerp2 = (a: number[], b: number[], p: number) => a.map((v, i) => lerp(v, b[i] ?? v, p))
const step = <T>(a: T) => a

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
      const channels = anim?.bones.get(i)
      if (!channels) return b.origin
      const p = { ...b.origin }
      for (const c of channels) {
        const v = sample(c.fs, t, lerp2)
        if (v) c.apply(p, v)
      }
      return p
    })
    const done = def.bones.map(() => false)
    const world = (i: number): Mat => {
      if (done[i]) return this.worlds[i]
      done[i] = true
      this.worlds[i] = this.boneWorld(i, locals[i], def.bones[i].parent >= 0 ? world(def.bones[i].parent) : null)
      return this.worlds[i]
    }
    def.bones.forEach((_, i) => world(i))
    def.constraints.forEach((c, k) => {
      const tl = anim?.constraints.get(k)
      const mix = (tl && sample(tl.mix, t, lerp2)) || null
      if (c.kind === 'ik') {
        const weight = mix?.[0] ?? c.weight
        const bend = tl ? (sample(tl.bend, t, step) ?? (c.bendPositive ? 1 : 0)) : c.bendPositive ? 1 : 0
        if (weight > 0) this.solve({ ...c, weight, bendPositive: bend > 0 }, locals)
      } else {
        this.transform(c, mix ?? c.mix, locals)
      }
    })
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

  /** Turn, move, scale and shear bones toward the target, as much as `mix` says; then the bones below them follow. */
  private transform(c: TransformDef, mix: number[], locals: Pose[]) {
    const [mr, mx, my, msx, msy, msh] = mix
    if (!(mr || mx || my || msx || msy || msh)) return
    const [orot, ox, oy, osx, osy, osh] = c.offset
    for (const b of c.bones) {
      const tm = this.worlds[c.target]
      const m = [...this.worlds[b]] as Mat
      if (mr) {
        const r = wrap(Math.atan2(tm[1], tm[0]) - Math.atan2(m[1], m[0]) + orot) * mr
        const [cs, sn] = [Math.cos(r), Math.sin(r)]
        ;[m[0], m[1], m[2], m[3]] = [cs * m[0] - sn * m[1], sn * m[0] + cs * m[1], cs * m[2] - sn * m[3], sn * m[2] + cs * m[3]]
      }
      if (mx || my) {
        const [px, py] = point(tm, ox, oy)
        m[4] += (px - m[4]) * mx
        m[5] += (py - m[5]) * my
      }
      const rescale = (i: number, mixed: number, offset: number) => {
        const s = Math.hypot(m[i], m[i + 1])
        if (!mixed || !s) return
        const k = (s + (Math.hypot(tm[i], tm[i + 1]) - s + offset) * mixed) / s
        m[i] *= k
        m[i + 1] *= k
      }
      rescale(0, msx, osx)
      rescale(2, msy, osy)
      if (msh) {
        const by = Math.atan2(m[3], m[2])
        const r = wrap(Math.atan2(tm[3], tm[2]) - Math.atan2(tm[1], tm[0]) - (by - Math.atan2(m[1], m[0])))
        const angle = by + (r + osh) * msh
        const s = Math.hypot(m[2], m[3])
        ;[m[2], m[3]] = [Math.cos(angle) * s, Math.sin(angle) * s]
      }
      this.worlds[b] = m
      this.refresh(b, locals)
    }
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
    const deform = d.type === 'mesh' ? this.deform[i] : null
    const out: number[] = []
    if (mesh.weighted) {
      const linear = (m: Mat): Mat => [m[0], m[1], m[2], m[3], 0, 0]
      let influence = 0
      mesh.weighted.forEach((v, k) => {
        let dx = 0
        let dy = 0
        if (!mesh.deformPerBone) {
          ;[dx, dy] = [deform?.[k * 2] ?? 0, deform?.[k * 2 + 1] ?? 0]
          if (dx || dy) [dx, dy] = point(linear(mesh.slotPose), dx, dy)
        }
        let [x, y] = [0, 0]
        v.bones.forEach((b, j) => {
          let [lx, ly] = [v.local[j * 2], v.local[j * 2 + 1]]
          if (mesh.deformPerBone) {
            lx += deform?.[influence * 2] ?? 0
            ly += deform?.[influence * 2 + 1] ?? 0
            influence++
          } else if (dx || dy) {
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
    // The slot a clipping polygon (saved on the context) ends with.
    let clipEnd = -1
    for (const i of this.order) {
      const s = this.def.slots[i]
      const d = s.displays[this.display[i]]
      const a = alpha * this.alpha[i]
      if (d?.type === 'clip' && d.mesh) {
        if (clipEnd >= 0) ctx.restore()
        const p = this.meshPoints(i, d, d.mesh)
        ctx.save()
        ctx.setTransform(1, 0, 0, 1, 0, 0)
        ctx.beginPath()
        for (let k = 0; k + 1 < p.length; k += 2) ctx.lineTo(...point(view, p[k], p[k + 1]))
        ctx.closePath()
        ctx.clip()
        clipEnd = d.clipEnd
      } else if (d && a > 0) this.drawSlot(ctx, view, i, s, d, a)
      if (i === clipEnd) {
        ctx.restore()
        clipEnd = -1
      }
    }
    if (clipEnd >= 0) ctx.restore()
    ctx.setTransform(1, 0, 0, 1, 0, 0)
    ctx.globalAlpha = 1
    ctx.globalCompositeOperation = 'source-over'
  }

  private drawSlot(ctx: CanvasRenderingContext2D, view: Mat, i: number, s: SlotDef, d: Display, a: number) {
    ctx.globalAlpha = Math.min(1, a)
    ctx.globalCompositeOperation = s.blend
    if (d.type === 'armature') {
      this.child(i, d)?.draw(ctx, mul(view, this.slotMatrix(i, d)), a)
      return
    }
    const region = this.skeleton.regions.get(d.texture)
    if (!region) return
    if (d.type === 'image') {
      const m = mul(view, this.slotMatrix(i, d))
      ctx.setTransform(m[0], m[1], m[2], m[3], m[4], m[5])
      const [x, y, w, h] = imageRect(d, region)
      ctx.drawImage(region.src, region.sx, region.sy, region.w, region.h, x, y, w, h)
    } else if (d.type === 'mesh' && d.mesh) {
      drawMesh(ctx, region, d.mesh, this.meshPoints(i, d, d.mesh), view)
    }
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
      } else if (d.type === 'mesh' && d.mesh) {
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
  const kx = d.size ? d.size[0] / fw : r.scale
  const ky = d.size ? d.size[1] / fh : r.scale
  return [-(d.pivotX * fw + fx) * kx, -(d.pivotY * fh + fy) * ky, r.w * kx, r.h * ky]
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
    arm.pose((n * k) / 12 / arm.def.fps)
    arm.bounds(IDENTITY, b)
  }
  if (Number.isFinite(b[0])) return b
  const box = arm.def.aabb
  return box ? [box[0], box[1], box[0] + box[2], box[1] + box[3]] : [-100, -100, 100, 100]
}

/** The armatures (or skins) and their animations, for a picker. */
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
export function mountSkeleton(canvas: HTMLCanvasElement, skeleton: Skeleton, playing: boolean): Player {
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
export async function drawSkeletonThumb(a: Asset, skeleton: Skeleton): Promise<ThumbUpload | null> {
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

/** The skeleton an asset holds, read by its format's reader with the atlas it's drawn on. */
export async function skeletonOf(a: Asset): Promise<Skeleton> {
  return a.meta?.spine != null ? (await import('./spine')).spineData(a) : (await import('./dragonbones')).dragonBonesData(a)
}
