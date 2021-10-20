use crate::common::types::{CanisterId, Controller, ValidationId, CallerId};
use ic_cdk::export::candid::CandidType;
use serde::Deserialize;
use std::time::SystemTime;

#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct BuildParams {
  pub git_ref: String,
  pub git_sha: String,
}

#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct Validation {
  pub caller_id: CallerId,
  pub build_settings: BuildParams,
  pub fetched: bool,
}

impl Validation {

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

