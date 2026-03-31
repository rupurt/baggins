# Ingestion Schema Normalization - Software Design Description

> Deliver a deterministic intake normalization pipeline that converts inbound clinical documents into canonical transit schemas.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage normalizes incoming intake records into a transit event envelope once per submission. Each event is enriched with immutable source lineage and a checksum, then routed to the shared schema registry for downstream medical coding consumers.

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

1. Schema registry layer stores canonical ingestion contracts used by all mission-one downstream consumers.
2. Normalization adapter layer parses supported intake formats into a canonical intermediate representation.
3. Event publisher layer emits the canonical payload with schema version metadata and lineage headers.

## Components

| Component | Purpose |
|-----------|---------|
| Intake parser | Parses accepted input formats into canonical intermediate fields. |
| Lineage enricher | Adds deterministic identifiers and traceability metadata. |
| Schema router | Validates against transit contracts and emits normalized events. |

## Interfaces

<!-- API contracts, message formats, protocols (if this voyage exposes/consumes APIs) -->

## Data Flow

<!-- How data moves through the system; sequence diagrams if helpful -->

## Error Handling

<!-- What can go wrong, how we detect it, how we recover -->

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
