# Eligibility Preflight Coverage Intake - SRS

## Summary

Epic: VFTR11pG0
Goal: Create and normalize coverage+eligibility inputs (payer, member, and benefit state) from existing claim and policy sources into a deterministic transit preflight payload.

## Scope

### In Scope

- [SCOPE-01] Capture payer plan metadata, member coverage snapshot, and required service-type constraints for incoming claim payloads.
- [SCOPE-02] Normalize source identifiers and contract versions into a deterministic preflight payload consumed by downstream voyages.

### Out of Scope

- [SCOPE-03] Any change to payer submission or adjudication logic.
- [SCOPE-04] Long-term member re-eligibility polling and external insurer negotiation workflows.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Build a normalized coverage/eligibility intake record containing payer policy version, service context, patient eligibility state, and validation timestamp. | SCOPE-01 | FR-01 | manual |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Ensure deterministic output ordering, schema validation, and auditable identifiers for every normalized intake record. | SCOPE-02 | NFR-01 | manual |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
