// Thumbnails drawn by this browser. The 3D viewer draws a model with its textures, which the server's renderer can't
// always find or read; once it has, the picture it makes replaces the server's thumbnail. The server keeps the best
// one it's been given: a drawing with every texture beats one with some missing, which beats its own.
import { reactive } from 'vue'
import { api, type Asset, type ThumbSource, type ThumbUpload } from './api'
import { auth } from './auth'
import { isModelAsset } from './viewers'
// Types only: three.js loads with the viewer, not with this module.
import type { ModelStage } from './modelStage'

/** How long to wait for a model's textures before drawing it with what has arrived. */
const TEXTURE_WAIT_MS = 30_000

/** Only signed-in accounts may change thumbnails. */
export function canDrawThumbs(): boolean {
  return !!auth.status?.signed_in
}

/** Whether a drawing from this browser could improve on the thumbnail the asset has. */
export function wantsBrowserThumb(a: Asset): boolean {
  return canDrawThumbs() && isModelAsset(a) && a.thumb_source !== 'browser'
}

/**
 * Wait for the stage's textures (up to half a minute), draw the model and send the picture. Resolves with the server's
 * answer, or null when the drawing wouldn't improve on what the asset has (some textures missing again).
 */
export async function drawThumb(stage: ModelStage, a: Asset): Promise<ThumbUpload | null> {
  let timer = 0
  const settled = await Promise.race([
    stage.texturesSettled().then(() => true),
    new Promise<boolean>((resolve) => (timer = window.setTimeout(() => resolve(false), TEXTURE_WAIT_MS))),
  ])
  clearTimeout(timer)
  const source: ThumbSource = settled && !stage.missingTextures.length ? 'browser' : 'browser-partial'
  if (source === 'browser-partial' && a.thumb_source === 'browser-partial') return null
  const png = await stage.snapshot(256)
  return api.putThumb(a.id, png, source)
}

/** The asset with a new thumbnail's details, for lists to show it. */
export function withThumb(a: Asset, r: ThumbUpload): Asset {
  return r.replaced ? { ...a, thumb_source: r.thumb_source, thumb_rev: r.thumb_rev ?? a.thumb_rev, has_thumb: true } : a
}

/** "Make thumbnails in this browser" for a pack: one run at a time, shown wherever the pack list is. */
export const thumbRun = reactive({
  packId: '',
  packName: '',
  done: 0,
  total: 0,
  improved: 0,
  failed: 0,
  /** The model being drawn. */
  current: '',
  running: false,
  error: '',
})
let cancelled = false

export function cancelThumbRun() {
  cancelled = true
}

async function packModels(packId: string): Promise<Asset[]> {
  const out: Asset[] = []
  for (const kind of ['model', 'animation']) {
    for (let offset = 0; ; ) {
      const r = await api.search({ pack: packId, kind, limit: 500, offset, sort: 'name' })
      out.push(...r.items)
      offset += r.items.length
      if (!r.items.length || offset >= r.total) break
    }
  }
  return out.filter(wantsBrowserThumb)
}

/**
 * Draw every model in a pack that hasn't a full browser thumbnail yet, one at a time, in a viewer kept out of sight.
 * `cancelThumbRun` stops it after the model being drawn.
 */
export async function makePackThumbs(packId: string, packName: string): Promise<void> {
  if (thumbRun.running) return
  Object.assign(thumbRun, { packId, packName, done: 0, total: 0, improved: 0, failed: 0, current: '', running: true, error: '' })
  cancelled = false
  // A fixed size, off the page: the stage renders to it like any viewer.
  const host = document.createElement('div')
  host.setAttribute('aria-hidden', 'true')
  host.style.cssText = 'position:fixed;left:-10000px;top:0;width:512px;height:512px;pointer-events:none'
  document.body.appendChild(host)
  try {
    const models = await packModels(packId)
    thumbRun.total = models.length
    const { ModelStage } = await import('./modelStage')
    for (const a of models) {
      if (cancelled) break
      thumbRun.current = a.name
      const stage = new ModelStage(host)
      try {
        await stage.load(a)
        stage.setPaused(true)
        const r = await drawThumb(stage, a)
        if (r?.replaced) thumbRun.improved++
      } catch {
        thumbRun.failed++
      } finally {
        stage.dispose()
        host.replaceChildren()
      }
      thumbRun.done++
    }
  } catch (e) {
    thumbRun.error = (e as Error).message
  } finally {
    host.remove()
    thumbRun.current = ''
    thumbRun.running = false
  }
}
