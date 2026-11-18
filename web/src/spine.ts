// Spine skeletons (`.json`), read into Hoard's 2D skeleton runtime (`skeleton2d.ts`) straight from Spine's published JSON
// format (3.x and 4.x), not with Spine's own runtime: bones (with IK and transform constraints), regions, meshes (weighted,
// linked), clipping, skins, and the animations' bone, slot, draw-order, deform and constraint timelines with their
// curves. The texture atlas is the libGDX `.atlas` beside the skeleton.
//
// Spine's space is y up with angles counter-clockwise; the runtime's is y down and clockwise, so every y and every angle
// is turned over on the way in.
import { fileUrl, joinPath, packFileUrl, type Asset } from './api'
import {
  DEG,
  IDENTITY,
  NO_DISPLAY,
  getJson,
  loadImage,
  num,
  toMatrix,
  wrap,
  type Anim,
  type ArmatureDef,
  type BoneDef,
  type Channel,
  type Constraint,
  type ConstraintTimeline,
  type DeformTimeline,
  type Display,
  type Ease,
  type Frame,
  type J,
  type Mesh,
  type Region,
  type Skeleton,
  type SlotDef,
  type SlotTimeline,
  type Weighted,
} from './skeleton2d'

/** The animations a skeleton starts with, when it has one by these names (the server picks the same). */
const STARTERS = ['idle', 'walk', 'run', 'animation']

const BLENDS: Record<string, GlobalCompositeOperation> = { additive: 'lighter', multiply: 'multiply', screen: 'screen' }

type Ctx = {
  /** Spine 4 or later: absolute bezier curves, `value` keys, unwrapped rotations. */
  v4: boolean
  fps: number
  bones: Map<string, number>
  slots: Map<string, number>
}

const list = (v: J): J[] => (Array.isArray(v) ? v : [])
const entries = (v: J): [string, J][] => (v && typeof v === 'object' && !Array.isArray(v) ? Object.entries(v) : [])
/** A colour's alpha, from `rrggbbaa`. */
const alphaOf = (hex: unknown): number => (typeof hex === 'string' && hex.length >= 8 ? parseInt(hex.slice(6, 8), 16) / 255 : 1)
const channelsOf = (hex: unknown): number[] =>
  typeof hex === 'string' ? [0, 2, 4, 6].map((i) => (hex.length >= i + 2 ? parseInt(hex.slice(i, i + 2), 16) / 255 : 1)) : [1, 1, 1, 1]

/**
 * How a key tweens into the next. Spine 4 writes a bezier per value with absolute times and values; it's normalized here
 * by the value that changes most, since the runtime eases a key's values together. Spine 3 writes one normalized bezier.
 */
function easeOf(c: Ctx, f: J, next: J | undefined, raw: (f: J) => number[], only?: number[]): Ease | null {
  const curve = f.curve
  if (curve === 'stepped') return null
  if (!next || curve == null) return { kind: 'linear' }
  if (typeof curve === 'number') return { kind: 'curve', c: [curve, num(f.c2, 0), num(f.c3, 1), num(f.c4, 1)] }
  if (!Array.isArray(curve)) return { kind: 'linear' }
  const cs = curve.map((v: unknown) => num(v, 0))
  if (!c.v4) return cs.length >= 4 ? { kind: 'curve', c: cs.slice(0, 4) } : { kind: 'linear' }
  const [t0, t1] = [num(f.time, 0), num(next.time, 0)]
  const [a, b] = [raw(f), raw(next)]
  let best = -1
  for (const k of only ?? a.map((_, i) => i)) if (cs.length >= k * 4 + 4 && Math.abs(b[k] - a[k]) > (best < 0 ? 1e-9 : Math.abs(b[best] - a[best]))) best = k
  if (best < 0 || t1 <= t0) return { kind: 'linear' }
  const [v0, dv] = [a[best], b[best] - a[best]]
  const q = cs.slice(best * 4, best * 4 + 4)
  return { kind: 'curve', c: [(q[0] - t0) / (t1 - t0), (q[1] - v0) / dv, (q[2] - t0) / (t1 - t0), (q[3] - v0) / dv] }
}

