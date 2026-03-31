---
# system-managed
id: VFT87TNgB
status: backlog
created_at: 2026-03-31T10:55:44
updated_at: 2026-03-31T10:56:31
# authored
title: Validate Coding Candidates And Emit Reject Retries
type: feat
operator-signal:
scope: VFT86RtYc/VFT8782cq
index: 1
---

# Validate Coding Candidates And Emit Reject Retries

## Summary

Validate extracted candidates against safety and business rules, emitting deterministic reject, retry, and escalation outcomes with machine-readable reason codes.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Validate each extracted coding candidate and route invalid/unsafe candidates to documented reject or retry outcomes with reason codes. <!-- verify: manual, SRS-01:start:end -->
