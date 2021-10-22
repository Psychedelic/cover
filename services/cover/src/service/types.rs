use crate::common::types::{CanisterId, ValidationId, CallerId};
use ic_cdk::export::candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct BuildParams {
  pub git_ref: String,
  pub git_sha: String,
}

#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct ValidationRequest {
  pub validation_id: Option<ValidationId>,
  pub canister_id: CanisterId,
  pub caller_id: CallerId,
  pub build_settings: BuildParams,
  pub fetched: bool,
}

#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct ValidationResult {
  validation_id: ValidationId,
  pub build_checksum: String,
  pub wasm_checksum: String,
  pub build_log_url: String,
  pub source_snapshot_url: String,
  pub status: String,
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

