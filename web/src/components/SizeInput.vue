<script setup lang="ts">
// A size typed the way people say it: "100 GB", "512 MB", "1.5 TB". The value is in MiB. A bare number is taken as
// gigabytes, and once it's entered (Enter, or leaving the field) the text is rewritten with its unit, so "100"
// visibly becomes "100 GB". Something that isn't a size is flagged and left as typed until it's fixed.
import { ref, watch } from 'vue'
import { t } from '../i18n'
import { formatMiB, parseMiB } from '../sizes'

const props = defineProps<{ modelValue: number; id?: string; disabled?: boolean }>()
const emit = defineEmits<{ 'update:modelValue': [number] }>()

const text = ref(formatMiB(props.modelValue))
const bad = ref(false)
// A new value from outside (saved, reset) shows at once; typing isn't interrupted, since it doesn't change it.
watch(() => props.modelValue, (v) => {
  text.value = formatMiB(v)
  bad.value = false
})

function commit() {
  const v = parseMiB(text.value)
  bad.value = v === null
  if (v === null) return
  text.value = formatMiB(v)
  if (v !== props.modelValue) emit('update:modelValue', v)
}
</script>

<template>
  <input
    :id="id"
    v-model="text"
    class="field size"
    type="text"
    inputmode="decimal"
    spellcheck="false"
    autocomplete="off"
    :disabled="disabled"
    :aria-invalid="bad || undefined"
    :title="bad ? t('size-invalid') : t('size-hint')"
    @change="commit"
    @keydown.enter.prevent="commit"
    @input="bad = false"
  />
</template>

<style scoped>
.size[aria-invalid='true'] { border-color: var(--danger); box-shadow: 0 0 0 1px var(--danger); }
</style>
