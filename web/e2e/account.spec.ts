import { request, type APIRequestContext, type Page } from '@playwright/test'
import { assetsOf, HOARD_HEADERS, PASSWORD, USERNAME } from './support/api'
import { MODELS } from './support/seed'
import { doubleClick, expect, openPack, test, tile, unique } from './support/test'

async function bearerSignedIn(baseURL: string, token: string): Promise<boolean> {
  const ctx = await request.newContext({ baseURL, extraHTTPHeaders: { Authorization: `Bearer ${token}` } })
  const status = (await (await ctx.get('/api/auth/status')).json()) as { signed_in: boolean }
  await ctx.dispose()
  return status.signed_in
}

/**
 * Make the page think it's on plain http (not a secure context), as on a LAN server, and record what the old
 * `execCommand('copy')` way copies, since a headless browser has no real clipboard for it.
 */
async function pretendPlainHttp(page: Page): Promise<void> {
  await page.addInitScript(() => {
    Object.defineProperty(window, 'isSecureContext', { get: () => false })
    const w = window as unknown as { __copied: { text: string; inDialog: boolean }[] }
    w.__copied = []
    document.execCommand = ((cmd: string) => {
      if (cmd !== 'copy') return false
      const el = document.activeElement
      if (!(el instanceof HTMLTextAreaElement)) return false
      const text = el.value.slice(el.selectionStart, el.selectionEnd)
      w.__copied.push({ text, inDialog: !!el.closest('dialog[open]') })
      return true
    }) as typeof document.execCommand
  })
}

/** Add an account with a role; returns its id and a request context signed in as it. */
async function person(api: APIRequestContext, baseURL: string, username: string, role: string) {
  const made = await api.post('/api/admin/users', { data: { username, password: PASSWORD, role } })
  expect(made.ok(), await made.text()).toBe(true)
  const users = ((await (await api.get('/api/admin/users')).json()) as { users: { id: number; username: string }[] }).users
  const id = users.find((u) => u.username === username)!.id
  const ctx = await request.newContext({ baseURL, extraHTTPHeaders: HOARD_HEADERS })
  expect((await ctx.post('/api/auth/login', { data: { username, password: PASSWORD } })).ok()).toBe(true)
  return { id, ctx }
}

function copied(page: Page) {
  return page.evaluate(() => (window as unknown as { __copied: { text: string; inDialog: boolean }[] }).__copied)
}

