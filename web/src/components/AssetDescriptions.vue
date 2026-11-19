<script setup lang="ts">
import { ref, watch } from 'vue'
import { api, type Asset } from '../api'
import { t } from '../i18n'

// What's said about an asset: the description someone wrote (editable), what the AI saw, what its file says, and the
// store page's blurb for its pack.
const props = defineProps<{
  asset: Asset
  editor: boolean
  /** How many files the asset's family has, when an edit goes to all of them. */
  familySize: number | null
  /** Saves the description; false if it didn't take. */
  save: (description: string) => Promise<boolean>
}>()

const editing = ref(false)
const draft = ref('')
const storeBlurb = ref<{ pack: string; store: string | null; url: string | null; text: string } | null>(null)
const storeOpen = ref(false)
watch(() => props.asset.id, () => (editing.value = false))
watch(
  () => props.asset.pack_id,
  async (pack) => {
    storeOpen.value = false
    if (storeBlurb.value?.pack === pack) return
    storeBlurb.value = null
    try {
      const ctx = await api.packContext(pack)
      if (props.asset.pack_id === pack && ctx.store_text) {
        storeBlurb.value = { pack, store: ctx.store, url: ctx.store_url, text: ctx.store_text }
      }
    } catch {
      // No context, no blurb.
    }
  },
  { immediate: true },
)
function edit() {
  draft.value = props.asset.description ?? ''
  editing.value = true
}
async function submit() {
  if (await props.save(draft.value.trim())) editing.value = false
}
</script>

<template>
  <section class="descs">
    <form v-if="editing" class="descedit" @submit.prevent="submit">
      <label class="visually-hidden" for="asset-desc">{{ t('asset-desc-label') }}</label>
      <textarea id="asset-desc" v-model="draft" class="field" rows="3" maxlength="2000"
        :placeholder="t('asset-desc-placeholder')" />
      <div class="descbtns">
        <button class="btn primary" type="submit">{{ t('asset-save') }}</button>
        <button class="btn" type="button" @click="editing = false">{{ t('asset-cancel') }}</button>
        <span v-if="familySize" class="hint">{{ t('asset-desc-family', { count: familySize }) }}</span>
      </div>
    </form>
    <template v-else>
      <p v-if="asset.description" class="desc mine">{{ asset.description }}</p>
      <button v-if="editor" type="button" class="linkish small" @click="edit">
        {{ asset.description ? t('asset-desc-edit') : t('asset-desc-write') }}
      </button>
    </template>
    <div v-if="asset.ai_description" class="descsrc">
      <span class="srclabel">{{ t('asset-desc-ai') }}</span>
      <p class="desc">{{ asset.ai_description }}</p>
    </div>
    <div v-if="asset.source_description" class="descsrc">
      <span class="srclabel">{{ t('asset-desc-file') }}</span>
      <p class="desc">{{ asset.source_description }}</p>
    </div>
    <div v-if="storeBlurb" class="descsrc">
      <span class="srclabel">
        {{ t('asset-desc-store-before') }}
        <a v-if="storeBlurb.url" :href="storeBlurb.url" target="_blank" rel="noopener noreferrer">{{ storeBlurb.store ? t('asset-desc-store-page', { store: storeBlurb.store }) : t('asset-desc-store-page-unnamed') }}</a>
        <template v-else>{{ storeBlurb.store ? t('asset-desc-store-page', { store: storeBlurb.store }) : t('asset-desc-store-page-unnamed') }}</template>
        {{ t('asset-desc-store-after') }}
      </span>
      <p class="desc store" :class="{ open: storeOpen }">{{ storeBlurb.text }}</p>
      <button v-if="storeBlurb.text.length > 240" type="button" class="linkish small" @click="storeOpen = !storeOpen">
        {{ storeOpen ? t('asset-less') : t('asset-more') }}
      </button>
    </div>
  </section>
</template>

<style scoped>
.desc { margin: 0; color: var(--muted); max-width: 60ch; white-space: pre-line; }
.desc.mine { color: var(--text); }
.descs { display: grid; gap: 0.6rem; justify-items: start; }
.descsrc { display: grid; gap: 0.15rem; }
.srclabel { font-size: 0.72rem; color: var(--faint); }
.desc.store { display: -webkit-box; -webkit-box-orient: vertical; -webkit-line-clamp: 4; line-clamp: 4; overflow: hidden; }
.desc.store.open { display: block; }
.descedit { display: grid; gap: 0.4rem; width: 100%; max-width: 60ch; }
.descedit textarea { resize: vertical; font: inherit; }
.descbtns { display: flex; gap: 0.4rem; align-items: center; }
.hint { font-size: 0.75rem; color: var(--faint); }
</style>
<style scoped src="./detailSection.css"></style>
