# Unified Billing Workflow API Platform - Software Design Description

> Expose shared deterministic APIs for conversational UIs and Transit workflow orchestration.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage defines the shared API foundation that normalizes request handling for both React surfaces. It provides a single command bus and trace middleware, with separate read models for biller and payer consumers built on shared case entities.

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
| Rust web runtime | internal | Handle secure HTTP API serving for both frontends. | axum/warp-compatible stack in repository. |
| Transit adapters | internal | Encode/decode command/event payloads and enforce event schema compatibility. | Mission transit contracts. |
| Cloud case store | internal | Persist workflow cases, notes, and evidence index records. | Shared data store for workflows. |
| AuthN/AuthZ service | internal | Verify roles and permissions for biller/payer APIs. | Workspace auth middleware. |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Transport | REST-first API with stable versioned paths | Fast implementation and easier frontend integration. |
| Replay strategy | Idempotency keys with dedup table per actor+command | Prevents duplicate financial state transitions. |
| Contract discipline | Schema strict JSON responses and explicit pagination | Supports deterministic rendering across UIs. |

## Architecture

1. **HTTP Gateway** validates auth, throttling, version headers, and request schema.
2. **Route Modules** resolve biller and payer query/mutation contracts.
3. **Application Service Layer** applies policy checks, command shaping, and state-machine guards.
4. **Transit Bridge** publishes commands/events and normalizes responses for API consumers.
5. **Persistence Adapter** syncs state and case metadata with cloud storage.
6. **Observability Layer** appends trace ids and command receipts for all writes.

## Components

| Component | Purpose |
|-----------|---------|
| API Router | Exposes versioned biller/payer endpoints and request/response schemas. |
| Policy Guard | Enforces role and tenant permissions before any mutation. |
| Command Orchestrator | Converts API actions to Transit command envelopes and tracks outcomes. |
| Idempotency Store | Rejects duplicate command submissions and returns canonical replay responses. |
| Workflow Data Access | Loads and persists case/patient/denial data for both UI surfaces. |
| Audit Sink | Captures action metadata and emits traceable action IDs. |

## Interfaces

| Interface | Method | Contract | Notes |
|-----------|--------|----------|-------|
| `/v1/biller/search` | GET | Query patients/cases; supports `q`, `status`, `owner`, `limit`, `cursor`. | Shared read model used by biller UI. |
| `/v1/payer/denials/search` | GET | Query denial queue; supports `payer`, `reason`, `status`, `age_min`, `age_max`. | Shared read model used by payer UI. |
| `/v1/cases/{case_id}` | GET | Returns case aggregate with workflow state and last 100 events. | Common consumer for both UIs. |
| `/v1/cases/{case_id}/action` | POST | Accepts `{command, params, confirm, request_id, idempotency_key}`. | Writes via Transit bridge. |

## Data Flow

1. Frontend calls versioned search/retrieve API endpoints under `/v1/...`.
2. API gateway validates auth, schema, and idempotency headers where applicable.
3. Application service builds a normalized command or query model and performs policy checks.
4. Mutating calls are emitted to Transit and persisted to case store in a single coordinated path.
5. Response includes command outcome, resulting case state snapshot, and trace identifiers.
6. Audit sink records action payload, actor identity, and error/transition state for replay and manual review.

## Error Handling

<!-- What can go wrong, how we detect it, how we recover -->

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Duplicate idempotency key | Existing completed action exists for actor+scope+command hash | Return original response with replay marker `idempotent_duplicate` | Preserve prior state and surface prior receipt |
| Invalid command payload | JSON/schema validation failure | Return structured validation error and blocked transition | Re-open chat precheck and resubmit corrected payload |
| Transit command failure | Transit publish or downstream reject | Return terminal/recoverable error with reason code and correlation ids | Retry handler or manual operator fallback depending on reason code |
