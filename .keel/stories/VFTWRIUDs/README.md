---
# system-managed
id: VFTWRIUDs
status: backlog
created_at: 2026-03-31T12:32:20
updated_at: 2026-03-31T12:33:13
# authored
title: Emit Deterministic Preflight Telemetry And Escalation Signals
type: chore
operator-signal:
scope: VFTWOTc7c/VFTWQSkBx
index: 2
---

# Emit Deterministic Preflight Telemetry And Escalation Signals

## Summary

Emit deterministic preflight telemetry for blocker generation, escalation state transitions, and risk-calculation fallback events.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] Classify blockers and emit reason-code handoff outcomes for each high-probability issue. <!-- verify: manual, SRS-02:start:end -->
- [ ] [SRS-NFR-01/AC-02] Emit deterministic telemetry for preflight blockers, missing-input fallbacks, and escalation transitions with schema version metadata. <!-- verify: manual, SRS-NFR-01:start:end -->
