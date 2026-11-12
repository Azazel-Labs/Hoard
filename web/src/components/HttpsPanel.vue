<script setup lang="ts">
// Settings → HTTPS: off, Hoard's own certificate (from its own certificate authority, installed once per device), or
// your own certificate. HTTPS is served on the same port as http://, and a change applies at once.
import { computed, onMounted, ref, watch } from 'vue'
import { api, type CertCheck, type CertInfo, type HttpsMode, type HttpsStatus } from '../api'
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
    showError(`Couldn't load HTTPS: ${(e as Error).message}`)
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
      showOk('HTTPS is off. Taking you to http://')
      setTimeout(() => location.replace(httpUrl), 800)
    } else if (s.on && !was) showOk('HTTPS is on')
    else if (s.on) showOk('Saved. New connections use the new certificate.')
    else showOk('HTTPS is off')
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
    if (change.http3 && s.http3_error) showError(`HTTP/3 couldn't start: ${s.http3_error}`)
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
  if (await copyText(text)) showOk('Copied')
  else showError('Couldn’t copy: select it and copy it yourself')
}

const fmt = new Intl.DateTimeFormat(undefined, { dateStyle: 'medium' })
const day = (unix: number) => fmt.format(new Date(unix * 1000))
const daysLeft = (c: CertInfo) => Math.floor((c.not_after * 1000 - Date.now()) / 86_400_000)

const MODES: { value: HttpsMode; label: string; hint: string }[] = [
  { value: 'off', label: 'Off', hint: 'Plain http:// only.' },
  {
    value: 'own',
    label: 'Hoard’s own certificate',
    hint: 'Hoard makes one for this server’s addresses. Install Hoard’s certificate authority once on each device and its browsers trust it.',
  },
  {
    value: 'custom',
    label: 'My certificate',
    hint: 'One you already have, e.g. from tailscale cert or Let’s Encrypt. Browsers trust it without installing anything.',
  },
]
</script>

