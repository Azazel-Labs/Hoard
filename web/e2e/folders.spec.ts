import { uploadPack, waitReady } from './support/api'
import { checker, hue } from './support/files'
import { expect, openPack, test, tile, unique } from './support/test'

// A pack browses as the folders it had on disk: a breadcrumb, the folders in this one with their counts, and only
// what's directly in it in the grid. A folder in an asset's path opens that folder of its pack.
test('a pack browses folder by folder, and a path opens its folder', async ({ page, api }) => {
  const name = `E2E Folders ${unique()}`
  const pic = (n: number) => checker(16, hue(n, 6), [255, 255, 255])
  const pack = await uploadPack(api, name, [
    { relpath: 'top.png', data: pic(0) },
    { relpath: 'UI/Buttons/ok.png', data: pic(1) },
    { relpath: 'UI/Buttons/cancel.png', data: pic(2) },
    { relpath: 'UI/panel.png', data: pic(3) },
    { relpath: 'Tiles/grass.png', data: pic(4) },
  ])
  try {
    await waitReady(api, [pack.id])
    await openPack(page, pack.id, name, '&folder=')
    const crumbs = page.locator('.folderbar .crumbs')
    const subfolders = page.locator('.subfolders .subfolder')
    await expect(page.getByRole('button', { name: 'Folders', exact: true })).toHaveAttribute('aria-pressed', 'true')
    await expect(subfolders).toHaveCount(2)
    await expect(subfolders.nth(0)).toContainText('Tiles')
    await expect(subfolders.nth(1)).toContainText('UI')
    await expect(subfolders.nth(1)).toContainText('3 assets')
    await expect(tile(page, 'Top')).toBeVisible()
    await expect(tile(page, 'Panel')).toHaveCount(0)

    await subfolders.nth(1).click()
    await expect(crumbs.getByRole('button')).toHaveText([name, 'UI'])
    await expect(tile(page, 'Panel')).toBeVisible()
    await expect(tile(page, 'Top')).toHaveCount(0)
    await expect(subfolders).toHaveCount(1)
    await expect(page).toHaveURL(/folder=UI/)

    // Everything at once again, then back into folders from an asset's path.
    await page.getByRole('button', { name: 'Folders', exact: true }).click()
    await expect(page.locator('.folderbar .crumbs')).toHaveCount(0)
    await expect(subfolders).toHaveCount(0)
    await expect(tile(page, 'Ok')).toBeVisible()
    await tile(page, 'Ok').click()
    await page.getByRole('button', { name: 'Buttons', exact: true }).click()
    await expect(crumbs.getByRole('button')).toHaveText([name, 'UI', 'Buttons'])
    await expect(tile(page, 'Cancel')).toBeVisible()
    await expect(tile(page, 'Panel')).toHaveCount(0)

    // The pack's name goes back to its top.
    await crumbs.getByRole('button', { name, exact: true }).click()
    await expect(subfolders).toHaveCount(2)
    await expect(tile(page, 'Top')).toBeVisible()
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})

// Adding a library location asks whether Hoard claims the folder or watches it, each with its dragon: one on treasure
// it keeps in order, one eyeing a heap it leaves as it is.
test('adding a location shows a dragon for claiming the folder and one for watching it', async ({ page }) => {
  await page.goto('/#settings/locations')
  await page.getByRole('button', { name: 'Add a location…' }).click()
  await page.getByRole('dialog', { name: 'Add a location' }).getByRole('button', { name: /^A folder/ }).click()
  const choices = page.getByRole('group', { name: 'What should Hoard do with this folder?' })
  const claim = choices.getByRole('radio', { name: /^Claim it/ })
  const watchIt = choices.getByRole('radio', { name: /^Watch it/ })
  await expect(claim).toBeChecked()
  const art = choices.locator('img')
  await expect(art).toHaveCount(2)
  for (const img of await art.all()) {
    await expect.poll(() => img.evaluate((el: HTMLImageElement) => el.complete && el.naturalWidth)).toBe(160)
  }
  // The pictures are part of the choice: clicking one picks it.
  await art.nth(1).click()
  await expect(watchIt).toBeChecked()
  await page.getByRole('button', { name: 'Cancel' }).click()
})
