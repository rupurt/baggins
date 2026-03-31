# Security And Compliance Boundaries — Brief

## Hypothesis

Medical billing AI automation at scale requires explicit policy boundaries before any payer-facing behavior, including what data can be used when and what escalations are required.

A defined security/compliance boundary should prevent silent assumptions and keep mission outcomes audit-safe.

## Problem Space

No concrete compliance decision matrix currently exists for:
- PHI handling during research and development;
- model call boundaries for local vs remote execution;
- confidence-based escalation and human-in-the-loop controls.

This bearing identifies the initial production-safe constraints for all mission work.

## Success Criteria

- [ ] A boundary matrix is authored and approved for data classes, access scope, and environment assumptions.
- [ ] Model/privacy boundaries are explicitly defined as `sift` local-first with documented remote exceptions and escape hatches.
- [ ] Payout-impacting recommendations are routed to explicit human review with confidence thresholds aligned to policy.

## Open Questions

- What escalation paths are required for ambiguous or high-risk recommendations?
- Which environment/state combinations are allowed for PHI-like payloads in this mission set?
