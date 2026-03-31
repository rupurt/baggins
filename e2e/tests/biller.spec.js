import { expect, test } from '@playwright/test';

const BACKEND_BASE = 'http://127.0.0.1:8080';
const BILLER_APP = 'http://127.0.0.1:5173';
const BILLER_HEADERS = {
  'x-role': 'biller',
  'x-actor-id': 'e2e-biller',
};

test.describe.configure({ mode: 'serial' });

test('biller search and command flow is connected to backend', async ({ page }) => {
  await page.goto(BILLER_APP, { waitUntil: 'domcontentloaded' });
  await expect(page.getByText('Baggins Biller Workspace')).toBeVisible();

  await page.getByTestId('biller-query-input').fill('MRN-1010');
  await Promise.all([
    page.waitForResponse(
      (response) =>
        response.url().includes('/v1/biller/search') &&
        response.request().method() === 'GET',
    ),
    page.getByTestId('biller-search-button').click(),
  ]);

  const firstResult = page.getByTestId('biller-result-BILLER-1001');
  await expect(firstResult).toBeVisible();
  await firstResult.click();

  const casePayload = await page.request.get(`${BACKEND_BASE}/v1/cases/BILLER-1001`, {
    headers: BILLER_HEADERS,
  });
  expect(casePayload.ok()).toBeTruthy();
  const caseBody = await casePayload.json();
  expect(caseBody.case_data.patient_token).toBeTruthy();
  expect(caseBody.case_data.mrn).toBeUndefined();

  const [actionResponse] = await Promise.all([
    page.waitForResponse(
      (response) =>
        response.url().includes('/v1/cases/BILLER-1001/action') &&
        response.request().method() === 'POST',
    ),
    page.getByTestId('biller-execute-button').click(),
  ]);
  const actionBody = await actionResponse.json();
  expect(actionBody.resulting_status).toBe('validated');
  expect(actionBody.resulting_queue_state).toBe('validation_complete');
  expect(actionBody.command).toBe('validate');
  expect(actionBody.command_id).toBeTruthy();
  expect(actionBody.trace_id).toBeTruthy();
  expect(actionBody.transition_id).toBeTruthy();
  expect(actionBody.command_outcome).toBe('executed');

  await expect(page.getByText('Loaded case BILLER-1001')).toBeVisible();
});

test('biller command preview and idempotent replay are deterministic', async ({ request }) => {
  const previewPayload = {
    command: 'retry',
    confirm: false,
    request_id: 'e2e-biller-preview-1',
    idempotency_key: 'e2e-biller-retry-biller-1002',
    params: { source: 'e2e', submitted_at: '2026-03-31T00:00:00Z' },
  };
  const previewResponse = await request.post(`${BACKEND_BASE}/v1/cases/BILLER-1002/action`, {
    headers: BILLER_HEADERS,
    data: previewPayload,
  });
  expect(previewResponse.ok()).toBeTruthy();
  const previewBody = await previewResponse.json();
  expect(previewBody.preview).toBe(true);
  expect(previewBody.resulting_status).toBe('retrying');
  expect(previewBody.resulting_queue_state).toBe('retry_pending');
  expect(previewBody.command_outcome).toBe('preview');

  const executePayload = {
    ...previewPayload,
    confirm: true,
    request_id: 'e2e-biller-exec-1',
  };
  const executeResponse1 = await request.post(
    `${BACKEND_BASE}/v1/cases/BILLER-1002/action`,
    {
      headers: BILLER_HEADERS,
      data: executePayload,
    },
  );
  expect(executeResponse1.ok()).toBeTruthy();
  const executeBody1 = await executeResponse1.json();
  expect(executeBody1.preview).toBe(false);
  expect(executeBody1.command_outcome).toBe('executed');
  expect(executeBody1.replayed).toBe(false);

  const executeResponse2 = await request.post(
    `${BACKEND_BASE}/v1/cases/BILLER-1002/action`,
    {
      headers: BILLER_HEADERS,
      data: executePayload,
    },
  );
  expect(executeResponse2.ok()).toBeTruthy();
  const executeBody2 = await executeResponse2.json();
  expect(executeBody2.replayed).toBe(true);
  expect(executeBody2.command_id).toBe(executeBody1.command_id);
});

test('biller search enforces role boundary', async () => {
  const payerHeaders = {
    'x-role': 'payer',
    'x-actor-id': 'e2e-blocked-biller',
  };
  const denied = await fetch(`${BACKEND_BASE}/v1/biller/search?limit=20`, {
    headers: payerHeaders,
  });
  expect(denied.status).toBe(403);
  const body = await denied.json();
  expect(body.error.code).toBe('FORBIDDEN');
  expect(body.error.next_allowed_actions).toContain('biller');
});
