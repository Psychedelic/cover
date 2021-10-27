use crate::common::types::{CanisterId, RequestId, CallerId};
use ic_cdk::export::candid::CandidType;
use serde::{Deserialize, Serialize};

/// Parameters required to execute a canister build
#[derive(CandidType, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct BuildParams {
  pub git_ref: String,
  pub git_sha: String,
}

/// Parameters gathered during validation build
#[derive(CandidType, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct ValidationResponse {
  pub validation_started_at: String,
  pub validation_completed_at: String,
  pub git_checksum: String,
  pub wasm_checksum: String,
  pub build_log_url: String,
  pub source_snapshot_url: String,
}

#[derive(CandidType, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct ValidationRequest {
  pub request_id: Option<RequestId>,
  pub canister_id: CanisterId,
  pub caller_id: CallerId,
  pub build_settings: BuildParams,
  pub validation: Option<ValidationResponse>,
  pub fetched: bool, // TODO refactor to changed_at
  pub status: String, // TODO use Enum
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
