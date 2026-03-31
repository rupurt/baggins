use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};
use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::sync::RwLock;

type SharedState = Arc<RwLock<AppState>>;
const DEFAULT_LIMIT: usize = 25;
const SERVER_LISTEN_ADDR: &str = "0.0.0.0:8080";

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TimelineEvent {
    timestamp: String,
    actor: String,
    state: String,
    detail: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct BillerCase {
    case_id: String,
    patient_name: String,
    mrn: String,
    claim_id: String,
    owner: String,
    status: String,
    queue_state: String,
    unresolved_actions: Vec<String>,
    risk_hints: Vec<String>,
    timeline: Vec<TimelineEvent>,
    evidence_links: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PayerDenialCase {
    case_id: String,
    patient_name: String,
    payer: String,
    denial_reason: String,
    status: String,
    age_days: u32,
    denial_history: Vec<String>,
    prior_attempts: u32,
    timeline: Vec<TimelineEvent>,
    evidence_links: Vec<String>,
}

#[derive(Clone, Debug)]
struct AppState {
    biller_cases: HashMap<String, BillerCase>,
    payer_cases: HashMap<String, PayerDenialCase>,
    idempotency: HashMap<String, ActionReceipt>,
    audit_log: HashMap<String, Vec<AuditRecord>>,
}

#[derive(Clone, Debug, Serialize)]
struct AuditRecord {
    audit_id: String,
    trace_id: String,
    request_id: String,
    actor: String,
    role: String,
    command: String,
    case_id: String,
    params_hash: String,
    outcome_code: String,
    occurred_at: String,
}

#[derive(Clone, Debug, Serialize)]
struct ActionReceipt {
    request_id: String,
    trace_id: String,
    command_id: String,
    transition_id: String,
    case_id: String,
    command: String,
    command_outcome: String,
    resulting_status: String,
    resulting_queue_state: String,
    next_allowed_actions: Vec<String>,
    replayed: bool,
}

#[derive(Debug, Clone, Copy)]
enum Role {
    Admin,
    Biller,
    Payer,
    Unknown,
}

impl Role {
    fn from_headers(headers: &HeaderMap) -> Self {
        headers
            .get("x-role")
            .and_then(|value| value.to_str().ok())
            .map(|role| match role.to_lowercase().as_str() {
                "admin" => Role::Admin,
                "biller" => Role::Biller,
                "payer" => Role::Payer,
                _ => Role::Unknown,
            })
            .unwrap_or(Role::Unknown)
    }

    fn as_str(&self) -> &'static str {
        match self {
            Role::Admin => "admin",
            Role::Biller => "biller",
            Role::Payer => "payer",
            Role::Unknown => "unknown",
        }
    }

    fn can_search_biller(&self) -> bool {
        matches!(self, Role::Admin | Role::Biller)
    }

    fn can_search_payer(&self) -> bool {
        matches!(self, Role::Admin | Role::Payer)
    }

    fn can_read_case(&self) -> bool {
        matches!(self, Role::Admin | Role::Biller | Role::Payer)
    }

    fn allowed_commands(&self) -> &'static [&'static str] {
        match self {
            Role::Admin => &[
                "validate",
                "retry",
                "escalate",
                "hold",
                "submit_draft",
                "triage",
                "draft_appeal",
                "submit_response",
            ],
            Role::Biller => &["validate", "retry", "escalate", "hold", "submit_draft"],
            Role::Payer => &["triage", "draft_appeal", "submit_response", "escalate", "hold"],
            Role::Unknown => &[],
        }
    }

    fn command_allowed(&self, command: &str) -> bool {
        self.allowed_commands().iter().any(|allowed| *allowed == command)
    }
}

