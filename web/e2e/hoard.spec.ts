import { expect, test } from './support/test'

type Seen = { seen: Record<string, number> | null }

test.describe('the dragon hoard', () => {
  // The cave is drawn every frame in software WebGL here, so a small window keeps each frame, and each click, quick.
  test.use({ viewport: { width: 800, height: 600 } })

  test('what the library holds beyond what was collected is loot; Collect records it', async ({ page, api }) => {
    // Start from an empty pile, so the whole library is there to collect.
    expect((await api.put('/api/me/hoard', { data: { seen: {} } })).ok()).toBe(true)
    await page.goto('/#settings/dragon')

    const cave = page.locator('.cave')
    await expect(cave.getByText(/^New loot to collect: [\d,]+$/)).toBeVisible()
    await expect(page.getByRole('img', { name: /^A dragon guarding its hoard/ })).toBeVisible()
    await expect(page.getByRole('list', { name: 'Your hoard' }).getByRole('listitem').first()).toBeVisible()
    await cave.getByRole('button', { name: 'Collect' }).click()
    await expect(cave.getByText('Nothing new to collect.')).toBeVisible()
    await expect(cave.getByRole('button', { name: 'Collect' })).toHaveCount(0)

    const seen = ((await (await api.get('/api/me/hoard')).json()) as Seen).seen!
    expect(Object.values(seen).reduce((a, b) => a + b, 0)).toBeGreaterThan(0)

    // Seen through an arcade screen unless turned off, and it can fill the screen.
    const arcade = cave.getByRole('button', { name: 'Arcade screen' })
    await expect(arcade).toHaveAttribute('aria-pressed', 'true')
    await arcade.click()
    await expect(arcade).toHaveAttribute('aria-pressed', 'false')
    await cave.getByRole('button', { name: 'Full screen' }).click()
    await expect(cave.getByRole('button', { name: 'Leave full screen' })).toBeVisible()
    await cave.getByRole('button', { name: 'Leave full screen' }).click()
    await expect(cave.getByRole('button', { name: 'Full screen' })).toBeVisible()

    // The record names kinds of asset, in lowercase letters.
    expect((await api.put('/api/me/hoard', { data: { seen: { 'Not A Kind': 1 } } })).status()).toBe(400)
  })
})
