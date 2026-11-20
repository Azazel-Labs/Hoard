<script setup lang="ts">
// Where the file came from when Hoard made it from another: a conversion rule's original (which can take the asset
// back) or an Unreal package's, how it was converted, and what Unreal's own registry said of it.
import { computed, ref } from 'vue'
import { api, originalUrl, type Asset } from '../api'
import { may } from '../auth'
import { formatDate, t } from '../i18n'
import { sourceExt, sourceUrl } from '../player'
import { confirmDialog, showOk } from '../ui'

const props = defineProps<{ asset: Asset }>()
const emit = defineEmits<{ updated: [Asset]; error: [message: string] }>()

const origin = computed(() => props.asset.origin ?? null)
const ueStatus = computed(() => props.asset.unreal ?? null)
const originFile = computed(() => origin.value?.path.split('/').pop() ?? '')
const recovered = computed(() => sourceExt(props.asset))
const ueTags = computed(() => {
  const tags = props.asset.meta.ue_tags
  return tags && typeof tags === 'object' ? Object.entries(tags as Record<string, string>) : []
})
/** The end of "This file was X…": its class and the engine version that saved it, as far as they're known. */
const originTail = computed(() => {
  const o = origin.value
  if (!o) return ''
  if (o.class && o.engine) return t('asset-origin-class-engine', { class: o.class, engine: o.engine })
  if (o.class) return t('asset-origin-class', { class: o.class })
  if (o.engine) return t('asset-origin-engine', { engine: o.engine })
  return t('asset-origin-end')
})
/** A conversion rule made it (Settings → Adding packs), rather than Unreal's packages. */
const byRule = computed(() => origin.value?.method === 'rule')
const puttingBack = ref(false)
/** Undo the conversion rule: the original file takes the asset back, and the rules leave it alone from then on. */
async function putBack() {
  const o = origin.value
  if (!o) return
  const ok = await confirmDialog({
    title: t('asset-rule-put-back-title', { file: originFile.value }),
    body: t('asset-rule-put-back-body'),
    confirm: t('asset-rule-put-back'),
  })
  if (!ok) return
  puttingBack.value = true
  try {
    await api.unconvert(props.asset.id)
    emit('updated', await api.asset(props.asset.id))
    showOk(t('asset-rule-put-back-done', { file: originFile.value }))
  } catch (e) {
    emit('error', (e as Error).message)
  } finally {
    puttingBack.value = false
  }
}
/** Who converted it, when, and any notes. */
const originHow = computed(() => {
  const o = origin.value
  if (!o) return ''
  const date = formatDate(o.converted_at)
  if (o.method === 'editor') return o.notes ? t('asset-origin-editor-notes', { date, notes: o.notes }) : t('asset-origin-editor', { date })
  return o.notes ? t('asset-origin-hoard-notes', { date, notes: o.notes }) : t('asset-origin-hoard', { date })
})
/** Anything to say: the section isn't there otherwise. */
const shown = computed(() => !!(origin.value || ueStatus.value || recovered.value || ueTags.value.length))
</script>

<template>
  <section v-if="shown" class="unreal">
    <h3>{{ byRule ? t('asset-rule-converted') : origin ? t('asset-unreal-converted') : t('asset-unreal-package') }}</h3>
    <div v-if="origin && byRule" data-testid="rule-origin">
      <p class="origin-note">
        {{ t('asset-rule-was', { date: formatDate(origin.converted_at), format: asset.ext.toUpperCase() }) }}
        <span class="mono">{{ originFile }}</span>
      </p>
      <p class="legend">
        <template v-if="origin.original_kept">
          {{
            origin.kept_until
              ? t('asset-rule-kept-until', { date: formatDate(origin.kept_until) })
              : origin.ext.toLowerCase() === 'wav'
                ? t('asset-rule-rebuilt')
                : ''
          }}
          <a class="btn" :href="originalUrl(asset)">{{ t('asset-origin-download', { ext: origin.ext }) }}</a>
          <button v-if="may('admin')" type="button" class="btn" :disabled="puttingBack" data-testid="rule-put-back" @click="putBack">
            {{ t('asset-rule-put-back') }}
          </button>
        </template>
        <template v-else>{{ t('asset-rule-let-go') }}</template>
      </p>
    </div>
    <template v-else-if="origin">
      <p class="origin-note">
        {{ t('asset-origin-before') }} <span class="mono">{{ originFile }}</span>{{ originTail }}
        {{ originHow }}
      </p>
      <p class="legend">
        <a v-if="origin.original_kept" class="btn" :href="originalUrl(asset)">{{ t('asset-origin-download', { ext: origin.ext }) }}</a>
        <template v-else>{{ t('asset-origin-discarded') }}</template>
      </p>
    </template>
    <p v-else-if="ueStatus?.status === 'needs_editor'" class="legend">
      {{ t('asset-ue-needs-editor-before', { reason: ueStatus.reason }) }}
      <span class="mono">hoard unreal-worker --editor &lt;{{ t('asset-ue-editor-path') }}&gt;</span> {{ t('asset-ue-needs-editor-after') }}
    </p>
    <p v-else-if="ueStatus?.status === 'unsupported'" class="legend">
      {{ t('asset-ue-unsupported', { reason: ueStatus.reason }) }}
    </p>
    <p v-else-if="ueStatus?.status === 'failed'" class="legend bad">
      {{ t('asset-ue-failed', { reason: ueStatus.reason }) }}
    </p>
    <p v-if="recovered && !origin" class="recovered">
      {{ t('asset-recovered', { ext: recovered }) }}
      <a class="btn" :href="sourceUrl(asset, true)">{{ t('asset-recovered-download', { ext: recovered }) }}</a>
    </p>
    <p v-else-if="!origin && !ueStatus && (asset.kind === 'model' || asset.kind === 'animation')" class="legend">
      {{ t('asset-ue-meshes-before') }}
      <span class="mono">hoard unreal-worker</span> {{ t('asset-ue-meshes-after') }}
    </p>
    <details v-if="ueTags.length">
      <summary>{{ t('asset-ue-registry-tags', { count: ueTags.length }) }}</summary>
      <dl>
        <template v-for="[k, v] in ueTags" :key="k">
          <dt>{{ k }}</dt>
          <dd class="mono">{{ v }}</dd>
        </template>
      </dl>
    </details>
  </section>
</template>

<style scoped>
.origin-note { margin: 0; font-size: var(--step--1); color: var(--muted); max-width: 52ch; }
.unreal .bad { color: var(--danger); }
.recovered { margin: 0 0 0.6rem; display: flex; flex-wrap: wrap; gap: 0.5rem; align-items: center; font-size: var(--step--1); color: var(--muted); }
.unreal summary { cursor: pointer; font-size: var(--step--1); color: var(--muted); margin-bottom: 0.4rem; }
.unreal dd { word-break: break-word; }
</style>
<style scoped src="./detailSection.css"></style>
