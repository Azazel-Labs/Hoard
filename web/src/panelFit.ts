import { computed, nextTick, onBeforeUnmount, ref, watch, type Ref } from 'vue'
import { prefs } from './prefs'

/** The elements the library's layout is measured from. */
export interface PanelFitElements {
  /** The whole app, whose width the grid and panel share. */
  app: Ref<HTMLElement | null>
  /** The filter sidebar. */
  side: Readonly<Ref<HTMLElement | null>>
  /** The grid's scroller. */
  main: Ref<HTMLElement | null>
  grid: Ref<HTMLElement | null>
  /** The panel beside the grid, if one is open. */
  panel: () => HTMLElement | null
}

/** Opening the detail panel leaves the tiles the size they were: the grid keeps its column width and drops a column,
 *  and the panel takes what's left over (so it's a little wider or narrower depending on the window), instead of the
 *  remaining columns stretching to fill the space. Dragging the panel's edge, or arrow keys on it, set its width. */
export function usePanelFit(els: PanelFitElements) {
  /** The room the grid and panel share, what the grid's padding and scrollbar take of it, and the column gap. */
  const room = ref({ width: 0, side: 0, chrome: 0, gap: 0 })
  const DETAIL_MIN = 340
  const DETAIL_MAX = 640
  const narrow = window.matchMedia('(max-width: 900px)')
  function measureRoom() {
    const app = els.app.value
    const side = els.side.value
    const main = els.main.value
    if (!app || !side || !main || narrow.matches) {
      room.value = { width: 0, side: 0, chrome: 0, gap: 0 }
      return
    }
    const ms = getComputedStyle(main)
    const chrome = parseFloat(ms.paddingLeft) + parseFloat(ms.paddingRight) + (main.offsetWidth - main.clientWidth)
    const gap = els.grid.value ? parseFloat(getComputedStyle(els.grid.value).columnGap) || 0 : room.value.gap
    room.value = { width: app.clientWidth, side: side.offsetWidth, chrome, gap }
  }
  const roomObserver = new ResizeObserver(measureRoom)
  watch([els.app, els.side], ([a, s]) => {
    roomObserver.disconnect()
    if (a) roomObserver.observe(a)
    if (s) roomObserver.observe(s)
  })
  watch(els.grid, () => nextTick(measureRoom))
  // Text size and density change the grid's padding and gap without resizing anything observed.
  watch(() => [prefs.textSize, prefs.density], () => nextTick(measureRoom))
  /** Whether the screen is phone-narrow, where the panel covers the grid. */
  const isNarrow = ref(narrow.matches)
  const onNarrow = () => {
    isNarrow.value = narrow.matches
    measureRoom()
  }
  narrow.addEventListener('change', onNarrow)
  onBeforeUnmount(() => {
    roomObserver.disconnect()
    narrow.removeEventListener('change', onNarrow)
  })
  /** The panel's width while its edge is being dragged. */
  const detailDrag = ref<number | null>(null)
  /** With the panel open: the columns to show, their width, and the panel's width. The tiles keep their size; the
   *  grid drops columns to make room. */
  const fit = computed(() => {
    const { width, side, chrome, gap } = room.value
    if (!width || !gap) return null
    const space = width - side - chrome
    // The columns the grid has without the panel, as auto-fill makes them.
    const n = Math.max(1, Math.floor((space + gap) / (prefs.tile + gap)))
    const col = (space - gap * (n - 1)) / n
    const used = (cols: number) => cols * col + (cols - 1) * gap
    const want = detailDrag.value ?? prefs.detailWidth
    if (want === null) {
      // Left alone, the panel takes what's over once enough columns go for it to fit.
      const cols = Math.floor((space - DETAIL_MIN + gap) / (col + gap))
      if (cols < 1) return null
      return { col, cols, detail: Math.min(DETAIL_MAX, Math.floor(space - used(cols))) }
    }
    // Dragged: as wide as asked, leaving the grid at least one column.
    const detail = Math.min(Math.max(want, DETAIL_MIN), space - col)
    if (detail < DETAIL_MIN) return null
    let cols = Math.max(1, Math.floor((space - detail + gap) / (col + gap)))
    // Mid-drag the edge follows the pointer; let go, and it settles on the nearest column so no gap is left over.
    if (detailDrag.value !== null) return { col, cols, detail }
    if (space - used(cols + 1) >= DETAIL_MIN && space - used(cols + 1) > detail - (col + gap) / 2) cols++
    return { col, cols, detail: Math.floor(space - used(cols)) }
  })

  /** Dragging the panel's left edge sets its width; double-clicking it goes back to the width that fits. */
  function startResize(e: PointerEvent) {
    const panel = els.panel()
    if (!panel || e.button !== 0) return
    e.preventDefault()
    const handle = e.currentTarget as HTMLElement
    handle.setPointerCapture(e.pointerId)
    const x0 = e.clientX
    const w0 = panel.offsetWidth
    detailDrag.value = w0
    document.documentElement.classList.add('resizing')
    const move = (m: PointerEvent) => (detailDrag.value = w0 + (x0 - m.clientX))
    const up = () => {
      handle.removeEventListener('pointermove', move)
      handle.removeEventListener('pointerup', up)
      handle.removeEventListener('pointercancel', up)
      document.documentElement.classList.remove('resizing')
      if (detailDrag.value !== null && Math.abs(detailDrag.value - w0) > 2) prefs.detailWidth = Math.round(fit.value?.detail ?? detailDrag.value)
      detailDrag.value = null
    }
    handle.addEventListener('pointermove', move)
    handle.addEventListener('pointerup', up)
    handle.addEventListener('pointercancel', up)
  }
  function resizeBy(e: KeyboardEvent) {
    const delta = e.key === 'ArrowLeft' ? 1 : e.key === 'ArrowRight' ? -1 : 0
    if (e.key === 'Home') prefs.detailWidth = null
    if (!delta) return
    e.preventDefault()
    const f = fit.value
    if (!f) return
    // A column at a time, so each press shows a change.
    prefs.detailWidth = Math.round(f.detail + delta * (f.col + room.value.gap))
  }
  const appStyle = computed(() => {
    const s: Record<string, string> = { '--tile': `${prefs.tile}px` }
    const f = fit.value
    if (f) {
      s['--side-w'] = `${room.value.side}px`
      s['--detail-w'] = `${f.detail}px`
      s['--fit-cols'] = String(f.cols)
      s['--fit-col'] = `${f.col}px`
    }
    return s
  })
  return { isNarrow, detailDrag, fit, startResize, resizeBy, appStyle }
}
