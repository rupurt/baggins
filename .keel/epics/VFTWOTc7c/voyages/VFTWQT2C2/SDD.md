# Smart Editing And Remediation Orchestration - Software Design Description

> Provide deterministic smart-edit recommendations and policy-safe remediation for identified blocking issues in one review pass.

**SRS:** [SRS.md](SRS.md)

## Overview

Create a deterministic recommendation pipeline that accepts high-confidence blockers from the preflight payload and emits safe, review-ready smart-edit suggestions with strict policy guardrails.

## Context & Boundaries

Consumes blocker payloads from Denial Risk Preflight and outputs recommendations to reviewer action queues. Does not apply edits directly.

```text
┌─────────────────────────────────────────┐
│     Smart Editing And Remediation        │
│                                         │
│  ┌──────────────────────┐               │
│  │  Preflight Blockers   │               │
│  └───────────┬──────────┘               │
│              │                          │
│      ┌───────▼────────┐                 │
│      │ Guardrail Policy│                 │
│      │ Evaluation      │                 │
│      └───────┬────────┘                 │
│              │                          │
│    ┌─────────▼──────────┐               │
│    │ Recommendation     │               │
│    │ Composer           │               │
│    └─────────┬──────────┘               │
│              │                          │
│     ┌────────▼──────────┐                │
│     │ Action Artifacts   │               │
│     └───────────────────┘                │
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Denial preflight payloads | internal | Source blockers and context | voyage VFTWQSkBx output |
| Policy allow/deny matrix | internal | Guardrails for safe recommendations | existing rule registry |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Suggestion confidence threshold | >=0.90 for auto-suggest | Minimizes low-confidence hallucination risk |
| Non-editable blocker classes | payer eligibility and policy gaps | Prevents unsafe automated corrections |

## Architecture

1. Recommendation resolver receives blocker payload.
2. Guardrail evaluator filters unsafe edits and injects mandatory rationale.
3. Suggestion composer emits deterministic proposal objects with confidence and guardrail status.
4. Artifacts are persisted with deterministic identifiers for audit.

## Components

### Recommendation Composer
- **Input:** blocker object + policy guardrail metadata
- **Output:** ordered recommendation list with action text, rationale, confidence, `policy_safe` flag
- **Failure mode:** explicit `GUARDRAIL_BLOCK` for unsupported edits

## Interfaces

### `SmartEditRecommendation`
- `blocker_id`
- `claim_id`
- `recommended_action`
- `rationale`
- `confidence`
- `policy_safe`
- `evidence_links[]`

## Data Flow

1. Receive high-confidence blockers from `VFTWQSkBx`.
2. Validate each blocker against policy guardrails.
3. Generate deterministic recommendation records.
4. Emit recommendation + escalation artifacts for operators.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Missing preflight blockers | required input validation | Emit empty recommendation set with `BLOCKER_INPUT_EMPTY` | Retry on next snapshot |
| Guardrail mismatch | policy matrix evaluation | Emit `GUARDRAIL_BLOCK` with refusal reason | Route for manual review |
