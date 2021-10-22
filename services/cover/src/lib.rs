mod common;
mod service;

use crate::common::types::{CallerId, ValidationId, CanisterId};
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

// #[query]
// fn my_validations() -> ValidationResult<Vec<CanisterId>> {
//   ValidationResult::data(
//     cover_service::all_validations()
//   )
// }


/*
   Validator API
*/
#[query]
fn fresh_validations() -> ValidationResult<Vec<CanisterId>> {
  ValidationResult::data(
    cover_service::fresh_validations()
  )
}

#[query]
fn fetch_validation(canister_id: CanisterId) -> ValidationResult<ValidationRequest> {
  cover_service::fetch_validation(&canister_id)
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
  fn list_canisters_ok() {
    MockContext::new()
      .with_caller(mock_principals::alice())
      .with_data(fake_registry())
      .inject();
    let fresh = fresh_validations();
    assert_eq!(fresh.data.unwrap().len(), 0);
  }

  #[test]
  fn list_add_request_ok() {
    MockContext::new()
      .with_caller(mock_principals::alice())
      .with_data(fake_registry())
      .inject();
    list_canisters_ok();
    request_validation(NewValidationRequest {
      canister_id: fake_canister1(),
      build_settings: fake_build_params(),
    });
    let fresh = fresh_validations();
    assert_eq!(fresh.data.unwrap().len(), 1);

    request_validation(NewValidationRequest {
      canister_id: fake_canister2(),
      build_settings: fake_build_params(),
    });
    let fresh = fresh_validations();
    assert_eq!(fresh.data.unwrap().len(), 2);
  }
}
