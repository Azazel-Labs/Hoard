<script setup lang="ts">
// Settings → Account: your password, the other accounts, connecting Claude and other AI apps, access tokens for
// scripts and programs, and whether people who aren't signed in may look around.
import { computed, onMounted, reactive, ref } from 'vue'
import { api, type AccessToken, type Account, type AccountSource, type Connection, type Role } from '../api'
import { auth, may, refreshAuth, signOut } from '../auth'
import { confirmDialog, copyText, showError, showOk } from '../ui'

const accounts = ref<Account[] | null>(null)
const tokens = ref<AccessToken[] | null>(null)
const connections = ref<Connection[] | null>(null)
const guest = ref<boolean | null>(null)
const busy = ref(false)
const origin = ref(location.origin)

const admin = computed(() => may('admin'))
const myRole = computed<Role>(() => auth.status?.user?.role ?? 'view')

const ROLES: { id: Role; label: string; hint: string }[] = [
  { id: 'view', label: 'Can look', hint: 'Search, preview and download' },
  { id: 'edit', label: 'Can change', hint: 'Also add, change, tag, collect and archive' },
  { id: 'admin', label: 'Admin', hint: 'Also Settings, accounts and maintenance, and deleting for good' },
]
const roleLabel = (r: Role) => ROLES.find((x) => x.id === r)!.label
/** Someone whose password the directory or OAuth provider keeps, not Hoard. */
const external = computed(() => !!auth.status?.user?.source && auth.status.user.source !== 'internal')
const SOURCES = computed<{ id: AccountSource; label: string }[]>(() => {
  const s = auth.status?.sign_in
  return [
    { id: 'internal', label: 'A Hoard password' },
    ...(s?.directory ? [{ id: 'ldap' as const, label: `Their ${s.directory} password` }] : []),
    ...(s?.oauth ? [{ id: 'oauth' as const, label: `Signing in with ${s.oauth}` }] : []),
  ]
})
const sourceWord = (s: AccountSource) => (s === 'ldap' ? 'directory' : s === 'oauth' ? 'OAuth' : '')
/** What a token may be given: up to what you can do. */
const tokenRoles = computed(() => ROLES.slice(0, ROLES.findIndex((r) => r.id === myRole.value) + 1))

async function load() {
  try {
    const [t, c] = await Promise.all([api.tokens(), api.connections()])
    tokens.value = t.tokens
    connections.value = c.connections
    if (admin.value) {
      const [a, s] = await Promise.all([api.accounts(), api.settings()])
      accounts.value = a.users
      guest.value = s.options.guest_browsing.value
      origin.value = (s.options.public_url.value || location.origin).replace(/\/+$/, '')
    }
  } catch (e) {
    showError(`Couldn't load accounts: ${(e as Error).message}`)
  }
}
onMounted(load)

async function run(what: string, f: () => Promise<unknown>): Promise<boolean> {
  busy.value = true
  try {
    await f()
    return true
  } catch (e) {
    showError(`${what}: ${(e as Error).message}`)
    return false
  } finally {
    busy.value = false
  }
}

// ---- your password
const pw = reactive({ current: '', next: '', again: '' })
const pwMismatch = computed(() => pw.again !== '' && pw.again !== pw.next)
const pwReady = computed(() => pw.current !== '' && pw.next.length >= 8 && pw.again === pw.next)
async function changePassword() {
  if (!pwReady.value) return
  if (await run("Couldn't change your password", () => api.changePassword(pw.current, pw.next))) {
    Object.assign(pw, { current: '', next: '', again: '' })
    showOk('Password changed. Your other devices are signed out.')
  }
}

// ---- other people
const person = reactive({ username: '', password: '', role: 'edit' as Role, source: 'internal' as AccountSource })
const personReady = computed(() => person.username.trim() !== '' && (person.source !== 'internal' || person.password.length >= 8))
async function addPerson() {
  if (!personReady.value) return
  const name = person.username.trim()
  const pw = person.source === 'internal' ? person.password : ''
  if (await run(`Couldn't add ${name}`, () => api.addAccount(name, pw, person.role, person.source))) {
    Object.assign(person, { username: '', password: '', role: 'edit', source: 'internal' })
    showOk(`${name} can sign in now.`)
    await load()
  }
}
async function setRole(a: Account, role: Role) {
  if (await run(`Couldn't change what ${a.username} can do`, () => api.setRole(a.id, role))) await load()
}
async function removePerson(a: Account) {
  const ok = await confirmDialog({
    title: `Remove ${a.username}?`,
    body: 'They’re signed out everywhere and can’t sign in again. Nothing they added to the library is removed.',
    confirm: 'Remove',
    danger: true,
  })
  if (ok && (await run(`Couldn't remove ${a.username}`, () => api.removeAccount(a.id)))) await load()
}

