# Transit Schema Contracting — Brief

## Hypothesis

If transit event schema is finalized before service work starts, downstream missions will be reliable, auditable, and policy-safe.

We can reduce claim rejection and replay complexity by standardizing schema versioning and event ownership up front.

## Problem Space

Current work has no canonical event contracts for encounter/context, claim lifecycle, or payment feedback. Ad hoc fields would create non-deterministic verifier behavior.

Research is needed to define and freeze minimal payload contracts for Mission 0 through Mission 2.

## Success Criteria

- [ ] A versioned schema package draft exists for encounter, claims, status, payment, and feedback events.
- [ ] Ownership, idempotency, and dead-letter handling are defined per event class.
- [ ] Every event includes verifier trace metadata and routing metadata for `paddles` + `sift` audits.

## Open Questions

- What event fields are mandatory for recursive verifier and audit trails?
- How should schema evolution and compatibility be managed across missions?
