use super::{get_validation_registry, get_validation_registry_mut};
use crate::common::types::{CanisterId, ValidationId};
use crate::service::constants::{Error, Ok};
use crate::service::types::{NewValidationRequest, Validation, BuildParams};
use crate::service::utils::ValidationResult;
use ic_kit::ic::{caller, id};
use ic_kit::interfaces::management::{
  CanisterSettings, CanisterStatus, CreateCanister, CreateCanisterArgument, InstallCode,
  InstallCodeArgument, InstallMode, WithCanisterId,
};
use ic_kit::interfaces::Method;
use std::ops::Not;

/// Builder API
///   - request_validation
pub fn request_validation(req: NewValidationRequest) -> ValidationResult<()> {
  let caller = caller();

  get_validation_registry_mut()
    .add_request(
      caller,
      &req.canister_id,
      &req.build_settings,
    )
    .map(|_| ValidationResult::success(Ok::validation_request_added()))
    .unwrap_or_else(|_| ValidationResult::fail(Error::validation_requested()))
}

/// ValidationAPI
///  - list_validations
///  - fetch_validation
///  - update_validation

/// Return list of unprocessed validations
pub fn list_validations() -> Vec<Validation> {
  let reg = get_validation_registry();
  reg.fresh.iter().map(|validation_id|
    reg.validations.get(&validation_id)
  ).map(|v| v.unwrap().clone())
    .collect()
}

pub fn fetch_validation(validation_id: ValidationId) -> Option<Validation> {
  let reg = get_validation_registry();
  reg.validations.get(&validation_id).cloned()
}