// ---- access tokens
const tokenName = ref('')
const tokenRole = ref<Role>('view')
const made = ref<{ name: string; token: string } | null>(null)
async function makeToken() {
  const name = tokenName.value.trim()
  if (!name) return
  await run("Couldn't make the token", async () => {
    const t = await api.makeToken(name, tokenRole.value)
    made.value = { name: t.name, token: t.token }
    tokenName.value = ''
    await load()
  })
}
async function copyToken() {
  if (!made.value) return
  if (await copyText(made.value.token)) showOk('Copied')
  else showError('Couldn’t copy: select the token and copy it yourself')
}
async function revoke(t: AccessToken) {
  const ok = await confirmDialog({
    title: `Revoke “${t.name}”?`,
    body: 'Anything still using it is turned away from now on.',
    confirm: 'Revoke',
    danger: true,
  })
  if (ok && (await run(`Couldn't revoke “${t.name}”`, () => api.revokeToken(t.id)))) {
    if (made.value?.name === t.name) made.value = null
    await load()
  }
}

// ---- Claude and other AI apps
const mcpUrl = computed(() => `${origin.value}/mcp`)
const claudeCode = computed(() => `claude mcp add --transport http --scope user hoard ${mcpUrl.value}`)
const local = computed(() => /^http:\/\/|\/\/(localhost|127\.|10\.|192\.168\.|172\.(1[6-9]|2\d|3[01])\.)/.test(origin.value))
async function copy(text: string) {
  if (await copyText(text)) showOk('Copied')
  else showError('Couldn’t copy: select it and copy it yourself')
}
async function disconnect(c: Connection) {
  const ok = await confirmDialog({
    title: `Disconnect ${c.app}?`,
    body: 'It’s turned away from now on, and has to be connected again to use Hoard.',
    confirm: 'Disconnect',
    danger: true,
  })
  if (ok && (await run(`Couldn't disconnect ${c.app}`, () => api.disconnect(c.id)))) await load()
}

// ---- guests
async function setGuest(on: boolean) {
  if (await run("Couldn't change that", () => api.saveSettings({ guest_browsing: on }))) {
    guest.value = on
    await refreshAuth()
  }
}

const fmt = new Intl.DateTimeFormat(undefined, { dateStyle: 'medium' })
const day = (at: string | null) => (at ? fmt.format(new Date(at)) : 'never')
</script>

