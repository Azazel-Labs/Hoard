<script setup lang="ts">
// Settings → Authentication: how people sign in besides Hoard's own accounts (which always work, so an admin can't be
// locked out): LDAP (a custom server or OpenLDAP), Microsoft Active Directory, or OAuth. Also how long sessions last,
// whether accounts are made at first sign-in, and which groups get which role.
import { computed, onMounted, ref } from 'vue'
import { api, type AuthConfig, type AuthMethod, type LdapConfig, type LdapTestResult, type LdapTls, type Role } from '../api'
import { t } from '../i18n'
import { copyText, showError, showOk } from '../ui'

const cfg = ref<AuthConfig | null>(null)
const saved = ref('')
const callback = ref('')
const busy = ref(false)
const error = ref('')

/** The Roles lists as typed: one group per line. */
const roleText = ref<Record<Role, string>>({ admin: '', edit: '', view: '' })

function adopt(s: { config: AuthConfig; callback_url: string }) {
  cfg.value = structuredClone(s.config)
  callback.value = s.callback_url
  roleText.value = { admin: s.config.roles.admin.join('\n'), edit: s.config.roles.edit.join('\n'), view: s.config.roles.view.join('\n') }
  for (const l of [cfg.value.ldap, cfg.value.ad]) if (!l.servers.length) l.servers = ['']
  saved.value = JSON.stringify(gathered())
}

async function load() {
  try {
    adopt(await api.authSettings())
  } catch (e) {
    showError(t('auth-load-failed', { error: (e as Error).message }))
  }
}
onMounted(load)

const lines = (s: string) => s.split(/[\n,]/).map((x) => x.trim()).filter(Boolean)
/** What would be saved. */
function gathered(): AuthConfig | null {
  const c = cfg.value
  if (!c) return null
  const tidy = (l: LdapConfig): LdapConfig => ({ ...l, servers: l.servers.map((s) => s.trim()).filter(Boolean) })
  return {
    ...c,
    roles: { admin: lines(roleText.value.admin), edit: lines(roleText.value.edit), view: lines(roleText.value.view) },
    ldap: tidy(c.ldap),
    ad: tidy(c.ad),
  }
}
const changed = computed(() => !!cfg.value && JSON.stringify(gathered()) !== saved.value)

async function save() {
  const next = gathered()
  if (!next || busy.value) return
  busy.value = true
  error.value = ''
  try {
    adopt(await api.saveAuthSettings(next))
    showOk(t('auth-saved'))
  } catch (e) {
    error.value = (e as Error).message
  } finally {
    busy.value = false
  }
}

// Lists with text are functions, so they're read in the current language each time they're shown.
const METHODS = (): { id: AuthMethod; label: string; hint: string }[] => [
  { id: 'internal', label: t('auth-method-internal'), hint: t('auth-method-internal-hint') },
  { id: 'ldap', label: 'LDAP', hint: t('auth-method-ldap-hint') },
  { id: 'ad', label: 'Microsoft Active Directory', hint: t('auth-method-ad-hint') },
  { id: 'oauth', label: 'OAuth', hint: t('auth-method-oauth-hint') },
]

const SESSIONS = (): { hours: number | null; label: string }[] => [
  { hours: null, label: t('auth-session-unused') },
  { hours: 1, label: t('auth-session-1h') },
  { hours: 8, label: t('auth-session-8h') },
  { hours: 24, label: t('auth-session-day') },
  { hours: 168, label: t('auth-session-week') },
  { hours: 720, label: t('auth-session-30d') },
]
const sessionChoices = computed(() => {
  const h = cfg.value?.session_hours ?? null
  const all = SESSIONS()
  return all.some((s) => s.hours === h) ? all : [...all, { hours: h, label: t('auth-session-hours', { hours: h ?? 0 }) }]
})

const ROLES = (): { id: Role; label: string }[] => [
  { id: 'view', label: t('auth-role-view') },
  { id: 'edit', label: t('auth-role-edit') },
  { id: 'admin', label: t('auth-role-admin') },
]

