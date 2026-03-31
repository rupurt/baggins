# INSTRUCTIONS.md

Procedural instructions for humans and agents working with Baggins through Keel.

## Downstream Contract

This file is downstream from Keel and should keep the engine's turn-loop discipline recognizable while describing how Baggins actually operates.

When syncing from a newer Keel version, preserve the `PROJECT-SPECIFIC` block instead of re-authoring the local operating surface from scratch.

## The Turn Loop

Use Keel's canonical `Orient -> Inspect -> Pull -> Ship -> Close` loop as the basis of daily work:

1. **Orient**: `keel heartbeat`, `keel health --scene`, `keel flow --scene`, `keel doctor --status`
2. **Inspect**: `keel mission next --status`, `keel pulse`, `keel roles`, `keel next --role <role> --explain`, `keel mission show <active-id>`
3. **Pull**: choose one bounded slice for the appropriate role or lane
4. **Ship**: execute the slice and record proof while the work is fresh
5. **Close**: land the relevant lifecycle transition and the sealing commit

## Primary Workflows

Define the role-specific procedures for this repository. Each workflow should document the context commands, typical actions, and constraints for that role.

### Operator (Implementation)
Focus on **evidence-backed delivery**.
- **Context**: `keel mission show <id>`, `keel mission next --status`, `keel next --role operator`, `keel story list`
- **Action**: Implement scoped work linked to voyage/story requirements, then append proof with `keel story record` or mission log entries where applicable.
- **Constraint**: Every AC must have a proof.

### Manager (Planning)
Focus on **strategic alignment and unblocking**.
- **Context**: `keel roles`, `keel next --role manager --explain`, `keel flow --scene`, `keel mission show`
- **Action**: Author `PRD.md`, `SRS.md`, `SDD.md`, resolve routing, decompose stories, and attach mission children.
- **Constraint**: Move voyages from `draft` to `planned` only when requirements are coherent.
- **Constraint**: Mission 0-type foundational missions must define verifier boundaries (`paddles` + `sift`) before execution commands begin.

### Explorer (Research)
Focus on **technical discovery and fog reduction**.
- **Context**: `keel bearing list`
- **Action**: Fill `BRIEF.md`, collect `EVIDENCE.md`, and `assess`; attach to relevant mission with `mission attach --bearing`.
- **Constraint**: Graduate to epics only when research is conclusive.

## Mission Execution Guardrails (Baggins-specific)

- Use `keel mission refine` to complete all placeholder fields before activation.
- Missions must have at least one attached child entity before activation.
- Any mission with board-verification impact (claim/payment logic) must include:
  - explicit objective, constraints, and halting rules in `CHARTER.md`
  - clear verification target in each `MG-*` goal
  - human escalation rules for low-confidence outcomes
- For any claim-generation or payer logic work:
  - create or update the mission-bearing research first
  - ensure verifier stack requirements are documented in `ARCHITECTURE.md` and mission artifacts

## Repo-Specific Build and Validation Surface

- `nix develop`
- `cargo` workspace checks (once Rust crates are present):
  - `cargo fmt --all -- --check`
  - `cargo clippy --all-targets --all-features`
  - `cargo test --all`
- `keel doctor --status` after any mission state transition.
- Include local evidence outputs in mission log entries whenever claims or acceptance improvements are claimed.

## Repo-Specific Turn Surfaces

<!-- BEGIN PROJECT-SPECIFIC -->
- Use this project loop for all delivery:
  - Mission zeroing: verify verifier scaffolding and policy boundaries before implementation.
  - Mission launch: require `mission refine` completion and at least one attached child.
  - Mission execution: tie every service slice to a mission story or voyage requirement.
  - Mission close: include payout-impact evidence and risk gate status in the closing log.
- Add local helper scripts in future (`just`, Makefile, or shell aliases) only after corresponding board entries are approved.
<!-- END PROJECT-SPECIFIC -->

## Hydration Checklist

Before relying on this file as a real operating contract, this repository currently requires:

- recursive verifier scope in missions (`paddles` + `sift` alignment)
- explicit mission proof artifacts for any payment-impacting claim logic
- completion gates on `keel doctor --status` for structural integrity
- mission logging that captures risk decisions and manual escalation rationale

## Hygiene Rules

- Use the CLI as the canonical lifecycle surface.
- Prefer board-backed proof over memory or chat summaries.
- Keep downstream wrappers truthful about the real repo commands.
- Update this file when the repo workflow changes materially.
