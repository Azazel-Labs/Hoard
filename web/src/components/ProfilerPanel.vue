<script setup lang="ts">
// Settings → Performance's profiler: the last ten minutes of Hoard, asked for every two seconds while the page is on
// screen (the server keeps them in memory; see its profiler.rs). Processor use as two lines, Hoard's and the whole
// machine's; Hoard's RAM as a line on a scale of its own (the machine's would flatten it); what was running as lanes; and the time spent serving requests as stacked bars, split into the
// database, the disk, sending and everything else, with a table by route. Every chart has its numbers on hover and
// in words beside it, so color is never the only label.
import { computed, onBeforeUnmount, onMounted, ref, shallowRef } from 'vue'
import { api, type ProfileSample, type ProfileServed } from '../api'
import { humanBytes } from '../format'
import { formatDate, formatNumber, t } from '../i18n'

const POLL_MS = 2000
/** Request time is added up in buckets this long, one bar each. */
const BUCKET_MS = 10_000
/** Plot margins, in pixels: room for the value labels on the left and the clock times below. */
const LEFT = 44
const RIGHT = 8
const CPU_H = 150
const RAM_H = 120
const REQ_H = 150
const AXIS = 20
const LANE_H = 28
const MAX_LANES = 6

const samples = shallowRef<ProfileSample[]>([])
const served = shallowRef<ProfileServed[]>([])
const seq = ref(0)
/** The server's clock at its last answer, and how far back it remembers, in milliseconds. */
const now = ref(0)
const keep = ref(600_000)
const failed = ref('')
const loaded = ref(false)

let timer: number | undefined
let asking = false
async function poll() {
  if (asking || document.hidden) return
  asking = true
  try {
    const p = await api.profile(seq.value)
    const from = p.now - p.keep
    samples.value = [...samples.value, ...p.samples].filter((s) => s.at >= from)
    served.value = [...served.value, ...p.served].filter((s) => s.at >= from)
    seq.value = p.seq
    now.value = p.now
    keep.value = p.keep
    failed.value = ''
    loaded.value = true
  } catch (e) {
    failed.value = (e as Error).message
  } finally {
    asking = false
  }
}

const root = ref<HTMLElement | null>(null)
const width = ref(640)
let resize: ResizeObserver | undefined
onMounted(() => {
  void poll()
  timer = window.setInterval(() => void poll(), POLL_MS)
  document.addEventListener('visibilitychange', poll)
  if (root.value) {
    resize = new ResizeObserver(([e]) => (width.value = Math.max(240, Math.floor(e.contentRect.width))))
    resize.observe(root.value)
  }
})
onBeforeUnmount(() => {
  window.clearInterval(timer)
  document.removeEventListener('visibilitychange', poll)
  resize?.disconnect()
})

// Time runs left to right across the same plot width in every chart, so they line up.
const from = computed(() => now.value - keep.value)
const plotW = computed(() => width.value - LEFT - RIGHT)
const x = (at: number) => LEFT + ((at - from.value) / keep.value) * plotW.value
const atOf = (px: number) => from.value + ((px - LEFT) / plotW.value) * keep.value
/** Where the pointer is across a chart, in its pixels. */
const across = (e: PointerEvent) => e.clientX - (e.currentTarget as Element).getBoundingClientRect().left
const clock = (at: number) => formatDate(at, { hour: 'numeric', minute: '2-digit' })
const clockSecs = (at: number) => formatDate(at, { hour: 'numeric', minute: '2-digit', second: '2-digit' })
/** Clock times on the minute, as far apart as the width needs: every one, two or five minutes. */
const ticks = computed(() => {
  const step = [60_000, 120_000, 300_000].find((s) => (s / keep.value) * plotW.value >= 80) ?? 300_000
  const out: number[] = []
  for (let at = Math.ceil(from.value / step) * step; at <= now.value; at += step) out.push(at)
  return out
})

const pct = (v: number) => formatNumber(v / 100, { style: 'percent', maximumFractionDigits: v < 10 ? 1 : 0 })
const share = (part: number, whole: number) =>
  formatNumber(whole ? part / whole : 0, { style: 'percent', maximumFractionDigits: 0 })
