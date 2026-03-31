# Eligibility Preflight Coverage Intake - Software Design Description

> Create and normalize coverage+eligibility inputs (payer, member, and benefit state) from existing claim and policy sources into a deterministic transit preflight payload.

**SRS:** [SRS.md](SRS.md)

## Overview

Collect claim context, member coverage snapshots, and payer policy metadata into a single deterministic preflight object. The voyage writes validated state to a transit event consumed by downstream eligibility decision logic.

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

The voyage is implemented as an intake normalizer with three layers:

- Source adapters for claim metadata, member records, and payer policy attributes.
- Normalization and validation stage that enforces schema and deterministic field ordering.
- Emission stage that persists `EligibilityPreflightInput` events to transit.

## Components

| Component | Purpose | Behavior |
|-----------|---------|----------|
| Coverage Adapter | Loads payer-specific coverage and policy data for the claim actor | Maps payer-specific keys into a canonical field map with versioned schema tags. |
| Eligibility Normalizer | Normalizes member and service context into deterministic structures | Adds explicit validity windows, coverage state, and missing-data flags. |
| Transit Producer | Publishes preflight payloads for downstream voyages | Writes idempotent records and preserves replay-safe event identifiers. |

## Interfaces

<!-- API contracts, message formats, protocols (if this voyage exposes/consumes APIs) -->

## Data Flow

Claim context + coverage + policy records -> adapters -> normalization pipeline -> schema validation -> deterministic preflight event persisted in transit.

## Error Handling

<!-- What can go wrong, how we detect it, how we recover -->

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
