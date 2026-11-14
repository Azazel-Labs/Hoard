<script setup lang="ts">
// Settings → Account: your password, the other accounts, connecting Claude and other AI apps, access tokens for
// scripts and programs, and whether people who aren't signed in may look around.
import { computed, onMounted, reactive, ref } from 'vue'
import { api, type AccessToken, type Account, type AccountSource, type Connection, type Role } from '../api'
import { auth, may, refreshAuth, signOut } from '../auth'
import { formatDate, t } from '../i18n'
import { confirmDialog, copyText, showError, showOk } from '../ui'

const accounts = ref<Account[] | null>(null)
const tokens = ref<AccessToken[] | null>(null)
const connections = ref<Connection[] | null>(null)
const guest = ref<boolean | null>(null)
const busy = ref(false)
const origin = ref(location.origin)

const admin = computed(() => may('admin'))
const myRole = computed<Role>(() => auth.status?.user?.role ?? 'view')

// A function, so the labels are read in the current language each time they're shown.
const ROLES = (): { id: Role; label: string; hint: string }[] => [
  { id: 'view', label: t('account-role-view'), hint: t('account-role-view-hint') },
  { id: 'edit', label: t('account-role-edit'), hint: t('account-role-edit-hint') },
  { id: 'admin', label: t('account-role-admin'), hint: t('account-role-admin-hint') },
]
const roleLabel = (r: Role) => ROLES().find((x) => x.id === r)!.label
/** Someone whose password the directory or OAuth provider keeps, not Hoard. */
const external = computed(() => !!auth.status?.user?.source && auth.status.user.source !== 'internal')
const SOURCES = computed<{ id: AccountSource; label: string }[]>(() => {
  const s = auth.status?.sign_in
  return [
    { id: 'internal', label: t('account-source-internal') },
    ...(s?.directory ? [{ id: 'ldap' as const, label: t('account-source-ldap', { directory: s.directory }) }] : []),
    ...(s?.oauth ? [{ id: 'oauth' as const, label: t('account-source-oauth', { provider: s.oauth }) }] : []),
  ]
})
const sourceWord = (s: AccountSource) => (s === 'ldap' ? t('account-source-word-directory') : s === 'oauth' ? 'OAuth' : '')
/** What a token may be given: up to what you can do. */
const tokenRoles = computed(() => {
  const all = ROLES()
  return all.slice(0, all.findIndex((r) => r.id === myRole.value) + 1)
})

async function load() {
  try {
    const [tk, c] = await Promise.all([api.tokens(), api.connections()])
    tokens.value = tk.tokens
    connections.value = c.connections
    if (admin.value) {
      const [a, s] = await Promise.all([api.accounts(), api.settings()])
      accounts.value = a.users
      guest.value = s.options.guest_browsing.value
      origin.value = (s.options.public_url.value || location.origin).replace(/\/+$/, '')
    }
  } catch (e) {
    showError(t('account-load-failed', { error: (e as Error).message }))
  }
}
onMounted(load)

