---
# system-managed
id: VFTDmEBDO
status: icebox
created_at: 2026-03-31T11:18:12
updated_at: 2026-03-31T11:18:12
# authored
title: Implement Shared REST API Gateway For Biller and Payer
type: feat
operator-signal:
scope: VFTDMW0m2/VFTDjpu84
index: 1
---

# Implement Shared REST API Gateway For Biller and Payer

## Summary

Implement the shared API gateway contract and route surface used by both conversational applications for search and action operations.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Expose versioned REST routes for biller search, payer denial search, and common case retrieval in the same API base path. <!-- verify: manual, SRS-01:start:end -->
- [ ] [SRS-01/AC-02] Every route validates role and tenant context before processing and returns `403` on scope mismatch. <!-- verify: manual, SRS-04:start:end -->
- [ ] [SRS-02/AC-01] Mutating endpoints require idempotency metadata and emit command receipts with request and trace IDs. <!-- verify: manual, SRS-02:start:end -->
- [ ] [SRS-03/AC-01] Actions performed through the gateway trigger Transit command/event writes and return status transitions consistently for both UIs. <!-- verify: manual, SRS-03:start:end -->
