# Baggins

Medical billing assistant platform built as an agentic system to maximize **total paid to the provider** for medical practices.

`baggins` (Billing Agentic Graph and Intelligence Network System) is designed for medical practices that need a practical, production-grade pipeline that can:

- Ingest clinical data generated before, during, and after a patient interaction
- Translate notes into billing-ready claims with consistent coding quality
- Detect avoidable payer failures before submission
- Learn from denials, remediations, and payment outcomes to improve future claim outcomes
- Use a recursive context engine to run a best-in-class medical verifier before submission

The project will be executed as a set of sequenced Keel missions, each delivering measurable capability and reducing risk before scaling further.

## Vision

The goal is not to maximize raw claim volume.  
The objective is to maximize **total paid reimbursements** by improving claim acceptance and payment while reducing denials and costly write-offs.

Target outcomes:

- Total paid dollars increase over time
- Reduction in preventable denials
- Faster claims cycles and better appeals quality
- Higher first-pass acceptance and cleaner claim narratives

## Core architecture

### Ingestion

The system ingests:

- Physician notes and reports before the visit (history, pre-procedure indications)
- Clinical documentation during the visit (assessment, procedure details, orders, dictation)
- Post-visit notes and updates (lab results, addenda, discharge summaries)
- Patient registration and insurance payloads
- Historical payer outcomes and remittance data

### Processing model

