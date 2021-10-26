use super::{get_validation_registry, get_validation_registry_mut};
use crate::common::types::{CanisterId, RequestId, CallerId};
use crate::service::constants::{Error, Ok};
use crate::service::types::{NewValidationRequest, ValidationRequest, BuildParams};
use crate::service::utils::ValidationResult;
use ic_kit::ic::{caller, id};
use ic_kit::interfaces::Method;
use std::ops::Not;
use crate::service::store::error::ErrorKind;

/// Adds a new validation request to registry
pub fn add_validation_request(req: NewValidationRequest) -> ValidationResult<()> {
  let caller = caller();

  get_validation_registry_mut()
    .add_request(
      &caller,
      &req.canister_id,
      &req.build_settings,
    )
    .map(|_| ValidationResult::success(Ok::validation_request_added()))
    .unwrap_or_else(|_| ValidationResult::fail(Error::validation_requested()))
}

/// Return list of unprocessed requested canister_ids
pub fn fresh_validation_requests() -> Vec<CanisterId> {
  let reg = get_validation_registry();
  reg.list_fresh_requests().iter().map(|(can_id, val_id)| can_id.clone()).collect()
}

/// Return list of unprocessed requested canister_ids
pub fn all_validation_requests(caller: Option<&CallerId>) -> Vec<ValidationRequest> {
  let reg = get_validation_registry();
  reg.list_all_requests(caller)
}


/// Fetch request from fresh list, mark it as fetched
/// returns ValidationRequest
pub fn fetch_validation_request(canister_id: &CanisterId) -> ValidationResult<ValidationRequest> {
  let val = get_validation_registry_mut().fetch_request(&canister_id).unwrap();
  ValidationResult::data(val)
}

/// Get a validation request
/// returns ValidationRequest
pub fn get_validation_request(validation_id: RequestId) -> ValidationResult<ValidationRequest> {
  let val = get_validation_registry_mut().get_request(validation_id).unwrap();
  ValidationResult::data(val.clone())
}

