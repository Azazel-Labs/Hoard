<script setup lang="ts">
// Settings → Maintenance → Updates: the address Hoard calls once a newer release is out and nothing is going on,
// and what it's waiting for.
import { computed, onMounted, onUnmounted, reactive, ref, watch } from 'vue'
import { ApiError, api, type Settings, type UpdateStatus } from '../api'
import { formatDate, t } from '../i18n'
import { showError, showOk } from '../ui'

const props = defineProps<{ settings: Settings }>()
const emit = defineEmits<{ saved: [Settings]; unauthorized: [] }>()

const form = reactive({ url: '', auth: '', removeAuth: false })
const status = ref<UpdateStatus | null>(null)
const busy = ref(false)
let timer: ReturnType<typeof setTimeout> | undefined
let gone = false

watch(
  () => props.settings.update_hook,
  (h) => Object.assign(form, { url: h.url, auth: '', removeAuth: false }),
  { immediate: true },
)

const keptAuth = computed(() => props.settings.update_hook.has_auth && !form.removeAuth)
const dirty = computed(() => form.url.trim() !== props.settings.update_hook.url || form.auth !== '' || form.removeAuth)

function failed(e: unknown, what: string) {
  if (e instanceof ApiError && e.status === 401) emit('unauthorized')
  showError(`${what}: ${(e as Error).message}`)
}

async function load() {
  clearTimeout(timer)
  try {
    status.value = await api.updateStatus()
  } catch (e) {
    if (e instanceof ApiError && e.status === 401) emit('unauthorized')
  }
  if (!gone) timer = setTimeout(load, 5000)
}
onMounted(load)
onUnmounted(() => {
  gone = true
  clearTimeout(timer)
})

async function save() {
  busy.value = true
  try {
    const url = form.url.trim()
    const patch: { update_hook_url: string; update_hook_auth?: string } = { update_hook_url: url }
    if (form.auth) patch.update_hook_auth = form.auth.trim()
    else if (form.removeAuth || !url) patch.update_hook_auth = ''
    emit('saved', await api.saveSettings(patch))
    showOk(url ? t('updates-saved') : t('updates-saved-none'))
    void load()
  } catch (e) {
    failed(e, t('updates-save-failed'))
  } finally {
    busy.value = false
  }
}

async function callNow() {
  busy.value = true
  try {
    const call = await api.callUpdateHook()
    if (call.ok) showOk(t('updates-call-answered', { result: call.result }))
    else showError(t('updates-call-refused', { result: call.result }))
    void load()
  } catch (e) {
    failed(e, t('updates-call-failed'))
  } finally {
    busy.value = false
  }
}

const when = (iso: string) => formatDate(iso, { dateStyle: 'medium', timeStyle: 'short' })

/** One line on where things stand: nothing to do, waiting for quiet, or how the last call went. */
const line = computed((): { state: string; text: string } | null => {
  const st = status.value
  if (!st) return null
  const last = st.hook.last
  if (last && (!st.update || last.version === st.update.version)) {
    return last.ok
      ? { state: 'ready', text: t('updates-called', { when: when(last.at), version: last.version, result: last.result }) }
      : { state: 'unavailable', text: t('updates-called-failed', { when: when(last.at), version: last.version, result: last.result }) }
  }
  if (!st.update) return { state: 'off', text: t('updates-latest', { version: st.version }) }
  if (!st.hook.set) return { state: 'off', text: t('updates-out', { version: st.update.version }) }
  if (st.idle) return { state: 'looking', text: t('updates-out-idle', { version: st.update.version }) }
  const parts = [
    ...st.busy.work.slice(0, 2),
    st.busy.work.length > 2 ? t('updates-busy-more', { count: st.busy.work.length - 2 }) : '',
    st.busy.queued ? t('updates-busy-jobs', { count: st.busy.queued }) : '',
    st.busy.uploads ? t('updates-busy-uploads', { count: st.busy.uploads }) : '',
  ].filter(Boolean)
  return { state: 'looking', text: t('updates-out-busy', { version: st.update.version, work: parts.join(t('updates-list-separator')) }) }
})
</script>

