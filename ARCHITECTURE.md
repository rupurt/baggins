# Baggins Architecture

This document is downstream from Keel and should describe the actual technical shape of Baggins. Keel owns the board engine; this file should explain the repo, runtime, and technical seams that agents need to understand before they change behavior.

## System Map

Baggins is a Rust-first, event-driven medical billing agent platform whose objective is to maximize total paid to the provider.

High-level responsibilities:
- Ingest physician-facing and care timeline notes (pre/during/post interaction).
- Run recursive verification on claim proposals before submission.
- Validate against payer policies, eligibility, and historical outcomes.
- Learn from denial/payment feedback and improve future claim behavior.

Primary entry points (planned):
- Keel mission entities: `.keel/` (planning/control plane).
- Runtime services: Rust microservices under `services/` (execution plane).
- Message fabric: `transit` streams and schema contracts.

## Key Components

Major runtime components:

1. **Ingestion Layer**
   - Adapters for EHR exports and note channels.
   - Emits normalized encounter context events.
2. **Verifier Orchestrator**
   - Uses `paddles` for recursive, mission-like validation loops.
3. **Model Execution Layer**
   - Uses `sift` for local and remote model calls.
   - Standardizes evidence capture for each recommendation.
4. **Coding Agent**
   - Produces CPT/ICD/modifier candidates with confidence and traceability.
5. **Claim Assembly & Validation Service**
   - Builds claim payloads and enforces hard payer policy gates.
6. **Submission, Recovery, and Learning Services**
   - Handles submission state, denial recovery, and feedback-based improvement.
7. **Operations & Audit**
   - Structured logs, trace IDs, immutable evidence trail, and explainability artifacts.

## Technical Boundaries

### Stable boundaries (should not churn)
- Event schemas in `transit` (`encounter`, `claim`, `status`, `payment`, `denial`, `model.feedback`).
- Mission and verification contract fields (goal, evidence, confidence score, escalation reason).
- Audit/trace schema linking model output to claim actions.

### Mutable boundaries (expected evolution)
- Coding heuristics and service internals.
- Learning and scoring models/rules.
- Payer-specific adapters and policy packs.

### Dependency rules

- Rust services should depend on explicit interface crates (domain contracts + transit client + verifier interfaces) and avoid duplicating schema logic.
- Verifier logic must remain deterministic to support auditability.
- Avoid hard-coded payer-specific assumptions in generic claim assembly.

## Operational Seams

- `keel` / `.keel/`: planning and execution control.
- `transit`: event transport and backpressure behavior.
- `paddles`: recursive orchestration and workflow control.
- `sift`: model execution and scoring surface.
- `proof/` or evidence outputs: acceptance and claim outcomes.

## Planned Transit Topics

- `encounter.raw`
- `encounter.normalized`
- `claims.draft`
- `claims.validation`
- `claims.eligibility`
- `claims.submission`
- `claims.status`
- `claims.payment`
- `claims.denial`
- `claims.appeal`
- `model.feedback`

## Safety and Governance Boundaries

- No claim-submission behavior ships without verifier approval and human-visible traceability.
- Escalation path is required for low-confidence or conflicting evidence.
- Every payout-impacting rule change must be reflected in mission scope and signed off via `keel` entities.