- The system uses Rust microservices connected by the `transit` message streaming platform.
- Services exchange normalized clinical and billing events.
- A recursive context engine orchestrated through [`paddles`](https://github.com/spoke-sh/paddles) (agentic mech-suite) coordinates claim-review loops and escalates for human review when confidence is low.
- Verification decisions call the local/remote model layer powered by [`sift`](https://github.com/rupurt/sift), enabling private on-device evaluation and cloud model fallback as needed.

### Verification stack

- `paddles` is the orchestration backbone for mission-goal workflows and recursive validation passes.
- `sift` is the model execution layer for claim verification, explanation generation, and adjudication scoring.
- The medical verifier is implemented as a repeatable recursion loop that:
  - extracts evidence from notes and encounter context
  - proposes claim actions
  - validates actions against policy and payer rules
  - requests additional evidence if confidence is weak
  - only commits claims when policy and payment-relevance checks pass

### Design principles

- Deterministic message contracts for all claim-relevant events
- Audit-first design for HIPAA, governance, and payer compliance
- Every claim decision should be explainable by source evidence
- Human-in-the-loop override path at every high-risk stage

## Nix development environment

This repo includes a Nix flake that provides a Rust-enabled shell.

```bash
nix develop
```

This shell provides:

- Rust toolchain (`rustc`, `cargo`, `rustfmt`, `clippy`)
- Common build dependencies for Rust services
- Git and utility tools needed for local development

## Keel missions

We will run implementation in ordered Keel missions. Each mission has clear acceptance criteria and a production-oriented deliverable.

### Mission 0 — Foundation and guardrails

- Stand up repo structure and CI basics
- Define event schema versioning rules for `transit`
- Add data-retention and security policy
- Add `paddles` orchestration bootstrap and `sift` runtime adapter interfaces (local + remote inference)
- Add contributor onboarding in docs and initial developer workflows

Acceptance criteria:
- New engineer can run `nix develop` and build an empty Rust workspace
- CI runs lint + format checks
- Security and compliance checklist exists and is versioned
- Recursive verifier interfaces are defined and can complete a dry-run check

### Mission 1 — Clinical ingestion fabric

- Implement ingestion microservice for pre/during/post visit notes
- Add adapters for source systems (EHR exports, note files, HL7/FHIR-friendly payloads)
- Normalize notes into structured `EncounterContext` events
- Route all source data through `transit` topics with dead-letter handling

Acceptance criteria:
- 95% of non-empty note batches produce one normalized context event
- Rejected payloads are explainably quarantined for review

### Mission 2 — Medical coding and evidence extraction agent

- Build the core coding agent:
  - maps conditions, procedures, and modifiers
  - builds CPT/ICD evidence chain from note spans
  - tracks uncertainty and confidence per code assignment
- Add a reviewer dashboard payload that shows missing evidence areas
- Integrate coding outputs into the recursive verifier through `paddles`
- Run `sift`-backed quality checks for unsupported claims and payer risk flags

Acceptance criteria:
- Claim-line candidate quality score above baseline for test notes
- Every recommended code has traceability to note evidence
- Verifier loop can auto-reject low-confidence claims and request correction context

### Mission 3 — Claim assembly and validation service

- Generate claim payload from context + coding output
- Implement payer-specific rule validation
- Enforce hard business constraints before submission

Acceptance criteria:
- Zero invalid claim payloads reaching submission in regression test set
- Rejection-free payload coverage improved versus baseline

### Mission 4 — Coverage and eligibility preflight agent

- Add eligibility and authorization checks before claim finalization
- Surface missing coverage gaps as blocking items
- Emit `EligibilityFailure` events when required data is missing

Acceptance criteria:
- Reduced avoidable denials from ineligibility/authorization misses
- Audit log captures every preflight check and decision

### Mission 5 — Denial prevention and smart editing service

- Introduce deny-list and correction suggestions for common failure patterns
- Create retry-safe editing workflows
- Implement payer feedback ingestion for post-submit updates

Acceptance criteria:
- Return rate from first denial class improves
- Fewer “non-recoverable” denial escalations

### Mission 6 — Appeals and recovery automation

- Add automated response builder for remediable denials
- Route high-confidence appeals into auto-submission
- Route ambiguous cases to staff review with evidence bundles

Acceptance criteria:
- Measurable reduction in aged unresolved denials
- Improved recovery dollars on remapped claims

### Mission 7 — Payment optimization learning loop

- Build feedback sink for remittance and payment-posting outcomes
- Add metrics to compare expected value vs actual paid by CPT/ICD/payer/provider
- Train and deploy rule updates from closed-loop outcomes

Acceptance criteria:
- Monthly net paid uplift tracked and visible
- Demonstrable increase in paid value for high-impact claims in next cycle

### Mission 8 — Reliability, observability, and ops hardening

- Add distributed tracing across microservices
- Add replay tooling for failed `transit` events
- Add SLOs and incident runbooks

Acceptance criteria:
- End-to-end claim path traceability is available
- MTTR reduction for event failures and submission issues

### Mission 9 — Clinical rollout and scale

- Pilot with a single specialty and limited payer set
- Expand to additional practices and specialties incrementally
- Add regional payer-specific policy packs

Acceptance criteria:
- Stable uptime and throughput under pilot workload
- Positive net paid trend before each scale step

## Initial reference architecture

Suggested `transit` topics:

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

### Service layout

- `ingest-service`: imports physician report + notes and emits normalized encounter events
- `coding-agent`: extracts coded claims and clinical evidence links
- `claim-service`: assembles and validates claims
- `payer-gatekeeper`: runs payer-specific checks and blocks high-risk submissions
- `submission-service`: submits claim batches and tracks state transitions
- `recovery-service`: handles denials, corrections, and appeals
- `learning-service`: updates strategy policies from payment outcomes
- `verifier-orchestrator`: runs recursive validation workflows across coding, policy, and payer checks

## Quality and safety priorities

- **Accuracy > automation:** system favors safe claims and transparent confidence scoring
- **Explainability:** every automated decision includes rationale and evidence source
- **Compliance:** strict audit trail, data minimization, and retention policy alignment
- **Human oversight:** any low-confidence action must be human-reviewable before irreversible submission

## Risks and controls

- Coding ambiguity (multiple valid billing interpretations)
- Payer policy drift and contract-specific variations
- Data variability in notes and dictation quality
- False automation in high-risk encounters

Controls:

- Continuous policy diffing against payer updates
- Versioned prompt and rule revisions for coding actions
- Human escalation on confidence thresholds
- Monthly outcome review with billing leadership

## Repository milestones

- Build each Keel mission as an independently deployable increment
- Keep services backward-compatible by strict event schema versions
- Track total provider payment and net paid growth in every release review
- Expand payer and specialty coverage only after measurable uplift

## Quick start

1. `nix develop`
2. Start the Baggins API server:
   `just baggins`
3. Run interactive UI builds in separate terminals:
   - Biller interface: `just serve-biller-ui`
   - Payer interface: `just serve-payer-ui`
4. Build both front-ends for deployment artifacts:
   `just build-frontend`
5. Launch remaining mission pipelines (`keel` / services / topics) as needed.

The API exposes:
- `GET /v1/biller/search`
- `GET /v1/payer/denials/search`
- `GET /v1/cases/{case_id}`
- `POST /v1/cases/{case_id}/action`
- `GET /v1/cases/{case_id}/audit`

---

`baggins` is built for measurable financial impact, not novelty. The Keel mission sequence above is intended to ship clinical value quickly while continuously reducing denial risk and improving reimbursement outcomes.
