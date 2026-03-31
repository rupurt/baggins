# Biller Conversational Interface - SRS

## Summary

Epic: VFTDJfTfN
Goal: Enable conversational patient search, queue routing, and action execution for biller workflows.

## Scope

### In Scope

- [SCOPE-01] Provide a conversational entry point for patient and case discovery from one screen.
- [SCOPE-02] Expose case queue and timeline context in the same conversational workflow.
- [SCOPE-03] Execute biller actions only through deterministic command confirmations that emit auditable receipts.

### Out of Scope

- [SCOPE-04] New AI policy-model logic for claim pricing decisions.
- [SCOPE-05] New claims adjudication integrations outside existing Transit-driven workflows.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | A biller can search patients and cases from one entry point, with deterministic ranking, pagination, and stable identifiers. | SCOPE-01 | FR-01 | manual |
| SRS-02 | A case dashboard summarizes queue state, workflow status, last event, and recommended next actions in one conversational context. | SCOPE-02 | FR-01 | manual |
| SRS-03 | Biller actions are executed through a command model that requires explicit confirmation and returns a command receipt with trace and idempotency context. | SCOPE-03 | FR-03 | manual |
| SRS-04 | Every action preview in chat surfaces intent impact (status changes, required artifacts, and potential blockers). | SCOPE-03 | FR-02 | manual |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Conversational search and action interactions must return within 500ms p95. | SCOPE-01 | NFR-01 | manual |
| SRS-NFR-02 | All mutating command pathways must produce immutable audit trace entries with user, command, rationale, and request identifiers. | SCOPE-03 | NFR-02 | manual |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
