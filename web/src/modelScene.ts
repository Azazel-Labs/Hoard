// Sums over a loaded model's scene: where the thumbnail camera stands, the parts that never move apart merged
// into one mesh each, and a model's GPU resources freed.
import * as THREE from 'three/webgpu'
import { mergeGeometries } from 'three/addons/utils/BufferGeometryUtils.js'
import { MAP_SLOTS, type AnyMaterial } from './modelMaterials'

/** Where the thumbnail camera stands: looking from `dir`, close enough that the model's silhouette from there fills
 *  90% of the frame. The same sums as `camera` in hoard-render, so a browser's thumbnail and the server's match. */
export function snapshotView(object: THREE.Object3D, dir: THREE.Vector3) {
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
  // Every vertex where it stands now (skinned and morphed), about the box center, in the camera's axes. Big meshes
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

/** The attribute a merged mesh keeps each part's color in, for the geometry view. */
export const PART_COLOR = 'partColor'

/** Merge the meshes that never move apart and look the same (one material, the same attributes) into one mesh
 *  each: a model of thousands of parts otherwise takes thousands of draws a frame, which is what makes a big CAD
 *  model slow to turn, not its triangles. Each part's vertices are put where it stands, and its color in the
 *  geometry view goes in `PART_COLOR`. Returns the meshes left to draw. */
export function mergeStill(root: THREE.Object3D, meshes: THREE.Mesh[]): THREE.Mesh[] {
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
export function disposeObject(root: THREE.Object3D) {
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

/** A node's name as clips are matched by it across skeletons: without Mixamo's or Blender's prefix, lowercase, as the
 *  server matches them (`animations::bone`). Three's loaders drop the `:` from `mixamorig:Hips`. */
function boneKey(name: string): string {
  return (name.split(/[:|]/).pop() ?? name).replace(/^mixamorig\d*/i, '').trim().toLowerCase()
}

/** How long a skeleton's bones are, added up, in world units: the distance from each node to its parent, for the
 *  nodes in `nodes` whose parent is in it too. Turning a bone doesn't change it, so a posed skeleton measures the
 *  same as one at rest. */
function boneLength(nodes: Map<string, THREE.Object3D>): number {
  const at = (o: THREE.Object3D) => o.getWorldPosition(new THREE.Vector3())
  let sum = 0
  for (const o of nodes.values()) {
    if (o.parent && nodes.get(o.parent.name) === o.parent) sum += at(o).distanceTo(at(o.parent))
  }
  return sum
}

/** The size of one unit of a node's position in world units: its parent's world scale. */
function unitOf(o: THREE.Object3D): number {
  const s = o.parent ? o.parent.getWorldScale(new THREE.Vector3()) : new THREE.Vector3(1, 1, 1)
  return (Math.abs(s.x) + Math.abs(s.y) + Math.abs(s.z)) / 3
}

/** A clip from another file with its tracks renamed to `root`'s nodes: by name, else by bone name without its prefix,
 *  so an `Armature|Hips` clip moves a `mixamorig:Hips` character. A track for a node it lacks is left as it is.
 *
 *  Given the clip's own file (`from`), its position keys are put in `root`'s units and to its size too: a step of
 *  10 from an FBX in centimetres, under a node scaled by 0.01, is 10 cm on that character but would be 10 metres on
 *  one made in metres, and a step that's a fifth of one character's height stays a fifth of a smaller one's. */
export function bindClip(clip: THREE.AnimationClip, root: THREE.Object3D, from?: THREE.Object3D): THREE.AnimationClip {
  const exact = new Map<string, THREE.Object3D>()
  const loose = new Map<string, THREE.Object3D>()
  root.traverse((o) => {
    if (!o.name) return
    exact.set(o.name, o)
    const k = boneKey(o.name)
    if (k && !loose.has(k)) loose.set(k, o)
  })
  const counterpart = (name: string) => exact.get(name) ?? loose.get(boneKey(name))
  // The bones both skeletons have, measured each in its own file, for how much bigger `root`'s is.
  let ratio = 0
  const source = new Map<string, THREE.Object3D>()
  if (from) {
    const target = new Map<string, THREE.Object3D>()
    from.traverse((o) => {
      const to = o.name ? counterpart(o.name) : undefined
      if (to && !source.has(o.name)) {
        source.set(o.name, o)
        target.set(to.name, to)
      }
    })
    from.updateMatrixWorld(true)
    root.updateMatrixWorld(true)
    const [a, b] = [boneLength(source), boneLength(target)]
    if (a > 1e-9 && b > 1e-9) ratio = b / a
  }
  let changed = false
  const tracks = clip.tracks.map((t) => {
    let node = ''
    try {
      node = THREE.PropertyBinding.parseTrackName(t.name).nodeName ?? ''
    } catch {
      return t
    }
    const toNode = !node || !t.name.startsWith(node) ? undefined : counterpart(node)
    const to = toNode && toNode.name !== node ? toNode.name : undefined
    // How much a position key grows going across: the size ratio, from the clip's units to `root`'s.
    const src = source.get(node)
    let k = ratio && src && toNode && t.name.endsWith('.position') ? (ratio * unitOf(src)) / unitOf(toNode) : 1
    if (!Number.isFinite(k) || Math.abs(k - 1) < 1e-3) k = 1
    if (!to && k === 1) return t
    changed = true
    const bound = t.clone()
    if (to) bound.name = to + t.name.slice(node.length)
    if (k !== 1) bound.values = bound.values.map((v) => v * k)
    return bound
  })
  return changed ? new THREE.AnimationClip(clip.name, clip.duration, tracks, clip.blendMode) : clip
}
