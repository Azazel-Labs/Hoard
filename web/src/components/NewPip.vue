<script setup lang="ts">
// A glinting gold pip by something new in Hoard (see news.ts). Given an `id`, it marks that thing: once it's been on
// screen for a moment it counts as seen and goes. Given `ids` (a page's new things), it marks the way there, with how
// many when there's more than one, and goes once they've all been seen where they are.
import { computed, onBeforeUnmount, ref, watch } from 'vue'
import { formatNumber, t } from '../i18n'
import { isNew, markSeen } from '../news'

const props = defineProps<{ id?: string; ids?: string[] }>()

const count = computed(() => (props.id ? (isNew(props.id) ? 1 : 0) : (props.ids ?? []).filter(isNew).length))
const el = ref<HTMLElement | null>(null)

/** How long a new thing has to be in sight to count as seen. */
const LOOK = 1500
let observer: IntersectionObserver | undefined
let timer: ReturnType<typeof setTimeout> | undefined
function stop() {
  observer?.disconnect()
  observer = undefined
  clearTimeout(timer)
}
watch(
  el,
  (node) => {
    stop()
    if (!node || !props.id) return
    const id = props.id
    observer = new IntersectionObserver(([e]) => {
      clearTimeout(timer)
      if (e.isIntersecting) timer = setTimeout(() => document.visibilityState === 'visible' && markSeen(id), LOOK)
    }, { threshold: 1 })
    observer.observe(node)
  },
  { flush: 'post' },
)
onBeforeUnmount(stop)
</script>

<template>
  <Transition name="newpip">
    <span
      v-if="count"
      ref="el"
      class="newpip"
      :class="{ many: count > 1 }"
      :data-new="id"
      role="img"
      :aria-label="t('news-pip', { count })"
      :title="t('news-pip', { count })"
    >{{ count > 1 ? formatNumber(count) : '' }}</span>
  </Transition>
</template>

<style scoped>
.newpip {
  --pip: #f2c55c;
  display: inline-grid;
  place-items: center;
  flex: none;
  width: 10px;
  height: 10px;
  margin-inline: 0.35rem;
  border-radius: 999px;
  background: var(--pip);
  /* A thin darker edge keeps the gold visible on the lightest themes. */
  outline: 1px solid rgb(150 105 10 / 0.35);
  vertical-align: middle;
  animation: glint 2.2s ease-in-out infinite;
}
.newpip.many {
  width: auto;
  min-width: 1.15rem;
  height: 1.15rem;
  padding: 0 0.3rem;
  font-size: 0.68rem;
  font-weight: 700;
  line-height: 1;
  color: #3b2a05;
  font-variant-numeric: tabular-nums;
}
@keyframes glint {
  0%, 100% { box-shadow: 0 0 0 0 rgb(242 197 92 / 0.55); }
  60% { box-shadow: 0 0 0 7px rgb(242 197 92 / 0); }
}
@media (prefers-reduced-motion: reduce) {
  .newpip { animation: none; }
}
.newpip-leave-active { transition: opacity var(--dur-2, 0.3s), scale var(--dur-2, 0.3s); }
.newpip-leave-to { opacity: 0; scale: 0.4; }
</style>
