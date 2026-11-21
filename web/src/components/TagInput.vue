<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { api, type TagSuggestion, type TagSuggestions } from '../api'
import { t } from '../i18n'

// A box for adding tags. It offers tags in use as you type, grouped by namespace, including those an old (merged)
// name you typed became. Enter adds the highlighted one, which is the best match. A tag nobody has used yet comes
// last, as "New tag", with any tags in use it looks like (a plural, another spelling, a typo) listed above it.
// Pasting a list separated by commas adds each, and Backspace in an empty box takes the last tag off.
const props = defineProps<{
  id: string
  /** Tags already on what's being tagged: they aren't offered again. */
  have: string[]
  placeholder?: string
}>()
const emit = defineEmits<{ add: [tags: string[]]; removeLast: [] }>()

const text = ref('')
const open = ref(false)
const found = ref<TagSuggestions | null>(null)
/** What `found` was found for: until it's for what's in the box, Enter adds what's typed. */
const foundFor = ref('')
const active = ref(0)
const input = ref<HTMLInputElement | null>(null)
defineExpose({ focus: () => input.value?.focus() })

interface Option { key: string; tag: string; count?: number; alias?: string; group: string; isNew?: boolean }

const options = computed<Option[]>(() => {
  const f = found.value
  if (!f) return []
  const have = new Set(props.have)
  const offered = f.matches.filter((m) => !have.has(m.tag))
  // Grouped by namespace, the groups in the order of their best match.
  const groups = new Map<string, TagSuggestion[]>()
  for (const m of offered) {
    const ns = m.tag.includes(':') ? m.tag.slice(0, m.tag.indexOf(':')) : ''
    groups.set(ns, [...(groups.get(ns) ?? []), m])
  }
  const out: Option[] = []
  for (const [ns, ms] of groups) out.push(...ms.map((m) => ({ key: `m:${m.tag}`, tag: m.tag, count: m.count, alias: m.alias, group: ns })))
  const typed = f.normalized
  if (typed && !f.exists && !have.has(typed)) {
    const near = f.near.filter((m) => !have.has(m.tag) && !out.some((o) => o.tag === m.tag))
    out.push(...near.map((m) => ({ key: `n:${m.tag}`, tag: m.tag, count: m.count, group: '\u0000near' })))
    out.push({ key: `new:${typed}`, tag: typed, group: '\u0000new', isNew: true })
  }
  return out
})
/** Where each group starts, for its heading. */
function heading(i: number): string | null {
  const o = options.value[i]
  if (i > 0 && options.value[i - 1].group === o.group) return null
  if (o.group === '\u0000near') return t('tag-suggest-near')
  if (o.group === '\u0000new' || !o.group) return i > 0 ? '' : null
  return o.group
}

let seq = 0
let timer: ReturnType<typeof setTimeout> | undefined
function fetchSoon(delay = 120) {
  clearTimeout(timer)
  const q = text.value
  const mine = ++seq
  timer = setTimeout(async () => {
    try {
      const f = await api.suggestTags(q)
      if (mine !== seq) return
      found.value = f
      foundFor.value = q
      active.value = 0
    } catch {
      if (mine === seq) found.value = null
    }
  }, delay)
}
watch(text, (v) => {
  if (!v.trim()) {
    found.value = null
    open.value = false
    clearTimeout(timer)
    seq++
    return
  }
  open.value = true
  fetchSoon()
})

/** Several tags typed or pasted at once, separated by commas, semicolons or lines. */
function split(s: string): string[] {
  return s.split(/[,;\n]/).map((x) => x.trim()).filter(Boolean)
}
function add(tags: string[]) {
  const have = new Set(props.have)
  const fresh = [...new Set(tags)].filter((x) => !have.has(x))
  text.value = ''
  if (fresh.length) emit('add', fresh)
}
function choose(o: Option) {
  add([o.tag])
  input.value?.focus()
}

