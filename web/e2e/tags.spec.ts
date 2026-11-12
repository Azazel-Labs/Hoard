import { addTags, tagsOf } from './support/api'
import { BIG } from './support/seed'
import { expect, test, unique } from './support/test'

// Each test tags a few of the big pack's images with names of its own, so runs and retries never meet.
test.describe('settings: tags', () => {
  test('merges tags, remembers the old names, and can forget one', async ({ page, api, seed }) => {
    const u = unique()
    const [a, b, c] = ['Img 100', 'Img 101', 'Img 102'].map((n) => seed.assets[`${BIG}/${n}`])
    await addTags(api, a, [`e2em${u}-alpha`])
    await addTags(api, b, [`e2em${u}-alpha`])
    await addTags(api, c, [`e2em${u}-beta`])

    await page.goto('/#settings/tags')
    await page.getByLabel('Find tags').fill(`e2em${u}`)
    const list = page.locator('.tags .list').first()
    await expect(list.getByRole('checkbox')).toHaveCount(2)
    await list.getByLabel(`e2em${u}-alpha`).check()
    await list.getByLabel(`e2em${u}-beta`).check()

    // The most used is the starting name; a typed one is shown as the server will keep it.
    const into = page.getByLabel('Merge into')
    await expect(into).toHaveValue(`e2em${u}-alpha`)
    await into.fill(`E2Em${u} Joined`)
    await expect(page.getByText(`Saved as e2em${u}-joined`)).toBeVisible()
    await page.getByRole('button', { name: 'Merge', exact: true }).click()
    const confirm = page.getByRole('dialog', { name: 'Merge 2 tags?' })
    await confirm.getByRole('button', { name: 'Merge', exact: true }).click()
    await expect(page.getByRole('status').filter({ hasText: `3 assets now tagged e2em${u}-joined` })).toBeVisible()
    await expect(list.getByRole('checkbox')).toHaveCount(1)
    await expect(list.getByLabel(`e2em${u}-joined`)).toBeVisible()
    const tagged = await tagsOf(api, c)
    expect(tagged).toContain(`e2em${u}-joined`)
    expect(tagged).not.toContain(`e2em${u}-beta`)

    // Both old names are kept as aliases; one can be let go.
    const alias = page.locator('li').filter({ hasText: `e2em${u}-alpha → e2em${u}-joined` })
    await expect(alias).toBeVisible()
    await expect(page.locator('li').filter({ hasText: `e2em${u}-beta → e2em${u}-joined` })).toBeVisible()
    await alias.getByRole('button', { name: 'Allow the old name again' }).click()
    await expect(alias).toHaveCount(0)
    const aliases = (await (await api.get('/api/tags/aliases')).json()) as { tag: string }[]
    expect(aliases.map((x) => x.tag)).not.toContain(`e2em${u}-alpha`)
    expect(aliases.map((x) => x.tag)).toContain(`e2em${u}-beta`)
  })

  test('renaming a prefixed tag without its prefix keeps the prefix', async ({ page, api, seed }) => {
    const u = unique()
    const id = seed.assets[`${BIG}/Img 110`]
    await addTags(api, id, [`set:e2eold${u}`])

    await page.goto('/#settings/tags')
    await page.getByLabel('Find tags').fill(`e2eold${u}`)
    const list = page.locator('.tags .list').first()
    await list.getByLabel(`set:e2eold${u}`).check()
    const name = page.getByLabel('New name')
    await expect(name).toHaveValue(`set:e2eold${u}`)
    await name.fill(`e2enew${u}`)
    await expect(page.getByText(`Saved as set:e2enew${u}`)).toBeVisible()
    await page.getByRole('button', { name: 'Rename', exact: true }).click()
    await page.getByRole('dialog', { name: `Rename set:e2eold${u}?` }).getByRole('button', { name: 'Rename', exact: true }).click()
    await expect(page.getByRole('status').filter({ hasText: `1 asset now tagged set:e2enew${u}` })).toBeVisible()

    const asset = await tagsOf(api, id)
    expect(asset).toContain(`set:e2enew${u}`)
    expect(asset).not.toContain(`set:e2eold${u}`)
    await expect(page.locator('li').filter({ hasText: `set:e2eold${u} → set:e2enew${u}` })).toBeVisible()
  })

  test('cancelling a merge changes nothing', async ({ page, api, seed }) => {
    const u = unique()
    const id = seed.assets[`${BIG}/Img 120`]
    await addTags(api, id, [`e2ekeep${u}`])
    await page.goto('/#settings/tags')
    await page.getByLabel('Find tags').fill(`e2ekeep${u}`)
    await page.getByLabel(`e2ekeep${u}`).check()
    await page.getByLabel('New name').fill(`e2egone${u}`)
    await page.getByRole('button', { name: 'Rename', exact: true }).click()
    await page.getByRole('dialog', { name: `Rename e2ekeep${u}?` }).getByRole('button', { name: 'Cancel' }).click()
    await expect(page.getByRole('dialog', { name: `Rename e2ekeep${u}?` })).toBeHidden()
    const asset = await tagsOf(api, id)
    expect(asset).toContain(`e2ekeep${u}`)
    await page.getByRole('button', { name: 'Cancel' }).click()
    await expect(page.getByLabel('New name')).toHaveCount(0)
  })
})
