---
# system-managed
id: VFT87TNgB
status: done
created_at: 2026-03-31T10:55:44
updated_at: 2026-03-31T10:58:22
# authored
title: Validate Coding Candidates And Emit Reject Retries
type: feat
operator-signal:
scope: VFT86RtYc/VFT8782cq
index: 1
started_at: 2026-03-31T10:58:04
submitted_at: 2026-03-31T10:58:20
completed_at: 2026-03-31T10:58:22
---

# Validate Coding Candidates And Emit Reject Retries

## Summary

Validate extracted candidates against safety and business rules, emitting deterministic reject, retry, and escalation outcomes with machine-readable reason codes.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Validate each extracted coding candidate and route invalid/unsafe candidates to documented reject or retry outcomes with reason codes. <!-- verify: manual, SRS-01:start:end, proof: ac-1.log, ac-2.log -->
