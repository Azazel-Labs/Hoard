<script setup lang="ts">
import { computed } from 'vue'
import type { TagSource } from '../api'

const props = defineProps<{
  tag: string; source?: TagSource; count?: number; removable?: boolean
  /** A colour shown before the tag (for a colour filter). */
  swatch?: string
}>()
defineEmits<{ remove: [] }>()

const parts = computed(() => {
  const i = props.tag.indexOf(':')
  return i > 0 ? { ns: props.tag.slice(0, i), value: props.tag.slice(i + 1) } : { ns: '', value: props.tag }
})
/** Past this many characters a tag fades out; its tooltip has the whole of it. */
const LONG = 30
const long = computed(() => parts.value.value.length > LONG)
const longNs = computed(() => parts.value.ns.length > 12)
const title = computed(() => {
  const who = props.source ? sourceLabel[props.source] : ''
  if (!long.value && !longNs.value) return who || undefined
  return who ? `${props.tag} (${who})` : props.tag
})
const sourceLabel: Record<TagSource, string> = {
  human: 'added by a person',
  rule: 'from the offline tagger',
  meta: 'measured from the file',
  ai: 'suggested by AI',
}
</script>

<template>
  <span class="chip" :class="[source && `src-${source}`, { long, 'long-ns': longNs }]" :title="title">
    <i v-if="swatch" class="swatch" :style="{ background: swatch }" aria-hidden="true" />
    <span v-if="parts.ns" class="ns">{{ parts.ns }}:</span>
    <span class="value">{{ parts.value }}</span>
    <span v-if="count !== undefined" class="count">{{ count }}</span>
    <button v-if="removable" class="x" type="button" :aria-label="`Remove tag ${tag}`" @click.stop="$emit('remove')">
      ×
    </button>
  </span>
</template>

<style scoped>
/* A long tag can't push the panel sideways: it stops at about 30 characters (less in a narrow spot) and fades. */
.chip { max-width: 100%; min-width: 0; }
.ns { flex: none; }
.value { min-width: 0; max-width: 30ch; overflow: hidden; }
.long .value { mask-image: linear-gradient(to right, #000 calc(100% - 2.5em), transparent); }
.long-ns .ns { max-width: 12ch; overflow: hidden; flex: 0 1 auto; mask-image: linear-gradient(to right, #000 calc(100% - 1.5em), transparent); }
.swatch {
  display: inline-block; width: 0.8em; height: 0.8em; margin-right: 0.3em; vertical-align: -0.08em;
  border-radius: 50%; box-shadow: inset 0 0 0 1px color-mix(in srgb, currentColor 30%, transparent);
}
.count { color: var(--faint); margin-left: 0.2em; font-variation-settings: 'MONO' 1; font-size: 0.9em; }
.x {
  border: 0; background: none; padding: 0 0 0 0.2em; cursor: pointer; color: var(--faint); line-height: 1;
}
.x:hover { color: var(--danger); }
</style>
