// Rive files (.riv), played with Rive's web runtime and its canvas renderer. The runtime's WebAssembly is served
// with the app rather than fetched from Rive's CDN, and so are the files: assets a file leaves on Rive's CDN aren't
// fetched either. The server can't draw these, so the browser makes their thumbnails.
import { Alignment, Fit, Layout, Rive, RuntimeLoader } from '@rive-app/canvas'
import wasmUrl from '@rive-app/canvas/rive.wasm?url'
import { api, fileUrl, type Asset, type ThumbUpload } from './api'
import { coverage } from './browserThumbs'

RuntimeLoader.setWasmUrl(wasmUrl)
RuntimeLoader.setWasmFallbackUrl(null)

/** How long to wait for the runtime and the file before giving up on it. */
const LOAD_WAIT_MS = 15_000

/** What plays: an artboard (the file's default when unnamed), and one of its state machines or animations. */
export type RivePick = { artboard?: string; stateMachine?: string; animation?: string }

/** The file's artboards, with their state machines and animations. */
export type RiveContents = { name: string; stateMachines: string[]; animations: string[] }[]

export async function riveBytes(a: Asset): Promise<ArrayBuffer> {
  const res = await fetch(fileUrl(a))
  if (!res.ok) throw new Error(`HTTP ${res.status}`)
  return res.arrayBuffer()
}

/** Play `bytes` on `canvas`. Resolves once the file is loaded and playing (or paused, without `autoplay`). */
export function mountRive(
  canvas: HTMLCanvasElement,
  bytes: ArrayBuffer,
  pick: RivePick,
  opts: { autoplay: boolean; interactive: boolean },
): Promise<Rive> {
  return new Promise((resolve, reject) => {
    let r: Rive | null = null
    const timer = window.setTimeout(() => fail(new Error('timed out')), LOAD_WAIT_MS)
    const fail = (e: Error) => {
      clearTimeout(timer)
      r?.cleanup()
      reject(e)
    }
    try {
      r = new Rive({
        canvas,
        // The runtime holds on to what it's given, so each player gets its own copy.
        buffer: bytes.slice(0),
        artboard: pick.artboard,
        stateMachines: pick.stateMachine,
        animations: pick.animation,
        autoplay: opts.autoplay,
        layout: new Layout({ fit: Fit.Contain, alignment: Alignment.Center }),
        enableRiveAssetCDN: false,
        shouldDisableRiveListeners: !opts.interactive,
        onLoad: () => {
          clearTimeout(timer)
          resolve(r!)
        },
        onLoadError: () => fail(new Error('load error')),
      })
    } catch (e) {
      fail(e instanceof Error ? e : new Error(String(e)))
    }
  })
}

export function riveContents(r: Rive): RiveContents {
  return (r.contents.artboards ?? []).map((b) => ({
    name: b.name,
    stateMachines: b.stateMachines.map((s) => s.name),
    animations: b.animations,
  }))
}

/** Draw the file's default artboard 256 px across, a moment into playing, and send it as the asset's thumbnail. */
export async function drawRiveThumb(a: Asset, bytes?: ArrayBuffer): Promise<ThumbUpload | null> {
  bytes ??= await riveBytes(a)
  const w = Number(a.meta.width) || 256
  const h = Number(a.meta.height) || 256
  const s = 256 / Math.max(w, h)
  const cw = Math.max(1, Math.round(w * s))
  const ch = Math.max(1, Math.round(h * s))
  // Off the page but with a size: the runtime skips drawing on a canvas that has none.
  const canvas = document.createElement('canvas')
  canvas.setAttribute('aria-hidden', 'true')
  canvas.width = cw
  canvas.height = ch
  canvas.style.cssText = `position:fixed;left:-10000px;top:0;pointer-events:none;width:${cw}px;height:${ch}px`
  document.body.appendChild(canvas)
  try {
    const r = await mountRive(canvas, bytes, {}, { autoplay: true, interactive: false })
    try {
      r.resizeDrawingSurfaceToCanvas(1)
      // Many files fade or slide in: give it half a second of playing. (A hidden page doesn't draw at all, and an empty
      // picture isn't sent.)
      await new Promise((resolve) => window.setTimeout(resolve, 500))
      r.pause()
      if (!coverage(canvas)) return null
      const png = await new Promise<Blob | null>((resolve) => canvas.toBlob(resolve, 'image/png'))
      return png ? api.putThumb(a.id, png, 'browser') : null
    } finally {
      r.cleanup()
    }
  } finally {
    canvas.remove()
  }
}
