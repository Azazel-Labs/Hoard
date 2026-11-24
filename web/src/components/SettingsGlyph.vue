<script setup lang="ts">
// A small line icon for each Settings page, drawn in the current color: beside its name in the list and, larger, on
// the page's own heading. Same hand as the top bar's icons (16 units, 1.5 strokes, round ends).
defineProps<{ page: string; size?: number }>()
</script>

<template>
  <svg
    :width="size ?? 16"
    :height="size ?? 16"
    viewBox="0 0 16 16"
    fill="none"
    stroke="currentColor"
    stroke-width="1.5"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
  >
    <template v-if="page === 'dragon'">
      <!-- A horned dragon's head, filled: with icons' dragon-head-solid (MIT, see src/assets/withicons), drawn on 24
           units and scaled to these 16. -->
      <g fill="currentColor" stroke="none" transform="scale(0.6667)">
        <path d="M12 4.6c3.6 0 6.6 2 6.6 5.8c0 2.6-1.2 4.2-2 5.6c-.3 2.5-.2 5.8-4.6 5.8S7.7 18.5 7.4 16c-.8-1.4-2-3-2-5.6c0-3.8 3-5.8 6.6-5.8m1.4 6c0 .9.5 1.6 1.4 1.6C16.1 12.2 17 11 17 9zm-2.8 0L7 9c0 2 .9 3.2 2.2 3.2c.9 0 1.4-.7 1.4-1.6m2.6 8.6l1-.3l-.3-1.1l-1 .4Zm-2.1-1l-1-.4l-.3 1.1l1 .3Z" />
        <path d="M14.6 5.4c.8-2.6 3.4-4 7-4.2c-2 1.6-3 3.8-3.2 6.8c-.8-1.4-2.2-2.4-3.8-2.6m-5.2 0c-1.6.2-3 1.2-3.8 2.6c-.2-3-1.2-5.2-3.2-6.8c3.6.2 6.2 1.6 7 4.2" />
      </g>
    </template>
    <template v-else-if="page === 'appearance'">
      <path d="M8 1.8a6.2 6.2 0 1 0 0 12.4c1 0 1.4-.6 1.4-1.3 0-1.1-1-1.4-1-2.3 0-.8.6-1.3 1.4-1.3h1.7a2.7 2.7 0 0 0 2.7-2.7C14.2 4 11.4 1.8 8 1.8Z" />
      <path d="M4.8 7.2h.01M6.4 4.6h.01M9.6 4.6h.01" stroke-width="2" />
    </template>
    <template v-else-if="page === 'library'">
      <path d="M2.5 13.5h11" />
      <rect x="3.5" y="7.5" width="2.2" height="6" rx="0.5" />
      <rect x="6.9" y="3" width="2.2" height="10.5" rx="0.5" />
      <rect x="10.3" y="5.5" width="2.2" height="8" rx="0.5" />
    </template>
    <template v-else-if="page === 'copies'">
      <rect x="5.5" y="5.5" width="8" height="8" rx="1" />
      <path d="M10.5 3.5V3a.5.5 0 0 0-.5-.5H3a.5.5 0 0 0-.5.5v7a.5.5 0 0 0 .5.5h.5" />
      <path d="m7.5 11.5 1.8-2.2 1.3 1.4 1-1.1" />
    </template>
    <template v-else-if="page === 'tags'">
      <path d="M2 3v4.2a1 1 0 0 0 .3.7l5.8 5.8a1 1 0 0 0 1.4 0l4.2-4.2a1 1 0 0 0 0-1.4L7.9 2.3a1 1 0 0 0-.7-.3H3a1 1 0 0 0-1 1Z" />
      <circle cx="5.2" cy="5.2" r="1" />
    </template>
    <template v-else-if="page === 'store-files'">
      <path d="M2.5 6.5V13a.5.5 0 0 0 .5.5h10a.5.5 0 0 0 .5-.5V6.5" />
      <path d="M1.8 3.8 3 2.5h10l1.2 1.3v1.4a1.6 1.6 0 0 1-3.1.5 1.6 1.6 0 0 1-3.1 0 1.6 1.6 0 0 1-3.1 0A1.6 1.6 0 0 1 1.8 5.2Z" />
      <path d="M6.5 13.5v-3.5h3v3.5" />
    </template>
    <template v-else-if="page === 'locations'">
      <path d="M2 4.5V12a1 1 0 0 0 1 1h10a1 1 0 0 0 1-1V6.5a1 1 0 0 0-1-1H8L6.5 3.5H3a1 1 0 0 0-1 1Z" />
      <path d="M2 8h12" />
    </template>
    <template v-else-if="page === 'tools'">
      <path d="M9.8 2.2a3.3 3.3 0 0 0-3.7 4.5L2.4 10.4a1.4 1.4 0 0 0 2 2l3.7-3.7a3.3 3.3 0 0 0 4.5-3.7l-2 2-1.8-.3-.3-1.8Z" />
    </template>
    <template v-else-if="page === 'ai'">
      <path d="M6.5 2.5 7.6 5.4a1.5 1.5 0 0 0 .9.9l2.9 1.1-2.9 1.1a1.5 1.5 0 0 0-.9.9L6.5 12.3 5.4 9.4a1.5 1.5 0 0 0-.9-.9L1.6 7.4l2.9-1.1a1.5 1.5 0 0 0 .9-.9Z" />
      <path d="M12.3 1.8v2.8M10.9 3.2h2.8M12 10.5v3M10.5 12h3" />
    </template>
    <template v-else-if="page === 'search'">
      <circle cx="7" cy="7" r="4.6" />
      <path d="m10.5 10.5 3.3 3.3" />
      <path d="m4.8 8.6 1.5-1.6 1.2 1.1 1.4-1.6" />
    </template>
    <template v-else-if="page === 'adding'">
      <path d="M8 1.8 13.5 4.5v6.3L8 13.5 2.5 10.8V4.5Z" />
      <path d="M2.5 4.5 8 7.2l5.5-2.7M8 7.2v6.3" />
      <path d="M12.5 10.5v4M10.5 12.5h4" stroke-width="1.7" />
    </template>
    <template v-else-if="page === 'downloads'">
      <path d="M8 2.5v7M5 6.5l3 3 3-3" />
      <path d="M3 13.5h10" />
    </template>
    <template v-else-if="page === 'account'">
      <circle cx="8" cy="5.5" r="2.8" />
      <path d="M2.8 13.8c.6-2.6 2.7-4.1 5.2-4.1s4.6 1.5 5.2 4.1" />
    </template>
    <template v-else-if="page === 'auth'">
      <path d="M8 1.8 13 3.6v4c0 3.1-2.1 5.4-5 6.6-2.9-1.2-5-3.5-5-6.6v-4Z" />
      <circle cx="8" cy="7" r="1.4" />
      <path d="M8 8.4v2" />
    </template>
    <template v-else-if="page === 'connection'">
      <circle cx="8" cy="8" r="6.2" />
      <path d="M1.8 8h12.4M8 1.8c1.7 1.7 2.5 3.8 2.5 6.2S9.7 12.5 8 14.2C6.3 12.5 5.5 10.4 5.5 8S6.3 3.5 8 1.8Z" />
    </template>
    <template v-else-if="page === 'performance'">
      <path d="M2.2 11.5a6 6 0 1 1 11.6 0" />
      <path d="m8 9.5 2.6-3.3" />
      <circle cx="8" cy="9.8" r="0.9" />
    </template>
    <template v-else-if="page === 'issues'">
      <path d="M7.1 2.6 1.9 11.8a1 1 0 0 0 .9 1.5h10.4a1 1 0 0 0 .9-1.5L8.9 2.6a1 1 0 0 0-1.8 0Z" />
      <path d="M8 6.2v3M8 11.2h.01" />
    </template>
    <template v-else-if="page === 'maintenance'">
      <path d="M13.5 8A5.5 5.5 0 0 1 3.4 11M2.5 8a5.5 5.5 0 0 1 10.1-3" />
      <path d="M12.8 1.8v3.3H9.5M3.2 14.2v-3.3h3.3" />
    </template>
    <template v-else-if="page === 'about'">
      <circle cx="8" cy="8" r="6.2" />
      <path d="M8 7.4v3.8M8 4.9h.01" />
    </template>
    <template v-else>
      <circle cx="8" cy="8" r="2.2" />
      <path d="M8 1.8v1.8M8 12.4v1.8M1.8 8h1.8M12.4 8h1.8M3.6 3.6l1.3 1.3M11.1 11.1l1.3 1.3M3.6 12.4l1.3-1.3M11.1 4.9l1.3-1.3" />
    </template>
  </svg>
</template>
