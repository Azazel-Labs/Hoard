<script setup lang="ts">
// Settings → Store files → teaching Hoard a file: the file on one side, a format for it on the other. Pick a field,
// click its value in the file, and the wizard writes the selector; what the format reads in the file, and in the
// library's files like it, shows as it changes. The format can be used here, or shared as a zip to send in.
import { computed, onBeforeUnmount, onMounted, reactive, ref, watch } from 'vue'
import { api, authHeaders, downloadFormatShare, type FilePlace, type FormatTrial, type StoreFormat } from '../api'
import { confirmDialog, showError, showOk } from '../ui'
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
    if (body.length > MAX_STORE_FILE) loadError.value = 'This file is too big to be read as a store’s.'
    text.value = body
    sample.value = body
    if (ext === 'json') {
      try {
        json.value = JSON.parse(body)
      } catch {
        loadError.value = 'This file isn’t valid JSON, so it’s shown as text.'
      }
    }
    draft.syntax = isJson.value ? 'json' : 'text'
  } catch (e) {
    loadError.value = `Couldn’t read the file: ${(e as Error).message}`
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
const activeLabel = computed(() => FIELDS.find((f) => f.key === active.value)?.label ?? '')

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
      title: 'Go back to the form?',
      body: 'Changes made in the TOML are lost: the form writes the format again from what it has.',
      confirm: 'Back to the form',
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
    const t = await api.tryStoreFormat(currentToml.value, example.value)
    if (n === seq) trial.value = t
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
  name: (file) => `Its files don’t take the name ${file}. Add it to Files.`,
  not_the_stores: () => 'What it recognises the store’s files by isn’t in this file. Change Recognise by.',
  no_title: () => 'It finds no title in this file. Pick the title.',
}
const miss = computed(() => {
  const m = trial.value?.example?.miss
  return m ? MISSES[m](fileName.value) : ''
})
const MISS_SHORT: Record<string, string> = { name: 'not its name', not_the_stores: 'not the store’s', no_title: 'no title' }
const lib = computed(() => trial.value?.library ?? null)
const libRead = computed(() => lib.value?.tried.filter((t) => t.title).length ?? 0)

// ---------------------------------------------------------------- using and sharing it

const saving = ref(false)
const canSave = computed(() => !!name.value && !!trial.value && !trial.value.error && !trying.value)
async function save() {
  saving.value = true
  try {
    const r = await api.saveStoreFormat(currentToml.value, replaces.value)
    showOk(
      r.packs
        ? `Saved ${r.file}. ${r.packs.toLocaleString()} ${r.packs === 1 ? 'pack is' : 'packs are'} being read again.`
        : `Saved ${r.file}.`,
    )
    emit('done', true)
  } catch (e) {
    showError(`Couldn’t save the format: ${(e as Error).message}`)
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
    showError(`Couldn’t make the zip: ${(e as Error).message}`)
  } finally {
    downloading.value = false
  }
}

async function cancel() {
  if (
    (asToml.value ? toml.value !== (props.format?.toml ?? '') : FIELDS.some((f) => draft.fields[f.key] !== '')) &&
    !(await confirmDialog({ title: 'Stop teaching this file?', body: 'The format isn’t saved.', confirm: 'Stop', danger: true }))
  )
    return
  emit('done', false)
}

const show = (v: string | null | undefined) => (v && v.trim() ? v : '—')
</script>

