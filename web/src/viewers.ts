// Which full-screen viewer suits an asset, and a way for the grid to ask the detail panel to open one.
import { reactive } from 'vue'
import { VIEWABLE_EXTS, type Asset } from './api'

export type ViewerKind = 'image' | 'model' | 'text'

/** Files that read as text, by extension. The server's kinds are too coarse (`data` covers binary `.uasset`, `document`
 *  covers PDF), so this is what decides; a file that turns out to be binary says so in the viewer. */
export const TEXT_EXTS = new Set([
  // Data and config.
  'json', 'jsonc', 'json5', 'ldtk', 'xml', 'yaml', 'yml', 'toml', 'ini', 'cfg', 'conf', 'properties', 'env',
  'csv', 'tsv', 'tmx', 'tsx', 'tx', 'atlas', 'plist', 'fnt',
  // Engine text formats: Godot scenes and resources, Unity's YAML-serialized assets.
  'tscn', 'tres', 'godot', 'import', 'prefab', 'unity', 'asset', 'mat', 'controller', 'overridecontroller',
  'physicmaterial', 'mask', 'meta', 'anim',
  // Shaders.
  'glsl', 'hlsl', 'wgsl', 'shader', 'vert', 'frag', 'comp', 'geom', 'tesc', 'tese', 'gdshader', 'usf', 'ush',
  'metal', 'cginc', 'fx', 'fxh', 'compute',
  // Documents and code that ships in packs.
  'txt', 'md', 'markdown', 'log', 'nfo', 'readme', 'license', 'html', 'htm', 'mtl', 'gltf',
  'lua', 'js', 'ts', 'cs', 'gd', 'py', 'c', 'h', 'cpp', 'hpp', 'sh', 'bat', 'ps1', 'cmake',
])

export function isModelAsset(a: Asset): boolean {
  return (a.kind === 'model' || a.kind === 'animation') && (VIEWABLE_EXTS.has(a.ext) || a.derivatives.includes('glb'))
}

export function isTextAsset(a: Asset): boolean {
  return TEXT_EXTS.has(a.ext.toLowerCase())
}

/** The viewer a double-click opens, or null when the side panel is all there is. */
export function viewerFor(a: Asset): ViewerKind | null {
  if (a.kind === 'texture' && a.has_thumb) return 'image'
  // A .gltf is JSON too, but the model viewer is what anyone wants.
  if (isModelAsset(a)) return 'model'
  if (isTextAsset(a)) return 'text'
  return null
}

/** A double-clicked tile asks for its asset full screen. The detail panel owns the viewers, and it may not have the
 *  asset yet (the first click is still opening it), so the request waits here until the panel shows that asset. */
export const viewRequest = reactive({ id: '', at: 0 })

export function requestView(id: string) {
  viewRequest.id = id
  viewRequest.at = Date.now()
}

/** Take the request if it's for this asset and recent: a stale one mustn't pop a viewer open minutes later. */
export function takeViewRequest(id: string): boolean {
  if (viewRequest.id !== id) return false
  const fresh = Date.now() - viewRequest.at < 5000
  viewRequest.id = ''
  return fresh
}

/** A double-click on a tile can't be a dblclick listener on the tile: its first click opens the side panel, the grid
 *  narrows and reflows, and the second click lands on whatever moved under the pointer (another tile, or the gap
 *  between them). So the first click is remembered by position, and a second click at the same spot soon after opens
 *  that tile's viewer, whatever is under it now. */
const DOUBLE_MS = 500
const DOUBLE_PX = 6
let lastTile = { id: '', x: 0, y: 0, at: 0 }

export function noteTileClick(id: string, e: MouseEvent) {
  // A keyboard "click" has no position and isn't half of a double-click.
  lastTile = e.detail === 1 ? { id, x: e.clientX, y: e.clientY, at: e.timeStamp } : { id: '', x: 0, y: 0, at: 0 }
}

function secondOf(e: MouseEvent): string {
  if (e.detail !== 2 || !lastTile.id || e.button !== 0) return ''
  if (e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return ''
  if (e.timeStamp - lastTile.at > DOUBLE_MS) return ''
  if (Math.abs(e.clientX - lastTile.x) > DOUBLE_PX || Math.abs(e.clientY - lastTile.y) > DOUBLE_PX) return ''
  return lastTile.id
}

if (typeof document !== 'undefined') {
  // Capture phase, so the second click never reaches what's under the pointer now: it would open a different asset.
  // Its mousedown is cancelled too, or the browser selects the word under it.
  document.addEventListener('mousedown', (e) => { if (secondOf(e)) e.preventDefault() }, true)
  document.addEventListener(
    'click',
    (e) => {
      const id = secondOf(e)
      if (!id) return
      lastTile.id = ''
      e.preventDefault()
      e.stopPropagation()
      requestView(id)
    },
    true,
  )
}
