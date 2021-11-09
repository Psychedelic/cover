use ic_cdk::export::candid::CandidType;
use serde::Deserialize;

use crate::common::types::{CallerId, CanisterId, ReqId};
use crate::service::store::error::ErrorKind;

#[derive(CandidType, Deserialize, Debug, PartialEq, Clone)]
pub struct BuildSettings {
    pub git_ref: String,
    pub git_tag: String,
}

#[derive(CandidType, Deserialize, Debug, PartialEq)]
pub enum ProgressStatus {
    Init,
    InProgress,
    Finished,
    Error,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct UpdateProgress {
    pub request_id: ReqId,
    pub canister_id: CanisterId,
    pub git_checksum: Option<String>,
    pub canister_checksum: Option<String>,
    pub wasm_checksum: Option<String>,
    pub build_log_url: Option<String>,
    pub source_snapshot_url: Option<String>,
    pub percentage: Option<f32>,
    pub status: ProgressStatus,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct Progress {
    pub request_id: ReqId,
    pub canister_id: CanisterId,
    pub started_at: String,
    pub updated_at: Option<String>,
    pub git_checksum: Option<String>,
    pub canister_checksum: Option<String>,
    pub wasm_checksum: Option<String>,
    pub build_log_url: Option<String>,
    pub source_snapshot_url: Option<String>,
    pub percentage: Option<f32>,
    pub status: ProgressStatus,
}

#[derive(CandidType, Debug, PartialEq, Clone)]
pub struct Request {
    pub request_id: ReqId,
    pub caller_id: CallerId,
    pub canister_id: CanisterId,
    pub build_settings: BuildSettings,
    pub created_at: String,
}

// TODO: define details
#[derive(CandidType, Deserialize, Debug, PartialEq)]
pub struct ProviderInfo {}

#[derive(CandidType, Deserialize)]
pub struct CreateRequest {
    pub canister_id: CanisterId,
    pub build_settings: BuildSettings,
}

#[derive(CandidType, Deserialize, Debug, PartialEq)]
pub struct Error {
    code: &'static str,
    message: &'static str,
    debug_log: Option<String>,
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        match kind {
            ErrorKind::RequestNotFound => Self {
                code: "ERR_001_001",
                message: "Request not found",
                debug_log: None,
            },
            ErrorKind::ProgressNotFound => Self {
                code: "ERR_002_001",
                message: "Progress not found",
                debug_log: None,
            },
            ErrorKind::InitExistedProgress => Self {
                code: "ERR_002_002",
                message: "Init existed Progress",
                debug_log: None,
            },
            ErrorKind::InvalidProgressStatus => Self {
                code: "ERR_002_003",
                message: "Invalid progress status",
                debug_log: None,
            },
        }
    }
}
