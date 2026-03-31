---
id: VFSyOLOaZ
---

# Transit Schema Contracting — Assessment

## Scoring Factors

| Factor | Score | Rationale |
|--------|-------|-----------|
| Impact | 3 | Expected value delivered if successful |
| Confidence | 3 | Certainty we can achieve the outcome |
| Effort | 3 | Resources and time required |
| Risk | 3 | Probability of negative outcomes |

*Scores range from 1-5:*
- 1 = Very Low
- 2 = Low
- 3 = Medium
- 4 = High
- 5 = Very High

## Findings

- A stable schema contract is needed early to keep mission state, verifier inputs, and audit trails deterministic across services [SRC-01].

## Opportunity Cost

Not defining contract metadata early increases replay risk, dead-letter ambiguity, and retriable payout-loop failures in later payer-facing missions [SRC-03].

## Dependencies

- Requires alignment with the existing event fabric and verifier flow in architecture docs before service implementation [SRC-03].

## Alternatives Considered

- Proceed with v1 contract baseline now vs. infer schema ad hoc during missions; ad hoc evolution creates irreversible drift risk [SRC-03].

## Recommendation

[x] Proceed → convert to epic [SRC-01]
[ ] Park → revisit later [SRC-02]
[ ] Decline → document learnings [SRC-03]
