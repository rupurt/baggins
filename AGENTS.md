# AGENTS.md

Shared guidance for AI agents working with Baggins.

## Downstream Contract

This repository uses Keel as its project-management engine. This file is downstream from Keel and should remain recognizable when upstream guidance changes.

`AGENTS.md` and `INSTRUCTIONS.md` are the sync-sensitive files in this scaffold. When you absorb a newer Keel version, preserve the `PROJECT-SPECIFIC` blocks.

## Read This First

1. `INSTRUCTIONS.md` for the repository-specific procedural loop.
2. `POLICY.md` for operational constraints.
3. `ARCHITECTURE.md` and `USER_GUIDE.md` for technical/product context.
4. `CODE_WALKTHROUGH.md` for source and data-flow orientation.
5. `CONSTITUTION.md` for conflict-resolution values.

## Core Principles

- Use Keel as the canonical planning and lifecycle surface.
- Prioritize maximizing total paid value over claim volume.
- Use explicit evidence for every acceptance claim.
- Build recursively: claims and rules must pass verifier checks before submission-facing milestones.
- Default to conservative automation; escalate uncertainty to humans.

## Mission Design Requirements

Every mission must satisfy:

1. A complete charter with authored goals, constraints, and halting rules.
2. At least one mission child (bearing/epic/voyage/ADR) before activation.
3. Board-verifiable goal targets (`board:` references) in `CHARTER.md`.
4. Explicit handoff criteria tied to proof artifacts.

Required sequence:

1. Design or refine the mission charter.
2. Create or attach child entities for technical decomposition.
3. Execute research and evidence generation where needed.
4. Activate only when validation conditions are met.

## Core Execution Constraints

- All revenue-impacting behavior changes require human approval.
- Never claim payer-payout outcomes without explicit test or historical metric evidence.
- Keep mission changes scoped to a bounded set of services/contexts per turn.
- Do not bypass model confidence gates in recursive verification.
- Maintain traceability between `paddles` orchestration decisions, `sift` model evidence, and claim output.

## Decision Resolution Hierarchy

When faced with ambiguity, resolve decisions in this descending order:
1. ADRs
2. CONSTITUTION
3. POLICY
4. ARCHITECTURE
5. Planning artifacts (MISSION, EPIC, VOYAGE, STORY)
6. Tool/runtime evidence and recent `keel doctor --status`

## Foundational Documents

These define the constraints and workflow of the Baggins environment:

- `INSTRUCTIONS.md` — Procedure and board loop.
- `POLICY.md` — Invariants and gates.
- `CONSTITUTION.md` — Collaboration rules.
- `ARCHITECTURE.md` — Technical shape and boundaries.
- `CODE_WALKTHROUGH.md` — Data flow and repo structure.
- `USER_GUIDE.md` — Product behaviors and success lens.
- `.keel/adrs/` — Binding architecture decisions.

Use this order when interpreting constraints: ADRs → Constitution → Policy → Architecture → Planning artifacts.

## Project-Specific Conventions

<!-- BEGIN PROJECT-SPECIFIC -->
- Use mission language aligned with `transit`-backed service architecture and recursive verifier stack (`paddles` + `sift`).
- Enforce "maximize provider payment" as the top-level design target in mission decomposition.
- For each ship-ready change, update the relevant `.keel/...` entity, plus evidence references in mission logs.
- Maintain `keel` commandability; avoid manual status edits that conflict with CLI-authoritative state.
<!-- END PROJECT-SPECIFIC -->

## Sync Notes

- Upstream source: Keel's `AGENTS.md`
- Preserve the project-specific block above during syncs.
- Keep procedural details in `INSTRUCTIONS.md`.
