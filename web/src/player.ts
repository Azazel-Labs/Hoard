// One shared audio element for previewing sounds straight from the grid: starting a sound stops the previous
// one, like a sampler. Components read `player` reactively to show play state and progress.
import { reactive } from 'vue'
import { fileUrl, type Asset } from './api'

export const player = reactive({ id: '' as string, playing: false, progress: 0 })

let audio: HTMLAudioElement | null = null
let raf = 0

function el(): HTMLAudioElement {
  if (audio) return audio
  audio = new Audio()
  audio.preload = 'none'
  audio.addEventListener('playing', () => (player.playing = true))
  audio.addEventListener('pause', () => (player.playing = false))
  audio.addEventListener('ended', () => {
    player.playing = false
    player.progress = 0
  })
  audio.addEventListener('error', () => stop())
  return audio
}

function tick() {
  if (audio && audio.duration > 0) player.progress = audio.currentTime / audio.duration
  if (player.playing) raf = requestAnimationFrame(tick)
}

/** Play `asset`, or pause/resume it if it's the one already loaded. */
export function toggle(asset: Asset) {
  const a = el()
  if (player.id === asset.id) {
    if (a.paused) void a.play().then(tick, () => stop())
    else a.pause()
    return
  }
  a.pause()
  player.id = asset.id
  player.progress = 0
  a.src = playableUrl(asset) ?? fileUrl(asset)
  void a.play().then(
    () => {
      cancelAnimationFrame(raf)
      raf = requestAnimationFrame(tick)
    },
    () => stop(),
  )
}

export function stop() {
  audio?.pause()
  player.id = ''
  player.playing = false
  player.progress = 0
}

export const PLAYABLE = new Set(['wav', 'ogg', 'mp3', 'flac', 'opus', 'm4a', 'aac', 'webm'])

/** A source file recovered from inside the asset (the WAV in an Unreal SoundWave), if there is one. */
export const sourceExt = (a: Asset) => (typeof a.meta.source_ext === 'string' ? a.meta.source_ext : null)
export const sourceUrl = (a: Asset, download = false) =>
  `/api/assets/${a.id}/derivative/source${download ? '?download=1' : ''}`

/** What the browser can play for this asset: the file itself, a rendering of it, or its recovered source. */
export function playableUrl(a: Asset): string | null {
  if (a.kind !== 'audio') return null
  if (PLAYABLE.has(a.ext)) return fileUrl(a)
  // MIDI and tracker modules: the server's rendering.
  if (a.meta.preview === 'mp3') return `/api/assets/${a.id}/derivative/preview`
  const src = sourceExt(a)
  return src && PLAYABLE.has(src) ? sourceUrl(a) : null
}
export const canPreview = (a: Asset) => playableUrl(a) !== null
