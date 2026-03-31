# Foundation Guardrail Baseline — Brief

## Hypothesis

Without explicit foundational conventions, schema contracts, and local compliance constraints, mission delivery will drift and risk non-traceable payout decisions.

Define a baseline now that prevents future drift and unlocks safe verifier automation.

## Problem Space

Baggins starts with no authored workspace conventions beyond initial repo scaffolding. The mission risks:
- inconsistent rust service structure before mission work begins;
- unclear evidence and audit expectations across missions;
- missing minimum compliance boundaries for synthetic data and payer-safe behavior.

## Success Criteria

- [ ] Baseline repo conventions are authored (`services/` layout, governance docs, and configuration policy) and approved for Mission 0 execution.
- [ ] Mission 0 and future missions share a single verifier baseline contract referencing `transit`, `paddles`, and `sift`.
- [ ] Mission 0 can reference completed audit/safety checkpoints via board-logged artifacts rather than informal assumptions.

## Open Questions

- Which repository conventions must be mandatory before any service implementation starts?
- What evidence and logging schema is sufficient for mission-level traceability?
