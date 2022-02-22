use ic_kit::candid::CandidType;
use serde::Deserialize;

use crate::common::types::{CanisterId, CanisterOwnerId};

#[derive(CandidType, Deserialize)]
pub struct SubmitVerification {
    pub owner_id: CanisterOwnerId,
    pub canister_id: CanisterId,
    pub canister_name: String,
    pub repo_url: String,
    pub commit_hash: String,
    pub wasm_hash: Option<String>,
    pub build_url: String,
    pub build_status: BuildStatus,
    pub canister_type: Option<CanisterType>,
    pub rust_version: Option<String>,
    pub dfx_version: String,
    pub optimize_count: u8,
    pub repo_visibility: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub struct RegisterVerification {
    pub owner_id: CanisterOwnerId,
    pub canister_id: CanisterId,
    pub canister_name: String,
    pub repo_url: String,
    pub commit_hash: String,
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
    pub build_url: Option<String>,
    pub build_status: BuildStatus,
    pub canister_type: Option<CanisterType>,
    pub rust_version: Option<String>,
    pub dfx_version: String,
    pub optimize_count: u8,
    pub repo_visibility: Option<String>,
    pub updated_by: CanisterOwnerId,
    pub updated_at: String,
}

#[derive(CandidType, PartialEq, Deserialize, Debug, Copy, Clone)]
pub enum BuildStatus {
    Pending,
    Building,
    Error,
    Success,
}

#[derive(CandidType, PartialEq, Deserialize, Debug, Copy, Clone)]
pub enum CanisterType {
    Rust,
    Motoko,
}
