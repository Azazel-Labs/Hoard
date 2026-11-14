<script setup lang="ts">
// Settings → Store files → teaching Hoard a file: the file on one side, a format for it on the other. Pick a field,
// click its value in the file, and the wizard writes the selector; what the format reads in the file, and in the
// library's files like it, shows as it changes. The format can be used here, or shared as a zip to send in.
import { computed, onBeforeUnmount, onMounted, reactive, ref, watch } from 'vue'
import { api, authHeaders, downloadFormatShare, type FilePlace, type FormatTrial, type StoreFormat } from '../api'
import { confirmDialog, showError, showOk } from '../ui'
import { t } from '../i18n'
import JsonPicker from './JsonPicker.vue'
import {
  FIELDS,
  MAX_STORE_FILE,
  draftToml,
  emptyDraft,
  jsonSelector,
  pathKey,
  slug,
  textRequire,
  textSelector,
  tomlName,
  valueStart,
  type FieldKey,
  type JsonPath,
} from '../storeFormats'

const props = defineProps<{ place?: FilePlace | null; format?: StoreFormat | null }>()
const emit = defineEmits<{ done: [saved: boolean] }>()

const fileName = computed(() => props.place?.path.split(/[\\/]/).pop() ?? '')
/** The file as it was read, or null while it loads (or when there's none). */
const text = ref<string | null>(null)
const loadError = ref('')
const json = ref<unknown>(undefined)
const isJson = computed(() => json.value !== undefined)
const lines = computed(() => (text.value ?? '').split(/\r?\n/))

const draft = reactive(emptyDraft())
/** TOML mode: the format as text, for what the form doesn't do (sizes, maps, levels of detail) and formats already
 *  written. */
const asToml = ref(false)
const toml = ref('')
/** The file of the format being changed, when it's one of the data folder's. */
const replaces = computed(() => (props.format?.custom ? props.format.file : null))

/** The field the next click in the file fills in. */
const active = ref<FieldKey | null>('title')
/** Where each field's value was picked: a path in JSON, a line in text. */
const pickedPath = reactive<Partial<Record<FieldKey, JsonPath>>>({})
const pickedLine = reactive<Partial<Record<FieldKey, number>>>({})
/** "Recognise by" was written by hand, so picking the title doesn't suggest it again. */
let requireTouched = false
const touchRequire = () => {
  requireTouched = true
}

const currentToml = computed(() => (asToml.value ? toml.value : draftToml(draft, fileName.value || undefined)))
const name = computed(() => (asToml.value ? tomlName(toml.value) : draft.name).trim())
const shareName = computed(() => `hoard-format-${slug(name.value) || 'store'}.zip`)

onMounted(async () => {
  if (props.format) {
    asToml.value = true
    toml.value = props.format.toml
  }
  if (!props.place) return
  const ext = fileName.value.toLowerCase().split('.').pop() ?? ''
  draft.files = fileName.value.toLowerCase()
  try {
    const res = await fetch(`/api/assets/${encodeURIComponent(props.place.asset_id)}/file`, { headers: authHeaders() })
    if (!res.ok) throw new Error(`${res.status} ${res.statusText}`)
    const body = await res.text()
    if (body.length > MAX_STORE_FILE) loadError.value = t('formatwizard-too-big')
    text.value = body
    sample.value = body
    if (ext === 'json') {
      try {
        json.value = JSON.parse(body)
      } catch {
        loadError.value = t('formatwizard-not-json')
      }
    }
    draft.syntax = isJson.value ? 'json' : 'text'
  } catch (e) {
    loadError.value = t('formatwizard-read-failed', { error: (e as Error).message })
  }
})

// ---------------------------------------------------------------- picking values

const marks = computed(() => {
  const m: Record<string, string> = {}
  for (const f of FIELDS) {
    const p = pickedPath[f.key]
    if (p && draft.fields[f.key]) m[pathKey(p)] = f.label
  }
  return m
})
const lineMarks = computed(() => {
  const m = new Map<number, string[]>()
  for (const f of FIELDS) {
    const i = pickedLine[f.key]
    if (i !== undefined && draft.fields[f.key]) m.set(i, [...(m.get(i) ?? []), f.label])
  }
  return m
})
const activeName = computed(() => FIELDS.find((f) => f.key === active.value)?.name ?? '')

