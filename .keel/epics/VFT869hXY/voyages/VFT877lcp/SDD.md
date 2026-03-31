# Medical Code Candidate Extraction - Software Design Description

> Extract CPT/ICD/RVU candidates from canonical evidence with confidence and lineage metadata.

**SRS:** [SRS.md](SRS.md)

## Overview

Evidence enters the coding candidate pipeline as canonical transit payloads. A deterministic extraction layer resolves rule-based candidates, then a ranking layer applies confidence and policy ordering, and an output adapter stamps deterministic lineage IDs plus schema metadata for each candidate record.

## Context & Boundaries

<!-- What's in scope, what's out of scope, external actors/systems we interact with -->

```
┌─────────────────────────────────────────┐
│              This Voyage                │
│                                         │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐ │
│  │         │  │         │  │         │ │
│  └─────────┘  └─────────┘  └─────────┘ │
└─────────────────────────────────────────┘
        ↑               ↑
   [External]      [External]
```

## Dependencies

<!-- External systems, libraries, services this design relies on -->

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|

## Architecture

1. Evidence ingestion layer normalizes clinical findings into coding-friendly extraction primitives.
2. Candidate extraction layer maps extracted concepts to candidate CPT/ICD targets.
3. Ranking/enrichment layer adds confidence, source evidence IDs, and deterministic sequence ordering.
4. Candidate publisher layer emits auditable payloads referenced by voyage and story IDs.

## Components

| Component | Purpose |
|-----------|---------|
| Evidence Intake Adapter | Converts normalized evidence payloads to extraction-ready internal shapes. |
| Rule Extraction Engine | Executes deterministic code-mapping rules to generate candidate rows. |
| Confidence Ranker | Scores candidates consistently for every replay. |
| Candidate Publisher | Emits ranking outputs with lineage and schema metadata. |

## Interfaces

<!-- API contracts, message formats, protocols (if this voyage exposes/consumes APIs) -->

## Data Flow

1. Receive canonical evidence bundle.
2. Extract codable concepts into normalized feature vectors.
3. Apply deterministic coding rules and rank candidates.
4. Persist candidate set with ids, lineage headers, and score signals.
5. Emit to downstream coding handoff artifact path.

## Error Handling

<!-- What can go wrong, how we detect it, how we recover -->

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Missing evidence fields | Missing required payload fields | Return error state and emit an incomplete-input artifact for upstream preflight | Retry after upstream normalization fix |
