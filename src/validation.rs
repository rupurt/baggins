use crate::models::{CodingCandidate, ValidationOutcome, ValidationResult};

pub const RC_REQ_CODE: &str = "RC-REQ-CODE";
pub const RC_LOW_CONFIDENCE: &str = "RC-LOW-CONFIDENCE";
pub const RC_UNSAFE_COMB: &str = "RC-UNSAFE-COMB";
pub const RC_OK: &str = "RC-OK";

pub fn validate_candidate(candidate: &CodingCandidate) -> ValidationResult {
    if candidate.code.trim().is_empty() {
        return ValidationResult {
            candidate: candidate.clone(),
            outcome: ValidationOutcome::Reject,
            reason_code: RC_REQ_CODE.to_string(),
            reason: "required_code_missing".to_string(),
        };
    }

    if candidate.rationale.contains("explicit-unsafe-combination") {
        return ValidationResult {
            candidate: candidate.clone(),
            outcome: ValidationOutcome::Escalate,
            reason_code: RC_UNSAFE_COMB.to_string(),
            reason: "policy_risk_detected".to_string(),
        };
    }

    if candidate.confidence < 0.35 {
        return ValidationResult {
            candidate: candidate.clone(),
            outcome: ValidationOutcome::Retry,
            reason_code: RC_LOW_CONFIDENCE.to_string(),
            reason: "confidence_below_threshold".to_string(),
        };
    }

    ValidationResult {
        candidate: candidate.clone(),
        outcome: ValidationOutcome::Ready,
        reason_code: RC_OK.to_string(),
        reason: "validation_passed".to_string(),
    }
}

pub fn validate_candidates(candidates: &[CodingCandidate]) -> Vec<ValidationResult> {
    let mut results = candidates
        .iter()
        .map(validate_candidate)
        .collect::<Vec<_>>();
    results.sort_by(|a, b| a.candidate.candidate_id.cmp(&b.candidate.candidate_id));
    results
}

#[cfg(test)]
mod tests {
    use super::{validate_candidate, RC_LOW_CONFIDENCE, RC_REQ_CODE, RC_UNSAFE_COMB};
    use crate::models::{CodingCandidate, SourcePointer, ValidationOutcome};

    fn pointer() -> SourcePointer {
        SourcePointer {
            document_id: "note-a".to_string(),
            span_start: 0,
            span_end: 3,
            note_excerpt: "example".to_string(),
            source_type: "clinical_note".to_string(),
        }
    }

    fn candidate(code: &str, confidence: f64, rationale: &str) -> CodingCandidate {
        CodingCandidate {
            candidate_id: format!("c-{code}"),
            code: code.to_string(),
            confidence,
            rationale: rationale.to_string(),
            source_pointers: vec![pointer()],
            schema_version: "1".to_string(),
        }
    }

    #[test]
    fn missing_code_is_reject_with_reason_code() {
        let result = validate_candidate(&candidate("", 0.9, "explicit"));
        assert_eq!(result.outcome, ValidationOutcome::Reject);
        assert_eq!(result.reason_code, RC_REQ_CODE);
    }

    #[test]
    fn low_confidence_is_retry_with_reason_code() {
        let result = validate_candidate(&candidate("99213", 0.20, "explicit"));
        assert_eq!(result.outcome, ValidationOutcome::Retry);
        assert_eq!(result.reason_code, RC_LOW_CONFIDENCE);
    }

    #[test]
    fn unsafe_pair_is_escalation() {
        let result = validate_candidate(&candidate("99213", 0.90, "explicit-unsafe-combination"));
        assert_eq!(result.outcome, ValidationOutcome::Escalate);
        assert_eq!(result.reason_code, RC_UNSAFE_COMB);
    }
}
