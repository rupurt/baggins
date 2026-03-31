import { expect, test } from '@playwright/test';

const BACKEND_BASE = 'http://127.0.0.1:8080';
const PAYER_APP = 'http://127.0.0.1:5174';
const PAYER_HEADERS = {
  'x-role': 'payer',
  'x-actor-id': 'e2e-payer',
};

test.describe.configure({ mode: 'serial' });

test('payer denial triage is connected to backend workflows', async ({ page }) => {
  await page.goto(PAYER_APP, { waitUntil: 'domcontentloaded' });
  await expect(page.getByText('Baggins Payer Workspace')).toBeVisible();

  await page.getByTestId('payer-payer-input').fill('UnitedCare');
  await Promise.all([
    page.waitForResponse(
      (response) =>
        response.url().includes('/v1/payer/denials/search') &&
        response.request().method() === 'GET',
    ),
    page.getByTestId('payer-search-button').click(),
  ]);

  const firstResult = page.getByTestId('payer-result-DENIAL-3001');
  await expect(firstResult).toBeVisible();
  await firstResult.click();

  await page.getByTestId('payer-command').selectOption('escalate');
  const [actionResponse] = await Promise.all([
    page.waitForResponse(
      (response) =>
        response.url().includes('/v1/cases/DENIAL-3001/action') &&
        response.request().method() === 'POST',
    ),
    page.getByTestId('payer-execute-button').click(),
  ]);

  const actionBody = await actionResponse.json();
  expect(actionBody.resulting_status).toBe('escalated');
  expect(actionBody.resulting_queue_state).toBe('manual_review_required');
  expect(actionBody.command).toBe('escalate');
  expect(actionBody.command_id).toBeTruthy();
  expect(actionBody.trace_id).toBeTruthy();
  expect(actionBody.transition_id).toBeTruthy();
  expect(actionBody.command_outcome).toBe('executed');
  await expect(page.locator('.case-box')).toContainText('DENIAL-3001');
});

test('payer search enforces biller role boundary', async () => {
  const billerHeaders = {
    'x-role': 'biller',
    'x-actor-id': 'e2e-blocked-payer',
  };
  const denied = await fetch(`${BACKEND_BASE}/v1/payer/denials/search?limit=20`, {
    headers: billerHeaders,
  });
  expect(denied.status).toBe(403);
  const body = await denied.json();
  expect(body.error.code).toBe('FORBIDDEN');
  expect(body.error.next_allowed_actions).toContain('payer');
});

test('payer command replay should return identical transition with same idempotency key', async ({ request }) => {
  const runTag = `payer-${Date.now()}-${Math.random().toString(36).slice(2, 10)}`;
  const testActor = `e2e-payer-${runTag}`;
  const headers = { ...PAYER_HEADERS, 'x-actor-id': testActor };
  const payload = {
    command: 'triage',
    confirm: true,
    request_id: `e2e-payer-triage-${runTag}`,
    idempotency_key: `e2e-payer-triage-${runTag}`,
    params: { source: 'e2e', submitted_at: '2026-03-31T00:00:00Z' },
  };
  const first = await request.post(`${BACKEND_BASE}/v1/cases/DENIAL-3002/action`, {
    headers,
    data: payload,
  });
  expect(first.ok()).toBeTruthy();
  const firstBody = await first.json();
  expect(firstBody.preview).toBe(false);
  expect(firstBody.replayed).toBe(false);
  expect(firstBody.resulting_status).toBe('triaged');
  expect(firstBody.resulting_queue_state).toBe('triage_in_progress');

  const second = await request.post(`${BACKEND_BASE}/v1/cases/DENIAL-3002/action`, {
    headers,
    data: payload,
  });
  expect(second.ok()).toBeTruthy();
  const secondBody = await second.json();
  expect(secondBody.replayed).toBe(true);
  expect(secondBody.command_id).toBe(firstBody.command_id);
});

test('payer role is denied for biller-only transitions', async () => {
  const blocked = await fetch(`${BACKEND_BASE}/v1/cases/DENIAL-3001/action`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      ...PAYER_HEADERS,
    },
    body: JSON.stringify({
      command: 'submit_draft',
      confirm: true,
      request_id: 'e2e-payer-disallow-1',
      idempotency_key: 'e2e-payer-disallow-1',
      params: { source: 'e2e' },
    }),
  });
  expect(blocked.status).toBe(403);
  const blockedBody = await blocked.json();
  expect(blockedBody.error.code).toBe('FORBIDDEN');
});