<template>
  <section class="wiz" aria-label="Format wizard">
    <header class="top">
      <div>
        <h3>{{ format && !place ? (format.custom ? `Changing ${format.name}` : `Starting from ${format.name}`) : 'Teach Hoard a store’s file' }}</h3>
        <p v-if="place" class="what">
          <span class="mono">{{ place.path }}</span> in <strong>{{ place.pack_name }}</strong>
        </p>
      </div>
      <button type="button" class="btn ghost small" @click="toggleToml">{{ asToml ? 'Back to the form' : 'Edit as TOML' }}</button>
    </header>

    <div class="panes" :class="{ single: !place }">
      <div v-if="place" class="file">
        <p class="armed" :class="{ on: active && !asToml }" aria-live="polite">
          <template v-if="asToml">The file, for reference.</template>
          <template v-else-if="active">Click the {{ activeLabel.toLowerCase() }} in the file{{ isJson ? '' : ', or select it' }}.</template>
          <template v-else>Pick a field on the right, then click its value here.</template>
        </p>
        <p v-if="loadError" class="what warn">{{ loadError }}</p>
        <p v-if="text === null && !loadError" class="what">Loading…</p>
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
          <label class="lbl" for="wiz-toml">The format</label>
          <textarea id="wiz-toml" v-model="toml" class="field mono toml" spellcheck="false" rows="18" />
          <p class="what">The syntax is in <span class="mono">docs/store-formats.md</span>.</p>
        </template>
        <template v-else>
          <label class="row2">
            <span class="lbl">Store</span>
            <input v-model="draft.name" class="field" placeholder="The store’s name, like Kenney" autocomplete="off" />
          </label>
          <label class="row2">
            <span class="lbl">Files</span>
            <input v-model="draft.files" class="field mono" placeholder="asset_info.json, *.nfo" spellcheck="false" />
          </label>
          <label class="row2 top-align">
            <span class="lbl">Recognise by</span>
            <textarea
              v-model="draft.require"
              class="field mono"
              rows="2"
              spellcheck="false"
              :placeholder="draft.syntax === 'json' ? 'A key only the store’s files have' : 'A line only the store’s files have'"
              @input="touchRequire"
            />
          </label>
          <p class="what indent">One way a line; any of them will do. Needed when other files have the same name.</p>
          <label class="row2">
            <span class="lbl">Seller</span>
            <input v-model="draft.seller" class="field" placeholder="When the file doesn’t say" autocomplete="off" />
          </label>
          <label class="row2">
            <span class="lbl">Tags</span>
            <input v-model="draft.tags" class="field" placeholder="For everything it describes, comma-separated" autocomplete="off" />
          </label>

          <h4>Fields</h4>
          <ul class="fields">
            <li v-for="f in FIELDS" :key="f.key" :class="{ armed: active === f.key }">
              <span class="lbl" :title="f.hint">{{ f.label }}</span>
              <input
                v-model="draft.fields[f.key]"
                class="field mono"
                spellcheck="false"
                :aria-label="`${f.label} selector`"
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
                Pick
              </button>
              <button
                type="button"
                class="btn ghost small"
                :class="{ unseen: !draft.fields[f.key] }"
                :aria-label="`Clear ${f.label}`"
                :disabled="!draft.fields[f.key]"
                @click="clearField(f.key)"
              >
                ×
              </button>
            </li>
          </ul>
        </template>

        <div class="preview" aria-live="polite">
          <h4>What it reads <span v-if="trying" class="muted">…</span></h4>
          <p v-if="trial?.error" class="what warn">{{ trial.error }}</p>
          <template v-else-if="trial">
            <template v-if="place">
              <dl v-if="read" class="read">
                <dt>Title</dt><dd data-read="title">{{ read.title }}</dd>
                <dt>Seller</dt><dd data-read="seller">{{ show(read.seller) }}</dd>
                <dt>License</dt><dd>{{ show(read.license) }}</dd>
                <dt>Store page</dt><dd class="mono">{{ show(read.url) }}</dd>
                <dt>Kind</dt><dd>{{ show(read.asset_type) }}</dd>
                <dt>Tags</dt><dd data-read="tags">{{ read.tags.length ? read.tags.join(', ') : '—' }}</dd>
                <template v-if="read.origin"><dt>Made in</dt><dd>{{ read.origin }}</dd></template>
                <template v-if="read.size"><dt>Size</dt><dd>{{ read.size }}</dd></template>
              </dl>
              <p v-else-if="miss" class="what warn">{{ miss }}</p>
              <p v-if="trial.example?.also" class="what">
                {{ trial.example.also }} reads this file too. This format would be tried first.
              </p>
            </template>
            <template v-if="lib">
              <p class="what">
                <template v-if="lib.files">
                  {{ lib.files.toLocaleString() }} {{ lib.files === 1 ? 'file' : 'files' }} in
                  {{ lib.packs.toLocaleString() }} {{ lib.packs === 1 ? 'pack' : 'packs' }} have a name it takes.
                  Of {{ lib.tried.length }} tried, it reads {{ libRead }}.
                </template>
                <template v-else>No file in the library has a name it takes.</template>
              </p>
              <ul v-if="lib.tried.length" class="tried">
                <li v-for="t in lib.tried.slice(0, 12)" :key="t.place.asset_id">
                  <span class="grow">{{ t.place.pack_name }}</span>
                  <span v-if="t.title" class="ok">{{ t.title }}</span>
                  <span v-else class="muted">{{ MISS_SHORT[t.miss ?? ''] ?? '' }}</span>
                </li>
              </ul>
            </template>
          </template>
        </div>

        <div v-if="sharing" class="share">
          <h4>Share it</h4>
          <p class="what">
            A zip of the format, this file as its sample and what the sample reads as, laid out like Hoard’s repository,
            to send in so everyone gets it. Take out anything in the sample that shouldn’t be shared first.
          </p>
          <textarea v-model="sample" class="field mono" rows="8" spellcheck="false" aria-label="Sample to share" />
          <div class="row">
            <button type="button" class="btn primary" :disabled="downloading || !name || !sample.trim()" @click="download">
              Download {{ shareName }}
            </button>
            <button type="button" class="btn ghost" @click="sharing = false">Close</button>
          </div>
        </div>

        <div class="row actions">
          <button type="button" class="btn primary" :disabled="!canSave || saving" @click="save">Use it here</button>
          <button v-if="place && text !== null && !sharing" type="button" class="btn ghost" :disabled="!read" @click="sharing = true">
            Share it
          </button>
          <button type="button" class="btn ghost" @click="cancel">Cancel</button>
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
