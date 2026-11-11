<script setup lang="ts">
// Interactive 3D preview. three.js with its WebGPU renderer (falls back to WebGL2 automatically).
// Models load from the pack's original file tree (/api/packs/:id/files/...), so relative references
// to .bin buffers, .mtl files and textures resolve exactly as they would on disk.
import { onBeforeUnmount, onMounted, ref, shallowRef, watch } from 'vue'
import type { Asset, AssetLink } from '../api'
import { packFileUrl } from '../api'

const props = defineProps<{ asset: Asset }>()
const host = ref<HTMLDivElement | null>(null)
const status = ref<'loading' | 'ready' | 'error'>('loading')
const error = ref('')
const clips = ref<string[]>([])
const activeClip = ref('')
const stats = ref('')

type Three = typeof import('three/webgpu')
let THREE: Three
let renderer: InstanceType<Three['WebGPURenderer']> | null = null
let mixer: InstanceType<Three['AnimationMixer']> | null = null
let resizeObs: ResizeObserver | null = null
let disposed = false
const clipMap = shallowRef(new Map<string, import('three').AnimationClip>())

export type Loaded = { object: import('three').Object3D; animations: import('three').AnimationClip[] }

async function loadModel(url: string, ext: string): Promise<Loaded> {
  switch (ext) {
    case 'glb':
    case 'gltf': {
      const { GLTFLoader } = await import('three/addons/loaders/GLTFLoader.js')
      const g = await new GLTFLoader().loadAsync(url)
      return { object: g.scene, animations: g.animations }
    }
    case 'fbx': {
      const { FBXLoader } = await import('three/addons/loaders/FBXLoader.js')
      const o = await new FBXLoader().loadAsync(url)
      return { object: o, animations: o.animations }
    }
    case 'obj': {
      const [{ OBJLoader }, { MTLLoader }] = await Promise.all([
        import('three/addons/loaders/OBJLoader.js'),
        import('three/addons/loaders/MTLLoader.js'),
      ])
      const text = await (await fetch(url)).text()
      const loader = new OBJLoader()
      const mtl = text.match(/^mtllib\s+(.+)$/m)?.[1]?.trim()
      if (mtl) {
        try {
          const base = url.slice(0, url.lastIndexOf('/') + 1)
          const materials = await new MTLLoader().setPath(base).loadAsync(mtl)
          materials.preload()
          loader.setMaterials(materials)
        } catch {
          // Missing MTL: fall back to default material.
        }
      }
      return { object: loader.parse(text), animations: [] }
    }
    case 'stl':
    case 'ply': {
      const mod = ext === 'stl' ? await import('three/addons/loaders/STLLoader.js') : await import('three/addons/loaders/PLYLoader.js')
      const Loader = 'STLLoader' in mod ? mod.STLLoader : (mod as typeof import('three/addons/loaders/PLYLoader.js')).PLYLoader
      const geom = await new Loader().loadAsync(url)
      geom.computeVertexNormals()
      return { object: new THREE.Mesh(geom, new THREE.MeshStandardMaterial({ color: 0xb8c0d0 })), animations: [] }
    }
    case 'dae': {
      const { ColladaLoader } = await import('three/addons/loaders/ColladaLoader.js')
      const c = await new ColladaLoader().loadAsync(url)
      if (!c) throw new Error('empty Collada file')
      return { object: c.scene, animations: c.scene.animations ?? [] }
    }
    default:
      throw new Error(`No browser loader for .${ext}`)
  }
}

function sourceFor(a: Asset | AssetLink, packId: string, derivatives: string[] = []): { url: string; ext: string } {
  if (derivatives.includes('glb')) return { url: `/api/assets/${a.id}/derivative/glb`, ext: 'glb' }
  const ext = a.path.split('.').pop()?.toLowerCase() ?? ''
  return { url: packFileUrl(packId, a.path), ext }
}

