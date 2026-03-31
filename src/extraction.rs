use crate::models::{CodingCandidate, EvidenceBundle};

const DEFAULT_CODE: &str = "Z00.00";

#[derive(Debug, Clone, PartialEq)]
struct RuleMatch {
    code: String,
    rationale: String,
    confidence: f64,
}

pub fn extract_candidates(bundle: &EvidenceBundle) -> Vec<CodingCandidate> {
    let mut candidates: Vec<CodingCandidate> = bundle
        .findings
        .iter()
        .flat_map(|finding| {
            candidate_matches(finding.concept.as_str(), &finding.suggested_code)
                .into_iter()
                .map(|rule| {
                    let candidate_id = stable_id(&[
                        "candidate",
                        &bundle.encounter_id,
                        &finding.finding_id,
                        &rule.code,
                        &rule.rationale,
                    ]);
                    CodingCandidate {
                        candidate_id,
                        code: rule.code,
                        confidence: rule.confidence,
                        rationale: rule.rationale,
                        source_pointers: vec![finding.source.clone()],
                        schema_version: bundle.schema_version.clone(),
                    }
                })
        })
        .collect();

    candidates.sort_by(|left, right| {
        right
            .confidence
            .partial_cmp(&left.confidence)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| left.candidate_id.cmp(&right.candidate_id))
    });

    candidates
}

fn candidate_matches(concept: &str, suggested_code: &Option<String>) -> Vec<RuleMatch> {
    let normalized = concept.to_lowercase();
    if let Some(code) = suggested_code.as_ref() {
        return vec![RuleMatch {
            code: code.clone(),
            rationale: "explicit-suggested-code".to_string(),
            confidence: 1.0,
        }];
    }

    if normalized.contains("fracture") {
        vec![RuleMatch {
            code: "99213".to_string(),
            rationale: "mapping-fracture".to_string(),
            confidence: 0.89,
        }]
    } else if normalized.contains("diabetes") {
        vec![RuleMatch {
            code: "E11.9".to_string(),
            rationale: "mapping-diabetes".to_string(),
            confidence: 0.95,
        }]
    } else if normalized.contains("procedure") || normalized.contains("surgery") {
        vec![RuleMatch {
            code: "00670".to_string(),
            rationale: "mapping-procedure".to_string(),
            confidence: 0.82,
        }]
    } else {
        vec![RuleMatch {
            code: DEFAULT_CODE.to_string(),
            rationale: "mapping-generic".to_string(),
            confidence: 0.60,
        }]
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
    format!("cand-{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::extract_candidates;
    use crate::models::{EvidenceBundle, EvidenceFinding, SourcePointer};

    fn fixture_bundle() -> EvidenceBundle {
        EvidenceBundle {
            encounter_id: "enc-1".to_string(),
            schema_version: "1".to_string(),
            findings: vec![
                EvidenceFinding {
                    finding_id: "f1".to_string(),
                    concept: "Patient has diabetes".to_string(),
                    suggested_code: None,
                    source: SourcePointer {
                        document_id: "note-a".to_string(),
                        span_start: 11,
                        span_end: 38,
                        note_excerpt: "history of diabetes".to_string(),
                        source_type: "clinical_note".to_string(),
                    },
                },
                EvidenceFinding {
                    finding_id: "f2".to_string(),
                    concept: "Procedure performed".to_string(),
                    suggested_code: Some("93000".to_string()),
                    source: SourcePointer {
                        document_id: "note-a".to_string(),
                        span_start: 52,
                        span_end: 78,
                        note_excerpt: "electrocardiogram completed".to_string(),
                        source_type: "clinical_note".to_string(),
                    },
                },
            ],
        }
    }

    #[test]
    fn extraction_is_deterministic_for_same_input() {
        let first = extract_candidates(&fixture_bundle());
        let second = extract_candidates(&fixture_bundle());

        assert_eq!(first, second);
        assert_eq!(first.len(), 2);
        assert!(first
            .iter()
            .all(|candidate| !candidate.candidate_id.is_empty()));
        assert!(first
            .iter()
            .all(|candidate| !candidate.source_pointers.is_empty()));
    }

    #[test]
    fn extraction_includes_schema_and_confidence() {
        let candidates = extract_candidates(&fixture_bundle());
        let first = candidates.first().expect("expected at least one candidate");

        assert_eq!(first.schema_version, "1");
        assert!(first.confidence > 0.0);
    }
}
