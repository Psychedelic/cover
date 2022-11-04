use ic_cdk::export::candid::CandidType;
use serde::Deserialize;

use crate::common::types::{CallerId, CanisterId};

#[derive(CandidType, Deserialize)]
pub struct SubmitVerification {
    pub caller_id: CallerId,
    pub delegate_canister_id: Option<CanisterId>,
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
    pub repo_visibility: String,
}

#[derive(CandidType, Deserialize)]
pub struct RegisterVerification {
    pub caller_id: CallerId,
    pub delegate_canister_id: Option<CanisterId>,
    pub canister_id: CanisterId,
    pub canister_name: String,
    pub repo_url: String,
    pub commit_hash: String,
    pub rust_version: Option<String>,
    pub dfx_version: String,
    pub optimize_count: u8,
    pub repo_visibility: String,
}

#[derive(CandidType, Deserialize)]
pub struct Verification {
    pub delegate_canister_id: Option<CanisterId>,
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
    pub repo_visibility: String,
    pub updated_by: CallerId,
    pub updated_at: u64,
}

#[derive(CandidType, Deserialize, Clone, Copy)]
pub enum BuildStatus {
    Pending,
    Building,
    Error,
    Success,
}

#[derive(CandidType, Deserialize, Clone, Copy)]
pub enum CanisterType {
    Rust,
    Motoko,
    Custom,
    Assets,
}
