import { uploadPack, waitReady } from './support/api'
import { checker } from './support/files'
import { expect, test } from './support/test'

test.describe('Issues', () => {
  test('an archive that won’t unpack is listed, opens its pack, and can be dismissed', async ({ page, api }) => {
    const name = `E2E Broken ${Date.now()}`
    // Starts like a zip, then isn't one.
    const broken = Buffer.concat([Buffer.from('PK\x03\x04'), Buffer.alloc(64, 7)])
    const pack = await uploadPack(api, name, [
      { relpath: 'broken.zip', data: broken },
      { relpath: 'fine.png', data: checker(8, [10, 20, 30], [200, 210, 220], 2) },
    ])
    await waitReady(api, [pack.id])

    await page.goto('/#settings/issues')
    await expect(page.getByRole('heading', { name: 'Issues', level: 2 })).toBeVisible()
    const group = page.getByTestId('issues-unpack')
    const item = group.getByRole('listitem').filter({ hasText: name })
    await expect(item).toBeVisible()
    await expect(item.getByRole('button', { name: 'broken.zip', exact: true })).toBeVisible()

    // The pack's name goes to the pack.
    await item.getByRole('button', { name, exact: true }).click()
    await expect(page.getByText(`pack:${name}`)).toBeVisible()

    await page.goto('/#settings/issues')
    await item.getByRole('button', { name: 'Dismiss broken.zip' }).click()
    await expect(item).toHaveCount(0)
  })
})
