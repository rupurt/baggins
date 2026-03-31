# Evidence Validation And Rejection Policy - Software Design Description

> Validate extracted evidence, reject unsafe coding suggestions, and preserve retry-safe exception channels.

**SRS:** [SRS.md](SRS.md)

## Overview

Extracted candidate bundles are scored, normalized, and validated in a deterministic policy layer. Any candidate failing policy checks is annotated with a reason code and routed into either reject, retry, or escalation channels.

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

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|

## Architecture

1. Candidate validator checks required fields, policy constraints, and safety thresholds.
2. Outcome router maps validation state to explicit actions: ready, retryable reject, hard reject, escalate.
3. Outcome recorder writes reason codes and auditable status transitions.

## Components

| Component | Purpose |
|-----------|---------|
| Candidate Validator | Enforces schema and business policy checks. |
| Outcome Router | Resolves final state into ready/reject/retry/escalate. |
| Reason Code Mapper | Produces structured, versioned reason metadata for all non-ready outcomes. |
| Outcome Envelope Writer | Persists validation outcomes for downstream audit. |

## Interfaces

<!-- API contracts, message formats, protocols (if this voyage exposes/consumes APIs) -->

## Data Flow

1. Accept ranked candidate batch from extraction voyage output.
2. Apply safety and policy checks.
3. Emit outcome with deterministic reason code and next action.
4. Persist outcome and forward only ready outputs.

## Error Handling

<!-- What can go wrong, how we detect it, how we recover -->

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Candidate missing required fields | Validation check failure | Route as reject or retry according to validation category | Re-ingest after intake correction |
