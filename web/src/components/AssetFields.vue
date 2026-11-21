<script setup lang="ts">
import { computed, nextTick, ref, watch } from 'vue'
import { api, type Asset, type AssetPatch, type Facet } from '../api'
import { t } from '../i18n'
import { showOk } from '../ui'

// An asset's details, each edited where it's shown: click it (or Enter on it), type, Enter or leaving it saves, Esc
// gives up. A save says so with Undo. What's being typed is the panel's own until it's saved, so the asset changing
// underneath it (a tag added, Reviewed pressed) doesn't lose it.
const props = defineProps<{
  asset: Asset
  editor: boolean
  /** Saves a change; false if it didn't take. */
  save: (patch: AssetPatch) => Promise<boolean>
}>()

type Key = 'category' | 'license' | 'source_url' | `field:${string}` | 'new-field'
const editing = ref<Key | null>(null)
const draft = ref('')
const draftName = ref('')
// The editors are drawn inside lists (where a template ref is a list too), so the one open is found in the section.
const root = ref<HTMLElement | null>(null)
watch(() => props.asset.id, () => (editing.value = null))

const fields = computed(() => Object.entries(props.asset.fields ?? {}))
const categories = ref<Facet[]>([])
let categoriesAsked = false

function label(key: Key): string {
  switch (key) {
    case 'category': return t('asset-field-category')
    case 'license': return t('asset-field-license')
    case 'source_url': return t('asset-field-source')
    default: return key.startsWith('field:') ? key.slice(6) : t('asset-field-new')
  }
}
function current(key: Key): string {
  if (key === 'new-field') return ''
  if (key.startsWith('field:')) return props.asset.fields?.[key.slice(6)] ?? ''
  return (props.asset[key as 'category' | 'license' | 'source_url'] ?? '') as string
}
function patchFor(key: Key, value: string, name = ''): AssetPatch {
  if (key === 'new-field') return { fields: { [name]: value } }
  if (key.startsWith('field:')) return { fields: { [key.slice(6)]: value } }
  return { [key]: value }
}

function start(key: Key) {
  if (!props.editor || editing.value === key) return
  editing.value = key
  draft.value = current(key)
  draftName.value = ''
  if (key === 'category' && !categoriesAsked) {
    categoriesAsked = true
    api.categories().then((c) => (categories.value = c), () => (categoriesAsked = false))
  }
  void nextTick(() => {
    const el = root.value?.querySelector<HTMLInputElement>('input.inline')
    el?.focus()
    el?.select()
  })
}

async function commit() {
  const key = editing.value
  if (!key) return
  const value = draft.value.trim()
  const name = draftName.value.trim()
  editing.value = null
  const was = current(key)
  if (key === 'new-field' ? !name || !value : value === was) return
  if (!(await props.save(patchFor(key, value, name)))) return
  const what = key === 'new-field' ? name : label(key)
  // Undo puts back what it was (a new field, nothing).
  const undo = key === 'new-field' ? patchFor(key, '', name) : patchFor(key, was)
  showOk(value ? t('asset-field-saved', { field: what }) : t('asset-field-cleared', { field: what }), {
    label: t('asset-undo'),
    run: () => void props.save(undo),
  })
}
function cancel() {
  editing.value = null
}
function onKey(e: KeyboardEvent) {
  if (e.key === 'Enter') {
    e.preventDefault()
    void commit()
  } else if (e.key === 'Escape') {
    // Gives up the edit; the panel stays open.
    e.preventDefault()
    e.stopPropagation()
    cancel()
  }
}
/** Leaving the field saves, unless focus only moved to the other half of a new field. */
function onBlur(e: FocusEvent) {
  const to = e.relatedTarget as HTMLElement | null
  if (to?.closest('.newfield') && editing.value === 'new-field') return
  void commit()
}

async function rate(n: number) {
  const was = props.asset.rating ?? 0
  const rating = n === was ? 0 : n
  if (!(await props.save({ rating }))) return
  showOk(rating ? t('asset-rated', { count: rating }) : t('asset-rating-cleared'), {
    label: t('asset-undo'),
    run: () => void props.save({ rating: was }),
  })
}
const hoverRating = ref(0)

