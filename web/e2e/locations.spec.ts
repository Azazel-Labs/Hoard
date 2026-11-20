import { type IncomingMessage, type Server, createServer } from 'node:http'
import type { AddressInfo } from 'node:net'
import { expect, test, unique } from './support/test'

// Adding a library location starts with what kind it is: a folder (on a disk, a NAS, or one a sync service keeps), or
// a bucket in S3 storage. Each is a big card with its picture.
test('adding a location asks first whether it’s a folder or a bucket in S3 storage', async ({ page }) => {
  await page.goto('/#settings/locations')
  await expect(page.getByTestId('s3-storage')).toHaveCount(0)
  await page.getByRole('button', { name: 'Add a location…' }).click()
  const kinds = page.getByRole('dialog', { name: 'Add a location' })
  await expect(kinds.getByText('What kind of location is it?')).toBeVisible()
  const folder = kinds.getByRole('button', { name: /^A folder/ })
  const bucket = kinds.getByRole('button', { name: /^S3 storage/ })
  await expect(folder).toContainText('Dropbox')
  await expect(bucket).toContainText('Cloudflare R2')
  for (const card of [folder, bucket]) await expect(card.locator('svg')).toBeVisible()

  // Escape puts it away, and nothing's begun.
  await page.keyboard.press('Escape')
  await expect(kinds).toBeHidden()
  await expect(page.getByRole('button', { name: 'Add a location…' })).toBeVisible()

  // A folder: the form that asks whether Hoard claims it or watches it.
  await page.getByRole('button', { name: 'Add a location…' }).click()
  await folder.click()
  await expect(kinds).toBeHidden()
  await expect(page.getByRole('group', { name: 'What should Hoard do with this folder?' })).toBeVisible()
  await page.getByRole('button', { name: 'Cancel' }).click()

  // A bucket: the service and its keys, then the bucket's name. Hoard keeps files there, so there's nothing to choose.
  await page.getByRole('button', { name: 'Add a location…' }).click()
  await bucket.click()
  await expect(page.getByLabel('Address')).toBeVisible()
  await expect(page.getByLabel('Secret access key')).toBeVisible()
  await expect(page.getByLabel('Bucket', { exact: true })).toBeVisible()
  await expect(page.getByRole('group', { name: 'What should Hoard do with this folder?' })).toHaveCount(0)
  await expect(page.getByRole('button', { name: 'Add location' })).toBeDisabled()
  await page.getByRole('button', { name: 'Cancel' }).click()
  await expect(page.getByLabel('Bucket', { exact: true })).toHaveCount(0)
})

/** Just enough of S3 for a bucket that's there and empty; it keeps the requests it's sent. */
async function fakeS3(): Promise<{ server: Server; url: string; seen: IncomingMessage[] }> {
  const seen: IncomingMessage[] = []
  const server = createServer((req, res) => {
    seen.push(req)
    req.resume()
    if (req.method === 'GET' && req.url?.includes('list-type=2')) {
      res.writeHead(200, { 'content-type': 'application/xml' })
      res.end(
        '<?xml version="1.0" encoding="UTF-8"?><ListBucketResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">' +
          '<Name>bucket</Name><Prefix></Prefix><KeyCount>0</KeyCount><MaxKeys>1000</MaxKeys><IsTruncated>false</IsTruncated>' +
          '</ListBucketResult>',
      )
    } else if (req.method === 'PUT') {
      res.writeHead(200, { etag: '"e2e"' })
      res.end()
    } else if (req.method === 'DELETE') {
      res.writeHead(204)
      res.end()
    } else {
      res.writeHead(404, { 'content-type': 'application/xml' })
      res.end('<?xml version="1.0" encoding="UTF-8"?><Error><Code>NoSuchKey</Code></Error>')
    }
  })
  await new Promise<void>((done) => server.listen(0, '127.0.0.1', done))
  return { server, url: `http://127.0.0.1:${(server.address() as AddressInfo).port}`, seen }
}

// The keys typed for the service are saved when the bucket's added (the server reaches the bucket with them to check
// it), and the S3 storage card is in Settings from then on, to change them.
test('a bucket in S3 storage is added with the keys typed for it', async ({ page, api }) => {
  const s3 = await fakeS3()
  const name = `E2E Bucket ${unique()}`
  try {
    await page.goto('/#settings/locations')
    await page.getByRole('button', { name: 'Add a location…' }).click()
    await page.getByRole('dialog', { name: 'Add a location' }).getByRole('button', { name: /^S3 storage/ }).click()
    await page.getByLabel('Address').fill(s3.url)
    await page.getByLabel('Access key ID').fill('e2e-key')
    await page.getByLabel('Secret access key').fill('e2e-secret')
    await page.getByLabel('Bucket', { exact: true }).fill('e2e-bucket')
    await page.getByLabel('Name').fill(name)
    await page.getByText('Shaders', { exact: true }).click()
    await expect(page.getByRole('checkbox', { name: 'Shaders' })).toBeChecked()
    await page.getByRole('button', { name: 'Add location' }).click()

    await expect(page.getByText(`Added ${name}`)).toBeVisible()
    const row = page.getByRole('listitem').filter({ hasText: name })
    await expect(row).toContainText('s3://e2e-bucket')
    await expect(row).toContainText('Shaders')
    await expect(page.getByTestId('s3-storage')).toBeVisible()
    await expect(page.getByTestId('s3-storage').getByLabel('Address')).toHaveValue(s3.url)
    // The bucket was reached with the keys typed.
    expect(s3.seen.some((r) => /Credential=e2e-key\//.test(r.headers.authorization ?? ''))).toBe(true)
  } finally {
    const list = (await (await api.get('/api/admin/locations')).json()) as { locations: { id: number; name: string }[] }
    for (const l of list.locations.filter((l) => l.name === name)) await api.delete(`/api/admin/locations/${l.id}?force=true`)
    await api.put('/api/admin/settings', {
      data: { s3_endpoint: null, s3_region: null, s3_access_key: null, s3_secret_key: null, s3_headers: null },
    })
    s3.server.close()
  }
})
