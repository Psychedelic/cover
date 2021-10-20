mod common;
mod service;

use crate::common::types::{CallerId, CanisterId};
use crate::service::cover_service;
use crate::service::types::{NewValidationRequest, Validation};
use crate::service::utils::ValidationResult;
use ic_kit::ic::caller;
use ic_kit::interfaces::management::WithCanisterId;
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
fn validation_requests() -> ValidationResult<Vec<Validation>> {
    cover_service::list_validations()
}

/*
   Validator API
*/


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
        assert_eq!(list_canisters().data.unwrap().len(), 0);
    }

    #[test]
    fn list_canisters_ok() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(fake_store(Some(fake_data())))
            .inject();
        let canisters = list_canisters();
        assert_eq!(canisters.data.as_ref().unwrap().len(), 2);
        assert!(!canisters.is_error);
        assert_eq!(canisters.message, None);
        assert_eq!(canisters.code, None);
        assert!(canisters.data.as_ref().unwrap().contains(&Validation {
            canister_id: fake_canister4(),
            name: "Alice canister 1".into(),
            canister_type: "".into(),
        }));
        assert!(canisters.data.as_ref().unwrap().contains(&Validation {
            canister_id: fake_canister5(),
            name: "Alice canister 2".into(),
            canister_type: "".into(),
        }));
    }
}
