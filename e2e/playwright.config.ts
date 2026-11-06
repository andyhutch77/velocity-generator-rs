import { defineConfig, devices } from '@playwright/test';

const isCI = !!process.env.CI;

export default defineConfig({
  testDir: './tests',
  // In CI: fail if a test was left as test.only, retry once to rule out
  // flakiness, and keep an HTML report plus traces of failed tests.
  forbidOnly: isCI,
  retries: isCI ? 1 : 0,
  reporter: isCI ? [['list'], ['html', { open: 'never' }]] : 'list',
  use: {
    baseURL: 'http://127.0.0.1:4173',
    trace: 'retain-on-failure',
  },
  projects: [{ name: 'chromium', use: { ...devices['Desktop Chrome'] } }],
  // Locally, start the dev server. In CI the workflow serves the release
  // build on the same port first, and Playwright reuses it.
  webServer: {
    command: 'trunk serve --address 127.0.0.1 --port 4173 --no-autoreload',
    cwd: '../crates/velocity-app',
    url: 'http://127.0.0.1:4173',
    reuseExistingServer: true,
    timeout: 300_000,
  },
});