/** Keys at their times (in frames), each lasting until the next. */
function keys<T>(c: Ctx, raw: J, value: (f: J) => T, curve: ((f: J) => number[]) | null, only?: number[]): Frame<T>[] {
  const fs = list(raw)
  return fs.map((f: J, i: number) => {
    const next = fs[i + 1]
    const start = num(f?.time, 0) * c.fps
    return {
      start,
      dur: next ? Math.max(0, num(next.time, 0) * c.fps - start) : 0,
      ease: curve ? easeOf(c, f, next, curve, only) : null,
      v: value(f),
    }
  })
}

const translate2: Channel['apply'] = (p, v) => {
  p.x += v[0]
  p.y += v[1]
}
const scale2: Channel['apply'] = (p, v) => {
  p.sx *= v[0]
  p.sy *= v[1]
}
const shearX: Channel['apply'] = (p, v) => {
  p.rot -= v[0]
  p.skew += v[0]
}

function boneChannels(c: Ctx, raw: J): Channel[] {
  const out: Channel[] = []
  for (const [type, fs] of entries(raw)) {
    const one = (k: string, fallback: number) => (f: J) => [num(f[k], fallback)]
    const two = (fallback: number) => (f: J) => [num(f.x, fallback), num(f.y, fallback)]
    if (type === 'rotate') {
      const angle = (f: J) => [num(f.value ?? f.angle, 0)]
      let values = list(fs).map((f) => angle(f)[0])
      // Spine 3 turns the short way between keys.
      if (!c.v4) values = values.reduce<number[]>((acc, v, i) => [...acc, i ? acc[i - 1] + wrap((v - acc[i - 1]) * DEG) / DEG : v], [])
      const at = new Map(list(fs).map((f, i) => [f, values[i]]))
      out.push({ fs: keys(c, fs, (f) => [-(at.get(f) ?? 0) * DEG], angle), apply: (p, v) => (p.rot += v[0]) })
    } else if (type === 'translate') {
      out.push({ fs: keys(c, fs, (f) => [num(f.x, 0), -num(f.y, 0)], two(0)), apply: translate2 })
    } else if (type === 'translatex') {
      out.push({ fs: keys(c, fs, one('value', 0), one('value', 0)), apply: (p, v) => (p.x += v[0]) })
    } else if (type === 'translatey') {
      out.push({ fs: keys(c, fs, (f) => [-num(f.value, 0)], one('value', 0)), apply: (p, v) => (p.y += v[0]) })
    } else if (type === 'scale') {
      out.push({ fs: keys(c, fs, two(1), two(1)), apply: scale2 })
    } else if (type === 'scalex') {
      out.push({ fs: keys(c, fs, one('value', 1), one('value', 1)), apply: (p, v) => (p.sx *= v[0]) })
    } else if (type === 'scaley') {
      out.push({ fs: keys(c, fs, one('value', 1), one('value', 1)), apply: (p, v) => (p.sy *= v[0]) })
    } else if (type === 'shear') {
      out.push({
        fs: keys(c, fs, (f) => [num(f.x, 0) * DEG, num(f.y, 0) * DEG], two(0)),
        apply: (p, v) => {
          p.rot -= v[0]
          p.skew += v[0] - v[1]
        },
      })
    } else if (type === 'shearx') {
      out.push({ fs: keys(c, fs, (f) => [num(f.value, 0) * DEG], one('value', 0)), apply: shearX })
    } else if (type === 'sheary') {
      out.push({ fs: keys(c, fs, (f) => [num(f.value, 0) * DEG], one('value', 0)), apply: (p, v) => (p.skew -= v[0]) })
    }
  }
  return out
}

function slotTimeline(c: Ctx, raw: J, attachments: string[]): SlotTimeline {
  const index = (name: unknown) => (typeof name === 'string' ? attachments.indexOf(name) : -1)
  const tl: SlotTimeline = { display: [], alpha: [] }
  for (const [type, fs] of entries(raw)) {
    if (type === 'attachment') tl.display = keys(c, fs, (f) => index(f.name), null)
    else if (type === 'rgba' || type === 'color') tl.alpha = keys(c, fs, (f) => alphaOf(f.color), (f) => channelsOf(f.color), [3])
    else if (type === 'rgba2' || type === 'twoColor') tl.alpha = keys(c, fs, (f) => alphaOf(f.light), (f) => channelsOf(f.light), [3])
    else if (type === 'alpha') tl.alpha = keys(c, fs, (f) => num(f.value, 1), (f) => [num(f.value, 1)])
  }
  return tl
}

