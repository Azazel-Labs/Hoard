import { defineConfig, devices } from '@playwright/test'

const browser = { ...devices['Desktop Chrome'], viewport: { width: 1400, height: 900 } }

// One hoardd for the run (see e2e/global-setup.ts); tests run in parallel against it, each making its own data where
// it changes anything.
export default defineConfig({
  testDir: './e2e',
  globalSetup: './e2e/global-setup.ts',
  globalTeardown: './e2e/global-teardown.ts',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 1 : 0,
  workers: process.env.CI ? 2 : undefined,
  timeout: 30_000,
  expect: { timeout: 10_000 },
  reporter: process.env.CI ? [['list'], ['html', { open: 'never' }]] : [['list']],
  use: {
    storageState: './e2e/.auth/state.json',
    trace: 'retain-on-failure',
    screenshot: 'only-on-failure',
    video: 'off',
    reducedMotion: 'reduce',
    colorScheme: 'light',
  },
  projects: [
    { name: 'chromium', use: browser, testIgnore: /library-wide[\\/]/ },
    // Tests that change a setting for the whole library run after the rest, so its effects don't reach them.
    { name: 'library-wide', use: browser, testMatch: /library-wide[\\/].*\.spec\.ts$/, dependencies: ['chromium'] },
  ],
})