function onKey(e: KeyboardEvent) {
  const n = options.value.length
  if (e.key === 'ArrowDown' || e.key === 'ArrowUp') {
    e.preventDefault()
    if (!open.value) {
      open.value = true
      fetchSoon(0)
      return
    }
    if (n) active.value = (active.value + (e.key === 'ArrowDown' ? 1 : -1) + n) % n
  } else if (e.key === 'Enter') {
    e.preventDefault()
    if (split(text.value).length > 1) add(split(text.value))
    else if (open.value && foundFor.value === text.value && options.value[active.value]) choose(options.value[active.value])
    else if (text.value.trim()) add([text.value.trim()])
  } else if (e.key === 'Tab' && open.value && options.value[active.value] && text.value.trim()) {
    // Tab fills in the highlighted tag, to add to it (a namespace's value, say).
    const o = options.value[active.value]
    if (!o.isNew && o.tag !== text.value) {
      e.preventDefault()
      text.value = o.tag
    }
  } else if (e.key === 'Escape') {
    if (open.value || text.value) {
      // Only the list or the text goes, not a dialog the box is in.
      e.preventDefault()
      e.stopPropagation()
      if (open.value) open.value = false
      else text.value = ''
    }
  } else if (e.key === 'Backspace' && !text.value && !e.repeat) {
    emit('removeLast')
  }
}
function onPaste(e: ClipboardEvent) {
  const pasted = e.clipboardData?.getData('text') ?? ''
  const tags = split(pasted)
  if (tags.length > 1) {
    e.preventDefault()
    add(split(text.value + pasted))
  }
}
function onBlur(e: FocusEvent) {
  // Clicking an option keeps it open long enough to take the click.
  if (!(e.relatedTarget instanceof HTMLElement && e.relatedTarget.closest('.taglist'))) open.value = false
}
const listId = computed(() => `${props.id}-list`)
const activeId = computed(() => (open.value && options.value[active.value] ? `${props.id}-opt-${active.value}` : undefined))
watch(active, async () => {
  document.getElementById(activeId.value ?? '')?.scrollIntoView({ block: 'nearest' })
})
function parts(tag: string) {
  const i = tag.indexOf(':')
  return i > 0 ? { ns: tag.slice(0, i + 1), value: tag.slice(i + 1) } : { ns: '', value: tag }
}
</script>

<template>
  <div class="taginput">
    <input
      :id="id"
      ref="input"
      v-model="text"
      class="field"
      role="combobox"
      autocomplete="off"
      spellcheck="false"
      :aria-expanded="open && options.length > 0"
      :aria-controls="listId"
      aria-autocomplete="list"
      :aria-activedescendant="activeId"
      :placeholder="placeholder"
      @keydown="onKey"
      @paste="onPaste"
      @blur="onBlur"
      @focus="text.trim() && (open = true)"
    />
    <ul v-show="open && options.length" :id="listId" class="taglist" role="listbox" tabindex="-1">
      <template v-for="(o, i) in options" :key="o.key">
        <li v-if="heading(i) !== null" class="grouphead" role="presentation">{{ heading(i) }}</li>
        <li
          :id="`${id}-opt-${i}`"
          role="option"
          class="opt"
          :class="{ on: i === active, isnew: o.isNew }"
          :aria-selected="i === active"
          @mousedown.prevent
          @mousemove="active = i"
          @click="choose(o)"
        >
          <span v-if="o.isNew" class="newlbl">{{ t('tag-suggest-new') }}</span>
          <span class="name"><span class="ns">{{ parts(o.tag).ns }}</span>{{ parts(o.tag).value }}</span>
          <span v-if="o.alias" class="alias">{{ t('tag-suggest-alias', { alias: o.alias }) }}</span>
          <span v-if="o.count !== undefined" class="count">{{ o.count }}</span>
        </li>
      </template>
    </ul>
  </div>
</template>

<style scoped>
.taginput { position: relative; flex: 1; min-width: 0; }
.taglist {
  position: absolute; z-index: 20; top: calc(100% + 4px); left: 0; right: 0;
  margin: 0; padding: 0.25rem; list-style: none; max-height: 17rem; overflow-y: auto;
  background: var(--panel); border: 1px solid var(--line); border-radius: var(--r-chip); box-shadow: var(--elev-3);
}
.grouphead {
  padding: 0.45rem 0.55rem 0.15rem; font-size: 0.68rem; letter-spacing: 0.05em; text-transform: uppercase; color: var(--faint);
}
.grouphead:empty { padding: 0; margin: 0.25rem 0.4rem; border-top: 1px solid var(--line); }
.opt {
  display: flex; align-items: baseline; gap: 0.5rem; padding: 0.3rem 0.55rem; border-radius: calc(var(--r-chip) - 2px);
  cursor: pointer; font-size: var(--step--1);
}
.opt.on { background: var(--raised); }
.name { min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.ns { color: var(--faint); }
.alias { color: var(--faint); font-size: 0.9em; white-space: nowrap; }
.count { margin-left: auto; color: var(--faint); font-variation-settings: 'MONO' 1; font-size: 0.9em; }
.newlbl { color: var(--accent); font-size: 0.85em; white-space: nowrap; }
</style>
