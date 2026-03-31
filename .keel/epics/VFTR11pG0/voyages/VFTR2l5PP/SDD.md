# Eligibility Decision And Escalation Voyage - Software Design Description

> Evaluate coverage and eligibility policies deterministically to return pass/fail/hold verdicts with explicit rationale and required remediation actions for payer submission.

**SRS:** [SRS.md](SRS.md)

## Overview

Evaluate normalized preflight input and produce a single deterministic eligibility verdict event with code-level rationale. Outcomes are emitted for downstream workflow orchestration and biller escalation surfaces.

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

This voyage uses a three-step pipeline:

- Policy rule evaluation engine with deterministic precedence and conflict resolution.
- Decision synthesis service that maps matching rules into `PASS`, `FAIL`, or `HOLD` outcomes.
- Escalation packager that emits remediation guidance and evidence links.

## Components

| Component | Purpose | Behavior |
|-----------|---------|----------|
| Rule Evaluator | Applies policy and coverage rules over normalized inputs | Produces normalized decision flags, blocking reasons, and confidence grade for each rule outcome. |
| Decision Composer | Aggregates rule outcomes into a single final verdict | Returns `PASS`/`FAIL`/`HOLD` with stable tie-breaking and fallback defaults. |
| Escalation Packager | Builds remediation workflow payloads | Adds missing fields, human handoff notes, and evidence references when outcome is `FAIL` or `HOLD`. |

## Interfaces

<!-- API contracts, message formats, protocols (if this voyage exposes/consumes APIs) -->

## Data Flow

Eligibility preflight event -> rule evaluator -> conflict resolution -> final decision -> transit decision event + escalation payload.

## Error Handling

<!-- What can go wrong, how we detect it, how we recover -->

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