/** A length of time in the current language: milliseconds under a second, seconds above. */
function duration(ms: number): string {
  if (ms < 1000) return formatNumber(ms, { style: 'unit', unit: 'millisecond', maximumFractionDigits: ms < 10 ? 1 : 0 })
  return formatNumber(ms / 1000, { style: 'unit', unit: 'second', maximumFractionDigits: ms < 10_000 ? 1 : 0 })
}

// ---- Processor use -------------------------------------------------------------------------------------------

const cpuY = (v: number) => 8 + (1 - Math.min(100, v) / 100) * (CPU_H - 8 - AXIS)
/** A line through the samples, broken where a few seconds are missing (Hoard was stopped, or the tab asleep). */
function line(pick: (s: ProfileSample) => number): string {
  let d = ''
  let last = -Infinity
  for (const s of samples.value) {
    d += `${s.at - last > 3500 ? 'M' : 'L'}${x(s.at).toFixed(1)},${cpuY(pick(s)).toFixed(1)}`
    last = s.at
  }
  return d
}
const hoardLine = computed(() => line((s) => s.hoard))
const machineLine = computed(() => line((s) => s.machine))
const latest = computed(() => samples.value.at(-1))

const cpuHover = ref<ProfileSample | null>(null)
function hoverCpu(e: PointerEvent) {
  const at = atOf(across(e))
  let best: ProfileSample | null = null
  for (const s of samples.value) if (!best || Math.abs(s.at - at) < Math.abs(best.at - at)) best = s
  cpuHover.value = best && Math.abs(best.at - at) < 5000 ? best : null
}
/** Where a tooltip sits: beside the point, on whichever side has room, so it never covers what it describes. */
const tipAt = (px: number) =>
  px > width.value / 2
    ? { left: `${px - 12}px`, transform: 'translateX(-100%)' }
    : { left: `${px + 12}px`, transform: 'none' }

// ---- RAM -----------------------------------------------------------------------------------------------------

/** The scale's top: a round number of bytes above Hoard's most, so the line fills the chart whatever its size. */
const ramMax = computed(() => {
  const most = Math.max(64 * 2 ** 20, ...samples.value.map((s) => s.memory))
  const unit = most >= 2 ** 30 ? 2 ** 30 : 2 ** 20
  const step = 10 ** Math.floor(Math.log10(most / unit)) * unit
  return [1, 1.5, 2, 2.5, 3, 4, 5, 6, 8, 10].map((m) => m * step).find((v) => v >= most * 1.1) ?? most
})
const ramY = (v: number) => 8 + (1 - Math.min(v, ramMax.value) / ramMax.value) * (RAM_H - 8 - AXIS)
/** Hoard's RAM as a line and the area under it, broken like the processor's where samples are missing. */
const ramPaths = computed(() => {
  let d = ''
  let area = ''
  let run: ProfileSample[] = []
  const close = () => {
    if (!run.length) return
    const pts = run.map((s) => `${x(s.at).toFixed(1)},${ramY(s.memory).toFixed(1)}`)
    d += `M${pts.join('L')}`
    const base = ramY(0).toFixed(1)
    area += `M${x(run[0].at).toFixed(1)},${base}L${pts.join('L')}L${x(run.at(-1)!.at).toFixed(1)},${base}Z`
    run = []
  }
  let last = -Infinity
  for (const s of samples.value) {
    if (s.at - last > 3500) close()
    run.push(s)
    last = s.at
  }
  close()
  return { d, area }
})
const ramPeak = computed(() => Math.max(0, ...samples.value.map((s) => s.memory)))
const machineRam = (s: ProfileSample) =>
  s.total ? t('profiler-ram-of', { used: humanBytes(s.used), total: humanBytes(s.total) }) : ''

const ramHover = ref<ProfileSample | null>(null)
function hoverRam(e: PointerEvent) {
  const at = atOf(across(e))
  let best: ProfileSample | null = null
  for (const s of samples.value) if (!best || Math.abs(s.at - at) < Math.abs(best.at - at)) best = s
  ramHover.value = best && Math.abs(best.at - at) < 5000 ? best : null
}

// ---- Working on ----------------------------------------------------------------------------------------------

