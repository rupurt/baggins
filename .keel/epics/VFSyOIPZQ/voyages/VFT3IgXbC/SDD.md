# Define Security And Compliance Boundaries - Software Design Description

> Document and enforce security/compliance boundaries before payer-facing work.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage turns boundary decisions into explicit run-time policy, separating local-only processing, human-review gates, and escalation criteria before any payer-facing behavior is planned.

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

1. `policy/` stores allowed input classes and environment routing rules.
2. `paddles` orchestrates decision policies with confidence thresholds.
3. `sift` executes local checks first, using remote fallback only for approved escape paths.
4. Human-review hooks pause high-impact or ambiguous recommendations.

## Components

<!-- For each major component: purpose, interface, behavior -->

## Interfaces

<!-- API contracts, message formats, protocols (if this voyage exposes/consumes APIs) -->

## Data Flow

1. Inputs are classified by sensitivity and workload class.
2. A local verifier run produces confidence and boundary compliance signals.
3. Escalation or remote execution is triggered only when policy allows it.

## Error Handling

<!-- What can go wrong, how we detect it, how we recover -->

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
