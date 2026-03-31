# GEMINI.md

Guidance for **Gemini CLI** and **Google AI Studio** when working with this repository.

## Shared Contract

Before doing work, read:

1. `AGENTS.md`
2. `INSTRUCTIONS.md`
3. `POLICY.md`
4. `ARCHITECTURE.md`

Those files are the repo-wide operating contract. This file should stay thin and only capture Gemini-specific harness notes.

## Project-Specific Gemini Notes

<!-- BEGIN PROJECT-SPECIFIC -->
Gemini agents should default to Keel-driven mission flow:
- Read the active mission before proposing edits.
- Decompose work into mission-linked slices instead of ad-hoc task chunks.
- Preserve recursive verifier guardrails (`paddles` + `sift`) when discussing coding, eligibility, or payout logic.
- Record reasoning assumptions and confidence explicitly in comments or log-backed notes.
<!-- END PROJECT-SPECIFIC -->
