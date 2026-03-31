# Conversational Biller Productivity Interface - Product Requirements

## Problem Statement

Biller workflow latency from chart review to claim readiness is high due to fragmented UI surfaces.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Reduce biller-to-ready workflow latency by making claim and patient operations conversational and action-driven. | Median time from “claim opened” to “ready for validation” | 35% reduction from baseline |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Primary User | Medical billers and claim coordinators | Faster claim progression with a single interface for search, review, and action. |

## Scope

### In Scope

- [SCOPE-01] Conversational web app shell for patient/case search, case timeline, and action execution.
- [SCOPE-02] Guided workflows for validation, coding review, and submission handoff triggers.
- [SCOPE-03] Action cards that show backend rationale and risk/rules before confirmation.

### Out of Scope

- Full payer rule engine changes.
- Billing-policy pricing optimization logic.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Allow billers to search patients, retrieve case queues, and open workflows in one screen. | GOAL-01 | must | Removes navigation overhead and manual context switching. |
| FR-02 | Provide conversational commands for common claim operations (validate, retry, escalate, submit draft). | GOAL-01 | must | Lowers cognitive load while keeping action intent explicit. |
| FR-03 | Execute approved commands via Rust APIs with immutable trace IDs and action receipts. | GOAL-01 | must | Makes each workflow transition auditable and replayable. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Maintain conversational action render latency under 500ms at p95. | GOAL-01 | must | Keeps the interface feeling immediate during workflows. |
| NFR-02 | Preserve an audit trace (user, command, params, decision rationale, and trace id) for every state-changing action. | GOAL-01 | must | Required for operational review and safety. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Problem outcome | Story-level evidence in acceptance logs + manual workflow walkthrough | Story-level verification artifacts linked during execution |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Biller operators are comfortable with guided conversational action patterns. | Lower adoption and slower throughput. | Pilot feedback with staged rollout and UI telemetry. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| How much of case progression should remain manual-confirmation for compliance? | Product owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [x] Case search and workflow entry are one-step operations from the conversational interface.
- [x] At least one end-to-end conversational action (open case -> action -> audit-ready result) is implemented.
- [ ] Latency target (median claim-ready reduction 35%) is met on rollout cohort.
<!-- END SUCCESS_CRITERIA -->
