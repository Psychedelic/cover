use ic_cdk::export::candid::CandidType;
use serde::Deserialize;

use crate::common::types::{CallerId, CanisterId, ProviderId, ReqId};
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

#[derive(CandidType, Deserialize)]
pub struct AddProvider {
    pub id: ProviderId,
    pub name: String,
    pub memo: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateProvider {
    pub id: ProviderId,
    pub name: String,
    pub memo: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub struct AddVerification {
    pub canister_id: CanisterId,
    pub git_checksum: String,
    pub git_ref: String,
    pub wasm_checksum: String,
    pub build_log_url: String,
    pub source_snapshot_url: String,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateVerification {
    pub canister_id: CanisterId,
    pub git_checksum: String,
    pub git_ref: String,
    pub wasm_checksum: String,
    pub build_log_url: String,
    pub source_snapshot_url: String,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct UpdateProgress {
    pub request_id: ReqId,
    pub canister_id: CanisterId,
    pub git_checksum: Option<String>,
    pub git_ref: Option<String>,
    pub wasm_checksum: Option<String>,
    pub build_log_url: Option<String>,
    pub source_snapshot_url: Option<String>,
    pub percentage: Option<f32>,
    pub status: ProgressStatus,
}

#[derive(CandidType, Deserialize)]
pub struct CreateRequest {
    pub canister_id: CanisterId,
    pub build_settings: BuildSettings,
}

#[derive(CandidType, PartialEq)]
pub struct Provider {
    pub id: ProviderId,
    pub name: String,
    pub memo: Option<String>,
    pub created_by: CallerId,
    pub created_at: String,
    pub updated_by: CallerId,
    pub updated_at: String,
}

#[derive(CandidType, PartialEq)]
pub struct Verification {
    pub canister_id: CanisterId,
    pub git_checksum: String,
    pub git_ref: String,
    pub wasm_checksum: String,
    pub build_log_url: String,
    pub source_snapshot_url: String,
    pub created_by: CallerId,
    pub created_at: String,
    pub updated_by: CallerId,
    pub updated_at: String,
}

#[derive(CandidType, Debug)]
pub struct Progress {
    pub request_id: ReqId,
    pub canister_id: CanisterId,
    pub started_at: String,
    pub updated_at: Option<String>,
    pub git_checksum: Option<String>,
    pub git_ref: Option<String>,
    pub wasm_checksum: Option<String>,
    pub build_log_url: Option<String>,
    pub source_snapshot_url: Option<String>,
    pub percentage: Option<f32>,
    pub status: ProgressStatus,
}

#[derive(CandidType, Debug, PartialEq, Clone)]
pub struct Request {
    pub request_id: ReqId,
    pub canister_id: CanisterId,
    pub build_settings: BuildSettings,
    pub created_by: CallerId,
    pub created_at: String,
}

// TODO: define details
#[derive(CandidType, Deserialize, Debug, PartialEq)]
pub struct ProviderInfo {}

#[derive(CandidType, Deserialize, Debug, PartialEq)]
pub struct Error {
    code: &'static str,
    message: &'static str,
    debug_log: Option<String>,
}

// ERR_{module}_{level}_{sequence}
//      Module
//          Request               001
//          Progress              002
//          Verification          003
//          Provider              004
//      Level
//          Api                   001
//          Service               002
//          Store                 003
impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        match kind {
            ErrorKind::RequestNotFound => Self {
                code: "ERR_001_001_001",
                message: "Request not found",
                debug_log: None,
            },
            ErrorKind::ProgressNotFound => Self {
                code: "ERR_002_001_001",
                message: "Progress not found",
                debug_log: None,
            },
            ErrorKind::InitExistedProgress => Self {
                code: "ERR_002_001_002",
                message: "Init existed progress",
                debug_log: None,
            },
            ErrorKind::InvalidProgressStatus => Self {
                code: "ERR_002_001_003",
                message: "Invalid progress status",
                debug_log: None,
            },
            ErrorKind::VerificationNotFound => Self {
                code: "ERR_003_001_001",
                message: "Verification not found",
                debug_log: None,
            },
            ErrorKind::ExistedVerification => Self {
                code: "ERR_003_001_002",
                message: "Existed verification",
                debug_log: None,
            },
            ErrorKind::ProviderNotFound => Self {
                code: "ERR_004_001_001",
                message: "Provider not found",
                debug_log: None,
            },
            ErrorKind::ExistedProvider => Self {
                code: "ERR_004_001_002",
                message: "Existed provider",
                debug_log: None,
            },
        }
    }
}
