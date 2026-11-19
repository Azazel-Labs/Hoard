import type { Page, Request } from '@playwright/test'
import { expect, test } from './support/test'

// Only the form: the model server and ChatGPT are answered here, and nothing is saved, so tagging stays as it was.
async function fakeLocal(page: Page, reachable: boolean, asked: Request[] = []) {
  await page.route('**/api/admin/llm/local', (route) => {
    asked.push(route.request())
    return route.fulfill({
      json: reachable
        ? {
            reachable: true,
            models: [{ id: 'someone/small-GGUF:Q4_K_M', status: 'unloaded', vision: false, failed: false }],
            suggested: [{ id: 'someone/other-GGUF:Q4_K_M', name: 'Other Model', size_gb: 2.5, license: 'MIT', note: 'Good at names.' }],
          }
        : { reachable: false, error: 'The model server isn’t running', models: [], suggested: [] },
    })
  })
}

test.describe('AI tagging settings', () => {
  test('the local model server lists its models and downloads one; ChatGPT, once signed in, lists its own', async ({ page }) => {
    const asked: Request[] = []
    await fakeLocal(page, true, asked)
    await page.route('**/api/admin/chatgpt', (route) =>
      route.fulfill({ json: { signed_in: true, email: 'someone@example.com', waiting: false, manage_url: 'https://example.com/' } }),
    )
    await page.route('**/api/admin/chatgpt/models', (route) =>
      route.fulfill({ json: { models: [{ id: 'gpt-best', name: 'Best', vision: true }, { id: 'gpt-other' }] } }),
    )
    await page.goto('/#settings/ai')
    const service = page.locator('#llm-provider')

    await service.selectOption({ label: 'Local model' })
    await page.locator('#llm-url').fill('http://llm.example:8080')
    await page.locator('#llm-url').dispatchEvent('change')
    const have = page.getByRole('list', { name: 'On the model server' })
    await expect(have.getByRole('listitem')).toHaveCount(1)
    await expect(have).toContainText('Ready · text only')
    // The server says which models it has, so there's no name to type.
    await expect(page.locator('#llm-model')).toHaveCount(0)
    const offered = page.getByRole('list', { name: 'Models to download' })
    await expect(offered).toContainText('2.5 GB · MIT')
    await offered.getByRole('button', { name: 'Download' }).click()
    await expect
      .poll(() => asked.map((r) => r.postDataJSON() as { action?: string; model?: string }).find((b) => b.action === 'download'))
      .toMatchObject({ model: 'someone/other-GGUF:Q4_K_M' })

    // ChatGPT: signed in, so its first (recommended) model is picked, and it takes images.
    await service.selectOption({ label: 'ChatGPT plan' })
    await expect(page.getByText('Signed in as someone@example.com')).toBeVisible()
    await expect(page.locator('#llm-model')).toHaveValue('gpt-best')
    await expect(page.getByLabel('Send thumbnails', { exact: false })).toBeChecked()
    await expect(have).toHaveCount(0)

    // A model server that isn't running: say so, and the model is named by hand.
    await page.unroute('**/api/admin/llm/local')
    await fakeLocal(page, false)
    await service.selectOption({ label: 'Local model' })
    await expect(page.getByText('The model server isn’t running')).toBeVisible()
    await expect(page.locator('#llm-model')).toBeVisible()
  })
})
