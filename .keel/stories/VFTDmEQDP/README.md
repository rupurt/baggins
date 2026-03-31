---
# system-managed
id: VFTDmEQDP
status: in-progress
created_at: 2026-03-31T11:18:12
updated_at: 2026-03-31T11:22:01
# authored
title: Add Audit, Traceability, And Idempotent Action Handling
type: feat
operator-signal:
scope: VFTDMW0m2/VFTDjpu84
index: 2
started_at: 2026-03-31T11:22:01
---

# Add Audit, Traceability, And Idempotent Action Handling

## Summary

Add idempotent execution guarantees and auditability to all action mutations so conversational workflows remain deterministic and recoverable across retries.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] Mutating endpoints reject duplicate action submissions by idempotency key and return prior result when replayed. <!-- verify: manual, SRS-02:start:end -->
- [ ] [SRS-02/AC-02] All mutations persist immutable audit records with user, actor role, command, params hash, and outcome code. <!-- verify: manual, SRS-02:start:end -->
- [ ] [SRS-02/AC-03] Failed transitions produce structured error envelopes with recoverable or terminal state flags and next allowed actions. <!-- verify: manual, SRS-02:start:end -->
- [ ] [SRS-03/AC-01] Actions performed through the gateway trigger Transit command/event writes and return status transitions consistently for both UIs. <!-- verify: manual, SRS-03:start:end -->
- [ ] [SRS-04/AC-01] Every route validates role and tenant context before processing and returns `403` on scope mismatch. <!-- verify: manual, SRS-04:start:end -->
- [ ] [SRS-04/AC-02] Every route enforces role-aware boundary checks for biller/payer callers before state transitions. <!-- verify: manual, SRS-04:start:end -->
- [ ] [SRS-NFR-02/AC-01] Audit trail and trace ids are retrievable through a dedicated read path for manual review and incident investigation. <!-- verify: manual, SRS-NFR-02:start:end -->