/** A mesh's points: in the slot's space, or (weighted) in each bone's, as `[count, (bone, x, y, weight)…]` per vertex. */
function meshOf(vertices: number[], count: number, uvs: number[], triangles: number[], bones: number): Mesh {
  const mesh: Mesh = { vertices: [], uvs, triangles, weighted: null, deformPerBone: false, slotPose: IDENTITY, bonePoses: new Map() }
  if (vertices.length === count * 2) {
    for (let i = 0; i + 1 < vertices.length; i += 2) mesh.vertices.push(vertices[i], -vertices[i + 1])
    return mesh
  }
  const weighted: Weighted[] = []
  for (let at = 0, v = 0; v < count && at < vertices.length; v++) {
    const n = vertices[at++]
    const w: Weighted = { bones: [], weights: [], local: [] }
    for (let k = 0; k < n; k++, at += 4) {
      // Every influence counts for the deform offsets, even one on a bone that isn't there.
      w.bones.push(Math.min(Math.max(0, vertices[at]), bones - 1))
      w.local.push(vertices[at + 1], -vertices[at + 2])
      w.weights.push(vertices[at + 3])
    }
    weighted.push(w)
  }
  return { ...mesh, weighted, deformPerBone: true }
}

const numbers = (v: J): number[] => list(v).map((x: unknown) => num(x, 0))

type Skin = { name: string; slots: Map<string, Map<string, J>> }

function displayOf(c: Ctx, key: string, att: J, skin: Skin, fallback: Skin | null, slotName: string): Display {
  const type = String(att?.type ?? 'region')
  const name = String(att?.name ?? key)
  let texture = String(att?.path ?? name)
  const seq = att?.sequence
  if (seq) texture += String(num(seq.start, 1) + num(seq.setup, 0)).padStart(num(seq.digits, 0), '0')
  const base: Display = { ...NO_DISPLAY, name: key, texture }
  const bones = c.bones.size
  if (type === 'region') {
    const [w, h] = [num(att.width, 0), num(att.height, 0)]
    return {
      ...base,
      type: 'image',
      matrix: toMatrix({ x: num(att.x, 0), y: -num(att.y, 0), rot: -num(att.rotation, 0) * DEG, skew: 0, sx: num(att.scaleX, 1), sy: num(att.scaleY, 1) }),
      size: w > 0 && h > 0 ? [w, h] : null,
    }
  }
  if (type === 'mesh' || type === 'skinnedmesh' || type === 'weightedmesh') {
    const uvs = numbers(att.uvs)
    return { ...base, type: 'mesh', mesh: meshOf(numbers(att.vertices), uvs.length / 2, uvs, numbers(att.triangles), bones) }
  }
  if (type === 'linkedmesh') {
    const from = att.skin ? null : skin
    const parentSkin = from ?? (att.skin === fallback?.name ? fallback : null)
    const parentKey = String(att.parent ?? '')
    const parent = parentSkin?.slots.get(slotName)?.get(parentKey) ?? fallback?.slots.get(slotName)?.get(parentKey)
    if (!parent || parent.type === 'linkedmesh') return base
    const mesh = displayOf(c, parentKey, parent, skin, fallback, slotName)
    // Its deform keys are its parent's, unless it says otherwise.
    const inherits = att.timelines ?? att.deform ?? true
    return { ...mesh, name: inherits ? parentKey : key, texture }
  }
  if (type === 'clipping') {
    const end = c.slots.get(String(att.end)) ?? -1
    return { ...base, type: 'clip', mesh: meshOf(numbers(att.vertices), num(att.vertexCount, 0), [], [], bones), clipEnd: end }
  }
  return base
}

