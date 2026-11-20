import { assetsOf, HOARD_HEADERS, uploadPack, waitReady, type Asset } from './support/api'
import { sound } from './support/files'
import { expect, test } from './support/test'

/** One triangle as ASCII PLY. No other test uploads PLY, so a rule converting it touches only this test's pack. */
const PLY = Buffer.from(
  [
    'ply', 'format ascii 1.0', 'element vertex 3', 'property float x', 'property float y', 'property float z',
    'element face 1', 'property list uchar int vertex_indices', 'end_header',
    '0 0 0', '1 0 0', '0 1 0', '3 0 1 2', '',
  ].join('\n'),
)

test.describe('Conversion rules', () => {
  test('a new rule says what it would convert in the library, converts it on yes, and the original can go back', async ({ page, api }) => {
    test.setTimeout(120_000)
    const pack = await uploadPack(api, 'E2E Conversion', [{ relpath: 'rock.ply', data: PLY }])
    await waitReady(api, [pack.id])
    const [rock] = await assetsOf(api, pack.id)
    expect(rock.path).toBe('rock.ply')

    try {
      await page.goto('/#settings/adding')
      const rules = page.getByTestId('conversion-rules')
      await rules.getByTestId('conv-add').click()
      const rule = rules.getByTestId('conv-rule-0')
      for (const ext of ['.obj', '.fbx']) await rule.getByLabel(ext, { exact: true }).uncheck()
      await rule.getByLabel('.ply', { exact: true }).check()
      await rules.getByTestId('conv-save').click()

      // It says how many files it would convert, and which, before anything changes.
      const ask = page.getByTestId('conv-dialog')
      await expect(ask.getByRole('heading', { name: 'Convert one file already in the library?' })).toBeVisible()
      await expect(ask.getByText('It’s in one pack.')).toBeVisible()
      const sample = ask.getByRole('list', { name: 'Files that would be converted' })
      await expect(sample).toContainText('rock.ply')
      await expect(sample).toContainText('→ rock.glb')
      await ask.getByTestId('conv-convert').click()
      await expect(page.getByText('Converting one file in the background')).toBeVisible()

      const saved = await (await api.get('/api/admin/settings')).json()
      expect(saved.conversion_rules.rules).toEqual([{ on: true, from: ['ply'], to: 'glb', upright: true }])

      // The GLB takes the PLY's place: same asset, the original kept and named on its page.
      await expect
        .poll(async () => ((await (await api.get(`/api/assets/${rock.id}`)).json()) as Asset).path, { timeout: 60_000 })
        .toBe('rock.glb')
      const converted = await (await api.get(`/api/assets/${rock.id}`)).json()
      expect(converted.origin).toEqual(expect.objectContaining({ method: 'rule', path: 'rock.ply', ext: 'ply' }))
      expect(converted.origin.kept_until).toBeTruthy()

      await page.goto(`/#asset=${rock.id}`)
      const note = page.getByTestId('rule-origin')
      await expect(note).toContainText('rock.ply')
      await expect(note).toContainText('The original is kept until')
      await note.getByTestId('rule-put-back').click()
      await page.getByRole('dialog').getByRole('button', { name: 'Put the original back' }).click()
      await expect(page.getByText('rock.ply is back')).toBeVisible()

      const back = await (await api.get(`/api/assets/${rock.id}`)).json()
      expect(back.path).toBe('rock.ply')
      expect(back.origin ?? null).toBeNull()

      // The rules leave a file that was put back alone.
      const again = await (await api.post('/api/admin/conversions/preview', { headers: HOARD_HEADERS, data: { rules: { rules: saved.conversion_rules.rules, keep_days: 30 } } })).json()
      expect(again.sample.map((f: { path: string }) => f.path)).not.toContain('rock.ply')
    } finally {
      await api.put('/api/admin/settings', { headers: HOARD_HEADERS, data: { conversion_rules: null } })
    }
  })

  // Other tests' WAVs share the library, so this one only asks, and doesn't save a rule that would convert them.
  test('a rule to FLAC takes WAVs, says what it keeps, and would convert the WAVs in the library', async ({ page, api }) => {
    // Named to sort first, so the dialog's list of files shows it.
    const pack = await uploadPack(api, '0 E2E FLAC', [{ relpath: 'tone.wav', data: sound('zap', 4) }])
    await waitReady(api, [pack.id])

    await page.goto('/#settings/adding')
    const rules = page.getByTestId('conversion-rules')
    await rules.getByTestId('conv-add').click()
    const rule = rules.locator('[data-testid^="conv-rule-"]').last()
    await expect(rule.getByLabel('.obj', { exact: true })).toBeVisible()
    await expect(rule.getByText('Turn models upright')).toBeVisible()

    await rule.getByTestId('conv-to').selectOption('flac')
    // Only what a FLAC is made from, chosen for it, and nothing about models.
    await expect(rule.getByLabel('.wav', { exact: true })).toBeChecked()
    await expect(rule.getByLabel('.obj', { exact: true })).toHaveCount(0)
    await expect(rule.getByText('Turn models upright')).toHaveCount(0)
    await expect(rule.getByTestId('conv-flac-what')).toContainText('The sound stays exactly the same')

    await rules.getByTestId('conv-save').click()
    const ask = page.getByTestId('conv-dialog')
    const sample = ask.getByRole('list', { name: 'Files that would be converted' })
    await expect(sample).toContainText('tone.wav')
    await expect(sample).toContainText('→ tone.flac')
    await ask.getByRole('button', { name: 'Cancel' }).click()
    await expect(ask).toBeHidden()

    // Nothing was saved or converted. (The other test saves rules of its own meanwhile.)
    const after = (await (await api.get('/api/admin/settings')).json()).conversion_rules
    expect(after.rules.filter((r: { to: string }) => r.to === 'flac')).toEqual([])
    const [tone] = await assetsOf(api, pack.id)
    expect(tone.path).toBe('tone.wav')
  })
})
