# Coding Handoff And Escalation Telemetry - Software Design Description

> Emit auditable handoff and escalation signals for each coding artifact so downstream claim assembly has deterministic acceptance state.

**SRS:** [SRS.md](SRS.md)

## Overview

Validated coding outputs are converted into auditable handoff artifacts with explicit ready/blocked/escalated state and deterministic evidence links for reconciliation by downstream claim assembly.

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

1. State projector writes readiness transitions as immutable evidence events.
2. Handoff formatter normalizes telemetry envelopes for downstream consumers.
3. Escalation enricher injects links to artifacts, inputs, and reason metadata.
4. Telemetry sink publishes records with deterministic IDs.

## Components

| Component | Purpose |
|-----------|---------|
| Handoff State Projector | Derives deterministic handoff status for each payload. |
| Telemetry Composer | Builds structured handoff messages with evidence references. |
| Escalation Router | Flags blocked/escalated outputs and forwards them to review pathways. |
| Audit Linker | Adds reproducible links for downstream audit artifacts. |

## Interfaces

<!-- API contracts, message formats, protocols (if this voyage exposes/consumes APIs) -->

## Data Flow

1. Read validation output stream from prior voyage.
2. Project each payload into ready/blocked/escalated state.
3. Compose and persist audit-safe handoff artifact with lineage links.
4. Emit telemetry to downstream consumers.

## Error Handling

<!-- What can go wrong, how we detect it, how we recover -->

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Downstream telemetry sink unavailable | Retry policy exhaustion | Escalate with block state and reason code | Reprocess with retry job and preserve artifact state |
