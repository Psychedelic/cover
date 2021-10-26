use crate::common::types::{CanisterId, RequestId, CallerId};
use ic_cdk::export::candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct BuildParams {
  pub git_ref: String,
  pub git_sha: String,
}

// // This is what #[derive(Serialize)] would generate.
// impl Serialize for BuildParams {
//   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//   {
//     let mut s = serializer.serialize_struct("Person", 3)?;
//     s.serialize_field("git_ref", &self.name)?;
//     s.serialize_field("git_sha", &self.age)?;
//     s.end()
//   }
// }

#[derive(CandidType, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct ValidationRequest {
  pub request_id: Option<RequestId>,
  pub canister_id: CanisterId,
  pub caller_id: CallerId,
  pub build_settings: BuildParams,
  pub fetched: bool,
}

#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct ValidationResult {
  validation_id: RequestId,
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

