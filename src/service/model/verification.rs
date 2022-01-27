use ic_kit::candid::CandidType;
use serde::Deserialize;

use crate::common::types::{CallerId, CanisterId};

#[derive(CandidType, Deserialize)]
pub struct SubmitVerification {
    pub owner_id: CallerId,
    pub canister_id: CanisterId,
    pub canister_name: String,
    pub repo_url: String,
    pub commit_hash: String,
    pub wasm_hash: Option<String>,
    pub build_url: String,
    pub build_status: BuildStatus,
    pub rust_version: Option<String>,
    pub dfx_version: String,
    pub optimize_count: u8,
}

#[derive(CandidType, Deserialize, PartialEq, Debug)]
pub struct Verification {
    pub canister_id: CanisterId,
    pub canister_name: String,
    pub repo_url: String,
    pub commit_hash: String,
    pub wasm_hash: Option<String>,
    pub build_url: String,
    pub build_status: BuildStatus,
    pub rust_version: Option<String>,
    pub dfx_version: String,
    pub optimize_count: u8,
    pub updated_by: CallerId,
    pub updated_at: String,
}

#[derive(CandidType, PartialEq, Deserialize, Debug)]
pub enum BuildStatus {
    Error,
    Success,
}
