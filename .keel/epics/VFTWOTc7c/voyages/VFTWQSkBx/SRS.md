# Denial Risk Preflight And Evidence Readiness - SRS

## Summary

Epic: VFTWOTc7c
Goal: Predict denial and escalation risk before claim assembly so reviewers can resolve high-probability issues pre-submission.

## Scope

### In Scope

- [SCOPE-01] Deterministic denial-risk scoring from claim, policy, and member context.
- [SCOPE-01] Blocker inventory generation with normalized reason codes and evidence links.
- [SCOPE-01] Replay-safe payload emission for pre-submission reviewer queue ingestion.

### Out of Scope

- [SCOPE-02] Direct claim submission workflow modifications.
- [SCOPE-02] Automatic application of policy edits without explicit human review.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Produce denial-risk payloads that include versioned policy references, risk score, and blocker inventory. | SCOPE-01 | FR-01 | manual |
| SRS-02 | Classify each blocker with reason codes and handoff states so reviewers can deterministically route action. | SCOPE-01 | FR-01 | manual |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Risk scoring and blocker payload generation are replay-safe and idempotent for identical input snapshots. | SCOPE-01 | NFR-01 | manual |
| SRS-NFR-02 | Emit telemetry for risk-calculation outcomes, misses, and blocked/reroute outcomes. | SCOPE-01 | NFR-02 | manual |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
