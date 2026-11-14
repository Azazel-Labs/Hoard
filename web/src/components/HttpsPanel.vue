<script setup lang="ts">
// Settings → Connection: off, Hoard's own certificate (from its own certificate authority, installed once per device), or
// your own certificate. HTTPS is served on the same port as http://, and a change applies at once.
import { computed, onMounted, ref, watch } from 'vue'
import { api, type CertCheck, type CertInfo, type HttpsMode, type HttpsStatus } from '../api'
import { formatDate, t } from '../i18n'
import { copyText, showError, showOk } from '../ui'

const st = ref<HttpsStatus | null>(null)
const busy = ref(false)
const mode = ref<HttpsMode>('off')
const names = ref<string[]>([])
const newName = ref('')
const cert = ref('')
const key = ref('')
const checked = ref<CertCheck | null>(null)
const checkError = ref('')
/** Whether this browser trusts https:// on this address: null while finding out or when there's nothing to try. */
const trusted = ref<boolean | null>(null)

const secure = location.protocol === 'https:'
const here = location.host
const host = location.hostname.replace(/^\[|\]$/g, '')
const onLocalhost = ['localhost', '127.0.0.1', '::1'].includes(host)
const httpsUrl = `https://${location.host}${location.pathname}${location.search}`
const httpUrl = `http://${location.host}${location.pathname}${location.search}`

function adopt(s: HttpsStatus) {
  st.value = s
  mode.value = s.mode
  names.value = [...s.names]
  probe()
}

async function load() {
  try {
    adopt(await api.https())
  } catch (e) {
    showError(t('https-load-failed', { error: (e as Error).message }))
  }
}
onMounted(load)

/** A request to https:// on this address works only if this browser trusts the certificate for it. */
async function probe() {
  trusted.value = null
  if (secure || !st.value?.on) return
  try {
    await fetch(`https://${location.host}/api/health`, { mode: 'no-cors', cache: 'no-store' })
    trusted.value = true
  } catch {
    trusted.value = false
  }
}

const covered = (n: string, list: string[]) =>
  list.some((c) => c === n || (c.startsWith('*.') && n.split('.').slice(1).join('.') === c.slice(2)))
const allNames = computed(() => [...(st.value?.found ?? []), ...names.value])

// The address in the browser's bar goes on Hoard's certificate: it can't see it from inside a container or behind
// port forwarding.
function chose() {
  if (mode.value === 'own' && st.value && !onLocalhost && !covered(host, allNames.value)) names.value = [...names.value, host]
}

