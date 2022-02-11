use ic_kit::candid::CandidType;
use serde::Deserialize;

use crate::common::types::{CanisterId, CanisterOwnerId};

#[derive(PartialEq, CandidType, Deserialize, Debug)]
pub struct BuildConfig {
    pub owner_id: CanisterOwnerId,
    pub canister_id: CanisterId,
    pub canister_name: String,
    pub repo_url: String,
    pub repo_access_token: String,
    pub commit_hash: String,
    pub rust_version: Option<String>,
    pub dfx_version: String,
    pub public_key: String,
    pub signature: String,
    pub optimize_count: u8,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(CandidType, Deserialize)]
pub struct SaveBuildConfig {
    pub owner_id: CanisterOwnerId,
    pub canister_id: CanisterId,
    pub canister_name: String,
    pub repo_url: String,
    pub repo_access_token: String,
    pub commit_hash: String,
    pub rust_version: Option<String>,
    pub dfx_version: String,
    pub public_key: String,
    pub signature: String,
    pub optimize_count: u8,
}

#[derive(CandidType, Deserialize)]
pub struct BuildConfigInfo {
    pub owner_id: CanisterOwnerId,
    pub canister_id: CanisterId,
}
