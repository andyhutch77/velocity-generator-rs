import { expect, test } from '@playwright/test';

test.beforeEach(async ({ page }) => {
  await page.goto('/');
});

test('shows the heading in the title bar and the window body', async ({ page }) => {
  await expect(
    page.getByRole('heading', { level: 1, name: 'Velocity Generator' }),
  ).toBeVisible();
  const bodyHeading = page
    .locator('#velocity-window')
    .getByRole('heading', { level: 2, name: 'Velocity Generator 🚀' });
  await expect(bodyHeading).toBeVisible();
  const size = (locator: typeof bodyHeading) =>
    locator.evaluate((el) => parseFloat(getComputedStyle(el).fontSize));
  const titleSize = await size(page.getByRole('heading', { level: 1 }));
  expect(await size(bodyHeading)).toBeGreaterThan(titleSize);
  await expect(page).toHaveTitle('Velocity Generator');
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

test('number of engineers defaults to 10, sits below the average and accepts typing', async ({
  page,
}) => {
  const engineers = page.getByLabel('Number of Engineers 🤼');
  await expect(engineers).toHaveValue('10');
  const average = await page.locator('#average').boundingBox();
  const field = await engineers.boundingBox();
  expect(field!.y).toBeGreaterThan(average!.y + average!.height - 1);
  await engineers.fill('12');
  await expect(engineers).toHaveValue('12');
  await expect(page.locator('#average output')).toHaveText('10');
});

test('number of productive days defaults to 10, sits below engineers and accepts typing', async ({
  page,
}) => {
  const days = page.getByLabel('Number Of Productive Days 📆');
  await expect(days).toHaveValue('10');
  const engineers = await page.getByLabel('Number of Engineers 🤼').boundingBox();
  const field = await days.boundingBox();
  expect(field!.y).toBeGreaterThan(engineers!.y);
  await days.fill('8');
  await expect(days).toHaveValue('8');
  await expect(page.locator('#average output')).toHaveText('10');
});

test('number of days off defaults to 25, sits below productive days and checks half days', async ({
  page,
}) => {
  const daysOff = page.getByLabel('Number Of Days off 🏖');
  await expect(daysOff).toHaveValue('25');
  const productive = await page.getByLabel('Number Of Productive Days 📆').boundingBox();
  const field = await daysOff.boundingBox();
  expect(field!.y).toBeGreaterThan(productive!.y);
  for (const good of ['0', '1', '1.5', '2', '2.5']) {
    await daysOff.fill(good);
    await expect(page.getByRole('alert')).toHaveCount(0);
  }
  for (const bad of ['abc', '-1', '1.3', '']) {
    await daysOff.fill(bad);
    await expect(page.getByRole('alert')).toContainText('Days off');
    await expect(daysOff).toHaveAttribute('aria-describedby', 'days-off-error');
  }
});

test('keyboard reaches each input with a visible focus outline', async ({ page }) => {
  for (const n of [1, 2, 3]) {
    await page.keyboard.press('Tab');
    const input = page.getByLabel(`Sprint ${n} velocity`);
    await expect(input).toBeFocused();
    await expect(input).not.toHaveCSS('outline-style', 'none');
  }
});
