use ic_kit::candid::CandidType;
use serde::Deserialize;

use crate::common::types::{CallerId, CanisterId, ProviderId, ReqId};

#[derive(CandidType, Deserialize, Debug, PartialEq, Clone)]
pub struct BuildSettings {
    pub git_ref: String,
    pub git_repo: String,
    pub git_sha: String,
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
    pub git_sha: String,
    pub git_ref: String,
    pub git_repo: String,
    pub wasm_checksum: String,
    pub build_log_url: String,
    pub source_snapshot_url: String,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateVerification {
    pub canister_id: CanisterId,
    pub git_sha: String,
    pub git_ref: String,
    pub git_repo: Option<String>,
    pub wasm_checksum: String,
    pub build_log_url: String,
    pub source_snapshot_url: String,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateProgress {
    pub request_id: ReqId,
    pub canister_id: CanisterId,
    pub git_sha: Option<String>,
    pub git_ref: Option<String>,
    pub git_repo: Option<String>,
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

#[derive(CandidType, Deserialize, PartialEq)]
pub struct Provider {
    pub id: ProviderId,
    pub name: String,
    pub memo: Option<String>,
    pub created_by: CallerId,
    pub created_at: String,
    pub updated_by: CallerId,
    pub updated_at: String,
}

#[derive(CandidType, Deserialize, PartialEq)]
pub struct Verification {
    pub canister_id: CanisterId,
    pub git_sha: String,
    pub git_ref: String,
    pub git_repo: String,
    pub wasm_checksum: String,
    pub build_log_url: String,
    pub source_snapshot_url: String,
    pub created_by: CallerId,
    pub created_at: String,
    pub updated_by: CallerId,
    pub updated_at: String,
}

#[derive(CandidType, Deserialize)]
pub struct Progress {
    pub request_id: ReqId,
    pub canister_id: CanisterId,
    pub started_at: String,
    pub updated_at: Option<String>,
    pub git_sha: Option<String>,
    pub git_ref: Option<String>,
    pub git_repo: Option<String>,
    pub wasm_checksum: Option<String>,
    pub build_log_url: Option<String>,
    pub source_snapshot_url: Option<String>,
    pub percentage: Option<f32>,
    pub status: ProgressStatus,
}

#[derive(CandidType, Deserialize, Debug, PartialEq, Clone)]
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

#[derive(CandidType, Deserialize)]
pub struct Error {
    pub code: &'static str,
    pub message: &'static str,
    pub debug_log: Option<String>,
}
