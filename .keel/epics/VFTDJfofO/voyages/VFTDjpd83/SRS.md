# Payer Conversational Triage And Appeals - SRS

## Summary

Epic: VFTDJfofO
Goal: Improve payer denial triage and appeal workflow through a conversational operator interface.

## Scope

### In Scope

- [SCOPE-01] Provide a payer-only conversational workspace for searching and triaging denials.
- [SCOPE-02] Present denial timeline, evidence links, and next-step recommendations.
- [SCOPE-03] Generate structured appeal/response packages with explicit state transitions and immutable artifacts.

### Out of Scope

- [SCOPE-04] Automated adjudication or EDI exchange generation.
- [SCOPE-05] Full patient billing policy configuration changes.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | A payer operator can search and filter denial cases by payer, denial reason, status, and date/aging buckets in one conversational workflow. | SCOPE-01 | FR-01 | manual |
| SRS-02 | A triage case view displays denial history, action history, cloud evidence references, and conversation recommendations from policy-safe templates. | SCOPE-02 | FR-02 | manual |
| SRS-03 | The system can generate and update appeal draft/response package states with immutable links, supporting edit/review/submit checkpoints. | SCOPE-03 | FR-03 | manual |
| SRS-04 | Every state change emits rationale metadata and explicit actor identifiers for compliance review. | SCOPE-02 | FR-02 | manual |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Conversational query and action rendering for denial triage should remain below 800ms p95 under normal load. | SCOPE-01 | NFR-01 | manual |
| SRS-NFR-02 | Every reviewable action and state mutation must be logged with operator identity, reason, and immutable references. | SCOPE-03 | NFR-02 | manual |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