// ---- LDAP and Active Directory
const dirKind = computed<'ldap' | 'ad' | null>(() => (cfg.value?.method === 'ldap' ? 'ldap' : cfg.value?.method === 'ad' ? 'ad' : null))
const dir = computed<LdapConfig | null>(() => (cfg.value && dirKind.value ? cfg.value[dirKind.value] : null))
const TLS = (): { id: LdapTls; label: string; hint: string }[] => [
  { id: 'starttls', label: 'StartTLS', hint: t('auth-tls-starttls-hint') },
  { id: 'ldaps', label: 'LDAPS', hint: t('auth-tls-ldaps-hint') },
  { id: 'none', label: t('auth-tls-none'), hint: t('auth-tls-none-hint') },
]
function serverType(type: string) {
  const l = cfg.value?.ldap
  if (!l) return
  l.server_type = type
  if (type === 'openldap') {
    l.username_attribute = 'uid'
    if (!l.user_filter.trim()) l.user_filter = '(objectClass=inetOrgPerson)'
  }
}

const testUser = ref('')
const testPassword = ref('')
const tested = ref<LdapTestResult | null>(null)
const testing = ref(false)
async function test() {
  const d = dir.value
  if (!d || !dirKind.value) return
  testing.value = true
  tested.value = null
  try {
    const l = { ...d, servers: d.servers.map((s) => s.trim()).filter(Boolean) }
    tested.value = await api.testLdap(dirKind.value, l, testUser.value.trim() || undefined, testUser.value.trim() ? testPassword.value : undefined)
  } catch (e) {
    showError(t('auth-test-failed', { error: (e as Error).message }))
  } finally {
    testing.value = false
  }
}
/** A role as a word in a sentence ("They’d be: can change."). */
function roleName(r: Role | undefined): string {
  return r === 'admin' ? t('auth-role-word-admin') : r === 'edit' ? t('auth-role-word-edit') : t('auth-role-word-view')
}
const roleWord = (r: Role | 'default' | null | undefined) =>
  r === 'default' ? t('auth-role-word-default', { role: roleName(cfg.value?.default_role) }) : r ? roleName(r) : null

// ---- OAuth
interface Preset {
  id: string
  label: string
  authorization_url: string
  token_url: string
  userinfo_url: string
  scopes: string
  username_claim: string
  groups_claim: string
  button: string
  note: string
}
const PRESETS: Preset[] = [
  {
    id: 'custom',
    get label() { return t('auth-preset-custom') },
    authorization_url: '',
    token_url: '',
    userinfo_url: '',
    scopes: 'openid profile email',
    username_claim: 'preferred_username',
    groups_claim: 'groups',
    button: '',
    get note() { return t('auth-preset-custom-note') },
  },
  {
    id: 'google',
    label: 'Google',
    authorization_url: 'https://accounts.google.com/o/oauth2/v2/auth',
    token_url: 'https://oauth2.googleapis.com/token',
    userinfo_url: 'https://openidconnect.googleapis.com/v1/userinfo',
    scopes: 'openid profile email',
    username_claim: 'email',
    groups_claim: '',
    button: 'Google',
    get note() { return t('auth-preset-google-note') },
  },
  {
    id: 'microsoft',
    label: 'Microsoft Entra ID',
    authorization_url: 'https://login.microsoftonline.com/organizations/oauth2/v2.0/authorize',
    token_url: 'https://login.microsoftonline.com/organizations/oauth2/v2.0/token',
    userinfo_url: '',
    scopes: 'openid profile email',
    username_claim: 'preferred_username',
    groups_claim: 'groups',
    button: 'Microsoft',
    get note() { return t('auth-preset-microsoft-note') },
  },
  {
    id: 'github',
    label: 'GitHub',
    authorization_url: 'https://github.com/login/oauth/authorize',
    token_url: 'https://github.com/login/oauth/access_token',
    userinfo_url: 'https://api.github.com/user',
    scopes: 'read:user',
    username_claim: 'login',
    groups_claim: '',
    button: 'GitHub',
    get note() { return t('auth-preset-github-note') },
  },
  {
    id: 'gitlab',
    label: 'GitLab',
    authorization_url: 'https://gitlab.com/oauth/authorize',
    token_url: 'https://gitlab.com/oauth/token',
    userinfo_url: 'https://gitlab.com/oauth/userinfo',
    scopes: 'openid profile email',
    username_claim: 'nickname',
    groups_claim: 'groups',
    button: 'GitLab',
    get note() { return t('auth-preset-gitlab-note') },
  },
]
const preset = computed(() => PRESETS.find((p) => p.id === cfg.value?.oauth.provider) ?? PRESETS[0])
function pick(id: string) {
  const o = cfg.value?.oauth
  const p = PRESETS.find((x) => x.id === id)
  if (!o || !p) return
  const { id: _id, label: _l, note: _n, ...fields } = p
  Object.assign(o, fields, { provider: id })
  // Anyone at all has an account with these, so new accounts are an admin's choice to turn on.
  if (id !== 'custom' && cfg.value) cfg.value.provision = false
}
/** Signing in makes accounts, with a provider anyone can have an account with, and no group needed. */
const openToAnyone = computed(
  () => cfg.value?.method === 'oauth' && cfg.value.provision && preset.value.id !== 'custom' && cfg.value.roles.view.length === 0,
)
const issuer = ref('')
async function discover() {
  const o = cfg.value?.oauth
  if (!o || !issuer.value.trim()) return
  busy.value = true
  try {
    Object.assign(o, await api.discoverOAuth(issuer.value.trim()))
    showOk(t('auth-discovered'))
  } catch (e) {
    showError((e as Error).message)
  } finally {
    busy.value = false
  }
}
async function copyCallback() {
  if (await copyText(callback.value)) showOk(t('auth-copied'))
  else showError(t('auth-copy-failed'))
}
</script>