function constraintTimelines(c: Ctx, anim: J, names: Map<string, number>): Map<number, ConstraintTimeline> {
  const out = new Map<number, ConstraintTimeline>()
  for (const [name, fs] of entries(anim.ik)) {
    const i = names.get(`ik:${name}`)
    if (i === undefined) continue
    out.set(i, {
      mix: keys(c, fs, (f) => [num(f.mix, 1)], (f) => [num(f.mix, 1)]),
      // Turned over with the y axis.
      bend: keys(c, fs, (f) => (f.bendPositive === false ? 1 : 0), null),
    })
  }
  for (const [name, fs] of entries(anim.transform)) {
    const i = names.get(`transform:${name}`)
    if (i === undefined) continue
    const mix = (f: J) => {
      if (!c.v4) {
        const [r, t, s, sh] = [num(f.rotateMix, 1), num(f.translateMix, 1), num(f.scaleMix, 1), num(f.shearMix, 1)]
        return [r, t, t, s, s, sh]
      }
      const x = num(f.mixX, 1)
      const sx = num(f.mixScaleX, 1)
      return [num(f.mixRotate, 1), x, num(f.mixY, x), sx, num(f.mixScaleY, sx), num(f.mixShearY, 1)]
    }
    out.set(i, { mix: keys(c, fs, mix, mix), bend: [] })
  }
  return out
}

function readAnimation(c: Ctx, name: string, raw: J, attachments: string[][], skins: string[], names: Map<string, number>): Anim {
  let last = 0
  const track = <T>(fs: Frame<T>[]) => {
    for (const f of fs) last = Math.max(last, f.start)
    return fs
  }
  const bones = new Map<number, Channel[]>()
  for (const [bone, tl] of entries(raw.bones)) {
    const i = c.bones.get(bone)
    if (i === undefined) continue
    const channels = boneChannels(c, tl)
    channels.forEach((ch) => track(ch.fs))
    bones.set(i, channels)
  }
  const slotTls = new Map<number, SlotTimeline>()
  for (const [slot, tl] of entries(raw.slots)) {
    const i = c.slots.get(slot)
    if (i === undefined) continue
    const s = slotTimeline(c, tl, attachments[i])
    track(s.display)
    track(s.alpha)
    slotTls.set(i, s)
  }
  // Deform keys, per skin, slot and attachment: under `attachments` (4.x, as `deform`), `deform` (3.8) or `ffd`.
  const deform: DeformTimeline[] = []
  const v4 = entries(raw.attachments).map(([skin, bySlot]) => [skin, entries(bySlot).map(([slot, byAtt]) => [slot, entries(byAtt).map(([att, t]) => [att, t?.deform])])] as const)
  const older = [...entries(raw.deform), ...entries(raw.ffd)].map(([skin, bySlot]) => [skin, entries(bySlot).map(([slot, byAtt]) => [slot, entries(byAtt)])] as const)
  for (const [skin, bySlot] of [...v4, ...older] as [string, [string, [string, J][]][]][]) {
    if (!skins.includes(skin)) continue
    for (const [slot, byAtt] of bySlot) {
      const i = c.slots.get(slot)
      if (i === undefined) continue
      for (const [att, fs] of byAtt) {
        const length = Math.max(0, ...list(fs).map((f) => num(f?.offset, 0) + list(f?.vertices).length))
        const frames = keys(
          c,
          fs,
          (f) => {
            const out = new Array<number>(length).fill(0)
            const offset = Math.max(0, Math.floor(num(f.offset, 0)))
            list(f.vertices).forEach((v, j) => (out[offset + j] = (offset + j) % 2 ? -num(v, 0) : num(v, 0)))
            return out
          },
          () => [0],
        )
        // A deform's curve is over its own 0..1, so no value is needed to normalize it.
        for (let k = 0; k < frames.length; k++) {
          const f = list(fs)[k]
          if (c.v4 && Array.isArray(f?.curve) && frames[k].dur > 0) {
            const [t0, t1] = [num(f.time, 0), num(list(fs)[k + 1]?.time, 0)]
            const q = f.curve.map((v: unknown) => num(v, 0))
            frames[k].ease = { kind: 'curve', c: [(q[0] - t0) / (t1 - t0), q[1], (q[2] - t0) / (t1 - t0), q[3]] }
          }
        }
        deform.push({ slot: i, display: att, frames: track(frames) })
      }
    }
  }
  const zOrder = track(
    keys(
      c,
      raw.drawOrder ?? raw.draworder,
      (f) =>
        list(f.offsets)
          .map((o): [number, number] => [c.slots.get(String(o?.slot)) ?? -1, num(o?.offset, 0)])
          .filter(([s]) => s >= 0)
          .sort((a, b) => a[0] - b[0])
          .flat(),
      null,
    ),
  )
  const constraintTls = constraintTimelines(c, raw, names)
  for (const t of constraintTls.values()) {
    track(t.mix)
    track(t.bend)
  }
  return { name, duration: last, bones, slots: slotTls, zOrder, deform, constraints: constraintTls }
}

