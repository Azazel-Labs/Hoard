<script setup lang="ts">
// "Download as": the asset in another format or smaller, made on the server (its progress on the button), or the
// pack's own file in that format when it has one.
import { computed, ref, watch } from 'vue'
import { api, asUrl, fileUrl, RESIZABLE_AS_ITSELF, type Asset, type ConversionStatus } from '../api'
import { humanBytes } from '../format'
import { extOf, formatsOf } from '../assetLinks'
import { t } from '../i18n'

const props = defineProps<{ asset: Asset }>()
const otherFormats = computed(() => formatsOf(props.asset))

// "Download as": another format, or the same one scaled down. '' = the original format.
// A conversion to a format the pack has the asset in already is left out: its own file is offered instead.
const asFormats = computed(() => {
  const own = new Set(otherFormats.value.map((l) => extOf(l.path).toLowerCase()))
  return (props.asset.formats ?? []).filter((f) => !own.has(f.format))
})
const PACK_FILE = 'file:'
const packFile = computed(() =>
  asFormat.value.startsWith(PACK_FILE) ? (otherFormats.value.find((l) => PACK_FILE + l.id === asFormat.value) ?? null) : null,
)
const sameFormat = computed(() => RESIZABLE_AS_ITSELF[props.asset.ext.toLowerCase()] ?? null)
const asFormat = ref('')
const asMax = ref(0)
const defaultAs = () =>
  sameFormat.value ? '' : (asFormats.value[0]?.format ?? (otherFormats.value[0] ? PACK_FILE + otherFormats.value[0].id : ''))
watch(() => props.asset.id, () => {
  asFormat.value = defaultAs()
  asMax.value = 0
}, { immediate: true })
// The format list arrives with the full asset detail, after the tile's summary: pick a default once it's there.
watch(asFormats, (list) => {
  const valid =
    asFormat.value === '' ? sameFormat.value !== null : list.some((f) => f.format === asFormat.value) || packFile.value !== null
  if (!valid) asFormat.value = defaultAs()
})
const asResizable = computed(() =>
  asFormat.value === '' ? sameFormat.value !== null : asFormats.value.some((f) => f.format === asFormat.value && f.resizable),
)
/** Powers of two below the image's longest side, largest first. */
const sizeChoices = computed(() => {
  const w = Number(props.asset.meta.width ?? 0)
  const h = Number(props.asset.meta.height ?? 0)
  const out: number[] = []
  for (let m = 8192; m >= 64; m /= 2) if (m < Math.max(w, h)) out.push(m)
  return out.slice(0, 6)
})
const asHref = computed(() => {
  if (packFile.value) return fileUrl(packFile.value, true)
  const max = asResizable.value ? asMax.value : 0
  if (asFormat.value === '') return sameFormat.value && max ? asUrl(props.asset, sameFormat.value, max) : null
  return asUrl(props.asset, asFormat.value, max || undefined)
})
// Conversions run on the server in the background: poll until it's made, then download. Cached ones are instant.
const asState = ref<ConversionStatus | null>(null)
const asError = ref('')
let asRun = 0
watch([() => props.asset.id, asFormat, asMax], () => {
  asRun++
  asState.value = null
  asError.value = ''
})
async function downloadAs(ev: MouseEvent) {
  const href = asHref.value
  if (!href || packFile.value || ev.ctrlKey || ev.metaKey || ev.shiftKey || ev.button !== 0) return
  ev.preventDefault()
  if (asState.value) return
  const run = ++asRun
  const format = asFormat.value || sameFormat.value || ''
  const max = asResizable.value && asMax.value ? asMax.value : undefined
  asError.value = ''
  asState.value = { state: 'queued' }
  try {
    for (;;) {
      const asked = Date.now()
      const st = await api.asStatus(props.asset, format, max)
      if (run !== asRun) return
      if (st.state === 'failed') throw new Error(st.error ?? t('asset-conversion-failed'))
      if (st.state === 'ready') break
      asState.value = st
      // Progress comes back as it moves; a few updates a second is plenty.
      await new Promise((r) => setTimeout(r, Math.max(0, 300 - (Date.now() - asked))))
    }
    const a = document.createElement('a')
    a.href = href
    a.download = ''
    document.body.appendChild(a)
    a.click()
    a.remove()
  } catch (e) {
    if (run === asRun) asError.value = e instanceof Error ? e.message : String(e)
  } finally {
    if (run === asRun) asState.value = null
  }
}
const asLabel = computed(() => {
  const st = asState.value
  if (st?.state === 'converting') {
    return st.progress !== undefined ? t('asset-as-converting-percent', { percent: Math.round(st.progress * 100) }) : t('asset-as-converting')
  }
  if (st) return t('asset-as-queued')
  return t('asset-download')
})
/** What a sprite sheet format can say, and so how the sheet comes out in it. */
const SHEET_NOTE: Record<string, () => string> = {
  'sheet-hoard': () => t('sheet-note-hoard'),
  'sheet-tp-hash': () => t('sheet-note-texturepacker'),
  'sheet-tp-array': () => t('sheet-note-texturepacker'),
  'sheet-starling': () => t('sheet-note-clockwise'),
  'sheet-cocos2d': () => t('sheet-note-clockwise'),
  'sheet-libgdx': () => t('sheet-note-libgdx'),
  'sheet-godot-frames': () => t('sheet-note-godot-frames'),
  'sheet-godot-tileset': () => t('sheet-note-godot-tileset'),
  'sheet-grid': () => t('sheet-note-grid'),
  'sheet-frames': () => t('sheet-note-frames'),
}
const asNote = computed(() => SHEET_NOTE[asFormat.value]?.() ?? '')
const showAs = computed(
  () => asFormats.value.length > 0 || otherFormats.value.length > 0 || (sameFormat.value !== null && sizeChoices.value.length > 0),
)
</script>

