# Intake Preflight Validation - Software Design Description

> Reject malformed or missing intake payloads before evidence generation and route clear retry states.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage implements a deterministic preflight pipeline that checks schema conformance, required fields, and business invariants before any downstream mutation or scoring step runs.

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

1. Validation layer executes contract checks and required-field validation.
2. Classification layer categorizes failures into hard reject, soft reject, and retryable buckets.
3. Routing layer applies standardized outcomes to downstream queues and audit logs.

## Components

| Component | Purpose |
|-----------|---------|
| Intake schema validator | Verifies payloads against versioned schema contracts. |
| Rule guard | Applies field-level and invariants checks. |
| Failure router | Emits structured rejection/retry events and state transitions. |

## Interfaces

<!-- API contracts, message formats, protocols (if this voyage exposes/consumes APIs) -->

## Data Flow

<!-- How data moves through the system; sequence diagrams if helpful -->

## Error Handling

<!-- What can go wrong, how we detect it, how we recover -->

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
