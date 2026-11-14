<script setup lang="ts">
// Settings → Store files: the formats Hoard reads stores' files with (see docs/store-formats.md), the files in the
// library none of them reads that look like a store's, and the wizard that teaches Hoard one.
import { onMounted, ref, watch } from 'vue'
import { api, type FilePlace, type StoreFormat, type StoreFormats } from '../api'
import { t } from '../i18n'
import { confirmDialog, showError, showOk } from '../ui'
import { teach } from '../storeFormats'
import FormatWizard from './FormatWizard.vue'

const data = ref<StoreFormats | null>(null)
/** The wizard, when it's open: on a file, on a format, or both. */
const open = ref<{ place?: FilePlace; format?: StoreFormat } | null>(null)

async function load() {
  try {
    data.value = await api.storeFormats()
  } catch (e) {
    showError(t('storefiles-load-failed', { error: (e as Error).message }))
  }
}
onMounted(load)

// A file's details asked to teach Hoard it.
watch(
  teach,
  (p) => {
    if (!p) return
    open.value = { place: p }
    teach.value = null
  },
  { immediate: true },
)

function done(saved: boolean) {
  open.value = null
  if (saved) void load()
}

async function remove(f: StoreFormat) {
  const ok = await confirmDialog({
    title: t('storefiles-remove-title', { name: f.name }),
    body: t('storefiles-remove-body', { file: f.file }),
    confirm: t('storefiles-remove'),
    danger: true,
  })
  if (!ok) return
  try {
    await api.removeStoreFormat(f.file)
    showOk(t('storefiles-removed', { name: f.name }))
    void load()
  } catch (e) {
    showError(t('storefiles-remove-failed', { name: f.name, error: (e as Error).message }))
  }
}
</script>

<template>
  <div class="sf">
    <FormatWizard v-if="open" :place="open.place" :format="open.format" @done="done" />
    <template v-else>
      <section>
        <div class="sh"><h3>{{ t('storefiles-unread') }}</h3></div>
        <p v-if="!data" class="what">{{ t('storefiles-loading') }}</p>
        <p v-else-if="!data.unread.length" class="what none">{{ t('storefiles-unread-none') }}</p>
        <template v-else>
          <p class="what">{{ t('storefiles-unread-what') }}</p>
          <ul class="list">
            <li v-for="u in data.unread" :key="u.name" :data-file="u.name">
              <span class="mono grow">{{ u.name }}</span>
              <span class="n">{{ t('storefiles-unread-packs', { count: u.packs }) }}</span>
              <span class="ex grow" :title="u.example.path">{{ t('storefiles-unread-example', { pack: u.example.pack_name }) }}</span>
              <button type="button" class="btn ghost small" @click="open = { place: u.example }">{{ t('storefiles-teach') }}</button>
            </li>
          </ul>
        </template>
      </section>

      <section>
        <div class="sh"><h3>{{ t('storefiles-formats') }}</h3></div>
        <p class="what">
          {{ t('storefiles-formats-what') }}
          <template v-if="data?.dir">{{ t('storefiles-dir-before') }} <span class="mono">{{ data.dir }}</span>{{ t('storefiles-dir-after') }}</template>
        </p>
        <ul v-if="data" class="list">
          <li v-for="f in data.formats" :key="f.file + f.custom" :data-format="f.name">
            <span class="grow">{{ f.name }}</span>
            <span class="n mono">{{ f.file }}</span>
            <span class="tag" :class="{ yours: f.custom }">{{ f.custom ? t('storefiles-yours') : t('storefiles-built-in') }}</span>
            <template v-if="f.custom">
              <button type="button" class="btn ghost small" @click="open = { format: f }">{{ t('storefiles-edit') }}</button>
              <button type="button" class="btn ghost small" @click="remove(f)">{{ t('storefiles-remove') }}</button>
            </template>
            <button v-else type="button" class="btn ghost small" @click="open = { format: f }">{{ t('storefiles-start-from') }}</button>
          </li>
        </ul>
      </section>
    </template>
  </div>
</template>

<style scoped>
.sf { display: grid; gap: 1.8rem; font-size: var(--step--1); }
.sf > section { max-width: 44rem; }
section { display: grid; gap: 0.6rem; }
.sh h3 { margin: 0; }
.what { margin: 0; color: var(--muted); line-height: 1.5; }
.none { font-style: italic; }
.list { margin: 0; padding: 0; list-style: none; border-top: 1px solid var(--line); }
.list li { display: flex; flex-wrap: wrap; gap: 0.8rem; align-items: center; padding: 0.3rem 0; border-bottom: 1px solid var(--line); }
.grow { flex: 1; min-width: 0; overflow-wrap: anywhere; }
.ex { color: var(--muted); }
.n { color: var(--muted); font-variant-numeric: tabular-nums; }
.tag { font-size: 0.85em; color: var(--muted); }
.tag.yours { color: var(--accent); }
.small { padding-block: 0.1rem; font-size: 0.85em; }
</style>
