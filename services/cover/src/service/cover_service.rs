use super::{get_validation_registry, get_validation_registry_mut};
use crate::common::types::{CanisterId, ValidationId};
use crate::service::constants::{Error, Ok};
use crate::service::types::{NewValidationRequest, ValidationRequest, BuildParams};
use crate::service::utils::ValidationResult;
use ic_kit::ic::{caller, id};
use ic_kit::interfaces::Method;
use std::ops::Not;
use crate::service::store::error::ErrorKind;

/// Builder API
///   - request_validation
pub fn request_validation(req: NewValidationRequest) -> ValidationResult<()> {
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

/// ValidationAPI
///  - fresh_validations
///  - fetch_validation
///  - update_validation

/// Return list of unprocessed requested canister_ids
pub fn fresh_validations() -> Vec<CanisterId> {
  let reg = get_validation_registry();
  reg.list_fresh().iter().map(|&v| v.clone()).collect()
}

/// Fetch request from fresh list, mark it as fetched
/// returns ValidationRequest
pub fn fetch_request(canister_id: &CanisterId) -> ValidationResult<ValidationRequest> {
  let val = get_validation_registry_mut().fetch_request(&canister_id).unwrap();
  ValidationResult::data(val.clone())
}
