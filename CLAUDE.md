# CLAUDE.md

Guidance for **Claude Code** when working with this repository.

## Shared Contract

Before doing work, read:

1. `AGENTS.md`
2. `INSTRUCTIONS.md`
3. `POLICY.md`
4. `ARCHITECTURE.md`

Those files are the repo-wide operating contract. This file should stay thin and only capture Claude-specific harness notes.

## Project-Specific Claude Notes

<!-- BEGIN PROJECT-SPECIFIC -->
Claude agents should treat `.keel/` as the source of truth for mission status and sequencing.
- Before any implementation suggestion, confirm current active mission and board state.
- Keep changes scoped to one mission slice, with explicit evidence and proof artifacts.
- Maintain the recursive verifier posture (`paddles` orchestration + `sift` decisions) for any revenue logic.
- Escalate any ambiguity around compliance, payer policy interpretation, or payout claims before coding.
<!-- END PROJECT-SPECIFIC -->
