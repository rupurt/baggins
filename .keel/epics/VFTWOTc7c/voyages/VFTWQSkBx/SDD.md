# Denial Risk Preflight And Evidence Readiness - Software Design Description

> Predict denial and escalation risk before claim assembly so reviewers can resolve high-probability issues pre-submission.

**SRS:** [SRS.md](SRS.md)

## Overview

Build a deterministic preflight stage that joins claim, coverage, and payer-context inputs, then emits a single blocker payload used by reviewer queues and smart-edit planning.

## Context & Boundaries

Inputs are claim snapshots, policy references, and denial-history signals represented in transit events. Output is read-only planning guidance for downstream review and remediation workflows.

```text
┌─────────────────────────────────────────┐
│          Denial Risk Preflight          │
│                                         │
│  ┌──────────────┐  ┌───────────────┐   │
│  │ claim/payer  │  │ policy/rules  │   │
│  │ data import  │  │ snapshot fetch│   │
│  └──────┬───────┘  └───────┬───────┘   │
│         │                  │            │
│         └────────┬─────────┘            │
│                  │                         │
│        ┌─────────▼──────────┐              │
│        │ Deterministic Rule │              │
│        │ Engine + Blockers  │              │
│        └─────────┬──────────┘              │
│                  │                         │
│      ┌───────────▼──────────────┐          │
│      │ Blocker Payload + Evidence │         │
│      └──────────────────────────┘          │
└─────────────────────────────────────────┘
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Transit claim events | internal | Source claim snapshots and handoff state | repo transit schema v1 |
| Policy metadata service | internal | Rule version lookup and reason-code registry | existing policy data source |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Input join keys | claim_id + payer_id + policy_version | Ensures deterministic replay and avoids accidental remaps |
| Blocker emission format | normalized structured JSON with reason code | Supports deterministic downstream consumption |

## Architecture

1. Ingress adapter loads claim and policy context.
2. Deterministic evaluator applies rule set by policy version.
3. Blocker composer adds reason codes, confidence labels, and evidence links.
4. Serializer emits a replay-safe planning payload with run metadata.

## Components

### Blocker Composer
- **Input:** joined policy and claim context
- **Output:** ordered blocker list with deterministic identifiers
- **Failure mode:** explicit `BLOCKER_UNKNOWN` reason when required fields are absent

## Interfaces

- `PreflightPayload` object consumed by reviewer queue and smart-edit voyage:
  - `claim_id`, `policy_version`, `risk_score`, `blockers[]`, `generated_at`, `evidence_link`

## Data Flow

1. Pull current claim + policy snapshot.
2. Evaluate deterministic rules.
3. Build blocker objects with reason codes.
4. Emit telemetry + handoff payload.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Missing required policy version | schema guard in evaluator | Set blocker `POLICY_LOOKUP_MISSING` and route to review | Retry after policy sync completes |
| Conflicting input versions | deterministic version comparison | Emit explicit `INCONSISTENT_CONTEXT` state | Require manual resolution before handoff |
