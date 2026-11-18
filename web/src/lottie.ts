// Lottie animations, played with lottie-web's canvas renderer (the build without expressions, which would need
// `eval`). The server hands over the animation's JSON (a .lottie's with its images inlined); a .json's images are
// fetched from beside it in the pack. The server can't draw these, so the browser makes their thumbnails.
import lottie, { type AnimationItem } from 'lottie-web/build/player/lottie_light_canvas'
import { api, joinPath, packFileUrl, type Asset, type ThumbUpload } from './api'
import { coverage } from './browserThumbs'

type LottieImage = { u?: string; p?: string; e?: number }
export type LottieDoc = { assets?: LottieImage[]; [k: string]: unknown }

/** How long to wait for an animation's images and fonts before giving up on it. */
const LOAD_WAIT_MS = 15_000

/** The animation to play: images it names but doesn't hold are pointed at the pack's copies. */
export async function lottieData(a: Asset): Promise<LottieDoc> {
  const doc = await api.lottie(a.id)
  const dir = a.path.includes('/') ? a.path.slice(0, a.path.lastIndexOf('/')) : ''
  for (const img of doc.assets ?? []) {
    if (!img.p || img.e === 1 || /^(data:|[a-z]+:\/\/|\/)/i.test(img.p)) continue
    img.p = packFileUrl(a.pack_id, joinPath(dir, `${img.u ?? ''}${img.p}`))
    img.u = ''
    img.e = 1
  }
  return doc
}

/** Play the animation in `container`, drawn by canvas. Resolves once it's ready to show (images and fonts in). */
export function mountLottie(container: HTMLElement, doc: LottieDoc, autoplay: boolean): Promise<AnimationItem> {
  const anim = lottie.loadAnimation({
    container,
    renderer: 'canvas',
    loop: true,
    autoplay,
    // lottie-web writes into the data it plays, so each player gets its own copy.
    animationData: structuredClone(doc),
    rendererSettings: { preserveAspectRatio: 'xMidYMid meet', clearCanvas: true, progressiveLoad: false },
  })
  return new Promise((resolve, reject) => {
    const timer = window.setTimeout(() => fail(new Error('timed out')), LOAD_WAIT_MS)
    const fail = (e: Error) => {
      clearTimeout(timer)
      anim.destroy()
      reject(e)
    }
    anim.addEventListener('DOMLoaded', () => {
      clearTimeout(timer)
      resolve(anim)
    })
    anim.addEventListener('data_failed', () => fail(new Error('data_failed')))
    anim.addEventListener('error', () => fail(new Error('error')))
  })
}

/**
 * Draw the animation 256 px across, at whichever of a few frames covers the most, and send it as the asset's
 * thumbnail.
 */
export async function drawLottieThumb(a: Asset, doc?: LottieDoc): Promise<ThumbUpload | null> {
  doc ??= await lottieData(a)
  const w = Number(a.meta.width) || 256
  const h = Number(a.meta.height) || 256
  const s = 256 / Math.max(w, h)
  const host = document.createElement('div')
  host.setAttribute('aria-hidden', 'true')
  host.style.cssText = `position:fixed;left:-10000px;top:0;pointer-events:none;width:${Math.max(1, Math.round(w * s))}px;height:${Math.max(1, Math.round(h * s))}px`
  document.body.appendChild(host)
  try {
    const anim = await mountLottie(host, doc, false)
    try {
      const canvas = host.querySelector('canvas')
      if (!canvas) return null
      const n = Math.max(1, Math.floor(anim.totalFrames))
      let best = { at: 0, cover: -1 }
      for (const f of [0.5, 0.25, 0.75, 0, 0.99]) {
        const at = Math.min(n - 1, Math.floor(n * f))
        anim.goToAndStop(at, true)
        const cover = coverage(canvas)
        if (cover > best.cover) best = { at, cover }
      }
      anim.goToAndStop(best.at, true)
      const png = await new Promise<Blob | null>((resolve) => canvas.toBlob(resolve, 'image/png'))
      return png ? api.putThumb(a.id, png, 'browser') : null
    } finally {
      anim.destroy()
    }
  } finally {
    host.remove()
  }
}