/** A Spine skeleton, read from its JSON: an armature per skin (each over the default skin), so the picker shows them. */
export function readSpine(doc: J): Skeleton {
  const info = doc?.skeleton ?? {}
  const rawBones = list(doc?.bones)
  if (!rawBones.length) throw new Error('no bones')
  const c: Ctx = {
    v4: parseInt(String(info.spine ?? '4'), 10) >= 4,
    fps: num(info.fps, 0) > 0 ? info.fps : 30,
    bones: new Map(rawBones.map((b, i) => [String(b?.name), i])),
    slots: new Map(list(doc.slots).map((s, i) => [String(s?.name), i])),
  }
  const bones: BoneDef[] = rawBones.map((b) => {
    const inherit = String(b?.inherit ?? b?.transform ?? 'normal')
    const [shx, shy] = [num(b?.shearX, 0), num(b?.shearY, 0)]
    return {
      name: String(b?.name ?? ''),
      parent: b?.parent != null ? (c.bones.get(String(b.parent)) ?? -1) : -1,
      length: num(b?.length, 0),
      origin: { x: num(b?.x, 0), y: -num(b?.y, 0), rot: -(num(b?.rotation, 0) + shx) * DEG, skew: (shx - shy) * DEG, sx: num(b?.scaleX, 1), sy: num(b?.scaleY, 1) },
      inheritRotation: b?.inheritRotation !== false && inherit !== 'onlyTranslation' && inherit !== 'noRotationOrReflection',
      inheritScale: b?.inheritScale !== false && inherit !== 'onlyTranslation' && inherit !== 'noScale' && inherit !== 'noScaleOrReflection',
      inheritTranslation: true,
    }
  })
  const skins: Skin[] = (Array.isArray(doc.skins) ? doc.skins.map((s: J) => [String(s?.name ?? ''), s?.attachments]) : entries(doc.skins)).map(
    ([name, atts]: [string, J]) => ({ name, slots: new Map(entries(atts).map(([slot, byKey]) => [slot, new Map(entries(byKey))])) }),
  )
  const fallback = skins.find((s) => s.name === 'default') ?? null
  const others = skins.filter((s) => s !== fallback)
  const constraints: Constraint[] = []
  const names = new Map<string, number>()
  const ordered = [
    ...list(doc.ik).map((k) => ({ kind: 'ik' as const, k })),
    ...list(doc.transform).map((k) => ({ kind: 'transform' as const, k })),
  ].sort((a, b) => num(a.k?.order, 0) - num(b.k?.order, 0))
  for (const { kind, k } of ordered) {
    const target = c.bones.get(String(k?.target)) ?? -1
    const chain = list(k?.bones).map((b) => c.bones.get(String(b)) ?? -1)
    if (target < 0 || !chain.length || chain.some((b) => b < 0)) continue
    names.set(`${kind}:${k.name}`, constraints.length)
    if (kind === 'ik') {
      // Turned over with the y axis: Spine's bend one way is the runtime's the other.
      constraints.push({ kind: 'ik', bone: chain[chain.length - 1], target, bendPositive: k.bendPositive === false, chain: chain.length - 1, weight: num(k.mix, 1) })
    } else {
      const x = num(k.mixX, num(k.translateMix, 1))
      const sx = num(k.mixScaleX, num(k.scaleMix, 1))
      constraints.push({
        kind: 'transform',
        bones: chain,
        target,
        offset: [-num(k.rotation, 0) * DEG, num(k.x, 0), -num(k.y, 0), num(k.scaleX, 0), num(k.scaleY, 0), -num(k.shearY, 0) * DEG],
        mix: [num(k.mixRotate, num(k.rotateMix, 1)), x, num(k.mixY, x), sx, num(k.mixScaleY, sx), num(k.mixShearY, num(k.shearMix, 1))],
      })
    }
  }
  const animationNames = Object.keys(doc.animations ?? {}).sort()
  const first = STARTERS.find((s) => animationNames.includes(s)) ?? animationNames[0] ?? ''
  const box: [number, number, number, number] | null =
    num(info.width, 0) > 0 && num(info.height, 0) > 0 ? [num(info.x, 0), -(num(info.y, 0) + info.height), info.width, info.height] : null
  const armatures: ArmatureDef[] = (others.length ? others : [fallback ?? { name: 'default', slots: new Map<string, Map<string, J>>() }]).map((skin) => {
    const attachments: string[][] = []
    const slots: SlotDef[] = list(doc.slots).map((s) => {
      const slotName = String(s?.name ?? '')
      const own = skin.slots.get(slotName)
      const base = skin === fallback ? undefined : fallback?.slots.get(slotName)
      const atts = new Map<string, J>([...(base ?? []), ...(own ?? [])])
      const displays = [...atts].map(([key, att]) => displayOf(c, key, att, skin, fallback, slotName))
      attachments.push([...atts.keys()])
      return {
        name: slotName,
        bone: c.bones.get(String(s?.bone)) ?? 0,
        displayIndex: typeof s?.attachment === 'string' ? [...atts.keys()].indexOf(s.attachment) : -1,
        alpha: alphaOf(s?.color),
        blend: BLENDS[String(s?.blend)] ?? 'source-over',
        displays,
      }
    })
    const skinNames = [skin.name, ...(fallback && fallback !== skin ? [fallback.name] : [])]
    const animations = animationNames.map((n) => readAnimation(c, n, doc.animations[n], attachments, skinNames, names))
    return { name: skin.name, fps: c.fps, bones, slots, constraints, animations, first, aabb: box }
  })
  return { armatures, regions: new Map() }
}

