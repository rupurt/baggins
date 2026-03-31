---
# system-managed
id: VFTDlpDAr
status: done
created_at: 2026-03-31T11:18:11
updated_at: 2026-03-31T11:25:45
# authored
title: Implement Actionable Chat Commands For Claim Workflow
type: feat
operator-signal:
scope: VFTDJfTfN/VFTDjPO70
index: 2
started_at: 2026-03-31T11:25:26
submitted_at: 2026-03-31T11:25:43
completed_at: 2026-03-31T11:25:45
---

# Implement Actionable Chat Commands For Claim Workflow

## Summary

Deliver command-driven claim workflow actions in chat so billers can execute high-frequency operations with explicit confirmation and deterministic receipts.

## Acceptance Criteria

- [x] [SRS-03/AC-01] The conversation layer accepts at least the commands: validate, retry, escalate, hold, and submit_draft, and sends a normalized command envelope to the backend. <!-- verify: manual, SRS-03:start:end -->
- [x] [SRS-03/AC-02] The system must display a command preview showing expected downstream state and blocked conditions before execution. <!-- verify: manual, SRS-03:start:end -->
- [x] [SRS-03/AC-03] Executed commands return a command receipt containing `command_id`, `trace_id`, and resulting transition id. <!-- verify: manual, SRS-03:start:end -->
- [x] [SRS-04/AC-01] Validation failures return structured reasons and actionable next steps instead of mutating state. <!-- verify: manual, SRS-04:start:end -->
