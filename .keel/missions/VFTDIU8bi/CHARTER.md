# Mission UX Conversational Billing Interfaces - Charter

Archetype: Strategic

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Ship a React-based conversational workflow for medical billers to search patients, open work queues, and execute claim actions through guided chat + deterministic actions. | board: VFTDJfTfN |
| MG-02 | Ship a React-based payer-facing interface for denial triage, appeal workflow, and evidence/claim timeline reconciliation. | board: VFTDJfofO |
| MG-03 | Deliver a Rust API layer that serves the React applications with Transit-driven workflows and cloud-stored workflow/case data. | board: VFTDMW0m2 |

## Constraints

- All conversational actions must map to explicit backend commands with trace IDs and deterministic output.
- PII and payer-sensitive artifacts require least-privilege access and audit logging for every action.
- Interfaces are read-write only through explicit user confirmation and must surface risk/impact before state transitions.
- API outputs must be replay-safe for the same request input and include stable request IDs.
- No direct payer policy changes in this mission; this mission defines interface and workflow APIs only.

## Halting Rules

- DO NOT halt while any MG-* goal has unfinished board work
- HALT when all MG-* goals with `board:` verification are satisfied
- YIELD to human when only `metric:` or `manual:` goals remain
