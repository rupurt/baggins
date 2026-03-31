# Create Transit Event Schema Baseline - Software Design Description

> Author and version initial transit event contracts and ownership semantics.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage standardizes transit event contracts, then defines ownership/idempotency and dead-letter semantics for event ownership and replay safety.

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

1. `transit` remains the event transport contract layer for encounter/claim/payment/status topics.
2. `sift` and `paddles` consume trace-rich events for deterministic verification.
3. Event registries define schema versions and compatibility guarantees.
4. Dead-letter and retry handlers route malformed or rejected events for forensic inspection.

## Components

<!-- For each major component: purpose, interface, behavior -->

## Interfaces

<!-- API contracts, message formats, protocols (if this voyage exposes/consumes APIs) -->

## Data Flow

1. Mission events are written with schema version and verifier trace metadata.
2. Consumers validate shape and ownership before state transitions.
3. Failures move to dead-letter channels with replay guidance.

## Error Handling

<!-- What can go wrong, how we detect it, how we recover -->

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
