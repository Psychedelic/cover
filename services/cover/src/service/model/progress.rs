use ic_kit::candid::CandidType;
use serde::Deserialize;

use crate::common::types::{CanisterId, ReqId};

#[derive(CandidType, Deserialize, Debug, PartialEq)]
pub enum ProgressStatus {
    Init,
    InProgress,
    Finished,
    Error,
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