#[derive(Debug, Clone, Deserialize)]
struct BillerSearchQuery {
    q: Option<String>,
    status: Option<String>,
    owner: Option<String>,
    limit: Option<usize>,
    cursor: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
struct PayerSearchQuery {
    payer: Option<String>,
    reason: Option<String>,
    status: Option<String>,
    age_min: Option<u32>,
    age_max: Option<u32>,
    limit: Option<usize>,
    cursor: Option<usize>,
}

#[derive(Debug, Serialize)]
struct PagedResponse<T> {
    request_id: String,
    items: Vec<T>,
    next_cursor: Option<usize>,
    count: usize,
}

#[derive(Debug, Clone, Deserialize)]
struct ActionRequest {
    command: String,
    params: Option<Value>,
    confirm: bool,
    request_id: Option<String>,
    idempotency_key: Option<String>,
}

#[derive(Debug, Serialize)]
struct ActionResponse {
    request_id: String,
    trace_id: String,
    command_id: String,
    transition_id: String,
    case_id: String,
    command: String,
    replayed: bool,
    command_outcome: String,
    resulting_status: String,
    resulting_queue_state: String,
    next_allowed_actions: Vec<String>,
    preview: bool,
    audit_id: Option<String>,
}

#[derive(Debug, Serialize)]
struct CaseEnvelope {
    request_id: String,
    case_id: String,
    case_type: String,
    case_data: Value,
    timeline_last_100: Vec<TimelineEvent>,
}

#[derive(Debug, Serialize)]
struct ErrorEnvelope {
    request_id: String,
    error: ErrorPayload,
}

#[derive(Debug, Serialize)]
struct ErrorPayload {
    code: &'static str,
    message: String,
    recoverable: bool,
    next_allowed_actions: Vec<String>,
}

#[derive(Debug, Serialize)]
struct AuditResponse {
    request_id: String,
    case_id: String,
    audit: Vec<AuditRecord>,
}

fn now_iso_now() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{now}")
}

fn make_hash(parts: &[&str]) -> String {
    let mut hasher = DefaultHasher::new();
    parts.iter().for_each(|part| part.hash(&mut hasher));
    format!("{:016x}", hasher.finish())
}

fn value_hash(value: &Option<Value>) -> String {
    match value {
        Some(value) => make_hash(&[&value.to_string()]),
        None => make_hash(&["null"]),
    }
}

fn paginate<T>(items: Vec<T>, cursor: Option<usize>, limit: usize) -> (Vec<T>, Option<usize>) {
    let offset = cursor.unwrap_or(0);
    let mut next_cursor = None;
    let total = items.len();
    let paged = items.into_iter().skip(offset).take(limit).collect::<Vec<_>>();
    if offset + paged.len() < total {
        next_cursor = Some(offset + paged.len());
    }
    (paged, next_cursor)
}

fn biller_sort_key(item: &BillerCase) -> String {
    format!("{}{}{}", item.status, item.queue_state, item.case_id)
}

fn payer_sort_key(item: &PayerDenialCase) -> String {
    format!("{}{}{}", item.payer, item.age_days, item.case_id)
}

fn biller_default_cases() -> HashMap<String, BillerCase> {
    let mut cases = HashMap::new();

    let case_one = BillerCase {
        case_id: "BILLER-1001".to_string(),
        patient_name: "Alex Kim".to_string(),
        mrn: "MRN-1010".to_string(),
        claim_id: "CLM-001".to_string(),
        owner: "operator-a".to_string(),
        status: "open".to_string(),
        queue_state: "ready_for_action".to_string(),
        unresolved_actions: vec![
            "validate".to_string(),
            "submit_draft".to_string(),
        ],
        risk_hints: vec!["duplicate diagnosis".to_string()],
        timeline: vec![
            TimelineEvent {
                timestamp: "2026-03-30T08:00:00Z".to_string(),
                actor: "inference-service".to_string(),
                state: "received".to_string(),
                detail: "case_ingested_from_transit".to_string(),
            },
            TimelineEvent {
                timestamp: "2026-03-30T08:01:00Z".to_string(),
                actor: "verifier".to_string(),
                state: "queued".to_string(),
                detail: "awaiting_bill_action".to_string(),
            },
        ],
        evidence_links: vec![
            "cloud://transit/evidence/BILLER-1001/patient-summary".to_string(),
        ],
    };
    cases.insert(case_one.case_id.clone(), case_one);

    let case_two = BillerCase {
        case_id: "BILLER-1002".to_string(),
        patient_name: "Morgan Chen".to_string(),
        mrn: "MRN-2040".to_string(),
        claim_id: "CLM-002".to_string(),
        owner: "operator-b".to_string(),
        status: "open".to_string(),
        queue_state: "needs_review".to_string(),
        unresolved_actions: vec![
            "validate".to_string(),
            "retry".to_string(),
            "hold".to_string(),
        ],
        risk_hints: vec!["missing supporting note".to_string()],
        timeline: vec![TimelineEvent {
            timestamp: "2026-03-30T09:00:00Z".to_string(),
            actor: "transit-ingress".to_string(),
            state: "received".to_string(),
            detail: "new_queue_item".to_string(),
        }],
        evidence_links: vec![
            "cloud://transit/evidence/BILLER-1002/patient-summary".to_string(),
        ],
    };
    cases.insert(case_two.case_id.clone(), case_two);

    cases
}

