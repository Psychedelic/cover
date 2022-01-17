use ic_kit::candid::CandidType;
use serde::Deserialize;

use crate::common::types::{CallerId, CanisterId};

#[derive(CandidType, Deserialize)]
pub struct SubmitVerification {
    pub canister_id: CanisterId,
    pub canister_name: String,
    pub repo_url: String,
    pub commit_hash: String,
    pub wasm_hash: String,
    pub rust_version: String,
    pub dfx_version: String,
    pub optimize_count: u8,
}

#[derive(CandidType, Deserialize, PartialEq, Debug)]
pub struct Verification {
    pub canister_id: CanisterId,
    pub canister_name: String,
    pub repo_url: String,
    pub commit_hash: String,
    pub wasm_hash: String,
    pub rust_version: String,
    pub dfx_version: String,
    pub optimize_count: u8,
    pub created_by: CallerId,
    pub created_at: String,
    pub updated_by: CallerId,
    pub updated_at: String,
}
