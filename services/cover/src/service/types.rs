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
  pub caller_id: Option<CallerId>,
  pub build_settings: BuildParams,
}

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct NewValidationRequest {
  pub canister_id: CanisterId,
  pub build_settings: BuildParams,
}

