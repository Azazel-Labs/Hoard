// DragonBones skeletons (`…_ske.json`), read into Hoard's 2D skeleton runtime (`skeleton2d.ts`): bones (with IK),
// images, meshes (weighted or not, with their deform), nested armatures, and the animations' bone, slot, draw-order and
// deform timelines with their easing. The texture atlas is the `…_tex.json` and image beside the skeleton.
import { fileUrl, joinPath, packFileUrl, type Asset } from './api'
import {
  DEG,
  IDENTITY,
  NO_DISPLAY,
  getJson,
  invert,
  loadImage,
  num,
  point,
  toMatrix,
  wrap,
  type Anim,
  type ArmatureDef,
  type BoneDef,
  type Channel,
  type Display,
  type DeformTimeline,
  type Ease,
  type Frame,
  type IkDef,
  type J,
  type Mat,
  type Mesh,
  type Pose,
  type Region,
  type Skeleton,
  type SlotDef,
  type SlotTimeline,
  type Weighted,
} from './skeleton2d'

function poseOf(t: J): Pose {
  const skX = num(t?.skX, 0) * DEG
  const skY = num(t?.skY, 0) * DEG
  return { x: num(t?.x, 0), y: num(t?.y, 0), rot: skY, skew: skX - skY, sx: num(t?.scX, 1), sy: num(t?.scY, 1) }
}

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
  return { vertices, uvs: list(d.uvs), triangles: list(d.triangles), weighted, deformPerBone: false, slotPose, bonePoses }
}

function parseDisplay(d: J, bones: number): Display {
  const type = d?.type ?? 'image'
  const name = String(d?.name ?? '')
  const base: Display = {
    ...NO_DISPLAY,
    name,
    texture: String(d?.path ?? name),
    matrix: d?.transform ? toMatrix(poseOf(d.transform)) : IDENTITY,
    pivotX: num(d?.pivot?.x, 0.5),
    pivotY: num(d?.pivot?.y, 0.5),
  }
  if (type === 'image') return { ...base, type: 'image' }
  if (type === 'mesh' && Array.isArray(d.vertices)) return { ...base, type: 'mesh', mesh: parseMesh(d, bones) }
  if (type === 'armature') {
    const action = (Array.isArray(d.actions) ? d.actions : []).find((a: J) => a?.gotoAndPlay || a?.name)
    return { ...base, type: 'armature', animation: String(action?.gotoAndPlay ?? action?.name ?? '') }
  }
  return base
}

const translate: Channel['apply'] = (p, v) => {
  p.x += v[0]
  p.y += v[1]
}
const rotate: Channel['apply'] = (p, v) => {
  p.rot += v[0]
  p.skew += v[1]
}
const scale: Channel['apply'] = (p, v) => {
  p.sx *= v[0]
  p.sy *= v[1]
}

function parseAnimation(raw: J, boneIndex: Map<string, number>, slotIndex: Map<string, number>): Anim {
  const bones = new Map<number, Channel[]>()
  for (const b of Array.isArray(raw.bone) ? raw.bone : []) {
    const i = boneIndex.get(b?.name)
    if (i === undefined) continue
    if (Array.isArray(b.frame) && !b.translateFrame && !b.rotateFrame && !b.scaleFrame) {
      // Before 5.5, one timeline of whole transforms.
      const tf = (f: J) => f.transform ?? {}
      bones.set(i, [
        { fs: frames(b.frame, (f) => [num(tf(f).x, 0), num(tf(f).y, 0)]), apply: translate },
        { fs: rotateFrames(b.frame, (f) => num(tf(f).skY, 0) * DEG, (f) => (num(tf(f).skX, 0) - num(tf(f).skY, 0)) * DEG), apply: rotate },
        { fs: frames(b.frame, (f) => [num(tf(f).scX, 1), num(tf(f).scY, 1)]), apply: scale },
      ])
    } else {
      bones.set(i, [
        { fs: frames(b.translateFrame, (f) => [num(f.x, 0), num(f.y, 0)]), apply: translate },
        { fs: rotateFrames(b.rotateFrame ?? [], (f) => num(f.rotate, 0) * DEG, (f) => num(f.skew, 0) * DEG), apply: rotate },
        { fs: frames(b.scaleFrame, (f) => [num(f.x, 1), num(f.y, 1)]), apply: scale },
      ])
    }
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
    constraints: new Map(),
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
    .map((k: J): IkDef => ({
      kind: 'ik',
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
    constraints: ik,
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
