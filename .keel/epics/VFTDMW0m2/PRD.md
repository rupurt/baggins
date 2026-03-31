# Shared Billing Workflow API Platform - Product Requirements

## Problem Statement

Multiple React interfaces need deterministic APIs for case lookup, workflow state, and Transit event orchestration across cloud-backed claim data.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Stand up a single Rust API surface that powers both biller and payer UIs with deterministic outputs and secure auth. | Ratio of cross-app requests routed through API platform | 95%+ by MVP |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Primary User | Frontend clients and claim processing services | Stable, typed APIs with predictable IDs and policy-safe mutations. |

## Scope

### In Scope

- [SCOPE-01] RESTful API endpoints for patient search, case retrieval, workflow state, and action mutation.
- [SCOPE-02] Transit-backed workflow orchestration interfaces for handoffs and approvals.
- [SCOPE-03] Cloud-data integration hooks for persistent cases, notes, and evidence references.

### Out of Scope

- [SCOPE-04] Rebuilding upstream payer adjudication engines.
- [SCOPE-05] Long-term data warehouse model migrations.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Expose secure APIs for patient search, workflow retrieval, and action execution used by both UIs. | GOAL-01 | must | Unifies frontend behavior and reduces duplicate integrations. |
| FR-02 | Include deterministic request IDs, idempotency controls, and replay-safe responses for all state transitions. | GOAL-01 | must | Prevents duplicate actions and ensures auditability. |
| FR-03 | Bridge Transit command/event channels with cloud case data stores and emit status telemetry. | GOAL-01 | must | Enables mixed operational and long-term state consistency. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Provide p95 API response latency under 300ms for read endpoints at typical workloads. | GOAL-01 | must | Preserves conversational UX responsiveness. |
| NFR-02 | Implement structured audit and tracing for every mutating API call. | GOAL-01 | must | Required for reviewability and incident response. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Problem outcome | Integration tests plus manual API walkthroughs with both React applications | Story-level verification artifacts linked during execution |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Existing Transit interfaces can be consumed with stable contract wrappers. | Extra implementation cost and design churn. | Dry-run integration tests with current Mission 3/4 outputs. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Do we need GraphQL, REST, or both to meet UI and partner integration needs? | Platform owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [x] Shared API handles patient/case search and workflow state retrieval for both interfaces.
- [x] All mutating actions produce stable trace IDs and idempotent behavior.
- [ ] 95% of cross-app traffic is served by shared API by end of MVP rollout.
<!-- END SUCCESS_CRITERIA -->