<template>
  <div v-if="st" class="https" :aria-busy="busy">
    <section>
      <div class="sh"><h3>Now</h3></div>
      <p v-if="st.error" class="bad">HTTPS is set up but couldn’t start: {{ st.error }}</p>
      <template v-if="st.on">
        <p class="what">
          HTTPS is on, on port {{ st.port }} like http://.
          <template v-if="secure">You’re using it now.</template>
          <a v-else :href="httpsUrl" class="link">Open {{ httpsUrl.replace(/\/$/, '') }}</a>
        </p>
        <p v-if="!secure && trusted === true" class="what good">This browser trusts it.</p>
        <p v-else-if="!secure && trusted === false" class="what warn">
          This browser doesn’t trust it for {{ host }} yet<template v-if="st.mode === 'own'">: install Hoard’s certificate authority below</template><template v-else-if="st.covers_this_address === false">: the certificate doesn’t cover this address</template>.
          You’ll see a warning you can click past.
        </p>
        <p v-if="st.cert" class="what">
          Certificate for <span class="mono">{{ st.cert.names.join(', ') }}</span>, until {{ day(st.cert.not_after) }}<template v-if="st.mode === 'own'"> (renewed by itself)</template>.
        </p>
        <p v-if="st.covers_this_address === false && st.this_address" class="what warn">
          It doesn’t cover <span class="mono">{{ st.this_address }}</span>, the address you’re using.
        </p>
      </template>
      <p v-else class="what">HTTPS is off: pages are served over plain http://.</p>
    </section>

    <section>
      <div class="sh"><h3>HTTPS</h3></div>
      <div class="choices" role="radiogroup" aria-label="HTTPS">
        <label v-for="m in MODES" :key="m.value" :class="{ on: mode === m.value }">
          <input v-model="mode" type="radio" name="https-mode" :value="m.value" :disabled="busy" @change="chose" />
          <span><strong>{{ m.label }}</strong><small>{{ m.hint }}</small></span>
        </label>
      </div>
    </section>

    <section v-if="mode === 'own'">
      <div class="sh"><h3>Addresses</h3></div>
      <p class="what">The certificate covers the addresses Hoard found, and any you add (a name your router gives it, a Tailscale name).</p>
      <ul class="chips">
        <li v-for="n in st.found" :key="n" class="chip mono">{{ n }}</li>
        <li v-for="n in names" :key="n" class="chip mono added">
          {{ n }}
          <button type="button" class="x" :aria-label="`Remove ${n}`" :disabled="busy" @click="removeName(n)">×</button>
        </li>
      </ul>
      <form class="add" @submit.prevent="addName">
        <label class="visually-hidden" for="https-name">Another address</label>
        <input id="https-name" v-model="newName" class="field mono" placeholder="Another address, e.g. nas.local" autocomplete="off" />
        <button type="submit" class="btn" :disabled="!newName.trim() || busy">Add</button>
      </form>
    </section>

    <section v-if="mode === 'custom'">
      <div class="sh"><h3>Your certificate</h3></div>
      <p v-if="st.custom && !pasted" class="what">
        Saved: <span class="mono">{{ st.custom.names.join(', ') || st.custom.subject }}</span> from {{ st.custom.issuer }},
        until {{ day(st.custom.not_after) }}. Paste a new one to replace it.
      </p>
      <p class="what">
        The certificate (with its chain) and its private key, as PEM text: the files that start with
        <span class="mono">-----BEGIN</span>.
      </p>
      <div class="pem">
        <label for="https-cert">Certificate</label>
        <textarea id="https-cert" v-model="cert" class="field mono" rows="5" spellcheck="false" placeholder="-----BEGIN CERTIFICATE-----" />
        <label class="btn ghost small file">Choose file…<input type="file" accept=".pem,.crt,.cer,.txt" @change="pick($event, 'cert')" /></label>
        <label for="https-key">Private key</label>
        <textarea id="https-key" v-model="key" class="field mono" rows="5" spellcheck="false" placeholder="-----BEGIN PRIVATE KEY-----" />
        <label class="btn ghost small file">Choose file…<input type="file" accept=".pem,.key,.txt" @change="pick($event, 'key')" /></label>
      </div>
      <p v-if="checkError" class="bad" role="alert">{{ checkError }}</p>
      <div v-else-if="checked" class="checked" role="status">
        <p>
          For <span class="mono">{{ checked.cert.names.join(', ') || checked.cert.subject }}</span>, from {{ checked.cert.issuer }},
          until {{ day(checked.cert.not_after) }}<template v-if="!checked.problem"> ({{ daysLeft(checked.cert) }} days)</template>.
        </p>
        <p v-if="checked.problem" class="bad">{{ checked.problem }}</p>
        <p v-else-if="checked.cert.self_signed" class="warn">It’s self-signed: browsers will warn about it.</p>
        <p v-if="checked.covers_this_address === false" class="warn">
          It doesn’t cover <span class="mono">{{ checked.this_address }}</span>, the address you’re using.
        </p>
        <p v-else-if="!checked.problem" class="good">The key matches.</p>
      </div>
      <details class="how">
        <summary>Where to get one</summary>
        <ul>
          <li>
            <strong>Tailscale:</strong> turn on HTTPS for your tailnet, then run
            <span class="mono">tailscale cert &lt;name&gt;.&lt;tailnet&gt;.ts.net</span> on the server and upload the two files.
            They last 90 days: run it again and upload them again before then.
          </li>
          <li>
            <strong>Let’s Encrypt or another ACME client</strong> (certbot, acme.sh, lego) for a domain you own: upload
            <span class="mono">fullchain.pem</span> and <span class="mono">privkey.pem</span>.
          </li>
          <li>
            <strong>A reverse proxy</strong> (Caddy, Traefik, nginx, Cloudflare Tunnel) that has the certificate: leave HTTPS off
            here and let the proxy send plain http:// to Hoard.
          </li>
        </ul>
      </details>
    </section>

    <div class="apply">
      <button type="button" class="btn primary" :disabled="!canApply" @click="apply">
        {{ busy ? 'Trying it…' : 'Apply' }}
      </button>
      <small v-if="changed">Hoard tries the new setup first, and keeps what it has if that doesn’t work.</small>
    </div>

    <section v-if="st.mode === 'own' && st.ca">
      <div class="sh">
        <h3>Trust Hoard on your devices</h3>
        <a class="btn" href="/api/https/ca.crt" download="hoard-ca.crt">Download hoard-ca.crt</a>
      </div>
      <p class="what">
        Install Hoard’s certificate authority once on each device, and its browsers trust this server (and any certificate
        Hoard makes for it later). It can only vouch for this Hoard. Fingerprint (SHA-256):
      </p>
      <p class="fp mono" @click="copy(st.ca.fingerprint)">{{ st.ca.fingerprint }}</p>
      <details class="how">
        <summary>Windows (Chrome, Edge)</summary>
        <p>Open the file → Install Certificate → Current User → Place all certificates in the following store → Browse → Trusted Root Certification Authorities → Finish. Restart the browser.</p>
      </details>
      <details class="how">
        <summary>macOS (Safari, Chrome)</summary>
        <p>Open the file; it goes into Keychain Access. Double-click “{{ st.ca.subject }}” there → Trust → When using this certificate: Always Trust.</p>
      </details>
      <details class="how">
        <summary>iPhone and iPad</summary>
        <p>Download it in Safari and allow it. Settings → Profile Downloaded → Install. Then Settings → General → About → Certificate Trust Settings → turn on “{{ st.ca.subject }}”.</p>
      </details>
      <details class="how">
        <summary>Android</summary>
        <p>Settings → Security → More security settings → Encryption &amp; credentials → Install a certificate → CA certificate, and pick the file. (The names vary a little between phones; search Settings for “certificate”.)</p>
      </details>
      <details class="how">
        <summary>Linux</summary>
        <p>
          Debian and Ubuntu: <span class="mono">sudo cp hoard-ca.crt /usr/local/share/ca-certificates/ &amp;&amp; sudo update-ca-certificates</span>.
          Fedora and Arch: <span class="mono">sudo trust anchor hoard-ca.crt</span>. Chrome on Linux keeps its own list:
          Settings → Privacy and security → Security → Manage certificates → Import.
        </p>
      </details>
      <details class="how">
        <summary>Firefox</summary>
        <p>Firefox keeps its own list: Settings → Privacy &amp; Security → Certificates → View Certificates → Authorities → Import, and tick “Trust this CA to identify websites”.</p>
      </details>
      <details class="how">
        <summary>The hoard command line tool</summary>
        <p><span class="mono">hoard --server https://{{ here }} --ca-cert hoard-ca.crt …</span>, or put the path in <span class="mono">HOARD_CA_CERT</span>.</p>
      </details>
    </section>

    <section>
      <div class="sh"><h3>Options</h3></div>
      <label class="check">
        <input type="checkbox" :checked="st.redirect" :disabled="busy" @change="toggle({ redirect: ($event.target as HTMLInputElement).checked })" />
        Send browsers on other devices from http:// to https://
      </label>
      <p class="what">
        Only page loads are sent over. The hoard command line tool, the Unreal worker, scripts and AI apps keep working on
        http://, and so does http://localhost on the server itself.
      </p>
      <label class="check">
        <input type="checkbox" :checked="st.http3" :disabled="busy" @change="toggle({ http3: ($event.target as HTMLInputElement).checked })" />
        HTTP/3
      </label>
      <p class="what">
        Faster on flaky Wi-Fi and phones. Needs HTTPS, and UDP port {{ st.port }} open as well as TCP (in Docker:
        <span class="mono">"{{ st.port }}:{{ st.port }}/udp"</span>).
        <template v-if="st.http3 && st.http3_running"> Running.</template>
        <template v-else-if="st.http3 && !st.on"> It starts when HTTPS is on.</template>
      </p>
      <p v-if="st.http3_error" class="bad">HTTP/3 couldn’t start: {{ st.http3_error }}</p>
    </section>

    <section>
      <div class="sh"><h3>If you get locked out</h3></div>
      <p class="what">
        <span class="mono">http://localhost:{{ st.port }}</span> on the server itself always works over plain http://. Or turn
        HTTPS off from the server’s command line and start Hoard again:
        <span class="mono">hoardd --https-off</span> (in Docker: <span class="mono">docker compose exec hoard hoardd --https-off</span>,
        then <span class="mono">docker compose restart hoard</span>).
      </p>
    </section>
  </div>
</template>

<style scoped>
.https { display: grid; gap: 1.6rem; font-size: var(--step--1); }
section { display: grid; gap: 0.6rem; }
.sh { display: flex; align-items: center; justify-content: space-between; gap: 1rem; flex-wrap: wrap; }
.sh h3 { margin: 0; }
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

.apply { display: flex; gap: 0.8rem; align-items: center; flex-wrap: wrap; }
.apply small { color: var(--muted); }
.check { display: flex; gap: 0.5rem; align-items: center; }
.small { padding-block: 0.1rem; font-size: 0.85em; }
@media (max-width: 560px) {
  .pem { grid-template-columns: minmax(0, 1fr); }
}
</style>
