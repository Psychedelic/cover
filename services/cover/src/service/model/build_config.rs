use crate::common::types::CallerId;
use crate::CanisterId;
use ic_kit::candid::CandidType;
use serde::Deserialize;

#[derive(PartialEq, CandidType, Deserialize, Debug)]
pub struct BuildConfig {
    pub user_id: CallerId,
    pub canister_id: CanisterId,
    pub canister_name: String,
    pub repo_url: String,
    pub user_repo_token: String,
    pub commit_hash: String,
    pub rust_version: String,
    pub dfx_version: String,
    pub optimize_times: u8,
    pub created_at: String,
    pub updated_at: String,
}
