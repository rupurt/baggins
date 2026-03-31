---
# system-managed
id: VFTDmDsDN
status: done
created_at: 2026-03-31T11:18:12
updated_at: 2026-03-31T11:25:45
# authored
title: Generate Conversational Appeal Drafts And Response Packages
type: feat
operator-signal:
scope: VFTDJfofO/VFTDjpd83
index: 2
started_at: 2026-03-31T11:22:01
submitted_at: 2026-03-31T11:25:43
completed_at: 2026-03-31T11:25:45
---

# Generate Conversational Appeal Drafts And Response Packages

## Summary

Create deterministic, editable conversational appeal and response package generation that emits auditable artifacts and supports payer-ready handoff.

## Acceptance Criteria

- [x] [SRS-03/AC-01] Operators can generate an appeal/response draft from case context using at least two policy-safe templates. <!-- verify: manual, SRS-03:start:end -->
- [x] [SRS-03/AC-02] Generated packages include required claim fields, denial reason, payer context, and evidence references. <!-- verify: manual, SRS-03:start:end -->
- [x] [SRS-03/AC-03] Package state transitions support draft → review → ready → submitted and store immutable status history. <!-- verify: manual, SRS-03:start:end -->
- [x] [SRS-04/AC-01] Failed transitions emit structured reasons that preserve the last stable package payload. <!-- verify: manual, SRS-04:start:end -->
