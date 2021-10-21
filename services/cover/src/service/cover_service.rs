use super::{get_validation_registry, get_validation_registry_mut};
use crate::common::types::{CanisterId, ValidationId};
use crate::service::constants::{Error, Ok};
use crate::service::types::{NewValidationRequest, ValidationRequest, BuildParams};
use crate::service::utils::ValidationResult;
use ic_kit::ic::{caller, id};
use ic_kit::interfaces::Method;
use std::ops::Not;

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
///  - list_validations
///  - fetch_validation
///  - update_validation

/// Return list of unprocessed validations
pub fn list_validations() -> Vec<ValidationRequest> {
  let reg = get_validation_registry();
  reg.fresh.iter().map(|validation_id|
    reg.requests.get(&validation_id)
  ).map(|v| v.unwrap().clone())
    .collect()
}

pub fn fetch_validation(validation_id: ValidationId) -> Option<ValidationRequest> {
  let reg = get_validation_registry();
  reg.requests.get(&validation_id).cloned()
}

#[cfg(test)]
pub mod test {
  use super::*;
  use ic_kit::*;
  use crate::service::store::registry::ValidationsRegistry;
  use std::collections::BTreeMap;
  use crate::common::types::CallerId;

  pub fn fake_registry() -> ValidationsRegistry { ValidationsRegistry::default() }

  pub fn fake_caller1() -> CallerId { CallerId::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap() }

  pub fn fake_canister1() -> CanisterId { CanisterId::from_text("rkp4c-7iaaa-aaaaa-aaaca-cai").unwrap() }

  pub fn fake_canister2() -> CanisterId { CanisterId::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap() }

  pub fn fake_canister3() -> CanisterId { CanisterId::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap() }

  pub fn fake_build_params() -> BuildParams {
    BuildParams {
      git_ref: "git@github.com/Psychedelic/cover".into(),
      git_sha: "".into(),
    }
  }

  #[test]
  fn adding_request_ok() {
    let mut registry = fake_registry();
    assert_eq!(registry.list_fresh().len(), 0);
    let r = registry.add_request(&fake_caller1(), &fake_canister1(), &fake_build_params());
    assert_eq!(r, Ok(1));
    assert_eq!(registry.list_fresh().len(), 1);

    let r = registry.add_request(&fake_caller1(), &fake_canister2(), &fake_build_params());
    assert_eq!(r, Ok(2));
    assert_eq!(registry.list_fresh().len(), 2);

    let r = registry.add_request(&fake_caller1(), &fake_canister3(), &fake_build_params());
    assert_eq!(r, Ok(3));
    assert_eq!(registry.list_fresh().len(), 3);

    assert_eq!(registry.list_fresh(), vec![
      &ValidationRequest {
        canister_id: fake_canister1(),
        caller_id: fake_caller1(),
        build_settings: fake_build_params(),
        fetched: false,
      },
      &ValidationRequest {
        canister_id: fake_canister2(),
        caller_id: fake_caller1(),
        build_settings: fake_build_params(),
        fetched: false,
      },
      &ValidationRequest {
        canister_id: fake_canister3(),
        caller_id: fake_caller1(),
        build_settings: fake_build_params(),
        fetched: false,
      }, ]);
  }

  #[test]
  fn fetching_request_ok() {
    let mut registry = fake_registry();
    let r1 = registry.add_request(&fake_caller1(), &fake_canister1(), &fake_build_params());
    let r2 = registry.add_request(&fake_caller1(), &fake_canister2(), &fake_build_params());
    {
      let req1 = registry.fetch_request(r1.unwrap()).unwrap();
      assert_eq!(req1.canister_id, fake_canister1());
      assert_eq!(req1.fetched, true);
    }
    assert_eq!(registry.list_fresh().len(), 1); // removed from fresh
    assert_eq!(registry.list_all(), vec![
      &ValidationRequest {
        canister_id: fake_canister1(),
        caller_id: fake_caller1(),
        build_settings: fake_build_params(),
        fetched: true,
      },
      &ValidationRequest {
        canister_id: fake_canister2(),
        caller_id: fake_caller1(),
        build_settings: fake_build_params(),
        fetched: false,
      },
    ]);
  }
}
