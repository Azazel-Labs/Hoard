// Settings → Authentication: an admin sets up signing in with an OAuth provider (a fake one here, run by the test),
// someone signs in with it and gets an account and a role from their groups, and a provider's refusal is shown on the
// sign-in page. Also LDAP's connectivity test, against a server that isn't there.
import { createServer, type Server } from 'node:http'
import type { AddressInfo } from 'node:net'
import { request } from '@playwright/test'
import { signedIn } from './support/api'
import { expect, test, unique } from './support/test'

interface Provider {
  url: string
  /** Who the provider says signs in next, and whether it refuses instead. */
  next: { name: string; groups: string[]; refuse?: boolean }
  /** What Hoard sent to the token address. */
  tokenRequests: URLSearchParams[]
  server: Server
}

/** A small OAuth provider: authorizes straight away, swaps any code for a token, and says who it is at /userinfo. */
async function fakeProvider(): Promise<Provider> {
  const p: Provider = { url: '', next: { name: '', groups: [] }, tokenRequests: [], server: createServer() }
  p.server.on('request', (req, res) => {
    const u = new URL(req.url ?? '/', p.url)
    if (u.pathname === '/authorize') {
      const back = new URL(u.searchParams.get('redirect_uri')!)
      back.searchParams.set('state', u.searchParams.get('state') ?? '')
      if (p.next.refuse) back.searchParams.set('error', 'access_denied')
      else back.searchParams.set('code', `code-${p.next.name}`)
      res.writeHead(302, { Location: back.toString() }).end()
    } else if (u.pathname === '/token' && req.method === 'POST') {
      let body = ''
      req.on('data', (c: Buffer) => (body += c.toString()))
      req.on('end', () => {
        const form = new URLSearchParams(body)
        p.tokenRequests.push(form)
        const ok = form.get('client_secret') === 'shh' && form.get('code') === `code-${p.next.name}` && !!form.get('code_verifier')
        res.writeHead(ok ? 200 : 400, { 'Content-Type': 'application/json' })
        res.end(JSON.stringify(ok ? { access_token: `at-${p.next.name}`, token_type: 'Bearer' } : { error: 'invalid_grant' }))
      })
    } else if (u.pathname === '/userinfo') {
      const ok = req.headers.authorization === `Bearer at-${p.next.name}`
      res.writeHead(ok ? 200 : 401, { 'Content-Type': 'application/json' })
      res.end(JSON.stringify(ok ? { sub: p.next.name, preferred_username: p.next.name, groups: p.next.groups } : {}))
    } else res.writeHead(404).end()
  })
  await new Promise<void>((r) => p.server.listen(0, '127.0.0.1', r))
  p.url = `http://127.0.0.1:${(p.server.address() as AddressInfo).port}`
  return p
}

// The settings are the server's, so these run one at a time and put them back after.
test.describe.configure({ mode: 'serial' })

