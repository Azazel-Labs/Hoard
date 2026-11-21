<script setup lang="ts">
// Above the results while one tag is browsed: the tag's own page. What it means, the asset whose picture stands for
// it, how many have it and the old names merged into it; someone who edits can change any of it, or rename the tag
// (giving another tag's name merges into that one).
import { computed, nextTick, onBeforeUnmount, ref, watch } from 'vue'
import { api, thumbUrl, type Asset, type TagInfo } from '../api'
import { t } from '../i18n'
import { mayEdit } from '../auth'
import { confirmDialog, showError, showOk } from '../ui'

const props = defineProps<{
  tag: string
  /** The first results, to pick the tag's picture from. */
  candidates: Asset[]
}>()
const emit = defineEmits<{ show: [tag: string]; changed: [] }>()

const editor = computed(() => mayEdit())
const info = ref<TagInfo | null>(null)
async function load() {
  const tag = props.tag
  try {
    const got = await api.tagInfo(tag)
    if (props.tag === tag) info.value = got
  } catch {
    // Without it the results still show; the page just isn't there.
    if (props.tag === tag) info.value = null
  }
}
async function save(body: { description?: string; image_id?: string }): Promise<boolean> {
  const tag = props.tag
  try {
    const got = await api.setTagInfo({ tag, ...body })
    if (props.tag === tag) info.value = got
    return true
  } catch (e) {
    showError(t('tag-page-save-failed', { error: (e as Error).message }))
    return false
  }
}

// What it means: click to write it; Enter saves (Shift+Enter starts a new line), Escape gives up.
const editing = ref(false)
const draft = ref('')
const box = ref<HTMLTextAreaElement | null>(null)
function edit() {
  if (!editor.value) return
  draft.value = info.value?.description ?? ''
  editing.value = true
  void nextTick(() => box.value?.focus())
}
async function saveDescription() {
  if (!editing.value) return
  const was = info.value?.description ?? ''
  const now = draft.value.trim()
  editing.value = false
  if (now === was) return
  if (await save({ description: now })) {
    showOk(t('tag-page-description-saved'), { label: t('asset-undo'), run: () => void save({ description: was }) })
  }
}
function onDescKey(e: KeyboardEvent) {
  if (e.key === 'Enter' && !e.shiftKey) {
    e.preventDefault()
    void saveDescription()
  } else if (e.key === 'Escape') {
    e.preventDefault()
    e.stopPropagation()
    editing.value = false
  }
}

// The picture: one of its assets, picked from the first results.
const picking = ref(false)
const pickEl = ref<HTMLElement | null>(null)
const choices = computed(() => props.candidates.filter((a) => a.has_thumb).slice(0, 24))
async function pick(id: string) {
  const was = info.value?.image_id ?? ''
  picking.value = false
  if (id === was) return
  if (await save({ image_id: id })) {
    showOk(id ? t('tag-page-picture-saved') : t('tag-page-picture-removed'), { label: t('asset-undo'), run: () => void save({ image_id: was }) })
  }
}
function away(e: PointerEvent) {
  if (picking.value && pickEl.value && !pickEl.value.contains(e.target as Node)) picking.value = false
}
document.addEventListener('pointerdown', away)
onBeforeUnmount(() => document.removeEventListener('pointerdown', away))