<template>
  <div v-if="cfg" class="authn" :aria-busy="busy">
    <section>
      <div class="sh"><h3>{{ t('auth-sign-in-with') }}</h3></div>
      <div class="choices" role="radiogroup" :aria-label="t('auth-sign-in-with')">
        <label v-for="m in METHODS()" :key="m.id" :class="{ on: cfg.method === m.id }" :data-method="m.id">
          <input v-model="cfg.method" type="radio" name="auth-method" :value="m.id" :disabled="busy" />
          <span><strong>{{ m.label }}</strong><small>{{ m.hint }}</small></span>
        </label>
      </div>
      <p v-if="cfg.method !== 'internal'" class="what">
        {{ t('auth-own-accounts-still-work') }}
      </p>
    </section>

    <section>
      <div class="sh"><h3>{{ t('auth-sessions') }}</h3></div>
      <div class="grid">
        <label for="auth-session">{{ t('auth-sign-out-after') }}</label>
        <select id="auth-session" v-model="cfg.session_hours" class="field">
          <option v-for="s in sessionChoices" :key="String(s.hours)" :value="s.hours">{{ s.label }}</option>
        </select>
      </div>
    </section>

    <template v-if="dir">
      <section>
        <div class="sh"><h3>{{ dirKind === 'ad' ? 'Active Directory' : t('auth-ldap-server') }}</h3></div>
        <div class="grid">
          <template v-if="dirKind === 'ldap'">
            <label for="ldap-type">{{ t('auth-server-type') }}</label>
            <select id="ldap-type" class="field" :value="cfg.ldap.server_type" @change="serverType(($event.target as HTMLSelectElement).value)">
              <option value="custom">{{ t('auth-server-type-custom') }}</option>
              <option value="openldap">OpenLDAP</option>
            </select>
          </template>
          <span class="lbl">{{ t('auth-servers') }}</span>
          <div class="servers">
            <div v-for="(_, i) in dir.servers" :key="i" class="row">
              <input
                v-model="dir.servers[i]"
                class="field mono grow"
                :placeholder="dirKind === 'ad' ? 'dc1.example.com' : 'ldap.example.com:389'"
                :aria-label="t('auth-server-n', { n: i + 1 })"
                autocomplete="off"
                spellcheck="false"
              />
              <button v-if="dir.servers.length > 1" type="button" class="btn ghost" :aria-label="t('auth-remove-server-n', { n: i + 1 })" @click="dir.servers.splice(i, 1)">
                {{ t('auth-remove') }}
              </button>
            </div>
            <div class="row">
              <button type="button" class="btn ghost small" @click="dir.servers.push('')">{{ t('auth-add-server') }}</button>
              <small>{{ t('auth-servers-hint') }}</small>
            </div>
          </div>

          <span class="lbl">{{ t('auth-security') }}</span>
          <div class="choices tight" role="radiogroup" :aria-label="t('auth-connection-security')">
            <label v-for="x in TLS()" :key="x.id" :class="{ on: dir.tls === x.id }">
              <input v-model="dir.tls" type="radio" :name="`${dirKind}-tls`" :value="x.id" />
              <span><strong>{{ x.label }}</strong><small>{{ x.hint }}</small></span>
            </label>
          </div>
          <template v-if="dir.tls !== 'none'">
            <span />
            <label class="check"><input v-model="dir.skip_verify" type="checkbox" /> {{ t('auth-skip-verify') }}</label>
            <label for="ldap-ca">{{ t('auth-ca') }}</label>
            <textarea
              id="ldap-ca"
              v-model="dir.ca_cert"
              class="field mono pem"
              rows="3"
              spellcheck="false"
              :placeholder="t('auth-ca-placeholder')"
            />
          </template>
        </div>
      </section>

      <section>
        <div class="sh"><h3>{{ t('auth-lookup') }}</h3></div>
        <div class="grid">
          <span />
          <label class="check"><input v-model="dir.anonymous" type="checkbox" /> {{ t('auth-anonymously') }}</label>
          <template v-if="!dir.anonymous">
            <label for="ldap-bind">{{ t('auth-reader-account') }}</label>
            <input
              id="ldap-bind"
              v-model="dir.bind_dn"
              class="field mono"
              :placeholder="dirKind === 'ad' ? 'hoard@example.com' : 'cn=hoard,ou=services,dc=example,dc=com'"
              autocomplete="off"
              spellcheck="false"
            />
            <label for="ldap-pw">{{ t('auth-reader-password') }}</label>
            <div class="row">
              <input
                id="ldap-pw"
                v-model="dir.bind_password"
                class="field grow"
                type="password"
                autocomplete="new-password"
                :placeholder="dir.bind_password_set && !dir.clear_bind_password ? t('auth-saved-type-to-replace') : ''"
              />
            </div>
          </template>
          <label for="ldap-base">{{ t('auth-people-under') }}</label>
          <input id="ldap-base" v-model="dir.base_dn" class="field mono" placeholder="ou=people,dc=example,dc=com" autocomplete="off" spellcheck="false" />
          <label for="ldap-attr">{{ t('auth-name-attribute') }}</label>
          <input id="ldap-attr" v-model="dir.username_attribute" class="field mono" autocomplete="off" spellcheck="false" />
          <label for="ldap-filter">{{ t('auth-only-matching') }}</label>
          <input id="ldap-filter" v-model="dir.user_filter" class="field mono" placeholder="(objectClass=person)" autocomplete="off" spellcheck="false" />
          <label for="ldap-gbase">{{ t('auth-groups-under') }}</label>
          <input
            id="ldap-gbase"
            v-model="dir.group_base_dn"
            class="field mono"
            :placeholder="t('auth-groups-under-placeholder')"
            autocomplete="off"
            spellcheck="false"
          />
          <template v-if="dir.group_base_dn.trim()">
            <label for="ldap-gfilter">{{ t('auth-their-groups') }}</label>
            <input id="ldap-gfilter" v-model="dir.group_filter" class="field mono" autocomplete="off" spellcheck="false" aria-describedby="ldap-gfilter-hint" />
            <span />
            <small id="ldap-gfilter-hint" class="what">{{ t('auth-group-filter-hint') }}</small>
          </template>
        </div>
      </section>

      <section>
        <div class="sh"><h3>{{ t('auth-test') }}</h3></div>
        <p class="what">{{ t('auth-test-hint') }}</p>
        <form class="add" @submit.prevent="test">
          <label class="visually-hidden" for="ldap-test-user">{{ t('auth-test-user') }}</label>
          <input id="ldap-test-user" v-model="testUser" class="field" :placeholder="t('auth-test-user-placeholder')" autocomplete="off" />
          <label class="visually-hidden" for="ldap-test-pw">{{ t('auth-test-password') }}</label>
          <input id="ldap-test-pw" v-model="testPassword" class="field" type="password" :placeholder="t('auth-test-password')" autocomplete="new-password" />
          <button type="submit" class="btn" :disabled="testing">{{ testing ? t('auth-testing') : t('auth-test-connectivity') }}</button>
        </form>
        <ul v-if="tested" class="results" role="status">
          <li v-for="s in tested.servers" :key="s.server" :class="s.ok ? 'good' : 'bad'">
            <span class="mono">{{ s.server }}</span>: {{ s.ok ? t('auth-test-server-ok') : s.error }}
          </li>
          <li v-if="tested.user" :class="tested.user.ok && tested.user.role !== null ? 'good' : 'bad'">
            <template v-if="tested.user.ok">
              {{ t('auth-test-signed-in-before') }}<strong>{{ tested.user.name }}</strong>{{ t('auth-test-signed-in-middle') }}<span class="mono">{{ tested.user.dn }}</span>{{ t('auth-test-signed-in-after') }}
              <template v-if="tested.user.role">{{ t('auth-test-role', { role: roleWord(tested.user.role) ?? '' }) }}</template>
              <template v-else>{{ t('auth-test-no-role') }}</template>
              <span v-if="tested.user.groups?.length" class="groups">{{ t('auth-test-groups', { groups: tested.user.groups.join(' · ') }) }}</span>
              <span v-else class="groups">{{ t('auth-test-no-groups') }}</span>
            </template>
            <template v-else>{{ tested.user.error }}</template>
          </li>
        </ul>
      </section>
    </template>

    <template v-if="cfg.method === 'oauth'">
      <section>
        <div class="sh"><h3>{{ t('auth-provider') }}</h3></div>
        <div class="grid">
          <label for="oauth-provider">{{ t('auth-provider') }}</label>
          <select id="oauth-provider" class="field" :value="preset.id" @change="pick(($event.target as HTMLSelectElement).value)">
            <option v-for="p in PRESETS" :key="p.id" :value="p.id">{{ p.label }}</option>
          </select>
          <span />
          <small class="what">{{ preset.note }}</small>
          <label for="oauth-callback">{{ t('auth-redirect') }}</label>
          <div class="row">
            <input id="oauth-callback" class="field mono grow" :value="callback" readonly @focus="($event.target as HTMLInputElement).select()" />
            <button type="button" class="btn" @click="copyCallback">{{ t('auth-copy') }}</button>
          </div>
          <span />
          <small class="what">{{ t('auth-redirect-hint') }}</small>
          <template v-if="preset.id === 'custom'">
            <label for="oauth-issuer">{{ t('auth-issuer') }}</label>
            <form class="row" @submit.prevent="discover">
              <input id="oauth-issuer" v-model="issuer" class="field mono grow" placeholder="https://auth.example.com/realms/main" autocomplete="off" spellcheck="false" />
              <button type="submit" class="btn" :disabled="!issuer.trim() || busy">{{ t('auth-discover') }}</button>
            </form>
          </template>
          <label for="oauth-client">{{ t('auth-client-id') }}</label>
          <input id="oauth-client" v-model="cfg.oauth.client_id" class="field mono" autocomplete="off" spellcheck="false" />
          <label for="oauth-secret">{{ t('auth-client-secret') }}</label>
          <input
            id="oauth-secret"
            v-model="cfg.oauth.client_secret"
            class="field"
            type="password"
            autocomplete="new-password"
            :placeholder="cfg.oauth.client_secret_set ? t('auth-saved-type-to-replace') : ''"
          />
          <label for="oauth-auth">{{ t('auth-authorization-url') }}</label>
          <input id="oauth-auth" v-model="cfg.oauth.authorization_url" class="field mono" autocomplete="off" spellcheck="false" />
          <label for="oauth-token">{{ t('auth-token-url') }}</label>
          <input id="oauth-token" v-model="cfg.oauth.token_url" class="field mono" autocomplete="off" spellcheck="false" />
          <label for="oauth-info">{{ t('auth-userinfo-url') }}</label>
          <input id="oauth-info" v-model="cfg.oauth.userinfo_url" class="field mono" :placeholder="t('auth-userinfo-placeholder')" autocomplete="off" spellcheck="false" />
          <label for="oauth-scopes">{{ t('auth-scopes') }}</label>
          <input id="oauth-scopes" v-model="cfg.oauth.scopes" class="field mono" autocomplete="off" spellcheck="false" />
          <label for="oauth-name">{{ t('auth-name-from') }}</label>
          <input id="oauth-name" v-model="cfg.oauth.username_claim" class="field mono" autocomplete="off" spellcheck="false" />
          <label for="oauth-groups">{{ t('auth-groups-from') }}</label>
          <input id="oauth-groups" v-model="cfg.oauth.groups_claim" class="field mono" :placeholder="t('auth-groups-from-placeholder')" autocomplete="off" spellcheck="false" />
          <label for="oauth-button">{{ t('auth-button') }}</label>
          <div class="row">
            <span class="what">{{ t('auth-sign-in-with') }}</span>
            <input id="oauth-button" v-model="cfg.oauth.button" class="field grow" :placeholder="t('auth-button-placeholder')" maxlength="40" />
          </div>
        </div>
      </section>
    </template>

    <template v-if="cfg.method !== 'internal'">
      <section>
        <div class="sh"><h3>{{ t('auth-accounts') }}</h3></div>
        <label class="check">
          <input v-model="cfg.provision" type="checkbox" />
          {{ t('auth-provision') }}
        </label>
        <p class="what">
          {{ cfg.provision ? t('auth-provision-on-hint') : t('auth-provision-off-hint') }}
        </p>
        <p v-if="openToAnyone" class="what bad" role="note">
          {{ t('auth-open-to-anyone', { provider: preset.label }) }}
        </p>
        <div class="grid">
          <label for="auth-role">{{ t('auth-new-accounts') }}</label>
          <select id="auth-role" v-model="cfg.default_role" class="field">
            <option v-for="r in ROLES()" :key="r.id" :value="r.id">{{ r.label }}</option>
          </select>
        </div>
      </section>

      <section>
        <div class="sh"><h3>{{ t('auth-roles-from-groups') }}</h3></div>
        <p class="what">
          {{ cfg.method === 'oauth' ? t('auth-roles-hint-oauth') : t('auth-roles-hint') }}
        </p>
        <div class="roles">
          <div v-for="r in ROLES().reverse()" :key="r.id">
            <label :for="`auth-groups-${r.id}`">{{ r.label }}</label>
            <textarea :id="`auth-groups-${r.id}`" v-model="roleText[r.id]" class="field mono" rows="3" spellcheck="false" />
          </div>
        </div>
      </section>
    </template>

    <div class="apply">
      <button type="button" class="btn primary" :disabled="!changed || busy" @click="save">{{ busy ? t('auth-saving') : t('auth-save') }}</button>
      <p v-if="error" class="bad" role="alert">{{ error }}</p>
    </div>
  </div>
