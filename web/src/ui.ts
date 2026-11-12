// App-wide feedback: stacked toasts (with an optional action such as Undo or Retry), a styled confirm dialog, and
// undoable actions that only commit once their toast runs out. UiLayer.vue renders all of it.
import { nextTick, reactive, ref, watch } from 'vue'

export type ToastKind = 'ok' | 'error' | 'info'
export interface ToastAction {
  label: string
  run: () => void
}
export interface Toast {
  id: number
  text: string
  kind: ToastKind
  action?: ToastAction
  duration: number
  paused: boolean
  /** Bumped when the same message is shown again, so the toast can nudge instead of stacking a copy. */
  repeat: number
}

interface Timer {
  handle?: ReturnType<typeof setTimeout>
  left: number
  started: number
  /** Runs when the toast goes away without its action being taken (timeout or dismissed). */
  onClose?: () => void
}

export const reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)')

const MAX = 4
export const toasts = reactive<Toast[]>([])
const timers = new Map<number, Timer>()
let seq = 0

export function toast(
  text: string,
  opts: { kind?: ToastKind; action?: ToastAction; duration?: number; onClose?: () => void } = {},
): number {
  const kind = opts.kind ?? 'ok'
  const duration = opts.duration ?? (kind === 'error' ? 7000 : opts.action ? 6000 : 3000)
  // The same message again (a poll failing twice, say): restart the one on screen rather than stacking a copy.
  const same = !opts.action && !opts.onClose && toasts.find((t) => t.text === text && t.kind === kind && !t.action)
  if (same) {
    same.repeat++
    same.duration = duration
    start(same.id, duration)
    return same.id
  }
  const id = ++seq
  toasts.push({ id, text, kind, action: opts.action, duration, paused: false, repeat: 0 })
  timers.set(id, { left: duration, started: 0, onClose: opts.onClose })
  start(id, duration)
  while (toasts.length > MAX) dismiss(toasts[0].id)
  return id
}

function start(id: number, ms: number) {
  const t = timers.get(id)
  if (!t) return
  clearTimeout(t.handle)
  t.left = ms
  t.started = Date.now()
  t.handle = setTimeout(() => dismiss(id), ms)
}

/** Remove a toast. Its onClose runs unless the action was taken (`acted`). */
export function dismiss(id: number, acted = false) {
  const i = toasts.findIndex((t) => t.id === id)
  if (i >= 0) toasts.splice(i, 1)
  const t = timers.get(id)
  timers.delete(id)
  if (!t) return
  clearTimeout(t.handle)
  if (!acted) t.onClose?.()
}

export function act(id: number) {
  const item = toasts.find((t) => t.id === id)
  dismiss(id, true)
  item?.action?.run()
}

/** Hovering or focusing a toast holds it on screen. */
export function pause(id: number) {
  const t = timers.get(id)
  const item = toasts.find((x) => x.id === id)
  if (!t || !item || item.paused) return
  clearTimeout(t.handle)
  t.left = Math.max(1200, t.left - (Date.now() - t.started))
  item.paused = true
}
export function resume(id: number) {
  const t = timers.get(id)
  const item = toasts.find((x) => x.id === id)
  if (!t || !item || !item.paused) return
  item.paused = false
  start(id, t.left)
}

export const showOk = (text: string, action?: ToastAction) => toast(text, { kind: 'ok', action })
export const showError = (text: string, action?: ToastAction) => toast(text, { kind: 'error', action })

// ------------------------------------------------------------------ undoable actions

const pending = new Map<number, () => void>()

/**
 * Show `text` with an Undo button and run `commit` only once the toast is gone. `apply` runs right away (hide the
 * thing being deleted); `revert` puts it back on Undo. Closing the tab commits anything still pending.
 */
export function undoable(text: string, steps: { apply?: () => void; revert: () => void; commit: () => void | Promise<void> }) {
  steps.apply?.()
  let id = 0
  const commit = () => {
    pending.delete(id)
    void steps.commit()
  }
  id = toast(text, {
    kind: 'info',
    duration: 6000,
    action: {
      label: 'Undo',
      run: () => {
        pending.delete(id)
        steps.revert()
      },
    },
    onClose: commit,
  })
  pending.set(id, commit)
}

window.addEventListener('pagehide', () => {
  for (const commit of [...pending.values()]) commit()
})

// ------------------------------------------------------------------ confirm dialog

export interface ConfirmOptions {
  title: string
  body?: string
  confirm: string
  cancel?: string
  danger?: boolean
}
export const dialog = reactive<{ open: ConfirmOptions | null; resolve: ((ok: boolean) => void) | null }>({
  open: null,
  resolve: null,
})

/** A modal yes/no question in the app's own style, in place of the browser's confirm(). */
export function confirmDialog(opts: ConfirmOptions): Promise<boolean> {
  dialog.resolve?.(false)
  return new Promise((resolve) => {
    dialog.open = opts
    dialog.resolve = resolve
  })
}
export function answerDialog(ok: boolean) {
  const r = dialog.resolve
  dialog.open = null
  dialog.resolve = null
  r?.(ok)
}

// ------------------------------------------------------------------ view transitions

/**
 * Run a DOM-changing update inside a view transition when the browser has them (and motion is welcome); otherwise
 * just run it. `type` lands on <html> as data-vt for the transition's CSS to key off.
 */
export function withTransition(update: () => void, type = 'swap', onReady?: () => void): Promise<void> {
  const doc = document as Document & {
    startViewTransition?: (cb: () => Promise<void>) => { finished: Promise<void>; ready: Promise<void> }
  }
  if (!doc.startViewTransition || reducedMotion.matches || document.hidden) {
    update()
    return Promise.resolve()
  }
  const root = document.documentElement
  root.dataset.vt = type
  const vt = doc.startViewTransition(async () => {
    update()
    await nextTick()
  })
  if (onReady) vt.ready.then(onReady, () => {})
  return vt.finished.finally(() => {
    if (root.dataset.vt === type) delete root.dataset.vt
  }).catch(() => {})
}

// ------------------------------------------------------------------ numbers

/** A number that eases toward `source` rather than jumping to it (result counts and the like). */
export function useTween(source: () => number, ms = 450) {
  const out = ref(source())
  let raf = 0
  watch(source, (to) => {
    cancelAnimationFrame(raf)
    const from = out.value
    if (reducedMotion.matches || document.hidden || from === to) {
      out.value = to
      return
    }
    const t0 = performance.now()
    const tick = (now: number) => {
      const p = Math.min(1, (now - t0) / ms)
      out.value = Math.round(from + (to - from) * (1 - (1 - p) ** 3))
      if (p < 1) raf = requestAnimationFrame(tick)
    }
    raf = requestAnimationFrame(tick)
  })
  return out
}

/** An entry in the command palette (Ctrl/Cmd-K). */
export interface Command {
  id: string
  label: string
  /** The section it's listed under: "Go to", "Actions", "Packs"… */
  group: string
  /** Shown on the right, dim: a shortcut, a count. */
  hint?: string
  /** Extra words it should match on but doesn't show. */
  keywords?: string
  run: () => void
}