interface Lane { label: string; spans: { start: number; end: number }[] }
/** One lane per thing that ran, its seconds joined into spans; the first few to start, the rest counted. */
const allLanes = computed<Lane[]>(() => {
  const lanes = new Map<string, Lane>()
  for (const s of samples.value) {
    for (const r of s.running) {
      let lane = lanes.get(r.label)
      if (!lane) lanes.set(r.label, (lane = { label: r.label, spans: [] }))
      const span = lane.spans.at(-1)
      if (span && s.at - span.end < 2500) span.end = s.at + 1000
      else lane.spans.push({ start: s.at - 1000, end: s.at })
    }
  }
  return [...lanes.values()]
})
const lanes = computed(() => allLanes.value.slice(0, MAX_LANES))
/** A lane's name, cut short to fit across the chart (SVG text doesn't wrap or ellipsize). */
const fit = (label: string) => {
  const room = Math.floor(plotW.value / 6.5)
  return label.length > room ? `${label.slice(0, Math.max(1, room - 1))}…` : label
}
const lanesH = computed(() => lanes.value.length * LANE_H)

// ---- Time serving requests -----------------------------------------------------------------------------------

type Part = 'db' | 'disk' | 'send' | 'other'
const PARTS: Part[] = ['db', 'disk', 'send', 'other']
const partLabel = (p: Part) =>
  ({
    db: t('profiler-part-db'),
    disk: t('profiler-part-disk'),
    send: t('profiler-part-send'),
    other: t('profiler-part-other'),
  })[p]
const partTitle = (p: Part) =>
  ({
    db: t('profiler-part-db-title'),
    disk: t('profiler-part-disk-title'),
    send: t('profiler-part-send-title'),
    other: t('profiler-part-other-title'),
  })[p]
/** A request's time in each part; the rest of its time is working the answer out. */
const partsOf = (r: ProfileServed): Record<Part, number> => ({
  db: r.db,
  disk: r.disk,
  send: r.send,
  other: Math.max(0, r.total - r.db - r.disk - r.send),
})

interface Bucket { start: number; count: number; parts: Record<Part, number>; slowest: ProfileServed | null }
const buckets = computed<Bucket[]>(() => {
  const first = Math.floor(from.value / BUCKET_MS) * BUCKET_MS
  const out: Bucket[] = []
  for (let start = first; start < now.value; start += BUCKET_MS) {
    out.push({ start, count: 0, parts: { db: 0, disk: 0, send: 0, other: 0 }, slowest: null })
  }
  for (const r of served.value) {
    const b = out[Math.floor((r.at - first) / BUCKET_MS)]
    if (!b) continue
    b.count++
    const p = partsOf(r)
    for (const k of PARTS) b.parts[k] += p[k]
    if (!b.slowest || r.total > b.slowest.total) b.slowest = r
  }
  return out
})
const bucketTotal = (b: Bucket) => PARTS.reduce((sum, k) => sum + b.parts[k], 0)
/** The scale's top: a round number above the busiest bucket. */
const reqMax = computed(() => {
  const most = Math.max(1, ...buckets.value.map(bucketTotal))
  const step = 10 ** Math.floor(Math.log10(most))
  return [1, 2, 2.5, 5, 10].map((m) => m * step).find((v) => v >= most) ?? most
})
const reqY = (v: number) => 8 + (1 - v / reqMax.value) * (REQ_H - 8 - AXIS)
const barW = computed(() => Math.max(1, (plotW.value * BUCKET_MS) / keep.value - 2))
/** Each bar's segments, the database at the bottom; a pixel of the card between them. */
function segments(b: Bucket) {
  let base = 0
  return PARTS.map((k) => {
    const y0 = reqY(base)
    base += b.parts[k]
    const y1 = reqY(base)
    const h = y0 - y1
    return { k, y: y1, h: h > 2 ? h - 1 : h }
  }).filter((s) => s.h > 0)
}

const totals = computed(() => {
  const sums: Record<Part, number> = { db: 0, disk: 0, send: 0, other: 0 }
  for (const r of served.value) {
    const p = partsOf(r)
    for (const k of PARTS) sums[k] += p[k]
  }
  return { count: served.value.length, sums, all: PARTS.reduce((s, k) => s + sums[k], 0) }
})
const requestSummary = computed(() =>
  [
    t('profiler-request-count', { count: totals.value.count }),
    ...PARTS.map((k) => `${partLabel(k)} ${share(totals.value.sums[k], totals.value.all)}`),
  ].join(' · '),
)