function pickJson(path: JsonPath) {
  const f = FIELDS.find((x) => x.key === active.value)
  if (!f || asToml.value) return
  draft.fields[f.key] = jsonSelector(json.value, path, f.many)
  pickedPath[f.key] = path
  if (f.key === 'title' && !requireTouched) {
    // The title's object, like `item` in {"item": {"title": …}}, tells the file apart from other JSON.
    draft.require = path.length > 1 && typeof path[0] === 'string' ? jsonSelector(json.value, [path[0]]) : ''
  }
  next()
}

/** A line was clicked, or text in it selected: the selection, or the line's value, is the field's. */
function pickText(i: number, ev: MouseEvent) {
  const f = FIELDS.find((x) => x.key === active.value)
  if (!f || asToml.value) return
  const line = lines.value[i]
  let start = valueStart(line)
  let end = line.trimEnd().length
  const sel = window.getSelection()
  const el = ev.currentTarget as HTMLElement
  if (sel && !sel.isCollapsed && sel.rangeCount) {
    const r = sel.getRangeAt(0)
    if (r.startContainer !== r.endContainer || r.startContainer.parentElement !== el) return
    start = r.startOffset
    end = r.endOffset
    if (!line.slice(start, end).trim()) return
  }
  if (start >= end) return
  draft.fields[f.key] = textSelector(line, start, end)
  pickedLine[f.key] = i
  if (f.key === 'title' && !requireTouched) draft.require = textRequire(lines.value, line.slice(start, end))
  sel?.removeAllRanges()
  next()
}

/** After a pick, nothing is armed: picking another field is a click on its Pick. */
function next() {
  active.value = null
}

function clearField(k: FieldKey) {
  draft.fields[k] = ''
  delete pickedPath[k]
  delete pickedLine[k]
}

// ---------------------------------------------------------------- TOML mode

async function toggleToml() {
  if (!asToml.value) {
    toml.value = draftToml(draft, fileName.value || undefined)
    asToml.value = true
    return
  }
  const changed = toml.value !== draftToml(draft, fileName.value || undefined)
  if (
    changed &&
    !(await confirmDialog({
      title: t('formatwizard-back-title'),
      body: t('formatwizard-back-body'),
      confirm: t('formatwizard-back-to-form'),
      danger: true,
    }))
  )
    return
  asToml.value = false
}

// ---------------------------------------------------------------- trying it

const trial = ref<FormatTrial | null>(null)
const trying = ref(false)
let seq = 0
let debounce: ReturnType<typeof setTimeout> | undefined
const example = computed(() => (props.place && text.value !== null ? { path: props.place.path, text: text.value } : null))
async function tryIt() {
  const n = ++seq
  trying.value = true
  try {
    const res = await api.tryStoreFormat(currentToml.value, example.value)
    if (n === seq) trial.value = res
  } catch (e) {
    if (n === seq) trial.value = { error: (e as Error).message }
  } finally {
    if (n === seq) trying.value = false
  }
}
watch([currentToml, example], () => {
  clearTimeout(debounce)
  debounce = setTimeout(tryIt, 400)
}, { immediate: true })
onBeforeUnmount(() => clearTimeout(debounce))

const read = computed(() => trial.value?.example?.read ?? null)
const MISSES: Record<string, (file: string) => string> = {
  name: (file) => t('formatwizard-miss-name', { file }),
  not_the_stores: () => t('formatwizard-miss-not-the-stores'),
  no_title: () => t('formatwizard-miss-no-title'),
}
const miss = computed(() => {
  const m = trial.value?.example?.miss
  return m ? MISSES[m](fileName.value) : ''
})
const MISS_SHORT: Record<string, string> = {
  get name() {
    return t('formatwizard-miss-short-name')
  },
  get not_the_stores() {
    return t('formatwizard-miss-short-not-the-stores')
  },
  get no_title() {
    return t('formatwizard-miss-short-no-title')
  },
}
const lib = computed(() => trial.value?.library ?? null)
const libRead = computed(() => lib.value?.tried.filter((x) => x.title).length ?? 0)

// ---------------------------------------------------------------- using and sharing it

const saving = ref(false)
const canSave = computed(() => !!name.value && !!trial.value && !trial.value.error && !trying.value)
async function save() {
  saving.value = true
  try {
    const r = await api.saveStoreFormat(currentToml.value, replaces.value)
    showOk(
      r.packs ? t('formatwizard-saved-packs', { file: r.file, packs: r.packs }) : t('formatwizard-saved', { file: r.file }),
    )
    emit('done', true)
  } catch (e) {
    showError(t('formatwizard-save-failed', { error: (e as Error).message }))
  } finally {
    saving.value = false
  }
}

