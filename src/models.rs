#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SourcePointer {
    pub document_id: String,
    pub span_start: usize,
    pub span_end: usize,
    pub note_excerpt: String,
    pub source_type: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EvidenceFinding {
    pub finding_id: String,
    pub concept: String,
    pub suggested_code: Option<String>,
    pub source: SourcePointer,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EvidenceBundle {
    pub encounter_id: String,
    pub schema_version: String,
    pub findings: Vec<EvidenceFinding>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CodingCandidate {
    pub candidate_id: String,
    pub code: String,
    pub confidence: f64,
    pub rationale: String,
    pub source_pointers: Vec<SourcePointer>,
    pub schema_version: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValidationResult {
    pub candidate: CodingCandidate,
    pub outcome: ValidationOutcome,
    pub reason_code: String,
    pub reason: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ValidationOutcome {
    Ready,
    Reject,
    Retry,
    Escalate,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum HandoffState {
    Ready,
    Blocked,
    Escalated,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HandoffEvent {
    pub candidate_id: String,
    pub state: String,
    pub reason_code: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HandoffArtifact {
    pub handoff_id: String,
    pub batch_id: String,
    pub state: HandoffState,
    pub ready_count: usize,
    pub blocked_count: usize,
    pub escalated_count: usize,
    pub blocked_reason_codes: Vec<String>,
    pub blocked_until: Option<String>,
    pub evidence_links: Vec<String>,
    pub events: Vec<HandoffEvent>,
}
