import { expect, test } from '@playwright/test';

test.beforeEach(async ({ page }) => {
  await page.goto('/');
});

test('shows the heading', async ({ page }) => {
  await expect(
    page.getByRole('heading', { level: 1, name: 'Velocity Calculator' }),
  ).toBeVisible();
});

test('shows the default average of 10', async ({ page }) => {
  for (const n of [1, 2, 3]) {
    await expect(page.getByLabel(`Sprint ${n} velocity`)).toHaveValue('10');
  }
  await expect(page.locator('#average output')).toHaveText('10');
});

test('average updates and rounds to two decimals', async ({ page }) => {
  await page.getByLabel('Sprint 1 velocity').fill('10');
  await page.getByLabel('Sprint 2 velocity').fill('11');
  await page.getByLabel('Sprint 3 velocity').fill('12');
  await expect(page.locator('#average output')).toHaveText('11');
  await page.getByLabel('Sprint 2 velocity').fill('10');
  await page.getByLabel('Sprint 3 velocity').fill('11');
  await expect(page.locator('#average output')).toHaveText('10.33');
});

for (const bad of ['', 'abc', '-1']) {
  test(`invalid input "${bad}" shows an alert and no average`, async ({ page }) => {
    await page.getByLabel('Sprint 2 velocity').fill(bad);
    await expect(page.getByRole('alert')).toContainText('Sprint 2 velocity');
    await expect(page.locator('#average output')).toHaveCount(0);
  });
}

test('keyboard reaches each input with a visible focus outline', async ({ page }) => {
  for (const n of [1, 2, 3]) {
    await page.keyboard.press('Tab');
    const input = page.getByLabel(`Sprint ${n} velocity`);
    await expect(input).toBeFocused();
    await expect(input).not.toHaveCSS('outline-style', 'none');
  }
});
