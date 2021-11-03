use crate::common::types::{CallerId, ReqId};

#[derive(Debug, PartialEq, Clone)]
pub struct BuildSettings {
    pub git_ref: String,
    pub git_tag: String,
}

#[derive(Debug, PartialEq)]
pub struct ValidationResponse {
    pub request_id: ReqId,
    pub validator_id: Option<CallerId>,
    pub validation_started_at: String,
    pub validation_completed_at: String,
    pub git_checksum: String,
    pub canister_checksum: String,
    pub wasm_checksum: String,
    pub build_log_url: String,
    pub source_snapshot_url: String,
    pub status: String, // TODO use Enum
}

// #[derive(CandidType, Clone, Deserialize, Debug)]
// pub struct NewValidationRequest {
//     pub canister_id: CanisterId,
//     pub build_settings: BuildSettings,
// }
