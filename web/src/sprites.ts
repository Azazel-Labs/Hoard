// Pictures that move: animated GIF, PNG and WebP files (the browser plays them), Aseprite files (played from the
// server's sheet of their frames, with each frame's own time) and sprite sheets (frames cut from a grid, guessed
// here from the sheet itself).
import { fileUrl, type Asset } from './api'

/** Files the browser animates itself. */
const NATIVE_ANIMATED = new Set(['gif', 'png', 'apng', 'webp'])
/** Animated files smaller than this play in the detail panel; bigger ones show their thumbnail. */
export const PLAY_IN_PANEL = 16 << 20
/** And smaller than this on a tile, while the pointer is over it. */
export const PLAY_ON_TILE = 4 << 20

const num = (v: unknown) => (typeof v === 'number' && Number.isFinite(v) ? v : 0)

/** A GIF, PNG or WebP with more than one frame. */
export const animatedNative = (a: Asset) => NATIVE_ANIMATED.has(a.ext.toLowerCase()) && num(a.meta.frames) > 1
/** An Aseprite file with more than one frame. */
export const animatedAseprite = (a: Asset) => ['aseprite', 'ase'].includes(a.ext.toLowerCase()) && num(a.meta.frames) > 1
/** A picture that's frames laid out in a grid, by its tags. */
export const spriteSheet = (a: Asset) =>
  a.kind === 'texture' && !animatedNative(a) && a.tags.some((t) => t.tag === 'spritesheet' || t.tag === 'frames')
/** What shows a native animation moving: the file itself, when it isn't too big. */
export const animatedSrc = (a: Asset, limit: number) => (animatedNative(a) && a.size <= limit ? fileUrl(a) : null)
export const framesUrl = (a: { id: string }) => `/api/assets/${a.id}/frames.png`

/** A run of an Aseprite file's frames, from its tags. */
export interface Animation {
  name: string
  from: number
  to: number
  direction: 'forward' | 'reverse' | 'pingpong'
}

export function animations(a: Asset): Animation[] {
  const list = a.meta.animations
  if (!Array.isArray(list)) return []
  return list.filter(
    (t): t is Animation => typeof t === 'object' && t !== null && typeof t.name === 'string' && typeof t.from === 'number' && typeof t.to === 'number',
  )
}

/** The frame indices an animation plays, in order: forward, backward, or there and back without repeating the ends. */
export function sequence(from: number, to: number, direction: Animation['direction']): number[] {
  const up = Array.from({ length: Math.max(0, to - from + 1) }, (_, i) => from + i)
  if (direction === 'reverse') return up.reverse()
  if (direction === 'pingpong') return [...up, ...up.slice(1, -1).reverse()]
  return up
}

export interface Grid {
  cols: number
  rows: number
}

/**
 * How a sheet is cut into frames: by the frame size in its name ("hero_32x32.png"), by the empty gutters between its
 * frames, or as a strip of square frames. `pixels` is the sheet as drawn (RGBA, `w`×`h`); `size` its real size, when
 * the drawn one is a smaller preview.
 */
export function guessGrid(pixels: Uint8ClampedArray | null, w: number, h: number, name: string, size?: { w: number; h: number }): Grid {
  const [rw, rh] = size && size.w > 0 && size.h > 0 ? [size.w, size.h] : [w, h]
  for (const m of name.matchAll(/(\d{1,4})\s*[x×]\s*(\d{1,4})/gi)) {
    const [fw, fh] = [Number(m[1]), Number(m[2])]
    if (fw > 0 && fh > 0 && rw % fw === 0 && rh % fh === 0 && (rw / fw) * (rh / fh) > 1) return { cols: rw / fw, rows: rh / fh }
  }
  if (pixels) {
    const empty = emptiness(pixels, w)
    const colHas = Array.from({ length: w }, (_, x) => {
      for (let y = 0; y < h; y++) if (!empty(x, y)) return true
      return false
    })
    const rowHas = Array.from({ length: h }, (_, y) => {
      for (let x = 0; x < w; x++) if (!empty(x, y)) return true
      return false
    })
    const grid = { cols: cut(colHas), rows: cut(rowHas) }
    if (grid.cols * grid.rows > 1) return grid
  }
  if (rw > rh && rw % rh === 0) return { cols: rw / rh, rows: 1 }
  if (rh > rw && rh % rw === 0) return { cols: 1, rows: rh / rw }
  return { cols: 1, rows: 1 }
}

/** Which pixels are background: see-through ones, or for a sheet with none, the colour of its corner. */
function emptiness(px: Uint8ClampedArray, w: number): (x: number, y: number) => boolean {
  let clear = false
  for (let i = 3; i < px.length; i += 4) {
    if (px[i] < 255) {
      clear = true
      break
    }
  }
  if (clear) return (x, y) => px[(y * w + x) * 4 + 3] < 8
  const [r, g, b] = [px[0], px[1], px[2]]
  return (x, y) => {
    const i = (y * w + x) * 4
    return Math.abs(px[i] - r) + Math.abs(px[i + 1] - g) + Math.abs(px[i + 2] - b) < 24
  }
}

/**
 * The most equal cells a line of pixels splits into such that every run of content sits inside one cell and every
 * cell has some: frames with gaps between them, even where a frame has gaps of its own.
 */
function cut(has: boolean[]): number {
  const len = has.length
  const runs: [number, number][] = []
  has.forEach((v, i) => {
    if (!v) return
    const last = runs[runs.length - 1]
    if (last && last[1] === i - 1) last[1] = i
    else runs.push([i, i])
  })
  let best = 1
  for (let n = 2; n <= Math.min(runs.length, 256); n++) {
    const cell = len / n
    if (Math.abs(cell - Math.round(cell)) > 0.01 * n) continue
    const of = (x: number) => Math.min(n - 1, Math.floor(x / cell))
    if (!runs.every(([a, b]) => of(a) === of(b))) continue
    if (new Set(runs.map(([a]) => of(a))).size === n) best = n
  }
  return best
}

/** Which cells of a grid have anything in them, in reading order: the frames to play (a last row is often short). */
export function filledCells(pixels: Uint8ClampedArray, w: number, h: number, grid: Grid): number[] {
  const empty = emptiness(pixels, w)
  const [cw, ch] = [w / grid.cols, h / grid.rows]
  const out: number[] = []
  for (let r = 0; r < grid.rows; r++) {
    for (let c = 0; c < grid.cols; c++) {
      const [x0, y0] = [Math.floor(c * cw), Math.floor(r * ch)]
      const [x1, y1] = [Math.floor((c + 1) * cw), Math.floor((r + 1) * ch)]
      let any = false
      // Every other pixel is plenty to tell an empty cell from a frame.
      for (let y = y0; y < y1 && !any; y += 1 + ((y1 - y0) >> 6)) {
        for (let x = x0; x < x1; x += 1 + ((x1 - x0) >> 6)) {
          if (!empty(x, y)) {
            any = true
            break
          }
        }
      }
      if (any) out.push(r * grid.cols + c)
    }
  }
  return out
}
