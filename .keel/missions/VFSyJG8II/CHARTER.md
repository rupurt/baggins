# Mission Three Claim Assembly And Validation Service - Charter

Archetype: Strategic

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Define deterministic claim assembly and validation workflows that produce replay-safe payloads for biller/payer transitions. | board: VFTMy4JOd |

## Constraints

- No raw or unsanitized patient identifiers may be introduced in claim-assembly payloads; use existing derived identifiers.
- All action transitions in this mission must remain replay-safe through request-level idempotency.
- No direct payer submission logic changes; this mission only defines assembly and validation state transitions.
- New outputs must include trace IDs and transition state for downstream audit consumers.

## Halting Rules

- DO NOT halt while any MG-* board-linked goal is incomplete or unachieved.
- HALT when all MG-* goals with `board:` verification are complete and auditable.
- YIELD to human when only `metric:` or `manual:` verification items remain.
