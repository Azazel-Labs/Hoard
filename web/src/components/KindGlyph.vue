<script setup lang="ts">
import type { AssetKind } from '../api'

defineProps<{ kind: AssetKind; size?: number }>()
</script>

<template>
  <svg
    :width="size ?? 40"
    :height="size ?? 40"
    viewBox="0 0 40 40"
    fill="none"
    :stroke="`var(--k-${kind})`"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
  >
    <template v-if="kind === 'audio'">
      <path d="M6 20v0M11 15v10M16 9v22M21 13v14M26 6v28M31 14v12M36 18v4" />
    </template>
    <template v-else-if="kind === 'model' || kind === 'animation'">
      <path d="M20 5 34 12.5v15L20 35 6 27.5v-15Z" />
      <path d="M6 12.5 20 20l14-7.5M20 20v15" />
      <path v-if="kind === 'animation'" d="M3 33c4 3 10 3 14 0" />
    </template>
    <template v-else-if="kind === 'font'">
      <text x="20" y="28" text-anchor="middle" font-size="20" :fill="`var(--k-${kind})`" stroke="none" font-family="serif">
        Aa
      </text>
    </template>
    <template v-else-if="kind === 'shader' || kind === 'material'">
      <circle cx="20" cy="20" r="13" />
      <path d="M11 14c5 4 13 4 18 0M9 22c6 5 16 5 22 0" />
    </template>
    <template v-else-if="kind === 'texture'">
      <rect x="7" y="7" width="26" height="26" rx="2" />
      <path d="M7 26l8-8 6 6 4-4 8 8" />
      <circle cx="26" cy="14" r="2.5" />
    </template>
    <template v-else-if="kind === 'video'">
      <rect x="6" y="10" width="28" height="20" rx="2" />
      <path d="M17 15v10l8-5Z" />
    </template>
    <template v-else>
      <path d="M11 5h12l7 7v23H11Z" />
      <path d="M23 5v7h7M15 20h10M15 25h10" />
    </template>
  </svg>
</template>