<template>
  <div class="hook">
    <p class="what">{{ t('updates-what') }}</p>
    <form class="grid" @submit.prevent="save">
      <label for="hook-url">{{ t('updates-url') }}</label>
      <input
        id="hook-url"
        v-model="form.url"
        class="field mono"
        type="url"
        spellcheck="false"
        :placeholder="t('updates-url-placeholder')"
        :disabled="busy"
      />
      <label for="hook-auth">{{ t('updates-auth') }}</label>
      <div class="key">
        <input
          id="hook-auth"
          v-model="form.auth"
          class="field mono"
          type="password"
          autocomplete="off"
          :placeholder="keptAuth ? t('updates-auth-saved') : t('updates-auth-placeholder')"
          :disabled="busy"
          @input="form.removeAuth = false"
        />
        <button v-if="keptAuth && !form.auth" type="button" class="link" @click="form.removeAuth = true">{{ t('updates-remove') }}</button>
      </div>
      <span />
      <div class="row">
        <button class="btn primary" type="submit" :disabled="busy || !dirty">{{ t('updates-save') }}</button>
        <button type="button" class="btn ghost" :disabled="busy || dirty || !settings.update_hook.url" :title="t('updates-call-now-title')" @click="callNow">
          {{ t('updates-call-now') }}
        </button>
      </div>
    </form>
    <p v-if="line && settings.update_hook.url" class="status" :data-state="line.state" role="status">
      <span class="dot" aria-hidden="true" />
      <span class="text">{{ line.text }}</span>
    </p>
    <details class="hint">
      <summary>{{ t('updates-what-calls') }}</summary>
      <ul>
        <li>
          <b>{{ t('updates-watchtower') }}</b>{{ t('updates-watchtower-at') }}<code>http://watchtower:8080/v1/update?container=hoard&amp;async=true</code>{{ t('updates-watchtower-with') }}<code>Bearer</code>{{ t('updates-watchtower-token') }}<code>WATCHTOWER_HTTP_API_TOKEN</code>{{ t('updates-watchtower-its') }}<code>update</code>{{ t('updates-watchtower-endpoint') }}
        </li>
        <li><b>{{ t('updates-portainer') }}</b>{{ t('updates-portainer-rest') }}</li>
        <li>
          <b>{{ t('updates-script') }}</b>{{ t('updates-script-listener') }}<code>adnanh/webhook</code>{{ t('updates-script-runs') }}<code>docker compose pull &amp;&amp; docker compose up -d</code>{{ t('updates-script-end') }}
        </li>
        <li><b>{{ t('updates-others') }}</b>{{ t('updates-others-rest') }}</li>
      </ul>
      <p>
        {{ t('updates-poll-before') }}<code>GET /api/admin/update</code>{{ t('updates-poll-until') }}<code>update</code>{{ t('updates-poll-and') }}<code>idle</code>{{ t('updates-poll-after') }}
      </p>
    </details>
  </div>
</template>

<style scoped>
.hook { margin-top: 1rem; }
.what { margin: 0 0 0.55rem; font-size: var(--step--1); color: var(--muted); line-height: 1.45; }
.grid { display: grid; grid-template-columns: max-content minmax(0, 1fr); gap: 0.5rem 0.8rem; align-items: center; font-size: var(--step--1); }
.grid > label { color: var(--muted); }
.grid > .field, .key .field { width: 100%; max-width: 32rem; min-width: 0; }
.key { display: flex; gap: 0.6rem; align-items: center; }
.key .field { flex: 1; }
.row { display: flex; gap: 0.6rem; flex-wrap: wrap; }
.link { background: none; border: 0; padding: 0; color: var(--muted); text-decoration: underline; cursor: pointer; font: inherit; }
.status { display: flex; align-items: baseline; gap: 0.45rem; margin: 0.7rem 0 0; font-size: var(--step--1); color: var(--muted); }
.status .text { overflow-wrap: anywhere; }
.status .dot { flex: none; width: 0.5rem; height: 0.5rem; border-radius: 50%; background: var(--faint); translate: 0 -0.05rem; }
.status[data-state='ready'] .dot { background: var(--ok); }
.status[data-state='unavailable'] { color: var(--danger); }
.status[data-state='unavailable'] .dot { background: var(--danger); }
.status[data-state='looking'] .dot { background: var(--accent); }
.hint { margin: 0.8rem 0 0; font-size: var(--step--1); color: var(--muted); line-height: 1.45; }
.hint summary { cursor: pointer; width: fit-content; }
.hint ul { margin: 0.4rem 0; padding-left: 1.2rem; display: grid; gap: 0.3rem; }
.hint p { margin: 0.4rem 0 0; }
.hint code { overflow-wrap: anywhere; }
@media (max-width: 560px) {
  .grid { grid-template-columns: minmax(0, 1fr); }
}
</style>