fn payer_default_cases() -> HashMap<String, PayerDenialCase> {
    let mut cases = HashMap::new();

    let case_one = PayerDenialCase {
        case_id: "DENIAL-3001".to_string(),
        patient_name: "Jordan Lee".to_string(),
        payer: "UnitedCare".to_string(),
        denial_reason: "Missing prior auth".to_string(),
        status: "new".to_string(),
        age_days: 4,
        denial_history: vec!["initial_submission_rejected".to_string()],
        prior_attempts: 1,
        timeline: vec![
            TimelineEvent {
                timestamp: "2026-03-28T12:00:00Z".to_string(),
                actor: "payer-gateway".to_string(),
                state: "denial_received".to_string(),
                detail: "payer returned reason code CO-17".to_string(),
            },
            TimelineEvent {
                timestamp: "2026-03-29T10:12:00Z".to_string(),
                actor: "recovery-agent".to_string(),
                state: "queued_for_triage".to_string(),
                detail: "added_to_payer_queue".to_string(),
            },
        ],
        evidence_links: vec![
            "cloud://transit/evidence/DENIAL-3001/denial-record".to_string(),
            "cloud://transit/evidence/DENIAL-3001/auth-request".to_string(),
        ],
    };
    cases.insert(case_one.case_id.clone(), case_one);

    let case_two = PayerDenialCase {
        case_id: "DENIAL-3002".to_string(),
        patient_name: "Riley Patel".to_string(),
        payer: "MediTrust".to_string(),
        denial_reason: "Procedure mismatch".to_string(),
        status: "open".to_string(),
        age_days: 11,
        denial_history: vec![
            "first_reject".to_string(),
            "reviewed_without_correction".to_string(),
        ],
        prior_attempts: 2,
        timeline: vec![TimelineEvent {
            timestamp: "2026-03-26T08:00:00Z".to_string(),
            actor: "payer-gateway".to_string(),
            state: "denial_received".to_string(),
            detail: "payer returned reason code PR-22".to_string(),
        }],
        evidence_links: vec![
            "cloud://transit/evidence/DENIAL-3002/denial-record".to_string(),
        ],
    };
    cases.insert(case_two.case_id.clone(), case_two);

    cases
}

fn seeded_state() -> AppState {
    AppState {
        biller_cases: biller_default_cases(),
        payer_cases: payer_default_cases(),
        idempotency: HashMap::new(),
        audit_log: HashMap::new(),
    }
}