async function run(what: string, f: () => Promise<unknown>): Promise<boolean> {
  busy.value = true
  try {
    await f()
    return true
  } catch (e) {
    showError(t('account-failed', { what, error: (e as Error).message }))
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
  if (await run(t('account-password-failed'), () => api.changePassword(pw.current, pw.next))) {
    Object.assign(pw, { current: '', next: '', again: '' })
    showOk(t('account-password-changed'))
  }
}

// ---- other people
const person = reactive({ username: '', password: '', role: 'edit' as Role, source: 'internal' as AccountSource })
const personReady = computed(() => person.username.trim() !== '' && (person.source !== 'internal' || person.password.length >= 8))
async function addPerson() {
  if (!personReady.value) return
  const name = person.username.trim()
  const pw = person.source === 'internal' ? person.password : ''
  if (await run(t('account-add-failed', { name }), () => api.addAccount(name, pw, person.role, person.source))) {
    Object.assign(person, { username: '', password: '', role: 'edit', source: 'internal' })
    showOk(t('account-added', { name }))
    await load()
  }
}
async function setRole(a: Account, role: Role) {
  if (await run(t('account-role-failed', { name: a.username }), () => api.setRole(a.id, role))) await load()
}
async function removePerson(a: Account) {
  const ok = await confirmDialog({
    title: t('account-remove-title', { name: a.username }),
    body: t('account-remove-body'),
    confirm: t('account-remove'),
    danger: true,
  })
  if (ok && (await run(t('account-remove-failed', { name: a.username }), () => api.removeAccount(a.id)))) await load()
}

// ---- access tokens
const tokenName = ref('')
const tokenRole = ref<Role>('view')
const made = ref<{ name: string; token: string } | null>(null)
async function makeToken() {
  const name = tokenName.value.trim()
  if (!name) return
  await run(t('account-token-failed'), async () => {
    const res = await api.makeToken(name, tokenRole.value)
    made.value = { name: res.name, token: res.token }
    tokenName.value = ''
    await load()
  })
}
async function copyToken() {
  if (!made.value) return
  if (await copyText(made.value.token)) showOk(t('account-copied'))
  else showError(t('account-copy-token-failed'))
}
async function revoke(tok: AccessToken) {
  const ok = await confirmDialog({
    title: t('account-revoke-title', { name: tok.name }),
    body: t('account-revoke-body'),
    confirm: t('account-revoke'),
    danger: true,
  })
  if (ok && (await run(t('account-revoke-failed', { name: tok.name }), () => api.revokeToken(tok.id)))) {
    if (made.value?.name === tok.name) made.value = null
    await load()
  }
}

// ---- Claude and other AI apps
const mcpUrl = computed(() => `${origin.value}/mcp`)
const claudeCode = computed(() => `claude mcp add --transport http --scope user hoard ${mcpUrl.value}`)
const local = computed(() => /^http:\/\/|\/\/(localhost|127\.|10\.|192\.168\.|172\.(1[6-9]|2\d|3[01])\.)/.test(origin.value))
async function copy(text: string) {
  if (await copyText(text)) showOk(t('account-copied'))
  else showError(t('account-copy-failed'))
}
async function disconnect(c: Connection) {
  const ok = await confirmDialog({
    title: t('account-disconnect-title', { app: c.app }),
    body: t('account-disconnect-body'),
    confirm: t('account-disconnect'),
    danger: true,
  })
  if (ok && (await run(t('account-disconnect-failed', { app: c.app }), () => api.disconnect(c.id)))) await load()
}

// ---- guests
async function setGuest(on: boolean) {
  if (await run(t('account-guest-failed'), () => api.saveSettings({ guest_browsing: on }))) {
    guest.value = on
    await refreshAuth()
  }
}

const day = (at: string | null) => (at ? formatDate(at, { dateStyle: 'medium' }) : t('account-never'))
</script>

<template>
  <div class="account" :aria-busy="busy">
    <section>
      <div class="sh">
        <h3>{{ t('account-you') }}</h3>
        <button type="button" class="btn" @click="signOut">{{ t('account-sign-out') }}</button>
      </div>
      <p class="what">
        {{ t('account-signed-in-as-before') }}<strong>{{ auth.status?.user?.name }}</strong>{{
          myRole === 'admin' ? t('account-signed-in-as-admin') : myRole === 'edit' ? t('account-signed-in-as-edit') : t('account-signed-in-as-view')
        }}
      </p>
      <p v-if="external" class="what">
        {{ auth.status?.user?.source === 'oauth' ? t('account-password-at-provider') : t('account-password-at-directory') }}
      </p>
      <form v-else class="grid" @submit.prevent="changePassword">
        <input type="text" autocomplete="username" :value="auth.status?.user?.name" hidden readonly />
        <label for="pw-current">{{ t('account-current-password') }}</label>
        <input id="pw-current" v-model="pw.current" class="field" type="password" autocomplete="current-password" />
        <label for="pw-new">{{ t('account-new-password') }}</label>
        <input id="pw-new" v-model="pw.next" class="field" type="password" autocomplete="new-password" minlength="8" aria-describedby="pw-rule" />
        <label for="pw-again">{{ t('account-new-password-again') }}</label>
        <input id="pw-again" v-model="pw.again" class="field" type="password" autocomplete="new-password" :aria-invalid="pwMismatch" />
        <span />
        <div class="row">
          <button type="submit" class="btn primary" :disabled="!pwReady || busy">{{ t('account-change-password') }}</button>
          <small id="pw-rule" :class="{ bad: pwMismatch }">{{ pwMismatch ? t('account-password-mismatch') : t('account-password-rule') }}</small>
        </div>
      </form>
    </section>

    <section v-if="admin">
      <div class="sh"><h3>{{ t('account-people') }}</h3></div>
      <p class="what">
        {{ t('account-people-hint') }}
      </p>
      <ul v-if="accounts" class="list">
        <li v-for="a in accounts" :key="a.id" :data-person="a.username">
          <span class="name">{{ a.username }} <small v-if="a.you">{{ t('account-you-tag') }}</small></span>
          <small>
            <span v-if="a.source !== 'internal'" class="src" :title="a.source === 'oauth' ? t('account-signs-in-oauth') : t('account-signs-in-directory')">{{ sourceWord(a.source) }} · </span>{{ t('account-since', { date: day(a.created_at) }) }}
          </small>
          <span v-if="a.you" class="role">{{ roleLabel(a.role) }}</span>
          <select
            v-else
            class="field role"
            :value="a.role"
            :disabled="busy"
            :aria-label="t('account-what-can-do', { name: a.username })"
            @change="setRole(a, ($event.target as HTMLSelectElement).value as Role)"
          >
            <option v-for="r in ROLES()" :key="r.id" :value="r.id" :title="r.hint">{{ r.label }}</option>
          </select>
          <button v-if="!a.you" type="button" class="btn ghost" :disabled="busy" @click="removePerson(a)">{{ t('account-remove') }}</button>
        </li>
      </ul>
      <form class="add" @submit.prevent="addPerson">
        <label class="visually-hidden" for="new-name">{{ t('account-name') }}</label>
        <input id="new-name" v-model="person.username" class="field" :placeholder="t('account-name')" autocomplete="off" maxlength="64" />
        <template v-if="SOURCES.length > 1">
          <label class="visually-hidden" for="new-source">{{ t('account-how-sign-in') }}</label>
          <select id="new-source" v-model="person.source" class="field role">
            <option v-for="s in SOURCES" :key="s.id" :value="s.id">{{ s.label }}</option>
          </select>
        </template>
        <template v-if="person.source === 'internal'">
          <label class="visually-hidden" for="new-password">{{ t('account-password-label') }}</label>
          <input id="new-password" v-model="person.password" class="field" type="password" :placeholder="t('account-password-placeholder')" autocomplete="new-password" />
        </template>
        <label class="visually-hidden" for="new-role">{{ t('account-what-they-can-do') }}</label>
        <select id="new-role" v-model="person.role" class="field role">
          <option v-for="r in ROLES()" :key="r.id" :value="r.id" :title="r.hint">{{ r.label }}</option>
        </select>
        <button type="submit" class="btn" :disabled="!personReady || busy">{{ t('account-add') }}</button>
      </form>
    </section>

    <section>
      <div class="sh"><h3>{{ t('account-ai-apps') }}</h3></div>
      <p class="what">
        {{ t('account-ai-apps-hint') }}
      </p>
      <div class="row">
        <input class="field mono grow" :value="mcpUrl" readonly :aria-label="t('account-mcp-address')" @focus="($event.target as HTMLInputElement).select()" />
        <button type="button" class="btn" @click="copy(mcpUrl)">{{ t('account-copy') }}</button>
      </div>
      <ul class="steps">
        <li>
          <strong>Claude Code:</strong> {{ t('account-claude-code-1') }}<span class="mono">{{ claudeCode }}</span>
          <button type="button" class="btn ghost small" @click="copy(claudeCode)">{{ t('account-copy') }}</button>{{ t('account-claude-code-2') }}<span class="mono">/mcp</span>{{ t('account-claude-code-3') }}<span class="mono">/mcp__hoard__tag_pack</span>{{ t('account-claude-code-4') }}
        </li>
        <li>
          <strong>claude.ai, Claude Desktop and Cowork:</strong> {{ local ? t('account-claude-ai-local') : t('account-claude-ai') }}
        </li>
      </ul>
      <ul v-if="connections?.length" class="list">
        <li v-for="c in connections" :key="c.id">
          <span class="name">{{ c.app }}</span>
          <small>{{ t('account-connection-meta', { user: c.user, since: day(c.created_at), used: day(c.last_used_at) }) }}</small>
          <button type="button" class="btn ghost" :disabled="busy" @click="disconnect(c)">{{ t('account-disconnect') }}</button>
        </li>
      </ul>
      <p v-else-if="connections" class="what none">{{ t('account-no-apps') }}</p>
    </section>

    <section>
      <div class="sh"><h3>{{ t('account-tokens') }}</h3></div>
      <p class="what">
        {{ t('account-tokens-hint-1') }}<span class="mono">hoard</span>{{ t('account-tokens-hint-2') }}<span class="mono">Authorization: Bearer &lt;token&gt;</span>{{ t('account-tokens-hint-3') }}
        <template v-if="admin">{{ t('account-tokens-admin') }}</template>
      </p>

      <div v-if="made" class="made" role="status">
        <p>{{ t('account-made-before') }}<strong>{{ made.name }}</strong>{{ t('account-made-after') }}</p>
        <div class="row">
          <input class="field mono" :value="made.token" readonly :aria-label="t('account-new-token')" @focus="($event.target as HTMLInputElement).select()" />
          <button type="button" class="btn primary" @click="copyToken">{{ t('account-copy') }}</button>
        </div>
        <p class="use">
          {{ t('account-cli-1') }}<span class="mono">hoard --server {{ origin }} --token &lt;token&gt;</span>{{ t('account-cli-2') }}<span class="mono">HOARD_TOKEN</span>{{ t('account-cli-3') }}
        </p>
        <button type="button" class="btn ghost" @click="made = null">{{ t('account-done') }}</button>
      </div>

      <ul v-if="tokens?.length" class="list">
        <li v-for="tok in tokens" :key="tok.id">
          <span class="name">{{ tok.name }}</span>
          <small>{{ roleLabel(tok.role) }} · {{
            tok.created_by
              ? t('account-token-meta-by', { made: day(tok.created_at), by: tok.created_by, used: day(tok.last_used_at) })
              : t('account-token-meta', { made: day(tok.created_at), used: day(tok.last_used_at) })
          }}</small>
          <button type="button" class="btn ghost" :disabled="busy" @click="revoke(tok)">{{ t('account-revoke') }}</button>
        </li>
      </ul>
      <p v-else-if="tokens" class="what none">{{ t('account-no-tokens') }}</p>
      <form class="add" @submit.prevent="makeToken">
        <label class="visually-hidden" for="token-name">{{ t('account-token-name') }}</label>
        <input id="token-name" v-model="tokenName" class="field" :placeholder="t('account-token-name-placeholder')" maxlength="80" autocomplete="off" />
        <label class="visually-hidden" for="token-role">{{ t('account-token-role') }}</label>
        <select id="token-role" v-model="tokenRole" class="field role">
          <option v-for="r in tokenRoles" :key="r.id" :value="r.id" :title="r.hint">{{ r.label }}</option>
        </select>
        <button type="submit" class="btn" :disabled="!tokenName.trim() || busy">{{ t('account-make-token') }}</button>
      </form>
    </section>

    <section v-if="guest !== null">
      <div class="sh"><h3>{{ t('account-guests') }}</h3></div>
      <label class="check">
        <input type="checkbox" :checked="guest" :disabled="busy" @change="setGuest(($event.target as HTMLInputElement).checked)" />
        {{ t('account-guests-allow') }}
      </label>
      <p class="what">{{ t('account-guests-hint') }}</p>
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
