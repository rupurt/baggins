# Biller Conversational Interface - Software Design Description

> Enable conversational patient search, queue routing, and action execution for biller workflows.

**SRS:** [SRS.md](SRS.md)

## Overview

Evidence-aware biller operations are delivered through a conversational frontend contract against a deterministic backend. The voyage introduces a query/read layer for search and case aggregation, a command orchestration layer for action execution, and an audit-first response envelope shared across both biller and payer workflows.

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
| Cloud workflow API | internal | Search and mutate workflow state used by conversational interfaces. | `/api/v1/...` (Rust services defined in this mission). |
| Transit command bus | internal | Deterministic command execution and replay-safe state transitions. | Internal event contracts defined in sibling APIs. |
| Auth provider | internal | Role-based access for biller operators. | Workspace OAuth / JWT strategy. |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Command model | Explicit command objects with preview + confirm UX | Prevents accidental mutations and supports auditability. |
| Action safety | Show impact summary before execution | Meets compliance and operator confidence constraints. |
| Search model | Server-side indexed search with deterministic sorting | Keeps UX predictable and testable. |

## Architecture

1. **Conversation Layer** captures intent, renders quick actions, and tracks active workflow context.
2. **Search Layer** exposes paginated patient/case lookup and queue retrieval with stable identifiers.
3. **Dashboard Layer** hydrates timeline, next-best-actions, and risk hints from case state.
4. **Action Layer** validates command shape, enforces confirmation policy, and forwards commands to Transit adapters.
5. **Trace Layer** writes immutable action receipts for every confirmation path.

## Components

| Component | Purpose |
|-----------|---------|
| Search Query Service | Provides patient/case lookups and queue filtering endpoints with deterministic sort and cursor support. |
| Case Context Aggregator | Combines claim metadata, timeline state, and pending task list into one payload. |
| Conversational Command Planner | Maps parser intent into a small action set with required preconditions and risks. |
| Command Execution Adapter | Calls shared API mutations, enforces confirm flags, and returns canonical receipts. |
| Audit Event Writer | Emits immutable action trace entries with decision context and actor identity. |

## Interfaces

| Interface | Method | Contract | Consumers |
|-----------|--------|----------|-----------|
| `/v1/biller/search` | GET | Search request with query, filters, and pagination; returns ranked match list. | Biller conversational UI |
| `/v1/biller/cases/{case_id}` | GET | Case aggregate with queue metadata, timeline, and next action suggestions. | Biller conversational UI |
| `/v1/biller/cases/{case_id}/command` | POST | Command envelope: `{command, params, confirm, idempotency_key}`; response with receipt and trace id. | Biller conversational UI |

## Data Flow

1. UI submits search/query request.
2. Search service applies authorization scope and returns ranked patient/case list.
3. UI selects case; dashboard service fetches case context and pending transitions.
4. Biller selects command; planner renders risk/impact; user confirms or declines.
5. Execution adapter sends command to shared API and receives command receipt.
6. Audit writer records immutable command trail and returns trace id back to UI.

## Error Handling

<!-- What can go wrong, how we detect it, how we recover -->

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Unauthorized user context | Permission check failure | Return access denied with required scope | Prompt for role re-auth or role update by admin |
| No command confirmation | Missing confirmation flag in write request | Return `MISSING_CONFIRMATION` and no state change | Re-ask confirmation in chat UI |
| Duplicate action submit | Duplicate idempotency key | Return prior receipt with status `idempotent_replay` | Client should reuse prior result |