fn biller_transition(
    case_record: &mut BillerCase,
    command: &str,
    actor: &str,
) -> Result<(String, String), String> {
    match command {
        "validate" => {
            case_record.status = "validated".to_string();
            case_record.queue_state = "validation_complete".to_string();
            case_record
                .unresolved_actions
                .retain(|candidate| candidate != "validate");
            case_record.timeline.push(TimelineEvent {
                timestamp: now_iso_now(),
                actor: actor.to_string(),
                state: "validated".to_string(),
                detail: "manual_validation_executed".to_string(),
            });
            Ok(("validated".to_string(), "validation_complete".to_string()))
        }
        "retry" => {
            case_record.status = "retrying".to_string();
            case_record.queue_state = "retry_pending".to_string();
            Ok(("retrying".to_string(), "retry_pending".to_string()))
        }
        "escalate" => {
            case_record.status = "escalated".to_string();
            case_record.queue_state = "manual_review_required".to_string();
            Ok(("escalated".to_string(), "manual_review_required".to_string()))
        }
        "hold" => {
            case_record.queue_state = "on_hold".to_string();
            case_record.status = "held".to_string();
            Ok(("held".to_string(), "on_hold".to_string()))
        }
        "submit_draft" => {
            case_record.status = "draft_submitted".to_string();
            case_record.queue_state = "submitted".to_string();
            case_record
                .unresolved_actions
                .retain(|candidate| candidate != "submit_draft");
            case_record.timeline.push(TimelineEvent {
                timestamp: now_iso_now(),
                actor: actor.to_string(),
                state: "draft_submitted".to_string(),
                detail: "conversation_command_executed".to_string(),
            });
            Ok(("draft_submitted".to_string(), "submitted".to_string()))
        }
        _ => Err(format!("unsupported biller command: {command}")),
    }
}

fn biller_transition_preview(
    case_record: &BillerCase,
    command: &str,
    actor: &str,
) -> Result<(String, String), String> {
    let mut preview = case_record.clone();
    biller_transition(&mut preview, command, actor)
}

fn payer_transition(
    case_record: &mut PayerDenialCase,
    command: &str,
    actor: &str,
) -> Result<(String, String), String> {
    match command {
        "triage" => {
            case_record.status = "triaged".to_string();
            case_record.timeline.push(TimelineEvent {
                timestamp: now_iso_now(),
                actor: actor.to_string(),
                state: "triaged".to_string(),
                detail: "payer_queue_triaged".to_string(),
            });
            Ok(("triaged".to_string(), "triage_in_progress".to_string()))
        }
        "draft_appeal" => {
            case_record.status = "appeal_draft".to_string();
            case_record.timeline.push(TimelineEvent {
                timestamp: now_iso_now(),
                actor: actor.to_string(),
                state: "appeal_draft".to_string(),
                detail: "appeal_package_draft_started".to_string(),
            });
            Ok(("appeal_draft".to_string(), "appeal_building".to_string()))
        }
        "submit_response" => {
            case_record.status = "ready_for_submission".to_string();
            case_record
                .denial_history
                .push("appeal_response_generated".to_string());
            case_record.timeline.push(TimelineEvent {
                timestamp: now_iso_now(),
                actor: actor.to_string(),
                state: "response_package_ready".to_string(),
                detail: "response_package_written".to_string(),
            });
            Ok((
                "ready_for_submission".to_string(),
                "response_submitted".to_string(),
            ))
        }
        _ => Err(format!("unsupported payer command: {command}")),
    }
}

fn payer_transition_preview(
    case_record: &PayerDenialCase,
    command: &str,
    actor: &str,
) -> Result<(String, String), String> {
    let mut preview = case_record.clone();
    payer_transition(&mut preview, command, actor)
}

fn role_next_actions(role: Role) -> Vec<String> {
    role.allowed_commands().iter().map(|value| value.to_string()).collect()
}

fn unauthorized(
    request_id: String,
    message: impl Into<String>,
    recoverable: bool,
    next_allowed: &[&str],
) -> (StatusCode, Json<ErrorEnvelope>) {
    (
        StatusCode::FORBIDDEN,
        Json(ErrorEnvelope {
            request_id,
            error: ErrorPayload {
                code: "FORBIDDEN",
                message: message.into(),
                recoverable,
                next_allowed_actions: next_allowed.iter().map(|value| value.to_string()).collect(),
            },
        }),
    )
}

