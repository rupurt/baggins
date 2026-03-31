# Claim Assembly Orchestration - Software Design Description

> Build validated claim assembly and mutation API and deterministic queue-state transitions for payer/biller workflows.

**SRS:** [SRS.md](SRS.md)

## Overview

Build the first-pass claim-assembly backend surface by combining biller and payer case data into one deterministic payload model, then provide the command-level validation and transition contract used by both conversational interfaces.

## Context & Boundaries

The voyage operates in the claim assembly service layer and consumes seeded biller/payer case state.
It does not modify payer adjudication policy or upstream adjudication engines.

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
| Rust server API | internal | Case retrieval and action endpoints for both personas | Existing axum service and request handlers in this repository |
| Transit event contract | internal | Keeps command outcomes and payload transitions consistent across services | Shared schema contracts used by mission workflows |
| Cloud workflow store | internal | Stores case snapshots and evidence for handoff state | Mission-scoped persistence layer |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| State boundary | Use current in-memory case aggregates and command transitions as planning boundary | Keeps scope to mission-three and validates deterministic transitions before introducing persistence changes |
| Determinism | Assemble payload from stable ordered fields and explicit queue-state naming | Prevents replay drift and helps e2e assertions |

## Architecture
1. **Assembly service** receives a case identifier and selected role context.
2. **Policy adapter** validates role constraints and allowed transitions.
3. **Transition engine** computes resulting status/queue state and command outcome.
4. **Payload composer** emits a deterministic assembled payload and audit context.

## Components

| Component | Purpose | Interface |
|-----------|---------|-----------|
| Case Loader | Loads biller and payer case slices for a single request | Case lookup + ownership checks |
| Transition Engine | Applies deterministic validation and status rules | Structured command/state transition service |
| Payload Composer | Produces deterministic claim assembly artifact | JSON response envelope for UI/API consumers |
| Audit Sink | Captures request identity and transition IDs | Request metadata and replay identifiers |

## Interfaces
| Interface | Method | Contract | Notes |
|-----------|--------|----------|-------|
| `/v1/cases/{case_id}` | GET | Returns enriched claim assembly snapshot and transition readiness metadata. | Must avoid MRN leakage by using derived tokens only. |
| `/v1/cases/{case_id}/action` | POST | Executes or previews validated transitions with idempotency semantics. | Returns `preview`, `executed`, and `replayed` markers. |

## Data Flow

1. API request enters with actor role and command payload.
2. Service loads target case and validates allowed operations for role.
3. Transition engine computes resulting state and queue impact deterministically.
4. Response payload is assembled with command metadata and trace identifiers.
5. Idempotency and replay metadata are persisted for repeated request handling.

## Error Handling
| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Unauthorized role | Missing or invalid `x-role` header | `403` with `FORBIDDEN` payload | Stop transition and show role-specific guidance |
| Unsupported command | Command not in persona allow-list | `400` with reason and allowed list | Present command alternatives |
| Missing dependencies | Case or required state not present | `404`/`500` depending on scope | Surface clear error and avoid implicit defaults |
