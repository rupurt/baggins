# Mission Two Medical Coding And Evidence Extraction Agent - Charter

Archetype: Strategic

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Transform normalized clinical intake evidence into deterministic medical coding candidates with confidence and auditable lineage. | board: VFT877lcp |
| MG-02 | Validate and route unsafe or ambiguous coding candidates with explicit reject/retry outcomes and traceable reason codes. | board: VFT8782cq |
| MG-03 | Emit auditable coding handoff state and escalation artifacts for downstream claim assembly and reconciliation. | board: VFT878Ncr |

## Constraints

- Coding suggestions cannot modify payer-submission policy directly in Mission Two; this mission only prepares structured coding evidence.
- Evidence artifacts must preserve deterministic IDs, schema versions, and reason codes for every rejection.
- No production integrations are allowed until evidence handoff and manual escalation workflows are verifiable end-to-end.
- All coding candidate generation and validation logic must remain deterministic and replay-safe.

## Halting Rules

- DO NOT halt while any MG-* goal with `board:` verification is unfinished.
- HALT when all MG-* goals with `board:` verification are complete and auditable.
- YIELD for manual decisioning when only non-board or advisory tasks remain.