test.describe('authentication', () => {
  let provider: Provider

  test.beforeAll(async () => {
    provider = await fakeProvider()
  })
  test.afterAll(async () => {
    provider.server.close()
    const api = await signedIn(process.env.HOARD_E2E_URL!)
    const { config } = (await (await api.get('/api/admin/auth')).json()) as { config: Record<string, unknown> }
    await api.put('/api/admin/auth', { data: { ...config, method: 'internal' } })
    await api.dispose()
  })

  test('an admin sets up OAuth, and someone signs in with it', async ({ page, api, baseURL, browser }) => {
    // Three browsers and two round trips through the provider.
    test.slow()
    const who = `oauth-${unique()}`
    await page.goto('/#settings/auth')
    await page.locator('[data-method="oauth"]').click()
    await page.getByLabel('Provider', { exact: true }).selectOption('custom')
    await expect(page.getByLabel('Redirect address')).toHaveValue(/\/api\/auth\/oauth\/callback$/)
    await page.getByLabel('Client ID').fill('hoard')
    await page.getByLabel('Client secret').fill('shh')
    await page.getByLabel('Authorization address').fill(`${provider.url}/authorize`)
    await page.getByLabel('Token address').fill(`${provider.url}/token`)
    await page.getByLabel('User info address').fill(`${provider.url}/userinfo`)
    await page.getByLabel('Button', { exact: true }).fill('Fake ID')
    await page.getByLabel('Can change', { exact: true }).fill('artists')
    await page.getByRole('button', { name: 'Save' }).click()
    await expect(page.getByRole('button', { name: 'Save' })).toBeDisabled()

    // The secret is kept but never sent back.
    const saved = (await (await api.get('/api/admin/auth')).json()) as { config: { method: string; oauth: { client_secret: string; client_secret_set: boolean } } }
    expect(saved.config.method).toBe('oauth')
    expect(saved.config.oauth).toMatchObject({ client_secret: '', client_secret_set: true })

    // Someone new, in the artists group: the account is made at sign-in and can change things.
    provider.next = { name: who, groups: ['artists', 'everyone'] }
    const ctx = await browser.newContext({ baseURL, storageState: { cookies: [], origins: [] } })
    const them = await ctx.newPage()
    await them.goto('/')
    const form = them.getByRole('dialog', { name: 'Sign in' })
    await form.getByRole('link', { name: 'Sign in with Fake ID' }).click()
    await expect(form).toBeHidden()
    await expect(them.getByLabel('Search assets')).toBeVisible()
    expect(provider.tokenRequests.at(-1)?.get('redirect_uri')).toMatch(/\/api\/auth\/oauth\/callback$/)
    const status = await (await them.request.get('/api/auth/status')).json()
    expect(status.user).toMatchObject({ name: who, role: 'edit', source: 'oauth' })

    // Their password is the provider's, so there's none to change here.
    await them.goto('/#settings/account')
    await expect(them.getByText('Your password is kept by the provider you sign in with')).toBeVisible()
    await expect(them.getByLabel('Current password')).toHaveCount(0)

    // They can't sign in with a password typed into Hoard.
    const bare = await request.newContext({ baseURL, extraHTTPHeaders: { 'X-Hoard': '1' } })
    expect((await bare.post('/api/auth/login', { data: { username: who, password: 'anything-at-all' } })).ok()).toBe(false)
    await bare.dispose()
    await ctx.close()

    // The admin sees how they sign in.
    await page.goto('/#settings/account')
    await expect(page.locator(`[data-person="${who}"]`)).toContainText('OAuth')

    // Out of the group, they're back to looking at the next sign-in.
    provider.next = { name: who, groups: ['everyone'] }
    const again = await browser.newContext({ baseURL, storageState: { cookies: [], origins: [] } })
    const p2 = await again.newPage()
    await p2.goto('/')
    await p2.getByRole('link', { name: 'Sign in with Fake ID' }).click()
    await expect(p2.getByLabel('Search assets')).toBeVisible()
    expect((await (await p2.request.get('/api/auth/status')).json()).user).toMatchObject({ name: who, role: 'view' })
    await again.close()
  })

  test('a provider that refuses says why on the sign-in page', async ({ browser, baseURL }) => {
    provider.next = { name: 'nobody', groups: [], refuse: true }
    const ctx = await browser.newContext({ baseURL, storageState: { cookies: [], origins: [] } })
    const page = await ctx.newPage()
    await page.goto('/')
    await page.getByRole('link', { name: 'Sign in with Fake ID' }).click()
    const form = page.getByRole('dialog', { name: 'Sign in' })
    await expect(form.getByRole('alert')).toContainText('access_denied')
    await expect(page).not.toHaveURL(/signin_error/)
    expect((await (await page.request.get('/api/auth/status')).json()).signed_in).toBe(false)
    await ctx.close()
  })

  test('LDAP’s test reports a server it can’t reach', async ({ page }) => {
    await page.goto('/#settings/auth')
    await page.locator('[data-method="ldap"]').click()
    await page.getByRole('textbox', { name: 'Server 1' }).fill('127.0.0.1:1')
    await page.getByRole('button', { name: 'Add a server' }).click()
    await page.getByRole('textbox', { name: 'Server 2' }).fill('127.0.0.1:2')
    await page.getByRole('radio', { name: /^None / }).check()
    await page.getByLabel('People are under').fill('dc=example,dc=com')
    await page.getByRole('button', { name: 'Test connectivity' }).click()
    const results = page.getByRole('status').filter({ hasText: '127.0.0.1:1' })
    await expect(results).toBeVisible({ timeout: 30_000 })
    await expect(results.locator('li.bad')).toHaveCount(2)
    // Nothing was saved.
    await page.reload()
    await expect(page.locator('[data-method="oauth"]')).toHaveClass(/on/)
  })
})