</template>

<style scoped>
.authn { display: grid; font-size: var(--step--1); }
section { display: grid; gap: 0.6rem; padding: 1rem 0 1.1rem; border-top: 1px solid var(--line); }
section:first-child { border-top: 0; padding-top: 0; }
.sh { display: flex; align-items: center; justify-content: space-between; gap: 1rem; min-height: 1.75rem; }
h3 { margin: 0; font-size: 0.7rem; font-weight: 650; letter-spacing: 0.04em; text-transform: uppercase; color: var(--faint); }
p { margin: 0; }
.what { color: var(--muted); line-height: 1.5; }
.what.bad { color: var(--danger); }
.bad { color: var(--danger); line-height: 1.5; }
.good { color: var(--ok); line-height: 1.5; }
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
.choices.tight label { padding: 0.4rem 0.6rem; }

.grid { display: grid; grid-template-columns: max-content minmax(0, 1fr); gap: 0.5rem 0.9rem; align-items: center; }
.grid > label:not(.check), .lbl { align-self: start; padding-top: 0.4rem; }
.grid > .choices, .grid > .servers { align-self: start; }
.servers { display: grid; gap: 0.4rem; }
.row { display: flex; gap: 0.5rem; align-items: center; flex-wrap: wrap; }
.row small { color: var(--muted); }
.grow { flex: 1 1 10rem; min-width: 0; }
.check { display: flex; gap: 0.5rem; align-items: center; }
.pem { resize: vertical; font-size: 0.85em; }
.small { padding-block: 0.1rem; font-size: 0.85em; }

.add { display: flex; gap: 0.5rem; flex-wrap: wrap; }
.add .field { flex: 1 1 12rem; min-width: 0; }
.results { margin: 0; padding: 0.6rem 0.8rem; list-style: none; display: grid; gap: 0.35rem; border: 1px solid var(--line); border-radius: var(--r-chip); background: var(--raised); }
.results li.good { color: var(--text); }
.results li.good::before { content: '✓ '; color: var(--ok); }
.results li.bad::before { content: '✕ '; }
.groups { display: block; color: var(--muted); overflow-wrap: anywhere; }

.roles { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 0.6rem; }
.roles > div { display: grid; gap: 0.25rem; }
.roles label { font-weight: 550; }
.roles textarea { resize: vertical; font-size: 0.9em; }

.apply { display: flex; padding: 0.4rem 0 1.1rem; gap: 0.8rem; align-items: center; flex-wrap: wrap; border-top: 1px solid var(--line); padding-top: 1rem; }
@media (max-width: 560px) {
  .grid { grid-template-columns: minmax(0, 1fr); }
  .grid > span:empty { display: none; }
  .grid > label:not(.check), .lbl { padding-top: 0; }
  .roles { grid-template-columns: minmax(0, 1fr); }
}
</style>
