# Payer Conversational Triage And Appeals - Software Design Description

> Improve payer denial triage and appeal workflow through a conversational operator interface.

**SRS:** [SRS.md](SRS.md)

## Overview

Payer triage is built as a deterministic command-and-query surface backed by the shared API and cloud case data. The voyage binds conversation controls to explicit review workflows and evidence-linked package generation.

## Context & Boundaries

<!-- What's in scope, what's out of scope, external actors/systems we interact with -->

```
┌─────────────────────────────────────────┐
│              This Voyage                │
│                                         │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐ │
│  │         │  │         │  │         │ │
│  └─────────┘  └─────────┘  └─────────┘ │
└─────────────────────────────────────────┘
        ↑               ↑
   [External]      [External]
```

## Dependencies

<!-- External systems, libraries, services this design relies on -->

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Shared workflow API | internal | Search, case retrieval, and action mutation endpoints. | REST routes for claimant/denial workflows. |
| Transit event schema | internal | Replay-safe state transition records and outcome payloads. | Internal contracts for case, denial, appeal states. |
| Cloud object storage | internal | Store evidence artifacts and appeal package attachments. | Secure object buckets configured by environment. |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Triage model | Deterministic recommendation templates + explicit approval | Keeps decisions auditable and avoids hidden AI behavior in mutating path. |
| Package lifecycle | Draft → review → ready → submitted states | Supports manual quality checks before outbound submissions. |
| Evidence anchoring | Evidence IDs required on every generated package | Enables downstream proof and payer auditability. |

## Architecture

1. **Queue Service** indexes denial claims and applies filters/pinned sort for operator views.
2. **Triage Timeline Service** hydrates case details from cloud case store and past events.
3. **Suggestion Engine** produces deterministic next-step recommendations based on denial codes and policy-safe templates.
4. **Package Builder** assembles appeal payloads with evidence references and tracks state transitions.
5. **Audit Layer** emits immutable logs for each review action and package mutation.

## Components

| Component | Purpose |
|-----------|---------|
| Denial Queue Index | Filtered retrieval by payer, reason, status, and age. |
| Triage Timeline Aggregator | Combines denials, notes, and workflow events into one case narrative. |
| Recommendation Composer | Maps denial metadata to safe operator prompts and suggested actions. |
| Response Package Builder | Creates draft/response artifacts and manages versioned state transitions. |
| Case Audit Writer | Persists operator action details and correlates to trace IDs. |

## Interfaces

| Interface | Method | Contract | Consumers |
|-----------|--------|----------|-----------|
| `/v1/payer/denials/search` | GET | Filter query by payer, reason, status, and date window; returns case list. | Payer conversational UI |
| `/v1/payer/denials/{case_id}` | GET | Denial case aggregate + timeline + evidence references. | Payer conversational UI |
| `/v1/payer/denials/{case_id}/responses` | POST | Create or update appeal package with template + evidence attachments. | Payer conversational UI |

## Data Flow

1. Operator opens payer denial workspace and submits filter/search intent.
2. Queue service returns ranked cases; selected case is hydrated through timeline and suggestion services.
3. Operator approves a recommended action and opens response package flow.
4. Package builder validates required fields/evidence links and stores draft state.
5. Mutations are emitted through API gateway to Transit and persisted to cloud store.
6. Audit layer writes immutable action and review entries with trace IDs.

## Error Handling

<!-- What can go wrong, how we detect it, how we recover -->

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Empty queue for query | No matching cases returned | Return empty state with actionable filter suggestions | Allow user to broaden date/payer filters |
| Missing required evidence | Package validation failure | Return structured reason list and blocking fields | Collect evidence, then re-run package generation |
| Concurrent package edits | Version conflict on case state | Return conflict marker with latest state snapshot | Refresh timeline and merge manually |