<template>
  <div class="account" :aria-busy="busy">
    <section>
      <div class="sh">
        <h3>You</h3>
        <button type="button" class="btn" @click="signOut">Sign out</button>
      </div>
      <p class="what">
        Signed in as <strong>{{ auth.status?.user?.name }}</strong>, who
        {{ myRole === 'admin' ? 'is an admin' : myRole === 'edit' ? 'can change the library' : 'can look but not change anything' }}.
      </p>
      <p v-if="external" class="what">
        Your password is kept by the {{ auth.status?.user?.source === 'oauth' ? 'provider' : 'directory' }} you sign in with, so you change it there.
      </p>
      <form v-else class="grid" @submit.prevent="changePassword">
        <input type="text" autocomplete="username" :value="auth.status?.user?.name" hidden readonly />
        <label for="pw-current">Current password</label>
        <input id="pw-current" v-model="pw.current" class="field" type="password" autocomplete="current-password" />
        <label for="pw-new">New password</label>
        <input id="pw-new" v-model="pw.next" class="field" type="password" autocomplete="new-password" minlength="8" aria-describedby="pw-rule" />
        <label for="pw-again">New password again</label>
        <input id="pw-again" v-model="pw.again" class="field" type="password" autocomplete="new-password" :aria-invalid="pwMismatch" />
        <span />
        <div class="row">
          <button type="submit" class="btn primary" :disabled="!pwReady || busy">Change password</button>
          <small id="pw-rule" :class="{ bad: pwMismatch }">{{ pwMismatch ? 'The new passwords don’t match.' : 'At least 8 characters.' }}</small>
        </div>
      </form>
    </section>

    <section v-if="admin">
      <div class="sh"><h3>People</h3></div>
      <p class="what">
        Each person can look (search, preview and download), change the library too (add, edit, tag, collect and
        archive), or be an admin as well (Settings, accounts and maintenance, and deleting for good).
      </p>
      <ul v-if="accounts" class="list">
        <li v-for="a in accounts" :key="a.id" :data-person="a.username">
          <span class="name">{{ a.username }} <small v-if="a.you">(you)</small></span>
          <small>
            <span v-if="a.source !== 'internal'" class="src" :title="`Signs in with the ${a.source === 'oauth' ? 'OAuth provider' : 'directory'}`">{{ sourceWord(a.source) }} · </span>since {{ day(a.created_at) }}
          </small>
          <span v-if="a.you" class="role">{{ roleLabel(a.role) }}</span>
          <select
            v-else
            class="field role"
            :value="a.role"
            :disabled="busy"
            :aria-label="`What ${a.username} can do`"
            @change="setRole(a, ($event.target as HTMLSelectElement).value as Role)"
          >
            <option v-for="r in ROLES" :key="r.id" :value="r.id" :title="r.hint">{{ r.label }}</option>
          </select>
          <button v-if="!a.you" type="button" class="btn ghost" :disabled="busy" @click="removePerson(a)">Remove</button>
        </li>
      </ul>
      <form class="add" @submit.prevent="addPerson">
        <label class="visually-hidden" for="new-name">Name</label>
        <input id="new-name" v-model="person.username" class="field" placeholder="Name" autocomplete="off" maxlength="64" />
        <template v-if="SOURCES.length > 1">
          <label class="visually-hidden" for="new-source">How they sign in</label>
          <select id="new-source" v-model="person.source" class="field role">
            <option v-for="s in SOURCES" :key="s.id" :value="s.id">{{ s.label }}</option>
          </select>
        </template>
        <template v-if="person.source === 'internal'">
          <label class="visually-hidden" for="new-password">Password (at least 8 characters)</label>
          <input id="new-password" v-model="person.password" class="field" type="password" placeholder="Password (8+ characters)" autocomplete="new-password" />
        </template>
        <label class="visually-hidden" for="new-role">What they can do</label>
        <select id="new-role" v-model="person.role" class="field role">
          <option v-for="r in ROLES" :key="r.id" :value="r.id" :title="r.hint">{{ r.label }}</option>
        </select>
        <button type="submit" class="btn" :disabled="!personReady || busy">Add</button>
      </form>
    </section>

    <section>
      <div class="sh"><h3>Claude and other AI apps</h3></div>
      <p class="what">
        Connect Claude (or any MCP app) and it can search and fetch assets for you, and describe and tag what Hoard hasn’t
        yet. It signs in to Hoard the first time; you allow it here in the browser.
      </p>
      <div class="row">
        <input class="field mono grow" :value="mcpUrl" readonly aria-label="Hoard's MCP address" @focus="($event.target as HTMLInputElement).select()" />
        <button type="button" class="btn" @click="copy(mcpUrl)">Copy</button>
      </div>
      <ul class="steps">
        <li>
          <strong>Claude Code:</strong> run <span class="mono">{{ claudeCode }}</span>
          <button type="button" class="btn ghost small" @click="copy(claudeCode)">Copy</button>, then <span class="mono">/mcp</span>
          to sign in. <span class="mono">/mcp__hoard__tag_pack</span> describes a pack.
        </li>
        <li>
          <strong>claude.ai, Claude Desktop and Cowork:</strong> Settings → Connectors → Add custom connector, paste the
          address, then Connect. These connect from Anthropic’s servers, so Hoard has to be reachable from the internet over
          HTTPS<template v-if="local"> (this address isn’t: set the Public address in Settings → Connection)</template>.
        </li>
      </ul>
      <ul v-if="connections?.length" class="list">
        <li v-for="c in connections" :key="c.id">
          <span class="name">{{ c.app }}</span>
          <small>for {{ c.user }} since {{ day(c.created_at) }} · last used {{ day(c.last_used_at) }}</small>
          <button type="button" class="btn ghost" :disabled="busy" @click="disconnect(c)">Disconnect</button>
        </li>
      </ul>
      <p v-else-if="connections" class="what none">No apps connected yet.</p>
    </section>

    <section>
      <div class="sh"><h3>Access tokens</h3></div>
      <p class="what">
        For the <span class="mono">hoard</span> command line tool, scripts, MCP clients and the Unreal worker: they send it as
        <span class="mono">Authorization: Bearer &lt;token&gt;</span>. A token can do what you choose, up to what you can,
        but never manage accounts or tokens. One that only looks is enough to search and download, and to sync a project.
        <template v-if="admin">You see everyone’s tokens and connected apps; others see their own.</template>
      </p>

      <div v-if="made" class="made" role="status">
        <p><strong>{{ made.name }}</strong>: copy it now. It isn’t shown again.</p>
        <div class="row">
          <input class="field mono" :value="made.token" readonly aria-label="The new token" @focus="($event.target as HTMLInputElement).select()" />
          <button type="button" class="btn primary" @click="copyToken">Copy</button>
        </div>
        <p class="use">
          For the CLI: <span class="mono">hoard --server {{ origin }} --token &lt;token&gt;</span>, or put it in
          <span class="mono">HOARD_TOKEN</span>.
        </p>
        <button type="button" class="btn ghost" @click="made = null">Done</button>
      </div>

      <ul v-if="tokens?.length" class="list">
        <li v-for="t in tokens" :key="t.id">
          <span class="name">{{ t.name }}</span>
          <small>{{ roleLabel(t.role) }} · made {{ day(t.created_at) }}{{ t.created_by ? ` by ${t.created_by}` : '' }} · last used {{ day(t.last_used_at) }}</small>
          <button type="button" class="btn ghost" :disabled="busy" @click="revoke(t)">Revoke</button>
        </li>
      </ul>
      <p v-else-if="tokens" class="what none">No tokens yet.</p>
      <form class="add" @submit.prevent="makeToken">
        <label class="visually-hidden" for="token-name">What will use it</label>
        <input id="token-name" v-model="tokenName" class="field" placeholder="What will use it, e.g. “Unreal worker”" maxlength="80" autocomplete="off" />
        <label class="visually-hidden" for="token-role">What it can do</label>
        <select id="token-role" v-model="tokenRole" class="field role">
          <option v-for="r in tokenRoles" :key="r.id" :value="r.id" :title="r.hint">{{ r.label }}</option>
        </select>
        <button type="submit" class="btn" :disabled="!tokenName.trim() || busy">Make a token</button>
      </form>
    </section>

    <section v-if="guest !== null">
      <div class="sh"><h3>Guests</h3></div>
      <label class="check">
        <input type="checkbox" :checked="guest" :disabled="busy" @change="setGuest(($event.target as HTMLInputElement).checked)" />
        Let anyone who can reach the server browse without signing in
      </label>
      <p class="what">They can search, preview and download, but not add, change or delete anything, or see Settings.</p>
    </section>
  </div>
