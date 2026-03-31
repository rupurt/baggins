---
# system-managed
id: VFTJi1N2S
index: 1
status: proposed
decided_at: 2026-03-31T11:41:46
supersedes: []
superseded-by: null
# authored
title: Pii Anonymized Patient Identifiers Via Derived Mrn Token
context: Patient records should not retain raw MRN/PII at rest. The system needs a stable identifier for matching and searching workflows while maintaining compliance.
applies-to:
  - src/server.rs
  - .keel/adrs/VFTJi1N2S/README.md
---

# Pii Anonymized Patient Identifiers Via Derived Mrn Token

## Status

**Proposed** — Awaiting human acceptance. Work in governed context is blocked.

## Context

Current biller workflows require patient lookups and continuity across cases, but raw MRNs are PII and should not be persisted.
The system needs a stable identifier to support search and matching without storing direct patient identifiers.

## Decision

Persist only a derived patient token and never persist raw MRN values.

- Compute a deterministic token in `src/server.rs` with `anonymize_mrn`:
  - Normalize input with `trim().to_ascii_lowercase()`.
  - Derive with `make_hash(&["baggins-anon-mrn-v1", normalized_mrn])`.
- Replace direct `mrn` persistence with `patient_token` in `BillerCase`.
- Update seeded data and biller search matching to compare `anonymize_mrn(query)` against `patient_token`.
- Preserve user-facing behavior by allowing billers to search via MRN-like input while keeping storage tokenized.

## Constraints

- **MUST:** Never persist raw MRN in case storage, seeded fixtures, or logs used for operational state.
- **MUST:** Use the same `anonymize_mrn` function for ingestion and query matching paths.
- **MUST NOT:** Introduce multiple identifier derivation schemes across handlers, routes, or adapters.
- **SHOULD:** Keep matching logic token-based and expose only non-PII patient descriptors in API responses.

## Consequences

### Positive

- Preserves patient record linking and coherence while reducing PII retention risk.
- Keeps biller MRN lookup behavior functionally intact for compatibility.
- Makes data minimization a default part of application state.

### Negative

- Derived tokens are stable and low-entropy MRNs remain potentially brute-forcible unless a secret rotation strategy is added later.
- Re-identification workflows must rely on an approved external source of truth rather than internal reverse lookups.

### Neutral

- Payer search behavior is unaffected because no MRN-based search exists in payer handlers in the current implementation.

## Verification

| Check | Type | Description |
|-------|------|-------------|
| `biller_case_stores_derived_patient_token_only` | automated | Verifies seeded biller cases store a derived `patient_token` and not raw MRN. |
| `anonymize_mrn_is_consistent` | automated | Verifies normalization and deterministic token output for equivalent MRN variants. |
| API/state review | manual | Confirm `BillerCase` and persisted payloads contain `patient_token` only. |
| Search smoke test | manual | Search `/v1/biller/search` with MRN-like values and validate results resolve via token match. |

## References

- `src/server.rs` (`anonymize_mrn`, `biller_search`, seeded fixtures, `BillerCase`).
- Mission work on PII governance and patient handling.
