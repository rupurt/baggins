# Baggins Code Walkthrough

This document orients contributors and agents to the source layout, key abstractions, and data flows in the Baggins codebase. For governance philosophy see [CONSTITUTION.md](CONSTITUTION.md); for architectural contracts see [ARCHITECTURE.md](ARCHITECTURE.md).

## Repository Layout

Current scaffold:
- `.keel/` — Keel board and mission lifecycle artifacts.
- `services/` (to be created) — Rust microservices per domain function.
- `scripts/` (to be created) — local developer and bootstrap scripts.
- `config/` (to be created) — policy packs, payer templates, and verifier configuration.
- `crates/` (to be created) — reusable Rust crates for shared contracts and adapters.
- `README.md` / `AGENTS.md` — living project and agent operating contract.

The execution plane will be Rust-first and built as a workspace of mission-bound services.

## Key Abstractions

Core abstractions to model early:

- `EncounterContext`: normalized patient interaction representation.
- `ClaimDraft`: claim candidate plus confidence metadata.
- `VerificationTrace`: recursive pass outputs from `paddles`.
- `SiftDecision`: model evidence and rationale payload.
- `TransitEvent`: transport envelope for each state transition.
- `PayerPolicy`: payer/contract constraints used in validation.

Services should preserve one-way data movement from raw input to final verified claim or recovery request.

## State and Lifecycle

States and transitions are represented through `transit` topics and mission artifacts:
- `ingest` → `normalized` → `validated` → `eligible` → `submission` → `settlement` / `denial`.
- Denial recovery introduces `appeal` and `rework` branches.

The state model must track:
- current decision context,
- confidence and escalation flags,
- proof references for any payout-related action.

## Command / Request Flow

Representative flow for a billing event:

1. Input notes and encounter metadata arrive at ingest adapters.
2. `transit` emits a normalized encounter event.
3. `coding-agent` generates candidate claim lines with evidence spans.
4. `verifier-orchestrator` runs recursive review loops (`paddles`) with model calls (`sift`).
5. `claim-service` builds and validates payloads, then transitions to submission if approved.
6. outcomes (`payment`, `denial`, `appeal`) return to learning and correction paths.

## Configuration

Configuration surfaces:
- environment for Rust services (local vs remote model routing),
- verifier thresholds,
- payer policy packs,
- retry/backoff and retry-safe correction settings,
- audit/export options.

Store config in clear, versioned formats and align all payer policy decisions to mission artifacts.

## Where to Look

Quick starts (as the codebase grows):

| I want to... | Start here |
|---------------|------------|
| Understand mission scaffolding | `.keel/` and `MISSION` markdown files |
| Add or change the Rust workspace | Planned `services/` entry points |
| Add verifier constraints | `ARCHITECTURE.md` + mission artifacts |
| Introduce payer policy logic | `services/payer-gatekeeper` (planned) and policy pack docs |
| Change state transitions | `transit` event contract and mission schema docs |
