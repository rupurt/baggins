# Medical Code Candidate Extraction - SRS

## Summary

Epic: VFT869hXY
Goal: Extract CPT/ICD/RVU candidates from canonical evidence with confidence and lineage metadata.

## Scope

### In Scope

- [SCOPE-01] Normalize and map evidence tokens into candidate coding payloads with deterministic ordering.
- [SCOPE-02] Preserve lineage identifiers and confidence metadata in each candidate output.

### Out of Scope

- [SCOPE-03] Payer-specific coding overrides and final payment adjudication logic.
- [SCOPE-04] New UI surfaces for claim editors.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Convert normalized clinical evidence into ranked medical coding candidate payloads with deterministic identifiers and confidence metadata. | SCOPE-01 | FR-01 | manual |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Candidate extraction should be replay-safe and produce identical outputs for identical input snapshots. | SCOPE-01 | NFR-01 | manual |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