const sharing = ref(false)
/** The sample sent with the format, which can be trimmed of anything private first. */
const sample = ref('')
const downloading = ref(false)
async function download() {
  downloading.value = true
  try {
    await downloadFormatShare(currentToml.value, { path: fileName.value, text: sample.value }, shareName.value)
  } catch (e) {
    showError(t('formatwizard-zip-failed', { error: (e as Error).message }))
  } finally {
    downloading.value = false
  }
}

async function cancel() {
  if (
    (asToml.value ? toml.value !== (props.format?.toml ?? '') : FIELDS.some((f) => draft.fields[f.key] !== '')) &&
    !(await confirmDialog({
      title: t('formatwizard-stop-title'),
      body: t('formatwizard-stop-body'),
      confirm: t('formatwizard-stop'),
      danger: true,
    }))
  )
    return
  emit('done', false)
}

const show = (v: string | null | undefined) => (v && v.trim() ? v : '—')
</script>

<template>
  <section class="wiz" :aria-label="t('formatwizard-label')">
    <header class="top">
      <div>
        <h3>
          {{
            format && !place
              ? format.custom
                ? t('formatwizard-changing', { name: format.name })
                : t('formatwizard-starting-from', { name: format.name })
              : t('formatwizard-title')
          }}
        </h3>
        <p v-if="place" class="what">
          {{ t('formatwizard-place-1') }}<span class="mono">{{ place.path }}</span>{{ t('formatwizard-place-2') }}<strong>{{ place.pack_name }}</strong>{{ t('formatwizard-place-3') }}
        </p>
      </div>
      <button type="button" class="btn ghost small" @click="toggleToml">{{ asToml ? t('formatwizard-back-to-form') : t('formatwizard-edit-toml') }}</button>
    </header>

    <div class="panes" :class="{ single: !place }">
      <div v-if="place" class="file">
        <p class="armed" :class="{ on: active && !asToml }" aria-live="polite">
          <template v-if="asToml">{{ t('formatwizard-file-reference') }}</template>
          <template v-else-if="active">{{
            isJson
              ? t('formatwizard-click-json', { field: activeName })
              : t('formatwizard-click-text', { field: activeName })
          }}</template>
          <template v-else>{{ t('formatwizard-pick-first') }}</template>
        </p>
        <p v-if="loadError" class="what warn">{{ loadError }}</p>
        <p v-if="text === null && !loadError" class="what">{{ t('formatwizard-loading') }}</p>
        <div v-else-if="isJson" class="src mono">
          <JsonPicker :value="json" :marks="marks" @pick="pickJson" />
        </div>
        <div v-else-if="text !== null" class="src mono lines">
          <div v-for="(l, i) in lines.slice(0, 2000)" :key="i" class="ln-row">
            <span class="ln" :class="{ on: lineMarks.has(i) }" @mouseup="pickText(i, $event)">{{ l || ' ' }}</span>
            <span v-for="m in lineMarks.get(i) ?? []" :key="m" class="mark">{{ m }}</span>
          </div>
        </div>
      </div>

      <div class="form">
        <template v-if="asToml">
          <label class="lbl" for="wiz-toml">{{ t('formatwizard-the-format') }}</label>
          <textarea id="wiz-toml" v-model="toml" class="field mono toml" spellcheck="false" rows="18" />
          <p class="what">{{ t('formatwizard-syntax-before') }}<span class="mono">docs/store-formats.md</span>{{ t('formatwizard-syntax-after') }}</p>
        </template>
        <template v-else>
          <label class="row2">
            <span class="lbl">{{ t('formatwizard-store') }}</span>
            <input v-model="draft.name" class="field" :placeholder="t('formatwizard-store-placeholder')" autocomplete="off" />
          </label>
          <label class="row2">
            <span class="lbl">{{ t('formatwizard-files') }}</span>
            <input v-model="draft.files" class="field mono" placeholder="asset_info.json, *.nfo" spellcheck="false" />
          </label>
          <label class="row2 top-align">
            <span class="lbl">{{ t('formatwizard-recognise') }}</span>
            <textarea
              v-model="draft.require"
              class="field mono"
              rows="2"
              spellcheck="false"
              :placeholder="draft.syntax === 'json' ? t('formatwizard-recognise-json') : t('formatwizard-recognise-text')"
              @input="touchRequire"
            />
          </label>
          <p class="what indent">{{ t('formatwizard-recognise-hint') }}</p>
          <label class="row2">
            <span class="lbl">{{ t('formatwizard-seller') }}</span>
            <input v-model="draft.seller" class="field" :placeholder="t('formatwizard-seller-placeholder')" autocomplete="off" />
          </label>
          <label class="row2">
            <span class="lbl">{{ t('formatwizard-tags') }}</span>
            <input v-model="draft.tags" class="field" :placeholder="t('formatwizard-tags-placeholder')" autocomplete="off" />
          </label>

          <h4>{{ t('formatwizard-fields') }}</h4>
          <ul class="fields">
            <li v-for="f in FIELDS" :key="f.key" :class="{ armed: active === f.key }">
              <span class="lbl" :title="f.hint">{{ f.label }}</span>
              <input
                v-model="draft.fields[f.key]"
                class="field mono"
                spellcheck="false"
                :aria-label="t('formatwizard-selector', { field: f.label })"
                :placeholder="f.hint"
              />
              <button
                v-if="place && text !== null"
                type="button"
                class="btn small"
                :class="active === f.key ? 'primary' : 'ghost'"
                :aria-pressed="active === f.key"
                @click="active = active === f.key ? null : f.key"
              >
                {{ t('formatwizard-pick') }}
              </button>
              <button
                type="button"
                class="btn ghost small"
                :class="{ unseen: !draft.fields[f.key] }"
                :aria-label="t('formatwizard-clear', { field: f.label })"
                :disabled="!draft.fields[f.key]"
                @click="clearField(f.key)"
              >
                ×
              </button>
            </li>
          </ul>
        </template>

        <div class="preview" aria-live="polite">
          <h4>{{ t('formatwizard-what-it-reads') }} <span v-if="trying" class="muted">…</span></h4>
          <p v-if="trial?.error" class="what warn">{{ trial.error }}</p>
          <template v-else-if="trial">
            <template v-if="place">
              <dl v-if="read" class="read">
                <dt>{{ t('formatwizard-read-title') }}</dt><dd data-read="title">{{ read.title }}</dd>
                <dt>{{ t('formatwizard-read-seller') }}</dt><dd data-read="seller">{{ show(read.seller) }}</dd>
                <dt>{{ t('formatwizard-read-license') }}</dt><dd>{{ show(read.license) }}</dd>
                <dt>{{ t('formatwizard-read-url') }}</dt><dd class="mono">{{ show(read.url) }}</dd>
                <dt>{{ t('formatwizard-read-kind') }}</dt><dd>{{ show(read.asset_type) }}</dd>
                <dt>{{ t('formatwizard-read-tags') }}</dt><dd data-read="tags">{{ read.tags.length ? read.tags.join(', ') : '—' }}</dd>
                <template v-if="read.origin"><dt>{{ t('formatwizard-read-origin') }}</dt><dd>{{ read.origin }}</dd></template>
                <template v-if="read.size"><dt>{{ t('formatwizard-read-size') }}</dt><dd>{{ read.size }}</dd></template>
              </dl>
              <p v-else-if="miss" class="what warn">{{ miss }}</p>
              <p v-if="trial.example?.also" class="what">
                {{ t('formatwizard-also', { format: trial.example.also }) }}
              </p>
            </template>
            <template v-if="lib">
              <p class="what">
                <template v-if="lib.files">
                  {{ t('formatwizard-library', { files: lib.files, packs: lib.packs, tried: lib.tried.length, read: libRead }) }}
                </template>
                <template v-else>{{ t('formatwizard-library-none') }}</template>
              </p>
              <ul v-if="lib.tried.length" class="tried">
                <li v-for="x in lib.tried.slice(0, 12)" :key="x.place.asset_id">
                  <span class="grow">{{ x.place.pack_name }}</span>
                  <span v-if="x.title" class="ok">{{ x.title }}</span>
                  <span v-else class="muted">{{ MISS_SHORT[x.miss ?? ''] ?? '' }}</span>
                </li>
              </ul>
            </template>
          </template>
        </div>

        <div v-if="sharing" class="share">
          <h4>{{ t('formatwizard-share') }}</h4>
          <p class="what">
            {{ t('formatwizard-share-hint') }}
          </p>
          <textarea v-model="sample" class="field mono" rows="8" spellcheck="false" :aria-label="t('formatwizard-sample')" />
          <div class="row">
            <button type="button" class="btn primary" :disabled="downloading || !name || !sample.trim()" @click="download">
              {{ t('formatwizard-download', { file: shareName }) }}
            </button>
            <button type="button" class="btn ghost" @click="sharing = false">{{ t('formatwizard-close') }}</button>
          </div>
        </div>

        <div class="row actions">
          <button type="button" class="btn primary" :disabled="!canSave || saving" @click="save">{{ t('formatwizard-use') }}</button>
          <button v-if="place && text !== null && !sharing" type="button" class="btn ghost" :disabled="!read" @click="sharing = true">
            {{ t('formatwizard-share') }}
          </button>
          <button type="button" class="btn ghost" @click="cancel">{{ t('formatwizard-cancel') }}</button>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.wiz { display: grid; gap: 1rem; }