<template>
  <template v-if="showAs">
    <div class="asrow">
      <label for="as-format">{{ t('asset-as-label') }}</label>
      <select id="as-format" v-model="asFormat" class="field">
        <option v-if="sameFormat" value="">{{ t('asset-as-original', { format: asset.ext.toUpperCase() }) }}</option>
        <option v-for="f in asFormats" :key="f.format" :value="f.format">{{ f.label }}</option>
        <optgroup v-if="otherFormats.length" :label="t('asset-as-pack-files')">
          <option v-for="l in otherFormats" :key="l.id" :value="PACK_FILE + l.id">
            {{ extOf(l.path) }} · {{ humanBytes(l.size ?? 0) }}
          </option>
        </optgroup>
      </select>
      <select v-if="asResizable && sizeChoices.length" v-model.number="asMax" class="field size" :aria-label="t('asset-as-size')">
        <option :value="0">{{ t('asset-as-full-size') }}</option>
        <option v-for="m in sizeChoices" :key="m" :value="m">{{ t('asset-as-up-to', { size: String(m) }) }}</option>
      </select>
      <a v-if="asHref" class="btn" :href="asHref" :aria-busy="asState ? 'true' : 'false'" @click="downloadAs">{{ asLabel }}</a>
      <span v-else class="ashint">{{ t('asset-as-hint') }}</span>
      <span v-if="asError" class="aserror" role="alert">{{ asError }}</span>
    </div>
    <p v-if="asNote" class="asnote">{{ asNote }}</p>
  </template>
</template>

<style scoped>
.asrow { display: flex; flex-wrap: wrap; align-items: center; gap: 0.4rem; font-size: var(--step--1); color: var(--muted); }
.asrow > label { flex: none; margin-right: 0.15rem; }
.asrow .field { flex: 1 1 7rem; width: auto; min-width: 0; font-size: var(--step--1); padding: 0.42em 0.55em; }
.asrow .field.size { flex: 0 1 8.5rem; }
.asrow .btn { flex: none; }
.ashint { color: var(--faint); }
.asnote { margin: -0.2rem 0 0; font-size: var(--step--1); color: var(--muted); }
.aserror { flex-basis: 100%; color: var(--danger); }
.asrow .btn[aria-busy='true'] { cursor: progress; font-variant-numeric: tabular-nums; }
</style>