const reqHover = ref<Bucket | null>(null)
function hoverReq(e: PointerEvent) {
  const first = buckets.value[0]?.start ?? 0
  const b = buckets.value[Math.floor((atOf(across(e)) - first) / BUCKET_MS)]
  reqHover.value = b && b.count ? b : null
}

const routeName = (r: { method: string; route: string }) => `${r.method} ${r.route || t('profiler-portal')}`

interface Route { key: string; name: string; count: number; time: number; avg: number; p95: number; bytes: number
  parts: Record<Part, number> }
/** Requests by route, the ones that took the most time in all first. */
const routes = computed<Route[]>(() => {
  const groups = new Map<string, ProfileServed[]>()
  for (const r of served.value) {
    const key = `${r.method} ${r.route}`
    const g = groups.get(key)
    if (g) g.push(r)
    else groups.set(key, [r])
  }
  return [...groups.entries()]
    .map(([key, rs]) => {
      const sorted = rs.map((r) => r.total).sort((a, b) => a - b)
      const time = sorted.reduce((a, b) => a + b, 0)
      const parts: Record<Part, number> = { db: 0, disk: 0, send: 0, other: 0 }
      for (const r of rs) {
        const p = partsOf(r)
        for (const k of PARTS) parts[k] += p[k]
      }
      return {
        key,
        name: routeName(rs[0]),
        count: rs.length,
        time,
        avg: time / rs.length,
        p95: sorted[Math.min(sorted.length - 1, Math.floor(sorted.length * 0.95))],
        bytes: rs.reduce((a, r) => a + r.bytes, 0),
        parts,
      }
    })
    .sort((a, b) => b.time - a.time)
    .slice(0, 15)
})
const routeSplit = (r: Route) => {
  const all = PARTS.reduce((s, k) => s + r.parts[k], 0)
  return PARTS.map((k) => ({ k, w: all ? (r.parts[k] / all) * 100 : 0 })).filter((p) => p.w > 0)
}
const routeSplitText = (r: Route) => {
  const all = PARTS.reduce((s, k) => s + r.parts[k], 0)
  return PARTS.map((k) => `${partLabel(k)} ${share(r.parts[k], all)}`).join(', ')
}
</script>