// ---- The atlas ----

type AtlasRegion = { name: string; x: number; y: number; w: number; h: number; rotate: boolean; offX: number; offY: number; origW: number; origH: number }
type AtlasPage = { file: string; pma: boolean; scale: number; regions: AtlasRegion[] }

/** A libGDX texture atlas, old (`xy`, `size`, `orig`, `offset`) or new (`bounds`, `offsets`): pages and their regions. */
export function parseAtlas(text: string): AtlasPage[] {
  const pages: AtlasPage[] = []
  let page: AtlasPage | null = null
  let region: AtlasRegion | null = null
  const nums = (v: string) => v.split(',').map((x) => parseFloat(x))
  for (const raw of text.split(/\r?\n/)) {
    const line = raw.trim()
    if (!line) {
      page = null
      region = null
      continue
    }
    const colon = line.indexOf(':')
    if (colon < 0) {
      if (!page) {
        page = { file: line, pma: false, scale: 1, regions: [] }
        pages.push(page)
        region = null
      } else {
        region = { name: line, x: 0, y: 0, w: 0, h: 0, rotate: false, offX: 0, offY: 0, origW: 0, origH: 0 }
        page.regions.push(region)
      }
      continue
    }
    if (!page) continue
    const key = line.slice(0, colon).trim()
    const value = line.slice(colon + 1).trim()
    const v = nums(value)
    if (!region) {
      if (key === 'pma') page.pma = value === 'true'
      else if (key === 'scale') page.scale = v[0] > 0 ? v[0] : 1
      continue
    }
    if (key === 'xy') [region.x, region.y] = v
    else if (key === 'size') [region.w, region.h] = v
    else if (key === 'bounds') [region.x, region.y, region.w, region.h] = v
    else if (key === 'orig') [region.origW, region.origH] = v
    else if (key === 'offset') [region.offX, region.offY] = v
    else if (key === 'offsets') [region.offX, region.offY, region.origW, region.origH] = v
    else if (key === 'rotate') region.rotate = value === 'true' || value === '90'
  }
  for (const p of pages) for (const r of p.regions) if (!(r.origW > 0 && r.origH > 0)) [r.origW, r.origH] = [r.w, r.h]
  return pages
}