/** A source shown as its address without the scheme, and only if it's a web address is it a link. */
const sourceLink = computed(() => {
  const s = props.asset.source_url ?? ''
  return /^https?:\/\//i.test(s) ? s : null
})
const sourceText = computed(() => (props.asset.source_url ?? '').replace(/^https?:\/\/(www\.)?/i, ''))

const rows = computed<Key[]>(() => {
  const keys: Key[] = ['category', 'license', 'source_url']
  // Someone who can't change anything sees only what's there.
  return props.editor ? keys : keys.filter((k) => current(k) || (k === 'license' && props.asset.pack_license))
})
const shown = computed(() => props.editor || rows.value.length || fields.value.length || props.asset.rating)
</script>

<template>
  <section v-if="shown" ref="root" class="fields">
    <h3>{{ t('asset-details-heading') }}</h3>
    <dl>
      <template v-for="key in rows" :key="key">
        <dt>{{ label(key) }}</dt>
        <dd>
          <template v-if="editing === key">
            <input
              v-model="draft"
              class="field inline"
              :aria-label="label(key)"
              :list="key === 'category' ? 'category-choices' : undefined"
              :maxlength="key === 'source_url' ? 2000 : 200"
              :inputmode="key === 'source_url' ? 'url' : undefined"
              :placeholder="key === 'license' ? (asset.pack_license ?? t('asset-field-license-placeholder')) : key === 'source_url' ? 'https://' : ''"
              autocomplete="off"
              spellcheck="false"
              @keydown="onKey"
              @blur="onBlur"
            />
            <datalist v-if="key === 'category'" id="category-choices">
              <option v-for="c in categories" :key="c.value" :value="c.value">{{ c.count }}</option>
            </datalist>
          </template>
          <span v-else class="value" :class="{ editable: editor }">
            <a v-if="key === 'source_url' && sourceLink" :href="sourceLink" target="_blank" rel="noopener noreferrer" class="src">{{ sourceText }}</a>
            <button
              v-if="editor || (key === 'source_url' && !sourceLink && current(key))"
              type="button"
              class="show"
              :class="{ empty: !current(key) }"
              :title="editor ? t('asset-field-edit-title', { field: label(key) }) : undefined"
              :disabled="!editor"
              @click="start(key)"
            >
              <template v-if="key === 'source_url' && sourceLink"><span class="visually-hidden">{{ t('asset-field-edit-title', { field: label(key) }) }}</span><svg viewBox="0 0 16 16" width="11" height="11" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="m10.5 2.5 3 3-7.5 7.5H3v-3z" /></svg></template>
              <template v-else-if="current(key)">{{ key === 'source_url' ? sourceText : current(key) }}</template>
              <template v-else-if="key === 'license' && asset.pack_license">{{ asset.pack_license }} <span class="inherit">{{ t('asset-field-from-pack') }}</span></template>
              <template v-else>{{ t('asset-field-add') }}</template>
            </button>
            <template v-else-if="key === 'license' && !current(key)">{{ asset.pack_license }} <span class="inherit">{{ t('asset-field-from-pack') }}</span></template>
            <template v-else-if="key !== 'source_url'">{{ current(key) }}</template>
          </span>
        </dd>
      </template>

      <template v-if="editor || asset.rating">
        <dt>{{ t('asset-field-rating') }}</dt>
        <dd>
          <span class="stars" role="group" :aria-label="t('asset-field-rating')" @mouseleave="hoverRating = 0">
            <button
              v-for="n in 5"
              :key="n"
              type="button"
              class="star"
              :class="{ on: n <= (hoverRating || asset.rating || 0), hover: hoverRating > 0 }"
              :disabled="!editor"
              :aria-pressed="n === asset.rating"
              :aria-label="t('asset-rate', { count: n })"
              :title="n === asset.rating ? t('asset-rating-clear-title') : t('asset-rate', { count: n })"
              @mouseenter="editor && (hoverRating = n)"
              @click="rate(n)"
            >
              <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M8 1.9l1.8 3.7 4.1.6-3 2.9.7 4.1L8 11.3l-3.6 1.9.7-4.1-3-2.9 4.1-.6z" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round" /></svg>
            </button>
          </span>
        </dd>
      </template>

      <template v-for="[name] in fields" :key="name">
        <dt class="own" :title="name">{{ name }}</dt>
        <dd>
          <input
            v-if="editing === `field:${name}`"
            v-model="draft"
            class="field inline"
            :aria-label="name"
            maxlength="2000"
            autocomplete="off"
            :placeholder="t('asset-field-remove-placeholder')"
            @keydown="onKey"
            @blur="onBlur"
          />
          <button v-else-if="editor" type="button" class="show" :title="t('asset-field-edit-title', { field: name })" @click="start(`field:${name}`)">{{ current(`field:${name}`) }}</button>
          <span v-else class="value">{{ current(`field:${name}`) }}</span>
        </dd>
      </template>
    </dl>
    <div v-if="editing === 'new-field'" class="newfield">
      <input v-model="draftName" class="field inline" :aria-label="t('asset-field-name')" :placeholder="t('asset-field-name')" maxlength="60" autocomplete="off" @keydown="onKey" @blur="onBlur" />
      <input v-model="draft" class="field inline" :aria-label="t('asset-field-value')" :placeholder="t('asset-field-value')" maxlength="2000" autocomplete="off" @keydown="onKey" @blur="onBlur" />
    </div>
    <button v-else-if="editor" type="button" class="linkish small addfield" @click="start('new-field')">{{ t('asset-field-new') }}</button>
  </section>
