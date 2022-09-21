use crate::common::types::{CanisterId, CanisterOwnerId};
use ic_cdk::export::candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct BuildConfig {
    pub owner_id: CanisterOwnerId,
    pub delegate_canister_id: Option<CanisterId>,
    pub canister_id: CanisterId,
    pub canister_name: String,
    pub repo_url: String,
    pub commit_hash: String,
    pub rust_version: Option<String>,
    pub dfx_version: String,
    pub optimize_count: u8,
    pub updated_at: u64, // Current system time given as nanoseconds since 1970-01-01
}

#[derive(CandidType, Deserialize)]
pub struct SaveBuildConfig {
    pub owner_id: CanisterOwnerId,
    pub delegate_canister_id: Option<CanisterId>,
    pub canister_id: CanisterId,
    pub canister_name: String,
    pub repo_url: String,
    pub commit_hash: String,
    pub rust_version: Option<String>,
    pub dfx_version: String,
    pub optimize_count: u8,
}

#[derive(CandidType, Deserialize)]
pub struct BuildConfigInfo {
    pub owner_id: CanisterOwnerId,
    pub canister_id: CanisterId,
}