async function setup() {
  disposed = false
  status.value = 'loading'
  clips.value = []
  activeClip.value = ''
  stats.value = ''
  try {
    THREE = await import('three/webgpu')
    const { OrbitControls } = await import('three/addons/controls/OrbitControls.js')
    const el = host.value
    if (!el || disposed) return

    renderer = new THREE.WebGPURenderer({ antialias: true, alpha: true })
    await renderer.init()
    renderer.setPixelRatio(Math.min(devicePixelRatio, 2))
    renderer.setSize(el.clientWidth, el.clientHeight)
    el.appendChild(renderer.domElement)

    const scene = new THREE.Scene()
    scene.add(new THREE.HemisphereLight(0xdfe8ff, 0x3a3226, 2.2))
    const key = new THREE.DirectionalLight(0xffffff, 2.4)
    key.position.set(3, 5, 4)
    scene.add(key)
    const camera = new THREE.PerspectiveCamera(35, el.clientWidth / el.clientHeight, 0.01, 1000)
    const controls = new OrbitControls(camera, renderer.domElement)
    controls.enableDamping = true

    const src = sourceFor(props.asset, props.asset.pack_id, props.asset.derivatives)
    const { object, animations } = await loadModel(src.url, src.ext)
    if (disposed) return
    scene.add(object)

    // Frame the model from a 3/4 angle.
    const box = new THREE.Box3().setFromObject(object)
    const size = box.getSize(new THREE.Vector3()).length() || 1
    const center = box.getCenter(new THREE.Vector3())
    controls.target.copy(center)
    // Fit the bounding sphere in the vertical field of view, with a little margin.
    const dist = (size / 2 / Math.sin(((camera.fov / 2) * Math.PI) / 180)) * 1.15
    camera.position.copy(center).add(new THREE.Vector3(0.75, 0.55, 1).normalize().multiplyScalar(dist))
    camera.near = size / 100
    camera.far = size * 100
    camera.updateProjectionMatrix()

    let tris = 0
    object.traverse((o) => {
      const m = o as import('three').Mesh
      if (m.isMesh && m.geometry) tris += (m.geometry.index?.count ?? m.geometry.attributes.position?.count ?? 0) / 3
    })
    stats.value = `${Math.round(tris).toLocaleString()} triangles`

    // Clips embedded in the file plus linked clip files ("Knight@Run.fbx").
    const map = new Map<string, import('three').AnimationClip>()
    animations.forEach((c, i) => map.set(c.name || `clip ${i + 1}`, c))
    for (const link of props.asset.links.filter((l) => l.rel === 'animation')) {
      try {
        const s = sourceFor(link, props.asset.pack_id)
        const loaded = await loadModel(s.url, s.ext)
        const label = link.path.split('/').pop()!.replace(/\.[^.]+$/, '').split('@').pop()!
        loaded.animations.forEach((c, i) => map.set(loaded.animations.length > 1 ? `${label} ${i + 1}` : label, c))
      } catch {
        // A clip that won't load shouldn't break the preview.
      }
    }
    clipMap.value = map
    clips.value = [...map.keys()]
    if (clips.value.length) {
      mixer = new THREE.AnimationMixer(object)
      activeClip.value = clips.value[0]
    }

    const timer = new THREE.Timer()
    renderer.setAnimationLoop((time) => {
      timer.update(time)
      mixer?.update(timer.getDelta())
      controls.update()
      renderer?.render(scene, camera)
    })
    resizeObs = new ResizeObserver(() => {
      if (!renderer || !el.clientWidth) return
      renderer.setSize(el.clientWidth, el.clientHeight)
      camera.aspect = el.clientWidth / el.clientHeight
      camera.updateProjectionMatrix()
    })
    resizeObs.observe(el)
    status.value = 'ready'
  } catch (e) {
    status.value = 'error'
    error.value = (e as Error).message
  }
}

function teardown() {
  disposed = true
  resizeObs?.disconnect()
  resizeObs = null
  mixer?.stopAllAction()
  mixer = null
  if (renderer) {
    renderer.setAnimationLoop(null)
    renderer.domElement.remove()
    renderer.dispose()
    renderer = null
  }
}

watch(activeClip, (name) => {
  if (!mixer) return
  mixer.stopAllAction()
  const clip = clipMap.value.get(name)
  if (clip) mixer.clipAction(clip).play()
})

watch(() => props.asset.id, () => {
  teardown()
  setup()
})
onMounted(setup)
onBeforeUnmount(teardown)
</script>

<template>
  <div class="viewer">
    <div ref="host" class="canvas checker" />
    <p v-if="status === 'loading'" class="overlay">Loading model…</p>
    <p v-else-if="status === 'error'" class="overlay err">Couldn't preview this model: {{ error }}</p>
    <div v-if="status === 'ready'" class="hud">
      <span class="mono">{{ stats }}</span>
      <label v-if="clips.length">
        <span class="visually-hidden">Animation</span>
        <select v-model="activeClip" class="field">
          <option v-for="c in clips" :key="c" :value="c">{{ c }}</option>
        </select>
      </label>
      <span class="hint">Drag to orbit, scroll to zoom</span>
    </div>
  </div>
</template>

<style scoped>
.viewer { position: relative; }
.canvas {
  aspect-ratio: 4 / 3; border-radius: var(--r-tile); border: 1px solid var(--line); overflow: hidden;
  box-shadow: inset 0 3px 0 var(--k-model); cursor: grab;
}
.canvas :deep(canvas) { display: block; width: 100%; height: 100%; }
.overlay { position: absolute; inset: 0; display: grid; place-items: center; margin: 0; color: var(--muted); font-size: var(--step--1); padding: 1rem; text-align: center; pointer-events: none; }
.err { color: var(--danger); }
.hud { display: flex; flex-wrap: wrap; gap: 0.5rem 0.9rem; align-items: center; margin-top: 0.45rem; font-size: 0.78rem; color: var(--muted); }
.hud .field { width: auto; padding: 0.2em 0.4em; font-size: 0.78rem; }
.hint { margin-left: auto; color: var(--faint); }
</style>