// Renaming: to a name in use, it merges into that tag.
const renaming = ref(false)
const newName = ref('')
const nameBox = ref<HTMLInputElement | null>(null)
const busy = ref(false)
function startRename() {
  newName.value = props.tag
  renaming.value = true
  void nextTick(() => nameBox.value?.select())
}
async function rename() {
  const into = newName.value.trim().toLowerCase()
  if (!into || into === props.tag) {
    renaming.value = false
    return
  }
  const from = props.tag
  const count = info.value?.count ?? 0
  let joins = false
  try {
    joins = (await api.tagInfo(into)).count > 0
  } catch {
    // Not a tag: the server says so when asked to rename.
  }
  const ok = await confirmDialog(
    joins
      ? { title: t('tags-merge-title', { count: 2 }), body: t('tags-merge-body', { tags: from, into, count }), confirm: t('tags-merge') }
      : { title: t('tags-rename-title', { tag: from }), body: t('tags-rename-body', { count, into }), confirm: t('tags-rename') },
  )
  if (!ok) return
  busy.value = true
  try {
    const r = await api.mergeTags([from], into)
    renaming.value = false
    showOk(t('tags-merged', { count: r.assets, into: r.into }))
    emit('changed')
    emit('show', r.into)
  } catch (e) {
    showError(t('tags-merge-failed', { error: (e as Error).message }))
  } finally {
    busy.value = false
  }
}
function onNameKey(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    e.preventDefault()
    e.stopPropagation()
    renaming.value = false
  }
}

// Another tag: start over (below everything it resets, since it runs straight away).
watch(
  () => props.tag,
  () => {
    info.value = null
    editing.value = false
    renaming.value = false
    picking.value = false
    void load()
  },
  { immediate: true },
)

/** Whether there's anything to show: someone who edits always gets the page, to fill in. */
const shown = computed(() => {
  const i = info.value
  return !!i && (editor.value || !!i.description || !!i.image_id || i.aliases.length > 0 || !!i.merged_into)
})
</script>

<template>
  <header v-if="info && shown" class="tagpage">
    <p v-if="info.merged_into" class="merged">
      {{ t('tag-page-merged', { tag: info.tag, into: info.merged_into }) }}
      <button type="button" class="linkbtn" @click="emit('show', info.merged_into)">{{ t('tag-page-show', { tag: info.merged_into }) }}</button>
    </p>
    <template v-else>
      <div ref="pickEl" class="picwrap">
        <button
          v-if="editor"
          type="button"
          class="pic"
          :class="{ empty: !info.image_id }"
          :title="t('tag-page-picture')"
          :aria-label="t('tag-page-picture')"
          :aria-expanded="picking"
          @click="picking = !picking"
        >
          <img v-if="info.image_id" :src="thumbUrl({ id: info.image_id })" alt="" />
          <svg v-else viewBox="0 0 16 16" width="18" height="18" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round">
            <rect x="2" y="3" width="12" height="10" rx="1.5" /><path d="m2.5 11.5 3.5-3.5 3 3 2-2 2.5 2.5" /><circle cx="10.5" cy="6" r="1" />
          </svg>
        </button>
        <span v-else-if="info.image_id" class="pic"><img :src="thumbUrl({ id: info.image_id })" alt="" /></span>
        <div v-if="picking" class="picker" role="dialog" :aria-label="t('tag-page-picture')">
          <p class="hint">{{ choices.length ? t('tag-page-picture-help') : t('tag-page-picture-nothing') }}</p>
          <div class="choices">
            <button
              v-for="a in choices"
              :key="a.id"
              type="button"
              class="choice"
              :class="{ on: a.id === info.image_id }"
              :title="a.name"
              :aria-label="a.name"
              @click="pick(a.id)"
            ><img :src="thumbUrl(a)" alt="" loading="lazy" /></button>
          </div>
          <button v-if="info.image_id" type="button" class="linkbtn" @click="pick('')">{{ t('tag-page-picture-none') }}</button>
        </div>
      </div>

      <div class="body">
        <div class="titlerow">
          <form v-if="renaming" class="rename" @submit.prevent="rename">
            <label class="visually-hidden" for="tag-rename">{{ t('tag-page-rename-label') }}</label>
            <input id="tag-rename" ref="nameBox" v-model="newName" class="field mono" autocomplete="off" spellcheck="false" @keydown="onNameKey" />
            <button class="btn primary small" type="submit" :disabled="busy">{{ t('asset-save') }}</button>
            <button class="btn ghost small" type="button" @click="renaming = false">{{ t('asset-cancel') }}</button>
          </form>
          <template v-else>
            <h2 class="name mono">{{ info.tag }}</h2>
            <span class="count">{{ t('tag-page-count', { count: info.count }) }}</span>
            <button v-if="editor" type="button" class="linkbtn small" :title="t('tag-page-rename-title')" @click="startRename">{{ t('tag-page-rename') }}</button>
          </template>
        </div>
        <p v-if="renaming" class="hint">{{ t('tag-page-rename-title') }}</p>
        <textarea
          v-if="editing"
          ref="box"
          v-model="draft"
          class="field desc-edit"
          rows="2"
          maxlength="4000"
          :aria-label="t('tag-page-describe-label')"
          :placeholder="t('tag-page-describe-placeholder')"
          @keydown="onDescKey"
          @blur="saveDescription"
        />
        <p v-else-if="info.description" class="desc" :class="{ editable: editor }" :title="editor ? t('tag-page-describe-edit') : undefined" @click="edit">{{ info.description }}</p>
        <button v-else-if="editor" type="button" class="linkbtn describe" @click="edit">{{ t('tag-page-describe') }}</button>
        <p v-if="info.aliases.length" class="aliases">{{ t('tag-page-aliases', { names: info.aliases.join(', '), count: info.aliases.length }) }}</p>
      </div>
    </template>
  </header>
