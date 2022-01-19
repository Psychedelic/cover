use ic_kit::candid::CandidType;
use serde::Deserialize;

use crate::common::types::{CallerId, CanisterId};

#[derive(PartialEq, CandidType, Deserialize, Debug)]
pub struct BuildConfig {
    pub user_id: CallerId,
    pub canister_id: CanisterId,
    pub canister_name: String,
    pub repo_url: String,
    pub commit_hash: String,
    pub rust_version: Option<String>,
    pub dfx_version: String,
    pub optimize_times: u8,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(CandidType, Deserialize)]
pub struct BuildConfigRequest {
    pub user_id: CallerId,
    pub canister_id: CanisterId,
    pub canister_name: String,
    pub repo_url: String,
    pub commit_hash: String,
    pub rust_version: Option<String>,
    pub dfx_version: String,
    pub optimize_times: u8,
}
