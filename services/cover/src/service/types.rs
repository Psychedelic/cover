use crate::common::types::{CanisterId, RequestId, CallerId};
use ic_cdk::export::candid::CandidType;
use serde::{Deserialize, Serialize};

/// Parameters required to execute a canister build
#[derive(CandidType, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct BuildParams {
    pub git_ref: String,
    pub git_tag: String,
    // pub docker: String, // fleek-rust-1, fleek-motoko
    // pub build_path: String, // ./build.sh
}

/// Parameters gathered during validation build
#[derive(CandidType, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct ValidationResponse {
    pub request_id: RequestId,
    pub canister_id: CanisterId,
    pub validator_id: Option<CallerId>,
    pub validation_started_at: String,
    pub validation_completed_at: String,
    pub git_checksum: String,
    pub canister_checksum: String,
    pub wasm_checksum: String,
    pub build_log_url: String,
    pub source_snapshot_url: String,
    pub status: String, // TODO use Enum
}

#[derive(CandidType, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct ValidationRequest {
    pub request_id: Option<RequestId>,
    pub canister_id: CanisterId,
    pub caller_id: CallerId,
    pub build_settings: BuildParams,
    // pub validation: Option<ValidationResponse>,
    pub fetched: bool, // TODO refactor to fetched_at
}

impl ValidationRequest {
    pub fn mark_fetched(&mut self) -> &Self {
        self.fetched = true;
        self
    }
}

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct NewValidationRequest {
    pub canister_id: CanisterId,
    pub build_settings: BuildParams,
}
