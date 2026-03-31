# Baggins Policy

This document is downstream from Keel and should describe the operational invariants that govern Baggins. Keel supplies the board engine and lifecycle discipline; this file defines the repo-specific rules that must remain true.

## Engine Contract

Baggins uses Keel as its project-management engine.

- The board lives in `.keel/`.
- The canonical tactical rhythm is the Turn Loop exposed by `keel turn`.
- Board mutations, proof, and lifecycle closure happen through the CLI, not manual file edits.
- Every mission must define and maintain:
  - an authored mission charter,
  - at least one mission-scoped child or research artifact,
  - a concrete board-verification target for each goal.

## The Core Objective: Zero Drift

The primary goal of the engine is to eliminate **Drift** — any gap between what the board describes and what the code actually delivers. Progress is blocked if any of the following are detected by the `doctor`:
- **Structural Drift:** Missing files, invalid IDs, or broken frontmatter.
- **Architectural Drift:** Working in a bounded context with a `proposed` ADR.
- **Requirement Drift:** Stories missing SRS references or acceptance criteria.
- **Scaffold Drift:** Presence of placeholder text (e.g., `{{goal}}`, `Item 1`).

Financial objective and prioritization:
- Ship for **maximum total paid to the provider**, with explicit, auditable confidence for each automated action.
- Avoid optimization patterns that increase raw claim count without raising paid value.
- Favor verifiable quality improvements that increase payment outcomes and reduce denial loops.

## Entity Invariants

Every entity in the `.keel/` directory must adhere to structural rules. Define what must always be true for each entity type in this repository:

- **Missions:** Must have a `CHARTER.md` with at least one `board:`-verifiable goal to be `active`.
- **Epics:** Status is *derived* from voyages. An epic is `draft` until its first voyage is `planned`.
- **Voyages:** Must have an `SRS.md` and `SDD.md` with authored content to transition from `draft` to `planned`.
- **Stories:** Must link to a `voyage/SRS` requirement via the `[SRS-XX/AC-YY]` format.
- **Bearings:** must progress from `exploring -> assessed -> laid` before attachment to active missions.
- **Missions:** cannot be active while any required `board:` goal points at a non-existing entity.

## Repo Invariants

These rules should always be true in this repository:

- Before any mission handoff or closure:
  - `keel doctor --status` is clean for known active entities.
  - all direct evidence claims in board artifacts are linked to a concrete output file.
  - review of `Policy`, `Constitution`, and `Mission` docs has occurred.
- Human review is mandatory for:
  - model selection/routing changes
  - credential, payer, or data handling changes
  - escalation logic where automation bypasses manual review
- Any claim/payment outcome claim requires supporting evidence:
  - metrics source reference,
  - methodology, and
  - date range for comparison.

## Safety Rails

- Release and rollback:
  - Start with dry-run validation in non-production staging.
  - Introduce one mission at a time to production.
  - Maintain a documented rollback path for each service.
- Data handling:
  - no raw PHI processing without explicit compliance approval in mission artifacts.
  - synthetic/sanitized data in early missions.
- Autonomous execution must pause when:
  - verifier confidence is under threshold and requires human follow-up,
  - any policy or compliance requirement is ambiguous,
  - or board structure is no longer synchronized with code changes.

## Local Exceptions

If Baggins needs exceptions from the default Keel operating model, document them explicitly here rather than letting them live in habit or chat memory.

## Safety and Verification Baseline

- `nix develop` is the baseline development entry command.
- `keel doctor --status`, `keel mission show <id>`, and `keel mission next --status` are required checks in mission reviews.
- All model calls that produce billing recommendations must have either:
  - verifier logging in `sift` + `paddles` audit channels, or
  - explicit fallback to deterministic policy checks.
