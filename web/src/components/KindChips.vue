<script setup lang="ts">
// Toggle chips for picking asset kinds (which kinds a library location takes).
import { KIND_LABEL, type AssetKind } from '../api'

const model = defineModel<AssetKind[]>({ required: true })
defineProps<{ kinds: AssetKind[] }>()

function toggle(k: AssetKind) {
  model.value = model.value.includes(k) ? model.value.filter((x) => x !== k) : [...model.value, k]
}
</script>

<template>
  <div class="kinds" role="group" aria-label="Asset kinds">
    <label v-for="k in kinds" :key="k" class="kind" :class="{ on: model.includes(k) }">
      <input type="checkbox" :checked="model.includes(k)" @change="toggle(k)" />
      {{ KIND_LABEL[k] ?? k }}
    </label>
  </div>
</template>

<style scoped>
.kinds { display: flex; flex-wrap: wrap; gap: 0.35rem; margin-bottom: 0.5rem; }
.kind {
  position: relative; display: inline-flex; align-items: center; gap: 0.35rem; padding: 0.25em 0.65em; border: 1px solid var(--line);
  border-radius: 999px; cursor: pointer; user-select: none; transition: border-color var(--dur-1), background-color var(--dur-1), color var(--dur-1);
}
.kind input { position: absolute; opacity: 0; pointer-events: none; }
.kind:hover { border-color: var(--muted); }
.kind.on { background: var(--text); color: var(--bg); border-color: var(--text); font-weight: 600; }
.kind:has(:focus-visible) { outline: 2px solid var(--accent); outline-offset: 2px; }
</style>
