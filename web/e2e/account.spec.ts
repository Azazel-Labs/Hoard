import { request, type Page } from '@playwright/test'
import { PASSWORD, USERNAME } from './support/api'
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
})
