mod common;
mod service;

use crate::common::types::{CallerId, ValidationId};
use crate::service::cover_service;
use crate::service::types::{NewValidationRequest, ValidationRequest};
use crate::service::utils::ValidationResult;
use ic_kit::ic::caller;
use ic_kit::macros::{query, update};

#[query]
fn whoami() -> CallerId {
  caller()
}

/*
    Builder API
*/
#[update]
fn request_validation(request: NewValidationRequest) -> ValidationResult<()> {
  cover_service::request_validation(request)
}

#[query]
fn list_validations() -> ValidationResult<Vec<ValidationRequest>> {
  ValidationResult::data(
    cover_service::list_validations()
  )
}

/*
   Validator API
*/
#[query]
fn fetch_validation(validation_id: ValidationId) -> ValidationResult<ValidationRequest> {
  ValidationResult::data(
    cover_service::fetch_validation(validation_id).unwrap()
  )
}


#[cfg(test)]
mod tests {
  // okay to inherit modules from parent
  use super::*;
  use crate::service::constants::*;
  use crate::service::test::*;
  use crate::service::CanisterInternalStoreTest;
  use ic_kit::interfaces::management::*;
  use ic_kit::*;

  #[test]
  fn whoami_success() {
    MockContext::new()
      .with_caller(mock_principals::bob())
      .inject();
    assert_eq!(whoami(), mock_principals::bob());
  }

  #[test]
  fn initial_state_success() {
    MockContext::new().inject();
    assert_eq!(list_validations().data.unwrap().len(), 0);
  }
}
