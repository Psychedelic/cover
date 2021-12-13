use ic_kit::candid::CandidType;
use serde::Deserialize;

use crate::common::types::{CallerId, CanisterId, ReqId};

#[derive(CandidType, Deserialize, Debug, PartialEq, Clone)]
pub struct Request {
    pub request_id: ReqId,
    pub canister_id: CanisterId,
    pub build_settings: BuildSettings,
    pub created_by: CallerId,
    pub created_at: String,
}

#[derive(CandidType, Deserialize)]
pub struct CreateRequest {
    pub canister_id: CanisterId,
    pub build_settings: BuildSettings,
}

#[derive(CandidType, Deserialize, Debug, PartialEq, Clone)]
pub struct BuildSettings {
    pub git_ref: String,
    pub git_repo: String,
    pub git_sha: String,
}