fn bad_request(
    request_id: String,
    message: impl Into<String>,
    recoverable: bool,
    next_allowed: &[&str],
) -> (StatusCode, Json<ErrorEnvelope>) {
    (
        StatusCode::BAD_REQUEST,
        Json(ErrorEnvelope {
            request_id,
            error: ErrorPayload {
                code: "BAD_REQUEST",
                message: message.into(),
                recoverable,
                next_allowed_actions: next_allowed.iter().map(|value| value.to_string()).collect(),
            },
        }),
    )
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "baggins",
        "request_id": make_hash(&[&now_iso_now(), "health"]),
        "version": "0.1.0",
    }))
}

async fn biller_search(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Query(params): Query<BillerSearchQuery>,
) -> impl IntoResponse {
    let request_id = make_hash(&[&now_iso_now(), "biller_search"]);
    let role = Role::from_headers(&headers);
    if !role.can_search_biller() {
        return unauthorized(request_id, "biller search requires biller role", true, &["biller"]).into_response();
    }

    let limit = params.limit.unwrap_or(DEFAULT_LIMIT).max(1).min(100);
    let cursor = params.cursor;

    let state = state.read().await;
    let mut matches = state
        .biller_cases
        .values()
        .filter(|case_data| {
            let text_filter = params.q.as_ref().map_or(true, |query| {
                let lowered = query.to_lowercase();
                case_data.patient_name.to_lowercase().contains(&lowered)
                    || case_data.mrn.to_lowercase().contains(&lowered)
                    || case_data.claim_id.to_lowercase().contains(&lowered)
            });
            let status_filter = params.status.as_ref().map_or(true, |status| {
                case_data.status.eq_ignore_ascii_case(status)
            });
            let owner_filter = params.owner.as_ref().map_or(true, |owner| {
                case_data.owner.eq_ignore_ascii_case(owner)
            });
            text_filter && status_filter && owner_filter
        })
        .cloned()
        .collect::<Vec<_>>();

    matches.sort_by_key(biller_sort_key);
    let (items, next_cursor) = paginate(matches, cursor, limit);

    (
        StatusCode::OK,
        Json(PagedResponse {
            request_id,
            items,
            next_cursor,
            count: items.len(),
        }),
    )
        .into_response()
}

async fn payer_search(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Query(params): Query<PayerSearchQuery>,
) -> impl IntoResponse {
    let request_id = make_hash(&[&now_iso_now(), "payer_search"]);
    let role = Role::from_headers(&headers);
    if !role.can_search_payer() {
        return unauthorized(request_id, "payer search requires payer role", true, &["payer"]).into_response();
    }

    let limit = params.limit.unwrap_or(DEFAULT_LIMIT).max(1).min(100);
    let cursor = params.cursor;

    let state = state.read().await;
    let mut matches = state
        .payer_cases
        .values()
        .filter(|case_data| {
            let payer_filter = params.payer.as_ref().map_or(true, |payer| {
                case_data.payer.eq_ignore_ascii_case(payer)
            });
            let reason_filter = params.reason.as_ref().map_or(true, |reason| {
                case_data
                    .denial_reason
                    .to_lowercase()
                    .contains(&reason.to_lowercase())
            });
            let status_filter = params.status.as_ref().map_or(true, |status| {
                case_data.status.eq_ignore_ascii_case(status)
            });
            let min_age_filter = params.age_min.map_or(true, |min_age| case_data.age_days >= min_age);
            let max_age_filter = params.age_max.map_or(true, |max_age| case_data.age_days <= max_age);
            payer_filter && reason_filter && status_filter && min_age_filter && max_age_filter
        })
        .cloned()
        .collect::<Vec<_>>();

    matches.sort_by_key(payer_sort_key);
    let (items, next_cursor) = paginate(matches, cursor, limit);

    (
        StatusCode::OK,
        Json(PagedResponse {
            request_id,
            items,
            next_cursor,
            count: items.len(),
        }),
    )
        .into_response()
}

