use crate::common::types::{CallerId, CanisterId, ReqId};
use crate::service::store::error::ErrorKind;
use ic_cdk::export::candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug, PartialEq, Clone)]
pub struct BuildSettings {
    pub git_ref: String,
    pub git_tag: String,
}

// pub struct ValidationResponse {
//    pub request_id: ReqId,
//    pub validator_id: Option<CallerId>,
//    pub validation_started_at: String,
//    pub validation_completed_at: String,
//    pub git_checksum: String,
//    pub canister_checksum: String,
//    pub wasm_checksum: String,
//    pub build_log_url: String,
//    pub source_snapshot_url: String,
//    pub status: String, // TODO use Enum
// }

#[derive(CandidType, Deserialize)]
pub struct NewValidationRequest {
    pub canister_id: CanisterId,
    pub build_settings: BuildSettings,
}

#[derive(CandidType, Debug, PartialEq, Clone)]
pub struct ValidationRequest {
    pub request_id: ReqId,
    pub caller_id: CallerId,
    pub canister_id: CanisterId,
    pub build_settings: BuildSettings,
    //  created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(CandidType, Deserialize, Debug, PartialEq)]
pub struct ProviderInfo {}

#[derive(CandidType, Deserialize, Debug, PartialEq)]
pub struct Error {
    code: &'static str,
    message: &'static str,
    debug_log: Option<String>,
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        match kind {
            ErrorKind::PendingRequestNotFound => Self {
                code: "ERR_001_001",
                message: "Pending request not found",
                debug_log: None,
            },
        }
    }
}
