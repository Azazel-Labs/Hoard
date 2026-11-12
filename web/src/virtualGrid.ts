// A windowed CSS grid: only the rows near the viewport are in the page, so a search that has scrolled through
// thousands of results stays as light as one that hasn't. The grid keeps its CSS (auto-fill columns, gaps); this
// reads the resolved column count and row height back from it and pads the grid above and below the rendered rows
// so the scroll height stays right.
import { computed, onBeforeUnmount, ref, watch, type Ref } from 'vue'

export function useVirtualGrid(opts: {
  scroller: Ref<HTMLElement | null>
  grid: Ref<HTMLElement | null>
  count: () => number
  /** Rows kept beyond each edge of the viewport. */
  overscan?: number
}) {
  const overscan = opts.overscan ?? 4
  const columns = ref(1)
  /** Row height plus the row gap. */
  const pitch = ref(240)
  const gap = ref(0)
  const firstRow = ref(0)
  const lastRow = ref(8)

  function measure() {
    const grid = opts.grid.value
    if (!grid) return
    const style = getComputedStyle(grid)
    // auto-fill makes its tracks whether or not there are items, so this is right before the first render too.
    columns.value = Math.max(1, style.gridTemplateColumns.split(' ').filter(Boolean).length)
    gap.value = parseFloat(style.rowGap) || 0
    const tile = grid.querySelector<HTMLElement>(':scope > .tile')
    const colWidth = (grid.clientWidth - (parseFloat(style.columnGap) || 0) * (columns.value - 1)) / columns.value
    // Before any tile exists, a square image plus two lines of text is close enough.
    pitch.value = (tile?.offsetHeight || colWidth + 48) + gap.value
  }

  function update() {
    const scroller = opts.scroller.value
    const grid = opts.grid.value
    if (!scroller || !grid) return
    measure()
    const top = grid.getBoundingClientRect().top - scroller.getBoundingClientRect().top + scroller.scrollTop
    const from = scroller.scrollTop - top
    firstRow.value = Math.max(0, Math.floor(from / pitch.value) - overscan)
    lastRow.value = Math.max(firstRow.value, Math.ceil((from + scroller.clientHeight) / pitch.value) + overscan)
  }

  let frame = 0
  const schedule = () => {
    if (!frame) frame = requestAnimationFrame(() => {
      frame = 0
      update()
    })
  }
  const resize = new ResizeObserver(schedule)
  watch([opts.scroller, opts.grid], ([s, g], old) => {
    for (const el of old ?? []) if (el) resize.unobserve(el)
    old?.[0]?.removeEventListener('scroll', schedule)
    s?.addEventListener('scroll', schedule, { passive: true })
    if (s) resize.observe(s)
    if (g) resize.observe(g)
    schedule()
  }, { immediate: true })
  watch(opts.count, schedule)
  onBeforeUnmount(() => {
    resize.disconnect()
    opts.scroller.value?.removeEventListener('scroll', schedule)
    cancelAnimationFrame(frame)
  })

  const rows = computed(() => Math.ceil(opts.count() / columns.value))
  const start = computed(() => Math.min(firstRow.value, rows.value) * columns.value)
  const end = computed(() => Math.min(opts.count(), (lastRow.value + 1) * columns.value))
  /** Padding standing in for the rows above and below the rendered ones. */
  const padding = computed(() => {
    const above = Math.min(firstRow.value, rows.value)
    const below = Math.max(0, rows.value - lastRow.value - 1)
    return { paddingTop: `${above * pitch.value}px`, paddingBottom: `${below * pitch.value}px` }
  })

  /** Scroll just enough that item `i` is on screen, and render its row now so it can take focus. */
  function reveal(i: number) {
    const scroller = opts.scroller.value
    const grid = opts.grid.value
    if (!scroller || !grid) return
    measure()
    const top = grid.getBoundingClientRect().top - scroller.getBoundingClientRect().top + scroller.scrollTop
    const rowTop = top + Math.floor(i / columns.value) * pitch.value
    const rowBottom = rowTop + pitch.value - gap.value
    // The sticky bar covers the top of the scroller.
    const bar = scroller.querySelector<HTMLElement>(':scope > .bar')?.offsetHeight ?? 0
    if (rowTop < scroller.scrollTop + bar) scroller.scrollTop = rowTop - bar - 8
    else if (rowBottom > scroller.scrollTop + scroller.clientHeight) scroller.scrollTop = rowBottom - scroller.clientHeight + 8
    update()
  }

  return { columns, start, end, padding, reveal, update }
}
