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
const sourceLabel: Record<TagSource, string> = {
  human: 'added by a person',
  rule: 'from the offline tagger',
  meta: 'measured from the file',
  ai: 'suggested by AI',
}
</script>

<template>
  <span class="chip" :class="source && `src-${source}`" :title="source ? sourceLabel[source] : undefined">
    <i v-if="swatch" class="swatch" :style="{ background: swatch }" aria-hidden="true" />
    <span v-if="parts.ns" class="ns">{{ parts.ns }}:</span>
    <span>{{ parts.value }}</span>
    <span v-if="count !== undefined" class="count">{{ count }}</span>
    <button v-if="removable" class="x" type="button" :aria-label="`Remove tag ${tag}`" @click.stop="$emit('remove')">
      ×
    </button>
  </span>
</template>

<style scoped>
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
