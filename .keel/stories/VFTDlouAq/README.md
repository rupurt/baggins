---
# system-managed
id: VFTDlouAq
status: icebox
created_at: 2026-03-31T11:18:11
updated_at: 2026-03-31T11:18:11
# authored
title: Build Biller Conversational Search And Case Dashboard
type: feat
operator-signal:
scope: VFTDJfTfN/VFTDjPO70
index: 1
---

# Build Biller Conversational Search And Case Dashboard

## Summary

Deliver a biller search and case dashboard flow in the conversational interface so operators can find, open, and understand workload context without switching screens.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] The search interface accepts patient name, MRN, and claim identifiers, returns deterministic ranked results, and supports cursor pagination. <!-- verify: manual, SRS-01:start:end -->
- [ ] [SRS-02/AC-01] Opening a result displays an aggregated case dashboard with queue state, workflow status, unresolved actions, and risk-aware quick hints. <!-- verify: manual, SRS-02:start:end -->
- [ ] [SRS-02/AC-02] Dashboard data includes last 20 timeline events and links to current cloud artifacts for operator review. <!-- verify: manual, SRS-02:start:end -->
- [ ] [SRS-04/AC-01] Attempting to open a case without action confirmation context must surface an explicit non-mutating preview and confirmation gate. <!-- verify: manual, SRS-04:start:end -->
