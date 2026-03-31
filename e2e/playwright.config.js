import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './tests',
  timeout: 45000,
  use: {
    trace: 'on-first-retry',
    actionTimeout: 10000,
    navigationTimeout: 10000,
  },
  projects: [
    {
      name: 'biller',
      testMatch: /biller\.spec\.js$/,
    },
    {
      name: 'payer',
      testMatch: /payer\.spec\.js$/,
    },
  ],
});