</template>

<style scoped>
.account { display: grid; gap: 1.8rem; font-size: var(--step--1); }
section { display: grid; gap: 0.6rem; }
.sh { display: flex; align-items: center; justify-content: space-between; gap: 1rem; }
.sh h3 { margin: 0; }
.what { margin: 0; color: var(--muted); line-height: 1.5; }
.what strong { color: var(--text); }
.grid { display: grid; grid-template-columns: max-content minmax(0, 18rem); gap: 0.45rem 0.9rem; align-items: center; }
.row { display: flex; gap: 0.6rem; align-items: center; flex-wrap: wrap; }
.row small { color: var(--muted); }
.row small.bad { color: var(--danger); }
.list { margin: 0; padding: 0; list-style: none; border-top: 1px solid var(--line); }
.list li { display: flex; gap: 0.8rem; align-items: center; padding: 0.45rem 0; border-bottom: 1px solid var(--line); }
.list .name { font-weight: 550; overflow-wrap: anywhere; }
.list .name small { font-weight: 400; color: var(--muted); }
.list li > small { flex: 1; color: var(--muted); }
.list .role:not(select) { color: var(--muted); }
select.role { flex: 0 0 auto; width: auto; }
.add { display: flex; gap: 0.5rem; flex-wrap: wrap; }
.add .field { flex: 1 1 12rem; min-width: 0; }
.made { display: grid; gap: 0.5rem; justify-items: start; padding: 0.8rem 0.9rem; border: 1px solid var(--accent); border-radius: var(--r-chip); background: var(--raised); }
.made p { margin: 0; }
.made .row { width: 100%; }
.made .field { flex: 1; min-width: 0; }
.made .use { color: var(--muted); line-height: 1.5; }
.mono { overflow-wrap: anywhere; }
.check { display: flex; gap: 0.5rem; align-items: center; }
.grow { flex: 1; min-width: 0; }
.steps { margin: 0; padding-left: 1.1rem; display: grid; gap: 0.4rem; color: var(--muted); line-height: 1.5; }
.steps strong { color: var(--text); }
.small { padding-block: 0.1rem; font-size: 0.85em; }
.none { font-style: italic; }
@media (max-width: 560px) {
  .grid { grid-template-columns: minmax(0, 1fr); }
  .grid > span:empty { display: none; }
  .list li { flex-wrap: wrap; }
}
</style>
