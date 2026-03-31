# Transit Schema Contracting - Product Requirements

> If transit event schema is finalized before service work starts, downstream missions will be reliable, auditable, and policy-safe.

We can reduce claim rejection and replay complexity by standardizing schema versioning and event ownership up front.

## Problem Statement

Current work has no canonical event contracts for encounter/context, claim lifecycle, or payment feedback. Ad hoc fields would create non-deterministic verifier behavior.

Research is needed to define and freeze minimal payload contracts for Mission 0 through Mission 2.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Validate bearing recommendation in delivery flow | Adoption signal | Initial rollout complete |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Product/Delivery Owner | Coordinates planning and execution | Reliable strategic direction |

## Scope

### In Scope

- [SCOPE-01] Define and version initial transit schema contracts with verifier trace metadata and ownership rules.

### Out of Scope

- [SCOPE-02] Broad claims or payer-specific feature work beyond the baseline schema contract.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Implement the core user workflow identified in bearing research. | GOAL-01 | must | Converts research recommendation into executable product capability. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Ensure deterministic behavior and operational visibility for the delivered workflow. | GOAL-01 | must | Keeps delivery safe and auditable during rollout. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

- Prove functional behavior through story-level verification evidence mapped to voyage requirements.
- Validate non-functional posture with operational checks and documented artifacts.

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Bearing findings reflect current user needs | Scope may need re-planning | Re-check feedback during first voyage |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| What event fields are mandatory for recursive verifier and audit trails? | Planner | Open |
| How should schema evolution and compatibility be managed across missions? | Planner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] A versioned schema package draft exists for encounter, claims, status, payment, and feedback events.
- [ ] Ownership, idempotency, and dead-letter handling are defined per event class.
- [ ] Every event includes verifier trace metadata and routing metadata for `paddles` + `sift` audits.
<!-- END SUCCESS_CRITERIA -->

## Research Analysis

*From bearing assessment:*

## Findings


- A stable schema contract is needed early to keep mission state, verifier inputs, and audit trails deterministic across services [SRC-01].


## Opportunity Cost


Not defining contract metadata early increases replay risk, dead-letter ambiguity, and retriable payout-loop failures in later payer-facing missions [SRC-03].


## Dependencies


- Requires alignment with the existing event fabric and verifier flow in architecture docs before service implementation [SRC-03].


## Alternatives Considered


- Proceed with v1 contract baseline now vs. infer schema ad hoc during missions; ad hoc evolution creates irreversible drift risk [SRC-03].

## Research Provenance

*Source records from bearing evidence:*

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | manual | repo-internal | /README.md | 2026-03-31 | 2026-03-31 | high | high | Transit topics and future services are already enumerated for encounter/claim/payment events. |
| SRC-02 | manual | repo-internal | /ARCHITECTURE.md | 2026-03-31 | 2026-03-31 | high | high | Transit and schema contracts are defined as part of the service architecture and verifier path. |
| SRC-03 | manual | repo-internal | /CODE_WALKTHROUGH.md | 2026-03-31 | 2026-03-31 | high | high | State transition and event ownership expectations are described for implementation handoff. |

---

*This PRD was seeded from bearing `VFSyOLOaZ`. See `bearings/VFSyOLOaZ/` for original research.*
