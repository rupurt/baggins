# Baggins Constitution

This document is downstream from Keel and should be hydrated with the collaboration philosophy of **Baggins**. Keel remains the upstream board engine; this file defines how this repository wants humans and agents to work together on top of that engine.

## Why This Exists

- The objective is maximizing **total paid to the provider**, not claim throughput or payer-agnostic ratios.
- This repository is designed to ship reliable, explainable medical billing improvements with strong safety, compliance, and evidence trails.
- Every decision should improve the chance that claims are paid by reducing avoidable failure and improving claim quality.

## Decision Hierarchy

Decision order in this repository:

1. ADRs
2. Constitution
3. POLICY.md
4. ARCHITECTURE.md
5. Board planning artifacts (`mission/mission-brief/board evidence`)
6. Chat context or temporary operational preference

## Project Values

- Outcomes first: maximize provider payment with defensible revenue quality.
- Verifier-first engineering: recursive validation before submission.
- Safe automation: high-confidence rules plus explicit human escalation paths.
- Evidence-backed behavior: every decision needs an auditable trail.
- Pace with guardrails: rapid delivery only inside explicit compliance and safety boundaries.

## Collaboration Rules

- Humans must approve:
  - model/clinical policy exceptions
  - compliance or legal interpretation changes
  - irreversible payer-submission behavior changes
- AI agents must default to conservative submission decisions and escalate low-confidence claims.
- “Good enough to ship” means:
  - mission goal has a verified board target
  - supporting evidence exists for all completion claims
  - no unresolved high-risk safety or compliance gaps
- If there is a disagreement between speed and correctness, prioritize correctness and traceability.

## Revision Notes

- Keep this document short and high authority.
- Prefer durable principles over temporary workflow details.
- Put procedural specifics in `INSTRUCTIONS.md`, not here.