</template>

<style scoped>
.fields dl { align-items: center; row-gap: 0.15rem; }
.fields dt { padding: 0.2rem 0; }
.own { max-width: 9rem; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.value { display: flex; align-items: center; gap: 0.35rem; min-width: 0; }
/* A value is its own edit button: it looks like text until pointed at. */
.show {
  min-width: 0; max-width: 100%; padding: 0.2rem 0.4rem; margin-left: -0.4rem; border: 0; border-radius: 4px;
  background: none; color: var(--text); font: inherit; text-align: left; cursor: text;
  overflow: hidden; text-overflow: ellipsis; white-space: nowrap;
  transition: background-color var(--dur-1), box-shadow var(--dur-1);
}
.show:disabled { cursor: auto; }
.show:not(:disabled):hover { background: color-mix(in srgb, var(--text) 6%, transparent); box-shadow: inset 0 0 0 1px var(--line); }
.show:focus-visible { outline: 2px solid var(--accent); outline-offset: 0; }
.show.empty { color: var(--faint); }
.src + .show { margin-left: 0; padding: 0.25rem; color: var(--faint); }
.src + .show:hover { color: var(--text); }
.src { min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; color: var(--text); text-underline-offset: 3px; text-decoration-color: var(--faint); }
.inherit { color: var(--faint); font-size: 0.75rem; margin-left: 0.2rem; }
input.inline { padding: 0.2rem 0.4rem; margin-left: -0.4rem; width: calc(100% + 0.4rem); font-size: inherit; }
.stars { display: inline-flex; margin-left: -0.3rem; }
.star { border: 0; background: none; padding: 0.2rem 0.15rem; color: var(--faint); cursor: pointer; line-height: 0; }
.star:disabled { cursor: auto; }
.star svg { fill: none; transition: fill var(--dur-1), color var(--dur-1), scale var(--dur-2) var(--ease-pop); }
.star.on { color: var(--star); }
.star.on svg { fill: currentColor; }
.star.on.hover { color: color-mix(in srgb, var(--star) 75%, var(--text)); }
.star:not(:disabled):hover svg { scale: 1.15; }
.star:focus-visible { outline: 2px solid var(--accent); border-radius: 4px; }
.newfield { display: grid; grid-template-columns: minmax(0, 0.8fr) minmax(0, 1.2fr); gap: 0.4rem; margin-top: 0.4rem; padding-left: 0.4rem; }
.addfield { margin-top: 0.4rem; color: var(--muted); }
</style>
<style scoped src="./detailSection.css"></style>
