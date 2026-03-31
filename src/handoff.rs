use crate::models::{
    HandoffArtifact, HandoffEvent, HandoffState, ValidationOutcome, ValidationResult,
};

pub fn project_handoff(batch_id: &str, validations: &[ValidationResult]) -> HandoffArtifact {
    let mut ready_count = 0;
    let mut blocked_count = 0;
    let mut escalated_count = 0;
    let mut blocked_codes = Vec::new();
    let mut events = Vec::new();

    for result in validations {
        let state = match result.outcome {
            ValidationOutcome::Ready => {
                ready_count += 1;
                "ready"
            }
            ValidationOutcome::Reject => {
                blocked_count += 1;
                blocked_codes.push(result.reason_code.clone());
                "blocked"
            }
            ValidationOutcome::Retry => {
                blocked_count += 1;
                blocked_codes.push(result.reason_code.clone());
                "blocked"
            }
            ValidationOutcome::Escalate => {
                escalated_count += 1;
                blocked_codes.push(result.reason_code.clone());
                "escalated"
            }
        };

        events.push(HandoffEvent {
            candidate_id: result.candidate.candidate_id.clone(),
            state: state.to_string(),
            reason_code: result.reason_code.clone(),
        });
    }

    events.sort_by(|left, right| {
        left.candidate_id
            .cmp(&right.candidate_id)
            .then_with(|| left.state.cmp(&right.state))
    });

    let state = if escalated_count > 0 {
        HandoffState::Escalated
    } else if blocked_count > 0 {
        HandoffState::Blocked
    } else {
        HandoffState::Ready
    };

    blocked_codes.sort();
    blocked_codes.dedup();

    let evidence_links = validations
        .iter()
        .map(|result| {
            format!(
                "evidence://{}/candidate/{}",
                batch_id, result.candidate.candidate_id
            )
        })
        .collect::<Vec<_>>();

    HandoffArtifact {
        handoff_id: stable_id(&[
            "handoff",
            batch_id,
            &state.to_string(),
            &ready_count.to_string(),
        ]),
        batch_id: batch_id.to_string(),
        state,
        ready_count,
        blocked_count,
        escalated_count,
        blocked_reason_codes: blocked_codes,
        blocked_until: if state == HandoffState::Blocked {
            Some("2026-01-01T00:00:00Z".to_string())
        } else {
            None
        },
        evidence_links,
        events,
    }
}

fn stable_id(parts: &[&str]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for part in parts {
        for byte in part.as_bytes() {
            hash ^= *byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0x9e3779b97f4a7c15u64;
    }
    format!("handoff-{hash:016x}")
}

impl std::fmt::Display for HandoffState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandoffState::Ready => write!(f, "ready"),
            HandoffState::Blocked => write!(f, "blocked"),
            HandoffState::Escalated => write!(f, "escalated"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::project_handoff;
    use crate::extraction::extract_candidates;
    use crate::models::{EvidenceBundle, EvidenceFinding, SourcePointer};
    use crate::validation::validate_candidates;

    #[test]
    fn ready_batch_handoff_is_ready() {
        let bundle = EvidenceBundle {
            encounter_id: "enc-2".to_string(),
            schema_version: "1".to_string(),
            findings: vec![EvidenceFinding {
                finding_id: "f1".to_string(),
                concept: "Procedure complete".to_string(),
                suggested_code: Some("93000".to_string()),
                source: SourcePointer {
                    document_id: "note-a".to_string(),
                    span_start: 1,
                    span_end: 15,
                    note_excerpt: "EKG performed".to_string(),
                    source_type: "clinical_note".to_string(),
                },
            }],
        };
        let candidates = extract_candidates(&bundle);
        let results = validate_candidates(&candidates);
        let handoff = project_handoff("batch-1", &results);

        assert_eq!(handoff.state.to_string(), "ready");
        assert_eq!(handoff.ready_count, 1);
        assert_eq!(handoff.blocked_reason_codes.len(), 0);
        assert_eq!(handoff.evidence_links.len(), 1);
    }

    #[test]
    fn blocked_batch_handoff_is_blocked_and_deterministic() {
        let mut candidates = extract_candidates(&EvidenceBundle {
            encounter_id: "enc-3".to_string(),
            schema_version: "1".to_string(),
            findings: vec![
                EvidenceFinding {
                    finding_id: "f1".to_string(),
                    concept: "unknown condition".to_string(),
                    suggested_code: Some("".to_string()),
                    source: SourcePointer {
                        document_id: "note-a".to_string(),
                        span_start: 1,
                        span_end: 3,
                        note_excerpt: "blank".to_string(),
                        source_type: "clinical_note".to_string(),
                    },
                },
                EvidenceFinding {
                    finding_id: "f2".to_string(),
                    concept: "fracture".to_string(),
                    suggested_code: Some("99213".to_string()),
                    source: SourcePointer {
                        document_id: "note-a".to_string(),
                        span_start: 5,
                        span_end: 13,
                        note_excerpt: "fracture follow-up".to_string(),
                        source_type: "clinical_note".to_string(),
                    },
                },
            ],
        });
        candidates[1].confidence = 0.1;
        let results = validate_candidates(&candidates);
        let handoff = project_handoff("batch-2", &results);

        assert_eq!(handoff.state.to_string(), "blocked");
        assert_eq!(handoff.blocked_count, 2);
        assert_eq!(handoff.ready_count, 0);
    }

    #[test]
    fn escalated_batch_handoff_is_escalated() {
        let mut candidates = extract_candidates(&EvidenceBundle {
            encounter_id: "enc-4".to_string(),
            schema_version: "1".to_string(),
            findings: vec![EvidenceFinding {
                finding_id: "f1".to_string(),
                concept: "high-risk procedure".to_string(),
                suggested_code: Some("93000".to_string()),
                source: SourcePointer {
                    document_id: "note-a".to_string(),
                    span_start: 1,
                    span_end: 15,
                    note_excerpt: "procedure with modifier conflict".to_string(),
                    source_type: "clinical_note".to_string(),
                },
            }],
        });
        candidates[0].rationale = "explicit-unsafe-combination".to_string();
        let results = validate_candidates(&candidates);
        let handoff = project_handoff("batch-3", &results);

        assert_eq!(handoff.state.to_string(), "escalated");
        assert_eq!(handoff.escalated_count, 1);
        assert_eq!(handoff.blocked_count, 0);
    }
}
