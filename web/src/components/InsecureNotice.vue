<script setup lang="ts">
// One line under the header when the portal is open over plain http:// from another device: what doesn't work
// without HTTPS, and where to set it up. Not on localhost (browsers treat it as secure) or over HTTPS. Dismissed per
// browser.
import { onMounted, ref } from 'vue'
import { t } from '../i18n'

defineProps<{ member: boolean }>()
const emit = defineEmits<{ settings: [] }>()

const KEY = 'hoard.httpNoticeDismissed'
const host = location.hostname.replace(/^\[|\]$/g, '')
const secureHere =
  location.protocol === 'https:' || host === 'localhost' || host.endsWith('.localhost') || host.startsWith('127.') || host === '::1'

function wasDismissed() {
  try {
    return localStorage.getItem(KEY) === '1'
  } catch {
    return false
  }
}
const show = ref(!secureHere && !wasDismissed())
/** https:// on this address works and this browser trusts it: offer to switch. */
const httpsReady = ref(false)
const httpsUrl = `https://${location.host}${location.pathname}${location.search}${location.hash}`

onMounted(async () => {
  if (!show.value) return
  try {
    await fetch(`https://${location.host}/api/health`, { mode: 'no-cors', cache: 'no-store' })
    httpsReady.value = true
  } catch {
    // HTTPS is off, or this browser doesn't trust the certificate.
  }
})

function dismiss() {
  show.value = false
  try {
    localStorage.setItem(KEY, '1')
  } catch {
    // Private window: it's back next time.
  }
}
</script>

<template>
  <div v-if="show" class="notice" role="note">
    <span class="text">
      {{ t('insecure-text') }}
    </span>
    <a v-if="httpsReady" class="go" :href="httpsUrl">{{ t('insecure-switch') }}</a>
    <button v-else-if="member" type="button" class="go" @click="emit('settings')">{{ t('insecure-set-up') }}</button>
    <button type="button" class="x" :aria-label="t('insecure-dismiss')" :title="t('insecure-dismiss')" @click="dismiss">×</button>
  </div>
</template>

<style scoped>
.notice {
  grid-area: token; display: flex; gap: 0.75rem; align-items: center; padding: 0.35rem 0.6rem 0.35rem 1rem;
  border-bottom: 1px solid var(--line); background: var(--raised); color: var(--muted); font-size: var(--step--1);
}
.text { flex: 1; min-width: 0; line-height: 1.45; }
.go {
  flex: none; padding: 0; border: 0; background: none; color: var(--accent); font: inherit; cursor: pointer;
  text-decoration: underline; text-underline-offset: 0.15em; white-space: nowrap;
}
.x {
  flex: none; width: 1.8rem; height: 1.8rem; border: 0; border-radius: var(--r-chip); background: none; color: var(--muted);
  font-size: 1.2rem; line-height: 1; cursor: pointer;
}
.x:hover { color: var(--text); background: var(--panel); }
</style>
