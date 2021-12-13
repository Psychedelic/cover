use ic_kit::candid::CandidType;
use serde::Deserialize;

use crate::common::types::{CallerId, CanisterId};

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
    pub git_repo: String,
    pub wasm_checksum: String,
    pub build_log_url: String,
    pub source_snapshot_url: String,
}

#[derive(CandidType, Deserialize)]
pub struct SubmitVerification {
    pub canister_id: CanisterId,
    pub git_sha: String,
    pub git_ref: String,
    pub git_repo: String,
    pub wasm_checksum: String,
    pub build_log_url: String,
    pub source_snapshot_url: String,
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