/** A page whose colours are multiplied by their alpha, put back the way the canvas draws them. */
function unpremultiply(img: HTMLImageElement): CanvasImageSource {
  const c = document.createElement('canvas')
  ;[c.width, c.height] = [img.naturalWidth, img.naturalHeight]
  const ctx = c.getContext('2d', { willReadFrequently: true })
  if (!ctx) return img
  ctx.drawImage(img, 0, 0)
  const data = ctx.getImageData(0, 0, c.width, c.height)
  const d = data.data
  for (let i = 0; i < d.length; i += 4) {
    const a = d[i + 3]
    if (a > 0 && a < 255) for (let k = 0; k < 3; k++) d[i + k] = Math.min(255, Math.round((d[i + k] * 255) / a))
  }
  ctx.putImageData(data, 0, 0)
  return c
}

/** The atlas's pieces, by name. One packed turned or trimmed gets a canvas of its own, upright at its whole size. */
function addRegions(out: Map<string, Region>, page: AtlasPage, image: CanvasImageSource) {
  for (const r of page.regions) {
    if (out.has(r.name) || r.w <= 0 || r.h <= 0) continue
    const scale = 1 / page.scale
    if (!r.rotate && r.offX === 0 && r.offY === 0 && r.origW === r.w && r.origH === r.h) {
      out.set(r.name, { src: image, sx: r.x, sy: r.y, w: r.w, h: r.h, scale, frame: null })
      continue
    }
    const c = document.createElement('canvas')
    ;[c.width, c.height] = [Math.max(1, Math.round(r.origW)), Math.max(1, Math.round(r.origH))]
    const ctx = c.getContext('2d')
    if (!ctx) continue
    // Offsets are from the bottom left.
    const [left, top] = [r.offX, r.origH - r.offY - r.h]
    if (r.rotate) {
      // Packed a quarter turn anticlockwise, `h` wide and `w` tall: turned back clockwise.
      ctx.translate(left + r.w, top)
      ctx.rotate(Math.PI / 2)
      ctx.drawImage(image, r.x, r.y, r.h, r.w, 0, 0, r.h, r.w)
    } else {
      ctx.drawImage(image, r.x, r.y, r.w, r.h, left, top, r.w, r.h)
    }
    out.set(r.name, { src: c, sx: 0, sy: 0, w: c.width, h: c.height, scale, frame: null })
  }
}

/** The atlases a skeleton may be drawn from: the ones it links to, then by Spine's naming (`spineboy-pro.json` → `spineboy.atlas`). */
function atlasCandidates(a: Asset): string[] {
  const out = new Set((a.links ?? []).filter((l) => /\.atlas(\.txt)?$/i.test(l.path)).map((l) => l.path))
  const stem = a.path.replace(/\.json$/i, '')
  for (const s of [stem, stem.replace(/-(pro|ess)$/i, '')]) {
    out.add(`${s}.atlas`)
    out.add(`${s}.atlas.txt`)
  }
  return [...out]
}

/** The skeleton and the atlas pieces it's drawn with. */
export async function spineData(a: Asset): Promise<Skeleton> {
  const skeleton = readSpine(await getJson(fileUrl(a)))
  for (const path of atlasCandidates(a)) {
    try {
      const r = await fetch(packFileUrl(a.pack_id, path), { credentials: 'same-origin' })
      if (!r.ok) continue
      const dir = path.includes('/') ? path.slice(0, path.lastIndexOf('/')) : ''
      for (const page of parseAtlas(await r.text())) {
        const img = await loadImage(packFileUrl(a.pack_id, joinPath(dir, page.file)))
        addRegions(skeleton.regions, page, page.pma ? unpremultiply(img) : img)
      }
    } catch {
      // Drawn without that atlas's pieces.
    }
    if (skeleton.regions.size) break
  }
  if (!skeleton.regions.size) throw new Error('no atlas')
  return skeleton
}