async fn get_case(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Path(case_id): Path<String>,
) -> impl IntoResponse {
    let request_id = make_hash(&[&now_iso_now(), &case_id, "get_case"]);
    let role = Role::from_headers(&headers);
    if !role.can_read_case() {
        return unauthorized(request_id, "case retrieval requires authenticated role", true, &["admin", "biller", "payer"])
            .into_response();
    }

    let state = state.read().await;
    if let Some(case_record) = state.biller_cases.get(&case_id) {
        let payload = serde_json::to_value(case_record).ok();
        if let Some(payload) = payload {
            return (
                StatusCode::OK,
                Json(CaseEnvelope {
                    request_id,
                    case_id: case_record.case_id.clone(),
                    case_type: "biller".to_string(),
                    case_data: payload,
                    timeline_last_100: case_record.timeline.iter().rev().take(100).cloned().collect(),
                }),
            )
                .into_response();
        }
    }

    if let Some(case_record) = state.payer_cases.get(&case_id) {
        let payload = serde_json::to_value(case_record).ok();
        if let Some(payload) = payload {
            return (
                StatusCode::OK,
                Json(CaseEnvelope {
                    request_id,
                    case_id: case_record.case_id.clone(),
                    case_type: "payer_denial".to_string(),
                    case_data: payload,
                    timeline_last_100: case_record.timeline.iter().rev().take(100).cloned().collect(),
                }),
            )
                .into_response();
        }
    }

    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "request_id": request_id,
            "error": {
                "code": "CASE_NOT_FOUND",
                "message": "no case matches requested case_id",
                "recoverable": false,
                "next_allowed_actions": Vec::<String>::new(),
            }
        })),
    )
        .into_response()
}

async fn get_case_audit(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Path(case_id): Path<String>,
) -> impl IntoResponse {
    let request_id = make_hash(&[&now_iso_now(), &case_id, "get_case_audit"]);
    let role = Role::from_headers(&headers);
    if !role.can_read_case() {
        return unauthorized(request_id, "audit retrieval requires authenticated role", true, &["admin", "biller", "payer"])
            .into_response();
    }

    let state = state.read().await;
    let audit = state.audit_log.get(&case_id).cloned().unwrap_or_default();
    (
        StatusCode::OK,
        Json(AuditResponse {
            request_id,
            case_id,
            audit,
        }),
    )
        .into_response()
}

