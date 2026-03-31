# Unified Billing Workflow API Platform - SRS

## Summary

Epic: VFTDMW0m2
Goal: Expose shared deterministic APIs for conversational UIs and Transit workflow orchestration.

## Scope

### In Scope

- [SCOPE-01] Ship REST API boundaries for search, case retrieval, and action execution.
- [SCOPE-02] Provide deterministic request/response contracts for both biller and payer web interfaces.
- [SCOPE-03] Bridge action state transitions to Transit command/event handling with idempotent semantics.

### Out of Scope

- [SCOPE-04] GraphQL federation redesign or partner API partner marketplaces.
- [SCOPE-05] Rebuild of upstream adjudication engines or payer policy parsers.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Provide versioned REST endpoints for patient search, denial/case retrieval, and action execution consumed by both biller and payer UIs. | SCOPE-01 | FR-01 | manual |
| SRS-02 | Every mutating request includes trace/context metadata and enforces idempotency keys for replay-safe outcomes. | SCOPE-03 | FR-02 | manual |
| SRS-03 | API layer publishes/read-translates workflow state between cloud persistence and Transit command/event streams. | SCOPE-03 | FR-03 | manual |
| SRS-04 | Role-aware endpoints enforce tenant and role boundaries for biller/payer callers. | SCOPE-02 | FR-01 | manual |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Read endpoints must return under 300ms p95 for typical MVP workloads. | SCOPE-01 | NFR-01 | manual |
| SRS-NFR-02 | All mutating calls emit structured trace/audit records with action outcome and idempotent replay behavior. | SCOPE-02 | NFR-02 | manual |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