<template>
  <section ref="root" class="profiler" :aria-busy="!loaded">
    <div class="sh"><h3>{{ t('profiler') }}</h3></div>
    <p class="what">{{ t('profiler-what') }}</p>
    <p v-if="failed" class="bad" role="alert">{{ t('profiler-failed', { error: failed }) }}</p>
    <p v-else-if="!samples.length" class="what">{{ t('profiler-starting') }}</p>

    <template v-if="samples.length">
      <h4>{{ t('profiler-cpu') }}</h4>
      <ul class="legend">
        <li><i class="sw hoard" aria-hidden="true" />{{ t('profiler-cpu-hoard') }} <b>{{ pct(latest!.hoard) }}</b></li>
        <li>
          <i class="sw machine" aria-hidden="true" />{{ t('profiler-cpu-machine') }} <b>{{ pct(latest!.machine) }}</b>
        </li>
      </ul>
      <div class="chart">
        <svg
          :width="width"
          :height="CPU_H"
          role="img"
          :aria-label="t('profiler-cpu-summary', { hoard: pct(latest!.hoard), machine: pct(latest!.machine) })"
          @pointermove="hoverCpu"
          @pointerleave="cpuHover = null"
        >
          <g class="grid">
            <template v-for="v in [0, 50, 100]" :key="v">
              <line :x1="LEFT" :x2="width - RIGHT" :y1="cpuY(v)" :y2="cpuY(v)" />
              <text :x="LEFT - 6" :y="cpuY(v) + 4" text-anchor="end">{{ pct(v) }}</text>
            </template>
            <text v-for="at in ticks" :key="at" :x="x(at)" :y="CPU_H - 4" text-anchor="middle">{{ clock(at) }}</text>
          </g>
          <path class="ln machine" :d="machineLine" />
          <path class="ln hoard" :d="hoardLine" />
          <g v-if="cpuHover">
            <line class="cross" :x1="x(cpuHover.at)" :x2="x(cpuHover.at)" :y1="8" :y2="CPU_H - AXIS" />
            <circle class="dot machine" :cx="x(cpuHover.at)" :cy="cpuY(cpuHover.machine)" r="4" />
            <circle class="dot hoard" :cx="x(cpuHover.at)" :cy="cpuY(cpuHover.hoard)" r="4" />
          </g>
        </svg>
        <div v-if="cpuHover" class="tip" :style="tipAt(x(cpuHover.at))" aria-hidden="true">
          <strong>{{ clockSecs(cpuHover.at) }}</strong>
          <span><i class="sw hoard" />{{ t('profiler-cpu-hoard') }}<b>{{ pct(cpuHover.hoard) }}</b></span>
          <span><i class="sw machine" />{{ t('profiler-cpu-machine') }}<b>{{ pct(cpuHover.machine) }}</b></span>
          <span>{{ t('profiler-ram') }}<b>{{ humanBytes(cpuHover.memory) }}</b></span>
          <span>{{ t('profiler-serving') }}<b>{{ formatNumber(cpuHover.serving) }}</b></span>
          <em v-for="r in cpuHover.running" :key="r.label">{{ r.label }}</em>
        </div>
      </div>

      <h4>{{ t('profiler-ram') }}</h4>
      <ul class="legend">
        <li><i class="sw hoard" aria-hidden="true" />{{ t('profiler-cpu-hoard') }} <b>{{ humanBytes(latest!.memory) }}</b></li>
        <li>{{ t('profiler-ram-peak') }} <b>{{ humanBytes(ramPeak) }}</b></li>
        <li v-if="latest!.total" class="aside">
          {{ t('profiler-cpu-machine') }} <b>{{ machineRam(latest!) }}</b>
        </li>
      </ul>
      <div class="chart">
        <svg
          :width="width"
          :height="RAM_H"
          role="img"
          :aria-label="t('profiler-ram-summary', { hoard: humanBytes(latest!.memory), peak: humanBytes(ramPeak) })"
          @pointermove="hoverRam"
          @pointerleave="ramHover = null"
        >
          <g class="grid">
            <template v-for="v in [0, ramMax / 2, ramMax]" :key="v">
              <line :x1="LEFT" :x2="width - RIGHT" :y1="ramY(v)" :y2="ramY(v)" />
              <text :x="LEFT - 6" :y="ramY(v) + 4" text-anchor="end">{{ v ? humanBytes(v) : 0 }}</text>
            </template>
            <text v-for="at in ticks" :key="at" :x="x(at)" :y="RAM_H - 4" text-anchor="middle">{{ clock(at) }}</text>
          </g>
          <path class="area" :d="ramPaths.area" />
          <path class="ln hoard" :d="ramPaths.d" />
          <g v-if="ramHover">
            <line class="cross" :x1="x(ramHover.at)" :x2="x(ramHover.at)" :y1="8" :y2="RAM_H - AXIS" />
            <circle class="dot hoard" :cx="x(ramHover.at)" :cy="ramY(ramHover.memory)" r="4" />
          </g>
        </svg>
        <div v-if="ramHover" class="tip" :style="tipAt(x(ramHover.at))" aria-hidden="true">
          <strong>{{ clockSecs(ramHover.at) }}</strong>
          <span><i class="sw hoard" />{{ t('profiler-cpu-hoard') }}<b>{{ humanBytes(ramHover.memory) }}</b></span>
          <span v-if="ramHover.total">
            <i class="sw machine" />{{ t('profiler-cpu-machine') }}<b>{{ machineRam(ramHover) }}</b>
          </span>
        </div>
      </div>

      <h4>{{ t('profiler-working') }}</h4>
      <p v-if="!lanes.length" class="what">{{ t('profiler-working-none') }}</p>
      <svg v-else :width="width" :height="lanesH" class="lanes" role="list" :aria-label="t('profiler-working')">
        <g v-for="(lane, i) in lanes" :key="lane.label" role="listitem" :aria-label="lane.label">
          <text :x="LEFT" :y="i * LANE_H + 11">{{ fit(lane.label) }}</text>
          <line class="track" :x1="LEFT" :x2="width - RIGHT" :y1="i * LANE_H + 19" :y2="i * LANE_H + 19" />
          <rect
            v-for="span in lane.spans"
            :key="span.start"
            class="span"
            :x="Math.max(LEFT, x(span.start))"
            :y="i * LANE_H + 15"
            :width="Math.max(3, x(span.end) - Math.max(LEFT, x(span.start)))"
            height="8"
            rx="2"
          >
            <title>{{ lane.label }}: {{ clockSecs(span.start) }} – {{ clockSecs(span.end) }}</title>
          </rect>
        </g>
      </svg>
      <p v-if="allLanes.length > MAX_LANES" class="what">
        {{ t('profiler-working-more', { count: allLanes.length - MAX_LANES }) }}
      </p>

      <h4>{{ t('profiler-requests') }}</h4>
      <p class="what">{{ t('profiler-requests-what') }}</p>
      <ul class="legend">
        <li v-for="k in PARTS" :key="k" :title="partTitle(k)">
          <i class="sw" :class="k" aria-hidden="true" />{{ partLabel(k) }}
        </li>
      </ul>
      <p v-if="!totals.count" class="what">{{ t('profiler-requests-none') }}</p>
      <template v-else>
        <div class="chart">
          <svg
            :width="width"
            :height="REQ_H"
            role="img"
            :aria-label="requestSummary"
            @pointermove="hoverReq"
            @pointerleave="reqHover = null"
          >
            <g class="grid">
              <template v-for="v in [0, reqMax / 2, reqMax]" :key="v">
                <line :x1="LEFT" :x2="width - RIGHT" :y1="reqY(v)" :y2="reqY(v)" />
                <text :x="LEFT - 6" :y="reqY(v) + 4" text-anchor="end">{{ duration(v) }}</text>
              </template>
              <text v-for="at in ticks" :key="at" :x="x(at)" :y="REQ_H - 4" text-anchor="middle">{{ clock(at) }}</text>
            </g>
            <g v-for="b in buckets" :key="b.start" :class="{ dim: reqHover && reqHover !== b }">
              <rect
                v-for="s in segments(b)"
                :key="s.k"
                :class="s.k"
                :x="Math.max(LEFT, x(b.start) + 1)"
                :y="s.y"
                :width="barW"
                :height="s.h"
              />
            </g>
          </svg>
          <div v-if="reqHover" class="tip" :style="tipAt(x(reqHover.start) + barW / 2)" aria-hidden="true">
            <strong>
              {{ clockSecs(reqHover.start) }} · {{ t('profiler-request-count', { count: reqHover.count }) }}
            </strong>
            <span v-for="k in PARTS" :key="k">
              <i class="sw" :class="k" />{{ partLabel(k) }}<b>{{ duration(reqHover.parts[k]) }}</b>
            </span>
            <em v-if="reqHover.slowest">
              {{
                t('profiler-slowest', { route: routeName(reqHover.slowest), time: duration(reqHover.slowest.total) })
              }}
            </em>
          </div>
        </div>
        <p class="summary">{{ requestSummary }}</p>

        <h4>{{ t('profiler-routes') }}</h4>
        <div class="table">
          <table>
            <thead>
              <tr>
                <th scope="col">{{ t('profiler-route') }}</th>
                <th scope="col" class="n">{{ t('profiler-count') }}</th>
                <th scope="col" class="n">{{ t('profiler-average') }}</th>
                <th scope="col" class="n">{{ t('profiler-p95') }}</th>
                <th scope="col">{{ t('profiler-split') }}</th>
                <th scope="col" class="n">{{ t('profiler-sent') }}</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="r in routes" :key="r.key">
                <th scope="row" class="route" :title="r.name">{{ r.name }}</th>
                <td class="n">{{ formatNumber(r.count) }}</td>
                <td class="n">{{ duration(r.avg) }}</td>
                <td class="n">{{ duration(r.p95) }}</td>
                <td>
                  <span class="split" role="img" :aria-label="routeSplitText(r)" :title="routeSplitText(r)">
                    <i v-for="p in routeSplit(r)" :key="p.k" :class="p.k" :style="{ width: `${p.w}%` }" />
                  </span>
                </td>
                <td class="n">{{ humanBytes(r.bytes) }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </template>
    </template>
  </section>
</template>

<style scoped>
/* The section is a card like the rest of Settings' (that look is SettingsPage's). Request parts take colors checked
   to stay apart for color-blind eyes, in light and dark; the gray "other" is the recessive one. */
.profiler {
  --p-db: light-dark(#2f6fae, #7fb4e6);
  --p-disk: light-dark(#c98a12, #e6b450);
  --p-send: light-dark(#a3365a, #e0708f);
  --p-other: light-dark(#bcc2cc, #585f6e);
  display: grid; gap: 0.5rem; font-size: var(--step--1);
}
p { margin: 0; }
.what { color: var(--muted); line-height: 1.45; max-width: 40rem; }
.bad { color: var(--danger); }
h4 { margin: 0.9rem 0 0; font-size: var(--step-0); font-weight: 650; }
.summary { color: var(--muted); font-variant-numeric: tabular-nums; }

.legend {
  display: flex; flex-wrap: wrap; gap: 0.35rem 1.1rem; margin: 0; padding: 0; list-style: none; color: var(--muted);
}
.legend li { display: inline-flex; align-items: center; gap: 0.4rem; }
.legend b { color: var(--text); font-weight: 600; font-variant-numeric: tabular-nums; }
.legend .aside { margin-left: auto; }
.sw {
  display: inline-block; flex: none; width: 0.75rem; height: 0.75rem; border-radius: 3px; background: var(--p-other);
}
.sw.hoard { background: var(--accent); }
.sw.machine { background: var(--muted); }
.sw.db { background: var(--p-db); }
.sw.disk { background: var(--p-disk); }
.sw.send { background: var(--p-send); }

.chart { position: relative; }
svg { display: block; overflow: visible; touch-action: pan-y; }
.grid line { stroke: var(--line); stroke-width: 1; }
.grid text, .lanes text { fill: var(--muted); font-size: 11px; font-variant-numeric: tabular-nums; }
.ln { fill: none; stroke-width: 2; stroke-linejoin: round; stroke-linecap: round; }
.ln.hoard { stroke: var(--accent); }
.ln.machine { stroke: var(--muted); }
.area { fill: color-mix(in srgb, var(--accent) 14%, transparent); }
.cross { stroke: var(--faint); stroke-width: 1; stroke-dasharray: 3 3; }
.dot { stroke: var(--card, var(--panel)); stroke-width: 2; }
.dot.hoard { fill: var(--accent); }
.dot.machine { fill: var(--muted); }
.lanes .track { stroke: var(--line); stroke-width: 1; }
.lanes .span { fill: var(--accent); }
rect.db { fill: var(--p-db); }
rect.disk { fill: var(--p-disk); }
rect.send { fill: var(--p-send); }
rect.other { fill: var(--p-other); }
.dim { opacity: 0.45; }

.tip {
  position: absolute; top: 4px; z-index: 2; display: grid; gap: 0.2rem; min-width: 13rem; max-width: 20rem;
  padding: 0.5rem 0.65rem; border: 1px solid var(--line); border-radius: 8px; background: var(--raised);
  box-shadow: var(--elev-2, var(--elev-1)); pointer-events: none; color: var(--text);
}
.tip span { display: flex; align-items: center; gap: 0.4rem; white-space: nowrap; }
.tip b { margin-left: auto; padding-left: 0.75rem; font-weight: 600; font-variant-numeric: tabular-nums; }
.tip em { color: var(--muted); font-style: normal; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }

.table { overflow-x: auto; }
table { width: 100%; border-collapse: collapse; font-variant-numeric: tabular-nums; }
th, td { padding: 0.35rem 0.5rem; text-align: left; white-space: nowrap; }
thead th { color: var(--muted); font-weight: 600; border-bottom: 1px solid var(--line); }
tbody tr + tr > * { border-top: 1px solid color-mix(in srgb, var(--line) 60%, transparent); }
.route {
  max-width: 22rem; overflow: hidden; text-overflow: ellipsis; font-weight: 400;
  font-family: var(--mono, ui-monospace, monospace); font-size: 0.95em;
}
.n { text-align: right; }
.split {
  display: flex; gap: 1px; width: 8rem; height: 0.6rem; border-radius: 3px; overflow: hidden;
  background: var(--well, var(--line));
}
.split i { display: block; height: 100%; background: var(--p-other); }
.split i.db { background: var(--p-db); }
.split i.disk { background: var(--p-disk); }
.split i.send { background: var(--p-send); }
</style>