test.describe('account', () => {
  test.describe('signing in', () => {
    // A browser of its own: signing out ends the session, and the other tests share the saved one.
    test.use({ storageState: { cookies: [], origins: [] } })

    test('refuses a wrong password, signs in with the right one, signs out', async ({ page }) => {
      await page.goto('/')
      const form = page.getByRole('dialog', { name: 'Sign in' })
      await expect(form).toBeVisible()
      await form.getByLabel('Name').fill(USERNAME)
      await form.getByLabel('Password').fill('not-the-password')
      await form.getByRole('button', { name: 'Sign in' }).click()
      await expect(form.getByRole('alert')).toBeVisible()

      await form.getByLabel('Password').fill(PASSWORD)
      await form.getByRole('button', { name: 'Sign in' }).click()
      await expect(form).toBeHidden()
      await expect(page.getByLabel('Search assets')).toBeVisible()

      await page.goto('/#settings/account')
      await page.getByRole('button', { name: 'Sign out' }).click()
      await expect(page.getByRole('dialog', { name: 'Sign in' })).toBeVisible()
      await page.reload()
      await expect(page.getByRole('dialog', { name: 'Sign in' })).toBeVisible()
    })
  })

  test('makes an access token, copies it, and revokes it', async ({ page, context, baseURL }) => {
    await context.grantPermissions(['clipboard-read', 'clipboard-write'])
    const name = `E2E token ${unique()}`
    await page.goto('/#settings/account')
    await page.getByLabel('What will use it').fill(name)
    await page.getByRole('button', { name: 'Make a token' }).click()

    const made = page.locator('.made[role="status"]')
    await expect(made).toContainText(`${name}: copy it now. It isn’t shown again.`)
    const token = await made.getByLabel('The new token').inputValue()
    expect(token.length).toBeGreaterThan(20)
    await made.getByRole('button', { name: 'Copy' }).click()
    await expect(page.getByRole('status').filter({ hasText: /^Copied$/ })).toBeVisible()
    expect(await page.evaluate(() => navigator.clipboard.readText())).toBe(token)
    expect(await bearerSignedIn(baseURL!, token)).toBe(true)

    await made.getByRole('button', { name: 'Done' }).click()
    await expect(made).toHaveCount(0)
    const row = page.locator('.account li').filter({ hasText: name })
    await row.getByRole('button', { name: 'Revoke' }).click()
    await page.getByRole('dialog', { name: `Revoke “${name}”?` }).getByRole('button', { name: 'Revoke' }).click()
    await expect(row).toHaveCount(0)
    expect(await bearerSignedIn(baseURL!, token)).toBe(false)
  })

  test('on plain http, Copy falls back to a selected textarea', async ({ page, api }) => {
    await pretendPlainHttp(page)
    const name = `E2E token ${unique()}`
    await page.goto('/#settings/account')
    expect(await page.evaluate(() => window.isSecureContext)).toBe(false)
    await page.getByLabel('What will use it').fill(name)
    await page.getByRole('button', { name: 'Make a token' }).click()
    const made = page.locator('.made[role="status"]')
    const token = await made.getByLabel('The new token').inputValue()
    await made.getByRole('button', { name: 'Copy' }).click()
    await expect(page.getByRole('status').filter({ hasText: /^Copied$/ })).toBeVisible()
    expect(await copied(page)).toEqual([{ text: token, inDialog: false }])
    // The textarea is gone again, and focus is back on the button.
    await expect(page.locator('body > textarea')).toHaveCount(0)
    await expect(made.getByRole('button', { name: 'Copy' })).toBeFocused()

    const { tokens } = (await (await api.get('/api/admin/tokens')).json()) as { tokens: { id: number; name: string }[] }
    const mine = tokens.find((t) => t.name === name)
    if (mine) await api.delete(`/api/admin/tokens/${mine.id}`)
  })

  test('on plain http, Copy inside a full-screen viewer copies from within the dialog', async ({ page, seed }) => {
    await pretendPlainHttp(page)
    await openPack(page, seed.packs[MODELS], MODELS, '&sort=name')
    await doubleClick(page, tile(page, 'Notes'))
    const viewer = page.getByRole('dialog', { name: 'Read Notes' })
    await expect(viewer.getByLabel('Notes contents')).toContainText('Notes for the e2e pack')
    await viewer.getByRole('button', { name: /^Copy/ }).click()
    await expect(viewer.getByRole('button', { name: /^Copied/ })).toBeVisible()
    const [entry] = await copied(page)
    expect(entry.inDialog, 'a modal dialog makes the rest of the page inert, so the textarea goes inside it').toBe(true)
    expect(entry.text).toContain('Notes for the e2e pack')
  })

  test.describe('roles', () => {
    test('one who can look can’t change anything, one who can change can’t administer, and tokens follow their maker', async ({ api, baseURL, seed }) => {
      const [asset] = await assetsOf(api, seed.packs[MODELS])
      const looker = await person(api, baseURL!, `look-${unique()}`, 'view')
      const changer = await person(api, baseURL!, `change-${unique()}`, 'edit')
      const bearers: APIRequestContext[] = []
      const bearer = async (token: string) => {
        const ctx = await request.newContext({ baseURL, extraHTTPHeaders: { Authorization: `Bearer ${token}` } })
        bearers.push(ctx)
        return ctx
      }
      try {
        const tag = { data: { add_tags: [`role-${unique()}`] } }
        // Looking is fine; changing, Settings and accounts aren't.
        expect((await looker.ctx.get(`/api/assets/${asset.id}`)).status()).toBe(200)
        expect(((await (await looker.ctx.get('/api/auth/status')).json()) as { user: { role: string } }).user.role).toBe('view')
        expect((await looker.ctx.patch(`/api/assets/${asset.id}`, tag)).status()).toBe(403)
        expect((await looker.ctx.get('/api/admin/settings')).status()).toBe(403)
        expect((await looker.ctx.get('/api/admin/users')).status()).toBe(403)
        // Its own tokens, but none that can do more than it can.
        expect((await looker.ctx.post('/api/admin/tokens', { data: { name: 'too much', role: 'edit' } })).status()).toBe(403)
        const lookToken = (await (await looker.ctx.post('/api/admin/tokens', { data: { name: 'engine' } })).json()) as { token: string; role: string }
        expect(lookToken.role).toBe('view')

        expect((await changer.ctx.patch(`/api/assets/${asset.id}`, tag)).status()).toBe(200)
        expect((await changer.ctx.get('/api/admin/settings')).status()).toBe(403)
        expect((await changer.ctx.post('/api/archive/purge', { data: {} })).status()).toBe(403)
        expect((await changer.ctx.patch(`/api/admin/users/${looker.id}`, { data: { role: 'admin' } })).status()).toBe(403)
        const changeToken = ((await (await changer.ctx.post('/api/admin/tokens', { data: { name: 'worker' } })).json()) as { token: string }).token

        // Each sees only their own tokens; an admin sees everyone's.
        const names = async (ctx: APIRequestContext) =>
          ((await (await ctx.get('/api/admin/tokens')).json()) as { tokens: { name: string }[] }).tokens.map((t) => t.name)
        expect(await names(looker.ctx)).toEqual(['engine'])
        expect(await names(api)).toEqual(expect.arrayContaining(['engine', 'worker']))

        const looking = await bearer(lookToken.token)
        expect((await looking.get(`/api/assets/${asset.id}`)).status()).toBe(200)
        expect((await looking.patch(`/api/assets/${asset.id}`, tag)).status()).toBe(403)
        const working = await bearer(changeToken)
        expect((await working.patch(`/api/assets/${asset.id}`, tag)).status()).toBe(200)

        // Given a lesser role, the account's tokens have it too. Nobody changes their own.
        expect((await api.patch(`/api/admin/users/${changer.id}`, { data: { role: 'view' } })).ok()).toBe(true)
        expect((await working.patch(`/api/assets/${asset.id}`, tag)).status()).toBe(403)
        expect((await changer.ctx.patch(`/api/assets/${asset.id}`, tag)).status()).toBe(403)
        const me = ((await (await api.get('/api/admin/users')).json()) as { users: { id: number; you: boolean }[] }).users.find((u) => u.you)!
        expect((await api.patch(`/api/admin/users/${me.id}`, { data: { role: 'view' } })).ok()).toBe(false)

        // An app that can only look isn't offered the tools that change things.
        const mcp = await looking.post('/mcp', { data: { jsonrpc: '2.0', id: 1, method: 'tools/list' } })
        const tools = ((await mcp.json()) as { result: { tools: { name: string }[] } }).result.tools.map((t) => t.name)
        expect(tools).toContain('search_assets')
        expect(tools).not.toContain('tag_asset')
      } finally {
        await Promise.all(bearers.map((b) => b.dispose()))
        for (const p of [looker, changer]) {
          await p.ctx.dispose()
          await api.delete(`/api/admin/users/${p.id}`)
        }
      }
    })

    test.describe('in the portal', () => {
      test.use({ storageState: { cookies: [], origins: [] } })

      test('one who can look sees no way to change things, and only their own settings', async ({ page, baseURL, seed }) => {
        const admin = await request.newContext({ baseURL, extraHTTPHeaders: HOARD_HEADERS })
        await admin.post('/api/auth/login', { data: { username: USERNAME, password: PASSWORD } })
        const name = `look-${unique()}`
        const looker = await person(admin, baseURL!, name, 'view')
        try {
          await page.goto('/')
          const form = page.getByRole('dialog', { name: 'Sign in' })
          await form.getByLabel('Name').fill(name)
          await form.getByLabel('Password').fill(PASSWORD)
          await form.getByRole('button', { name: 'Sign in' }).click()
          await expect(form).toBeHidden()
          await expect(page.getByLabel('Search assets')).toBeVisible()
          await expect(page.getByRole('button', { name: 'Import folder' })).toHaveCount(0)
          await expect(page.getByRole('button', { name: /Add files/ })).toHaveCount(0)

          await openPack(page, seed.packs[MODELS], MODELS, '&sort=name')
          await tile(page, 'Crate').first().click()
          const detail = page.getByRole('complementary', { name: 'Asset details' })
          await expect(detail.getByRole('heading', { name: 'Tags' })).toBeVisible()
          await expect(detail.getByLabel('Add a tag')).toHaveCount(0)
          await expect(detail.getByRole('button', { name: 'Archive', exact: true })).toHaveCount(0)

          await page.goto('/#settings/account')
          const nav = page.getByRole('navigation', { name: 'Settings' })
          await expect(nav.getByRole('button')).toHaveText(['Appearance', 'Library', 'Account'])
          await expect(page.getByText('can look but not change anything')).toBeVisible()
          await expect(page.getByRole('heading', { name: 'People' })).toHaveCount(0)
          await expect(page.getByLabel('What it can do').locator('option')).toHaveText(['Can look'])
        } finally {
          await looker.ctx.dispose()
          await admin.delete(`/api/admin/users/${looker.id}`)
          await admin.dispose()
        }
      })
    })

    test('an admin chooses what each person can do', async ({ page, api }) => {
      const name = `person-${unique()}`
      await page.goto('/#settings/account')
      await page.getByPlaceholder('Name', { exact: true }).fill(name)
      await page.getByPlaceholder('Password (8+ characters)').fill(PASSWORD)
      const pick = page.getByLabel('What they can do')
      await expect(pick).toHaveValue('edit')
      await pick.selectOption('view')
      await page.getByRole('button', { name: 'Add', exact: true }).click()
      const role = page.getByLabel(`What ${name} can do`)
      await expect(role).toHaveValue('view')
      await role.selectOption('admin')
      const users = async () => ((await (await api.get('/api/admin/users')).json()) as { users: { id: number; username: string; role: string }[] }).users
      await expect.poll(async () => (await users()).find((u) => u.username === name)?.role).toBe('admin')
      await api.delete(`/api/admin/users/${(await users()).find((u) => u.username === name)!.id}`)
    })
  })
})