async fn perform_action(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Path(case_id): Path<String>,
    Json(payload): Json<ActionRequest>,
) -> impl IntoResponse {
    let role = Role::from_headers(&headers);
    let actor = headers
        .get("x-actor-id")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("anonymous")
        .to_string();
    let request_id = payload.request_id.unwrap_or_else(|| make_hash(&[&now_iso_now(), &case_id, &actor]));
    let params_hash = value_hash(&payload.params);
    let idempotency = payload.idempotency_key.clone().unwrap_or_else(|| {
        make_hash(&[&actor, &case_id, &payload.command, &params_hash, &request_id])
    });
    let idempotency_key = format!("{actor}:{case_id}:{}", idempotency);
    let mut state_guard = state.write().await;

    if !role.command_allowed(&payload.command) {
        return bad_request(
            request_id,
            format!("command {} is not permitted for role {}", payload.command, role.as_str()),
            false,
            &role.allowed_commands(),
        )
        .into_response();
    }

    if let Some(stored) = state_guard.idempotency.get(&idempotency_key) {
        let mut replay = stored.clone();
        replay.replayed = true;
        return (
            StatusCode::OK,
            Json(ActionResponse {
                request_id: replay.request_id,
                trace_id: replay.trace_id,
                command_id: replay.command_id,
                transition_id: replay.transition_id,
                case_id: replay.case_id,
                command: replay.command,
                replayed: true,
                command_outcome: replay.command_outcome,
                resulting_status: replay.resulting_status,
                resulting_queue_state: replay.resulting_queue_state,
                next_allowed_actions: replay.next_allowed_actions,
                preview: false,
                audit_id: Some(replay.command_id.clone()),
            }),
        )
            .into_response();
    }

    let trace_id = make_hash(&[&now_iso_now(), &actor, &payload.command, &case_id]);
    let command_id = make_hash(&[&case_id, &payload.command, &request_id, &trace_id]);
    let transition_id = make_hash(&[&command_id, "transition"]);
    let command_outcome = if payload.confirm {
        "executed".to_string()
    } else {
        "preview".to_string()
    };
    let result: Result<(String, String), String> = if payload.confirm {
        if let Some(case_record) = state_guard.biller_cases.get_mut(&case_id) {
            biller_transition(case_record, &payload.command, &actor)
        } else if let Some(case_record) = state_guard.payer_cases.get_mut(&case_id) {
            payer_transition(case_record, &payload.command, &actor)
        } else {
            Err(format!("case {case_id} not found"))
        }
    } else if let Some(case_record) = state_guard.biller_cases.get(&case_id) {
        biller_transition_preview(case_record, &payload.command, &actor)
    } else if let Some(case_record) = state_guard.payer_cases.get(&case_id) {
        payer_transition_preview(case_record, &payload.command, &actor)
    } else {
        Err(format!("case {case_id} not found"))
    };

    let (resulting_status, resulting_queue) = match result {
        Ok((resulting_status, resulting_queue)) => (resulting_status, resulting_queue),
        Err(error_message) => {
            return bad_request(
                request_id,
                error_message,
                true,
                &role.allowed_commands(),
            )
            .into_response();
        }
    };

    let next_actions = role_next_actions(role);
    let mut audit_id = None;

    if payload.confirm {
        let audit = AuditRecord {
            audit_id: command_id.clone(),
            trace_id: trace_id.clone(),
            request_id: request_id.clone(),
            actor: actor.clone(),
            role: role.as_str().to_string(),
            command: payload.command.clone(),
            case_id: case_id.clone(),
            params_hash,
            outcome_code: "OK".to_string(),
            occurred_at: now_iso_now(),
        };
        state_guard
            .audit_log
            .entry(case_id.clone())
            .or_default()
            .push(audit.clone());
        audit_id = Some(audit.audit_id.clone());
    }

    let response = ActionReceipt {
        request_id: request_id.clone(),
        trace_id: trace_id.clone(),
        command_id: command_id.clone(),
        transition_id: transition_id.clone(),
        case_id: case_id.clone(),
        command: payload.command.clone(),
        command_outcome: command_outcome.clone(),
        resulting_status: resulting_status.clone(),
        resulting_queue_state: resulting_queue.clone(),
        next_allowed_actions: next_actions.clone(),
        replayed: false,
    };

    if payload.confirm {
        state_guard.idempotency.insert(idempotency_key.clone(), response);
    }

    (
        StatusCode::OK,
        Json(ActionResponse {
            request_id,
            trace_id,
            command_id,
            transition_id,
            case_id,
            command: payload.command,
            replayed: false,
            command_outcome,
            resulting_status,
            resulting_queue_state: resulting_queue,
            next_allowed_actions: next_actions,
            preview: !payload.confirm,
            audit_id,
        }),
    )
        .into_response()
}

pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let state: SharedState = Arc::new(RwLock::new(seeded_state()));
    let app = Router::new()
        .route("/v1/health", get(health_check))
        .route("/v1/biller/search", get(biller_search))
        .route("/v1/payer/denials/search", get(payer_search))
        .route("/v1/cases/:case_id", get(get_case))
        .route("/v1/cases/:case_id/action", post(perform_action))
        .route("/v1/cases/:case_id/audit", get(get_case_audit))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(SERVER_LISTEN_ADDR).await?;
    let addr: SocketAddr = listener.local_addr()?;
    println!("baggins server listening on http://{addr}");
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::{make_hash, Role};

    #[test]
    fn role_policy_is_deterministic() {
        assert_eq!(Role::Admin.allowed_commands().len(), 8);
        assert!(Role::Unknown.allowed_commands().is_empty());
        let first = make_hash(&["a", "b", "c"]);
        let second = make_hash(&["a", "b", "c"]);
        assert_eq!(first, second);
    }
}
