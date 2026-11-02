import { expect, test } from '@playwright/test';

test.beforeEach(async ({ page }) => {
  await page.goto('/');
});

test('shows the heading', async ({ page }) => {
  await expect(
    page.getByRole('heading', { level: 1, name: 'Velocity Calculator' }),
  ).toBeVisible();
});

test('label is linked to its input and typing works', async ({ page }) => {
  const input = page.getByLabel('Sprint name');
  await input.fill('Sprint 1');
  await expect(input).toHaveValue('Sprint 1');
});

test('keyboard reaches the button with a visible focus outline', async ({ page }) => {
  await page.keyboard.press('Tab');
  await expect(page.getByLabel('Sprint name')).toBeFocused();
  await page.keyboard.press('Tab');
  const button = page.getByRole('button', { name: 'OK' });
  await expect(button).toBeFocused();
  await expect(button).not.toHaveCSS('outline-style', 'none');
  await page.keyboard.press('Enter');
});
