# Baggins User Guide

This guide is downstream from Keel and should describe the product, workflows, and operator-visible outcomes of Baggins.

## Product Story

Baggins is an agentic medical billing platform for provider practices. It ingests clinical reporting before, during, and after a visit, then runs recursive verification through `paddles` plus model-backed checks from `sift` to generate, validate, and improve claims. The operating objective is to maximize **total paid to the provider**, not claim count.

Primary users:
- Medical billing operators
- Coding supervisors
- Revenue-cycle leads
- Clinical teams that need billing evidence quality guarantees

## Core User Flows

Primary workflows:

1. **First-run / onboarding**
   - Provision practice context and payer policy packs.
   - Connect note inputs and enrollment/eligibility data.
   - Run Mission 0 foundation checks (keel board and verifier scaffolding).
2. **Primary loop: submit-ready claim cycle**
   - Ingest clinical data.
   - Normalize encounter context in `transit` events.
   - Run recursive verification in `paddles`:
     - coding suggestion,
     - policy/payer checks,
     - confidence and escalation checks.
   - Commit verified claim payloads only when guardrails pass.
3. **Recovery and appeals loop**
   - Capture denials and payments as feedback.
   - Route remediable cases automatically; escalate ambiguous cases for human review.
   - Learn and adjust policy/rule configuration via controlled missions.

## Personas

Primary personas:

- **Billing Operator:** executes operational claim work and monitors exceptions.
- **Verifier Reviewer:** adjudicates low-confidence or escalated recommendations.
- **Revenue Lead:** tracks paid-value outcomes and approves payout-oriented rule changes.
- **Practice Owner:** monitors compliance and outcome trend toward net payment uplift.

## Acceptance Lens

A good outcome is:
- Higher total paid outcomes with lower avoidable denials.
- Explainable claim decisions with visible evidence paths.
- Fewer production reversions from incorrect automated claim actions.
- Fast exception handling for cases that need human intervention.