.top { display: flex; gap: 1rem; align-items: flex-start; justify-content: space-between; }
.top h3 { margin: 0 0 0.2rem; }
h4 { margin: 0.6rem 0 0.3rem; font-size: 1em; }
.what { margin: 0; color: var(--muted); line-height: 1.5; }
.warn { color: var(--danger, #c0392b); }
.muted { color: var(--muted); }
.panes { display: grid; grid-template-columns: minmax(0, 1fr) minmax(0, 1fr); gap: 1.4rem; align-items: start; }
.panes.single { grid-template-columns: minmax(0, 1fr); max-width: 44rem; }
@media (max-width: 900px) { .panes { grid-template-columns: minmax(0, 1fr); } }
.file { position: sticky; top: 0.5rem; display: grid; gap: 0.5rem; min-width: 0; }
.armed { margin: 0; padding: 0.4rem 0.6rem; border-radius: 6px; background: var(--surface-2, transparent); color: var(--muted); }
.armed.on { background: color-mix(in srgb, var(--accent) 14%, transparent); color: var(--text); }
.src {
  max-height: 70vh; overflow: auto; padding: 0.5rem 0.5rem 0.5rem 0; border: 1px solid var(--line); border-radius: 6px;
  font-size: 0.92em; line-height: 1.45;
}
.lines { padding-left: 0.5rem; }
.ln-row { display: flex; gap: 0.4rem; align-items: baseline; }
.ln { white-space: pre-wrap; overflow-wrap: anywhere; cursor: text; flex: 0 1 auto; min-width: 0; border-radius: 3px; }
.ln:hover { background: color-mix(in srgb, var(--accent) 8%, transparent); }
.ln.on { background: color-mix(in srgb, var(--accent) 16%, transparent); }
.mark {
  font-size: 0.8em; padding: 0 0.4rem; border-radius: 999px; background: var(--accent); color: var(--on-accent, #fff);
  white-space: nowrap;
}
.form { display: grid; gap: 0.5rem; min-width: 0; }
.row2 { display: grid; grid-template-columns: 7.5rem minmax(0, 1fr); gap: 0.6rem; align-items: center; }
.row2.top-align { align-items: start; }
.indent { padding-left: 8.1rem; font-size: 0.92em; }
@media (max-width: 600px) { .row2 { grid-template-columns: minmax(0, 1fr); gap: 0.2rem; } .indent { padding-left: 0; } }
.lbl { color: var(--muted); }
.fields { margin: 0; padding: 0; list-style: none; display: grid; gap: 0.3rem; }
.fields li { display: grid; grid-template-columns: 7.5rem minmax(0, 1fr) auto auto; gap: 0.4rem; align-items: center; }
.unseen { visibility: hidden; }
.fields li.armed .lbl { color: var(--accent); font-weight: 600; }
.toml { min-height: 22rem; font-size: 0.92em; }
.preview { border-top: 1px solid var(--line); margin-top: 0.4rem; display: grid; gap: 0.4rem; }
.read { margin: 0; display: grid; grid-template-columns: 7.5rem minmax(0, 1fr); gap: 0.2rem 0.6rem; }
.read dt { color: var(--muted); }
.read dd { margin: 0; overflow-wrap: anywhere; }
.tried { margin: 0; padding: 0; list-style: none; border-top: 1px solid var(--line); }
.tried li { display: flex; gap: 0.8rem; padding: 0.2rem 0; border-bottom: 1px solid var(--line); }
.grow { flex: 1; min-width: 0; overflow-wrap: anywhere; }
.ok { overflow-wrap: anywhere; text-align: right; }
.share { border-top: 1px solid var(--line); display: grid; gap: 0.5rem; padding-top: 0.2rem; }
.row { display: flex; flex-wrap: wrap; gap: 0.5rem; }
.actions { margin-top: 0.6rem; }
.small { padding-block: 0.1rem; font-size: 0.85em; }
</style>
