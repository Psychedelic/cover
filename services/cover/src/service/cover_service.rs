use super::{get_validation_registry, get_validation_registry_mut};
use crate::common::types::{CanisterId, ValidationId, CallerId};
use crate::service::constants::{Error, Ok};
use crate::service::types::{NewValidationRequest, ValidationRequest, BuildParams};
use crate::service::utils::ValidationResult;
use ic_kit::ic::{caller, id};
use ic_kit::interfaces::Method;
use std::ops::Not;
use crate::service::store::error::ErrorKind;

/// Adds a new validation request to registry
pub fn request_validation(req: NewValidationRequest) -> ValidationResult<()> {
  let caller = caller();

  get_validation_registry_mut()
    .add_validation(
      &caller,
      &req.canister_id,
      &req.build_settings,
    )
    .map(|_| ValidationResult::success(Ok::validation_request_added()))
    .unwrap_or_else(|_| ValidationResult::fail(Error::validation_requested()))
}

/// Return list of unprocessed requested canister_ids
pub fn fresh_validations() -> Vec<CanisterId> {
  let reg = get_validation_registry();
  reg.list_fresh().iter().map(|(can_id, val_id)| can_id.clone()).collect()
}

/// Return list of unprocessed requested canister_ids
pub fn all_validations(caller: Option<&CallerId>) -> Vec<ValidationRequest> {
  let reg = get_validation_registry();
  reg.list_all(caller)
}


/// Fetch request from fresh list, mark it as fetched
/// returns ValidationRequest
pub fn fetch_validation(canister_id: &CanisterId) -> ValidationResult<ValidationRequest> {
  let val = get_validation_registry_mut().fetch_validation(&canister_id).unwrap();
  ValidationResult::data(val)
}

/// Get a validation request
/// returns ValidationRequest
pub fn get_request(validation_id: ValidationId) -> ValidationResult<ValidationRequest> {
  let val = get_validation_registry_mut().get_request(validation_id).unwrap();
  ValidationResult::data(val.clone())
}