function addName() {
  const n = newName.value.trim().replace(/^[a-z]+:\/\//i, '').replace(/[/:?#].*$/, '').toLowerCase()
  if (!n) return
  if (!covered(n, allNames.value)) names.value = [...names.value, n]
  newName.value = ''
}
function removeName(n: string) {
  names.value = names.value.filter((x) => x !== n)
}

const sameNames = computed(() => JSON.stringify([...names.value].sort()) === JSON.stringify([...(st.value?.names ?? [])].sort()))
const pasted = computed(() => cert.value.trim() !== '' || key.value.trim() !== '')
const changed = computed(() => !!st.value && (mode.value !== st.value.mode || !sameNames.value || (mode.value === 'custom' && pasted.value)))
const canApply = computed(() => {
  if (!changed.value || busy.value) return false
  if (mode.value === 'custom' && !st.value?.custom && !pasted.value) return false
  if (mode.value === 'custom' && pasted.value && (!checked.value || checked.value.problem)) return false
  return true
})

async function apply() {
  if (!canApply.value || !st.value) return
  const was = st.value.on
  busy.value = true
  try {
    const s = await api.saveHttps({
      mode: mode.value,
      names: names.value,
      ...(mode.value === 'custom' && pasted.value ? { cert: cert.value, key: key.value } : {}),
    })
    cert.value = ''
    key.value = ''
    checked.value = null
    adopt(s)
    if (!s.on && secure) {
      showOk(t('https-off-redirecting'))
      setTimeout(() => location.replace(httpUrl), 800)
    } else if (s.on && !was) showOk(t('https-now-on'))
    else if (s.on) showOk(t('https-saved'))
    else showOk(t('https-now-off'))
  } catch (e) {
    showError((e as Error).message)
  } finally {
    busy.value = false
  }
}

async function toggle(change: { http3?: boolean; redirect?: boolean }) {
  busy.value = true
  try {
    const s = await api.saveHttps(change)
    st.value = { ...s }
    if (change.http3 && s.http3_error) showError(t('https-http3-failed-toast', { error: s.http3_error }))
  } catch (e) {
    showError((e as Error).message)
  } finally {
    busy.value = false
  }
}

// ---- my certificate
let timer: ReturnType<typeof setTimeout> | undefined
watch([cert, key], () => {
  clearTimeout(timer)
  checked.value = null
  checkError.value = ''
  if (!pasted.value) return
  timer = setTimeout(check, 400)
})
async function check() {
  try {
    checked.value = await api.checkCert(cert.value, key.value)
    checkError.value = ''
  } catch (e) {
    checked.value = null
    checkError.value = (e as Error).message
  }
}
async function pick(e: Event, into: 'cert' | 'key') {
  const file = (e.target as HTMLInputElement).files?.[0]
  if (!file) return
  const text = await file.text()
  if (into === 'cert') cert.value = text
  else key.value = text
  ;(e.target as HTMLInputElement).value = ''
}

async function copy(text: string) {
  if (await copyText(text)) showOk(t('https-copied'))
  else showError(t('https-copy-failed'))
}

const day = (unix: number) => formatDate(unix * 1000, { dateStyle: 'medium' })
const daysLeft = (c: CertInfo) => Math.floor((c.not_after * 1000 - Date.now()) / 86_400_000)

// A function, so the labels are read in the current language each time they're shown.
const MODES = (): { value: HttpsMode; label: string; hint: string }[] => [
  { value: 'off', label: t('https-mode-off'), hint: t('https-mode-off-hint') },
  { value: 'own', label: t('https-mode-own'), hint: t('https-mode-own-hint') },
  { value: 'custom', label: t('https-mode-custom'), hint: t('https-mode-custom-hint') },
]
/** Why this browser doesn't trust https:// here yet. */
const untrusted = computed(() =>
  st.value?.mode === 'own'
    ? t('https-untrusted-own', { host })
    : st.value?.covers_this_address === false
      ? t('https-untrusted-uncovered', { host })
      : t('https-untrusted', { host }),
)
</script>

<template>
  <div v-if="st" class="https" :aria-busy="busy">
    <section>
      <div class="sh"><h3>{{ t('https-now') }}</h3></div>
      <p v-if="st.error" class="bad">{{ t('https-start-failed', { error: st.error }) }}</p>
      <template v-if="st.on">
        <p class="what">
          {{ t('https-on-port', { port: String(st.port) }) }}
          <template v-if="secure">{{ t('https-using-it') }}</template>
          <a v-else :href="httpsUrl" class="link">{{ t('https-open', { url: httpsUrl.replace(/\/$/, '') }) }}</a>
        </p>
        <p v-if="!secure && trusted === true" class="what good">{{ t('https-trusted') }}</p>
        <p v-else-if="!secure && trusted === false" class="what warn">
          {{ untrusted }}
        </p>
        <p v-if="st.cert" class="what">
          {{ t('https-cert-for-before') }}<span class="mono">{{ st.cert.names.join(', ') }}</span>{{ st.mode === 'own' ? t('https-cert-for-after-own', { date: day(st.cert.not_after) }) : t('https-cert-for-after', { date: day(st.cert.not_after) }) }}
        </p>
        <p v-if="st.covers_this_address === false && st.this_address" class="what warn">
          {{ t('https-not-covered-before') }}<span class="mono">{{ st.this_address }}</span>{{ t('https-not-covered-after') }}
        </p>
      </template>
      <p v-else class="what">{{ t('https-is-off') }}</p>
    </section>

    <section>
      <div class="sh"><h3>HTTPS</h3></div>
      <div class="choices" role="radiogroup" aria-label="HTTPS">
        <label v-for="m in MODES()" :key="m.value" :class="{ on: mode === m.value }">
          <input v-model="mode" type="radio" name="https-mode" :value="m.value" :disabled="busy" @change="chose" />
          <span><strong>{{ m.label }}</strong><small>{{ m.hint }}</small></span>
        </label>
      </div>
    </section>

    <section v-if="mode === 'own'">
      <div class="sh"><h3>{{ t('https-addresses') }}</h3></div>
      <p class="what">{{ t('https-addresses-hint') }}</p>
      <ul class="chips">
        <li v-for="n in st.found" :key="n" class="chip mono">{{ n }}</li>
        <li v-for="n in names" :key="n" class="chip mono added">
          {{ n }}
          <button type="button" class="x" :aria-label="t('https-remove-name', { name: n })" :disabled="busy" @click="removeName(n)">×</button>
        </li>
      </ul>
      <form class="add" @submit.prevent="addName">
        <label class="visually-hidden" for="https-name">{{ t('https-another-address') }}</label>
        <input id="https-name" v-model="newName" class="field mono" :placeholder="t('https-another-address-placeholder')" autocomplete="off" />
        <button type="submit" class="btn" :disabled="!newName.trim() || busy">{{ t('https-add') }}</button>
      </form>
    </section>

    <section v-if="mode === 'custom'">
      <div class="sh"><h3>{{ t('https-your-cert') }}</h3></div>
      <p v-if="st.custom && !pasted" class="what">
        {{ t('https-saved-cert-before') }}<span class="mono">{{ st.custom.names.join(', ') || st.custom.subject }}</span>{{ t('https-saved-cert-after', { issuer: st.custom.issuer, date: day(st.custom.not_after) }) }}
      </p>
      <p class="what">
        {{ t('https-pem-before') }}<span class="mono">-----BEGIN</span>{{ t('https-pem-after') }}
      </p>
      <div class="pem">
        <label for="https-cert">{{ t('https-certificate') }}</label>
        <textarea id="https-cert" v-model="cert" class="field mono" rows="5" spellcheck="false" placeholder="-----BEGIN CERTIFICATE-----" />
        <label class="btn ghost small file">{{ t('https-choose-file') }}<input type="file" accept=".pem,.crt,.cer,.txt" @change="pick($event, 'cert')" /></label>
        <label for="https-key">{{ t('https-private-key') }}</label>
        <textarea id="https-key" v-model="key" class="field mono" rows="5" spellcheck="false" placeholder="-----BEGIN PRIVATE KEY-----" />
        <label class="btn ghost small file">{{ t('https-choose-file') }}<input type="file" accept=".pem,.key,.txt" @change="pick($event, 'key')" /></label>
      </div>
      <p v-if="checkError" class="bad" role="alert">{{ checkError }}</p>
      <div v-else-if="checked" class="checked" role="status">
        <p>
          {{ t('https-checked-before') }}<span class="mono">{{ checked.cert.names.join(', ') || checked.cert.subject }}</span>{{
            checked.problem
              ? t('https-checked-after', { issuer: checked.cert.issuer, date: day(checked.cert.not_after) })
              : t('https-checked-after-days', { issuer: checked.cert.issuer, date: day(checked.cert.not_after), days: daysLeft(checked.cert) })
          }}
        </p>
        <p v-if="checked.problem" class="bad">{{ checked.problem }}</p>
        <p v-else-if="checked.cert.self_signed" class="warn">{{ t('https-self-signed') }}</p>
        <p v-if="checked.covers_this_address === false" class="warn">
          {{ t('https-not-covered-before') }}<span class="mono">{{ checked.this_address }}</span>{{ t('https-not-covered-after') }}
        </p>
        <p v-else-if="!checked.problem" class="good">{{ t('https-key-matches') }}</p>
      </div>
      <details class="how">
        <summary>{{ t('https-where-to-get') }}</summary>
        <ul>
          <li>
            <strong>Tailscale:</strong> {{ t('https-get-tailscale-before') }}<span class="mono">tailscale cert &lt;name&gt;.&lt;tailnet&gt;.ts.net</span>{{ t('https-get-tailscale-after') }}
          </li>
          <li>
            <strong>{{ t('https-get-acme-title') }}</strong> {{ t('https-get-acme-before') }}<span class="mono">fullchain.pem</span>{{ t('https-get-acme-and') }}<span class="mono">privkey.pem</span>{{ t('https-get-acme-after') }}
          </li>
          <li>
            <strong>{{ t('https-get-proxy-title') }}</strong> {{ t('https-get-proxy') }}
          </li>
        </ul>
      </details>
    </section>

    <div class="apply">
      <button type="button" class="btn primary" :disabled="!canApply" @click="apply">
        {{ busy ? t('https-trying') : t('https-apply') }}
      </button>
      <small v-if="changed">{{ t('https-apply-hint') }}</small>
    </div>

    <section v-if="st.mode === 'own' && st.ca">
      <div class="sh">
        <h3>{{ t('https-trust-title') }}</h3>
        <a class="btn" href="/api/https/ca.crt" download="hoard-ca.crt">{{ t('https-download-ca') }}</a>
      </div>
      <p class="what">
        {{ t('https-trust-hint') }}
      </p>
      <p class="fp mono" @click="copy(st.ca.fingerprint)">{{ st.ca.fingerprint }}</p>
      <details class="how">
        <summary>Windows (Chrome, Edge)</summary>
        <p>{{ t('https-ca-windows') }}</p>
      </details>
      <details class="how">
        <summary>macOS (Safari, Chrome)</summary>
        <p>{{ t('https-ca-macos', { subject: st.ca.subject }) }}</p>
      </details>
      <details class="how">
        <summary>{{ t('https-ca-ios-title') }}</summary>
        <p>{{ t('https-ca-ios', { subject: st.ca.subject }) }}</p>
      </details>
      <details class="how">
        <summary>Android</summary>
        <p>{{ t('https-ca-android') }}</p>
      </details>
      <details class="how">
        <summary>Linux</summary>
        <p>
          {{ t('https-ca-linux-1') }}<span class="mono">sudo cp hoard-ca.crt /usr/local/share/ca-certificates/ &amp;&amp; sudo update-ca-certificates</span>{{ t('https-ca-linux-2') }}<span class="mono">sudo trust anchor hoard-ca.crt</span>{{ t('https-ca-linux-3') }}
        </p>
      </details>
      <details class="how">
        <summary>Firefox</summary>
        <p>{{ t('https-ca-firefox') }}</p>
      </details>
      <details class="how">
        <summary>{{ t('https-ca-cli-title') }}</summary>
        <p>{{ t('https-ca-cli-before') }}<span class="mono">hoard --server https://{{ here }} --ca-cert hoard-ca.crt …</span>{{ t('https-ca-cli-middle') }}<span class="mono">HOARD_CA_CERT</span>{{ t('https-ca-cli-after') }}</p>
      </details>
    </section>

    <section>
      <div class="sh"><h3>{{ t('https-options') }}</h3></div>
      <label class="check">
        <input type="checkbox" :checked="st.redirect" :disabled="busy" @change="toggle({ redirect: ($event.target as HTMLInputElement).checked })" />
        {{ t('https-redirect') }}
      </label>
      <p class="what">
        {{ t('https-redirect-hint') }}
      </p>
      <label class="check">
        <input type="checkbox" :checked="st.http3" :disabled="busy" @change="toggle({ http3: ($event.target as HTMLInputElement).checked })" />
        HTTP/3
      </label>
      <p class="what">
        {{ t('https-http3-hint-before', { port: String(st.port) }) }}<span class="mono">"{{ st.port }}:{{ st.port }}/udp"</span>{{ t('https-http3-hint-after') }}
        <template v-if="st.http3 && st.http3_running"> {{ t('https-http3-running') }}</template>
        <template v-else-if="st.http3 && !st.on"> {{ t('https-http3-waits') }}</template>
      </p>
      <p v-if="st.http3_error" class="bad">{{ t('https-http3-failed', { error: st.http3_error }) }}</p>
    </section>

    <section>
      <div class="sh"><h3>{{ t('https-lockout-title') }}</h3></div>
      <p class="what">
        {{ t('https-lockout-1') }}<span class="mono">http://localhost:{{ st.port }}</span>{{ t('https-lockout-2') }}<span class="mono">hoardd --https-off</span>{{ t('https-lockout-3') }}<span class="mono">docker compose exec hoard hoardd --https-off</span>{{ t('https-lockout-4') }}<span class="mono">docker compose restart hoard</span>{{ t('https-lockout-5') }}
      </p>
    </section>
  </div>
</template>

<style scoped>
.https { display: grid; font-size: var(--step--1); }
/* The rest of Settings' look: rules between sections, small headings. It follows the Address section. */
section { display: grid; gap: 0.6rem; padding: 1rem 0 1.1rem; border-top: 1px solid var(--line); }
.sh { display: flex; align-items: center; justify-content: space-between; gap: 1rem; flex-wrap: wrap; min-height: 1.75rem; }
h3 { margin: 0; font-size: 0.7rem; font-weight: 650; letter-spacing: 0.04em; text-transform: uppercase; color: var(--faint); }
p { margin: 0; }
.what { color: var(--muted); line-height: 1.5; }
.bad { color: var(--danger); line-height: 1.5; }
.warn { color: var(--text); line-height: 1.5; }
.good { color: var(--ok); line-height: 1.5; }
.link { margin-left: 0.3rem; }
.mono { overflow-wrap: anywhere; }

.choices { display: grid; gap: 0.4rem; }
.choices label {
  display: flex; gap: 0.6rem; align-items: flex-start; padding: 0.55rem 0.7rem; border: 1px solid var(--line);
  border-radius: var(--r-chip); cursor: pointer; transition: border-color var(--dur-1), background-color var(--dur-1);
}
.choices label.on { border-color: var(--muted); background: var(--raised); }
.choices input { margin: 0.2rem 0 0; }
.choices span { display: grid; gap: 0.1rem; }
.choices small { color: var(--muted); font-size: inherit; line-height: 1.45; }

.chips { display: flex; flex-wrap: wrap; gap: 0.35rem; margin: 0; padding: 0; list-style: none; }
.chip { display: inline-flex; align-items: center; gap: 0.3rem; padding: 0.15rem 0.55rem; border: 1px solid var(--line); border-radius: var(--r-chip); color: var(--muted); }
.chip.added { color: var(--text); border-color: var(--muted); }
.x { border: 0; background: none; color: var(--muted); cursor: pointer; padding: 0 0.1rem; font-size: 1.1em; line-height: 1; }
.x:hover { color: var(--text); }
.add { display: flex; gap: 0.5rem; flex-wrap: wrap; }
.add .field { flex: 1 1 12rem; min-width: 0; }

.pem { display: grid; grid-template-columns: max-content minmax(0, 1fr) max-content; gap: 0.45rem 0.7rem; align-items: start; }
.pem label:not(.file) { padding-top: 0.4rem; }
.pem textarea { resize: vertical; font-size: 0.85em; min-height: 5.5rem; }
.file { position: relative; overflow: hidden; cursor: pointer; }
.file input { position: absolute; inset: 0; opacity: 0; cursor: pointer; }
.checked { display: grid; gap: 0.3rem; padding: 0.6rem 0.8rem; border: 1px solid var(--line); border-radius: var(--r-chip); background: var(--raised); line-height: 1.5; }

.how { color: var(--muted); line-height: 1.5; }
.how summary { cursor: pointer; color: var(--text); }
.how p, .how ul { margin: 0.35rem 0 0.2rem; }
.how ul { padding-left: 1.1rem; display: grid; gap: 0.35rem; }
.how strong { color: var(--text); }
.fp { padding: 0.4rem 0.6rem; border-radius: var(--r-chip); background: var(--raised); font-size: 0.85em; cursor: copy; }

.apply { display: flex; padding-bottom: 1.1rem; gap: 0.8rem; align-items: center; flex-wrap: wrap; }
.apply small { color: var(--muted); }
.check { display: flex; gap: 0.5rem; align-items: center; }
.small { padding-block: 0.1rem; font-size: 0.85em; }
@media (max-width: 560px) {
  .pem { grid-template-columns: minmax(0, 1fr); }
}
</style>