</template>

<style scoped>
.tagpage {
  display: flex; gap: 0.9rem; align-items: flex-start;
  margin: 0 0 0.9rem; padding: 0.75rem 0.9rem;
  border: 1px solid var(--line); border-radius: 10px; background: var(--panel);
}
.merged { margin: 0; color: var(--muted); }
.picwrap { position: relative; flex: none; }
.pic {
  display: grid; place-items: center; width: 64px; height: 64px; padding: 0; overflow: hidden;
  border: 1px solid var(--line); border-radius: 8px; background: var(--raised); color: var(--faint);
}
button.pic { cursor: pointer; }
button.pic:hover { border-color: var(--accent); color: var(--accent); }
.pic.empty { border-style: dashed; }
.pic img { width: 100%; height: 100%; object-fit: cover; }
.picker {
  position: absolute; top: calc(100% + 6px); left: 0; z-index: 20; width: min(22rem, 80vw);
  display: grid; gap: 0.5rem; padding: 0.7rem;
  border: 1px solid var(--card-on-line, var(--line)); border-radius: 10px; background: var(--raised); box-shadow: var(--elev-2);
}
.choices { display: grid; grid-template-columns: repeat(auto-fill, minmax(56px, 1fr)); gap: 0.35rem; }
.choice { aspect-ratio: 1; padding: 0; overflow: hidden; border: 2px solid transparent; border-radius: 6px; background: var(--panel); cursor: pointer; }
.choice:hover { border-color: color-mix(in srgb, var(--accent) 60%, transparent); }
.choice.on { border-color: var(--accent); }
.choice img { width: 100%; height: 100%; object-fit: cover; }
.body { flex: 1; min-width: 0; display: grid; gap: 0.3rem; }
.titlerow { display: flex; flex-wrap: wrap; align-items: baseline; gap: 0.3rem 0.75rem; }
.name { margin: 0; font-size: var(--step-1, 1.15rem); font-weight: 600; overflow-wrap: anywhere; }
.count { color: var(--muted); font-variant-numeric: tabular-nums; font-size: var(--step--1); }
.rename { display: flex; flex-wrap: wrap; gap: 0.4rem; align-items: center; }
.rename .field { width: auto; flex: 1 1 12rem; max-width: 22rem; }
.desc { margin: 0; line-height: 1.5; white-space: pre-line; overflow-wrap: anywhere; }
.desc.editable { cursor: text; border-radius: 4px; }
.desc.editable:hover { background: color-mix(in srgb, var(--text) 5%, transparent); }
.desc-edit { width: 100%; resize: vertical; font: inherit; }
.describe { justify-self: start; padding-inline: 0; color: var(--muted); }
.aliases, .hint { margin: 0; color: var(--faint); font-size: var(--step--1); }
.small { font-size: var(--step--1); }
</style>
