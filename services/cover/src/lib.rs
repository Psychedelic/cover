mod common;
mod service;

use crate::common::types::{CallerId, RequestId, CanisterId};
use crate::service::cover_service;
use crate::service::types::{NewValidationRequest, ValidationRequest, BuildParams};
use crate::service::utils::ValidationResult;
use ic_kit::ic::caller;
use ic_kit::macros::{query, update};
use serde::{Deserialize, Serialize};

#[query]
fn whoami() -> CallerId {
    caller()
}

#[query]
fn json(str: String) -> () {
    ic_kit::ic::print(format!("JSON received: {:?}", str));
    let request: BuildParams = serde_json::from_str(str.as_ref()).unwrap();
    ic_kit::ic::print(format!("Parsed: {:?}", serde_json::to_string_pretty(&request)));
}

/*
    Builder API
*/

/*
    Allow creation of validation requests
    add_validation_request
    add_validation_request_json
 */
#[update]
fn add_validation_request(request: NewValidationRequest) -> ValidationResult<()> {
    cover_service::add_validation_request(request)
}
#[update]
fn add_validation_request_json(str: String) -> ValidationResult<()> {
    let request: NewValidationRequest = serde_json::from_str(str.as_ref()).unwrap();
    cover_service::add_validation_request(request)
}

#[query]
fn my_validations() -> Vec<ValidationRequest> {
    let caller = caller();
    cover_service::all_validation_requests(Some(&caller))
}

/*
   Validator API
*/

// returns json with validation params or empty string if none found
#[update]
fn fetch_validation_json() -> String {
    let res = cover_service::fetch_next_request();
    match res.data {
        Some(value) => serde_json::to_string_pretty(&value).unwrap(),
        _ => "".to_string(), //String::from("{ \"error\": \"Not found\" }")
    }
}

#[update]
fn insert_validation_result(json: String) -> ValidationResult<ValidationRequest> {
    // let request: NewValidationRequest = serde_json::from_str(str.as_ref()).unwrap();
    // cover_service::fetch_validation_request(&canister_id)
    unimplemented!()
}


#[update]
fn fetch_validation(canister_id: CanisterId) -> ValidationResult<ValidationRequest> {
    cover_service::fetch_request_by_canister_id(&canister_id)
}

#[query]
fn fresh_validations() -> Vec<CanisterId> {
    cover_service::fresh_validation_requests()
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
    use crate::service::cover_service::fetch_next_request;

    #[test]
    fn json_test() {
        MockContext::new()
            .with_caller(mock_principals::bob())
            .inject();
        let str = "{\"git_ref\": \"REF\", \"git_sha\":\"SHA\"}".to_string();
        json(str);
    }


    #[test]
    fn whoami_success() {
        MockContext::new()
            .with_caller(mock_principals::bob())
            .inject();
        assert_eq!(whoami(), mock_principals::bob());
    }

    #[test]
    fn list_fresh_ok() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(fake_registry())
            .inject();
        let fresh = fresh_validations();
        assert_eq!(fresh.len(), 0);
    }

    #[test]
    fn list_add_request_ok() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(fake_registry())
            .inject();
        list_fresh_ok();
        add_validation_request(NewValidationRequest {
            canister_id: fake_canister1(),
            build_settings: fake_build_params(),
        });
        let fresh = fresh_validations();
        assert_eq!(fresh.len(), 1);

        add_validation_request(NewValidationRequest {
            canister_id: fake_canister2(),
            build_settings: fake_build_params(),
        });
        let fresh = fresh_validations();
        assert_eq!(fresh.len(), 2);
    }

    #[test]
    fn list_fetch_validation_ok() {
        list_add_request_ok();
        let fresh = fresh_validations();
        assert_eq!(fresh.len(), 2);

        let json = fetch_validation_json();
        assert_eq!(fresh_validations().len(), 1);

        let json = fetch_validation_json();
        assert_eq!(fresh_validations().len(), 0);

        let json = fetch_validation_json();
        assert_eq!(fresh_validations().len(), 0);
        assert_eq!(json, ""); // "{ \"error \": \"Not found\" }")
    }

    #[test]
    fn list_add_request_json_ok() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(fake_registry())
            .inject();
        list_fresh_ok();
        let str = r#"{
      "canister_id": "rrkah-fqaaa-aaaaa-aaaaq-cai",
      "build_settings": {
        "git_ref": "REF",
        "git_sha": "SHA",
        "additionalParam": "should be skipped"
      }
    }"#;
        add_validation_request_json(str.to_string());
        let fresh = fresh_validations();
        assert_eq!(fresh.len(), 1);
    }

    #[test]
    fn list_my_validations() {
        let mut reg = fake_registry();

        let mut context = MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(reg)
            .inject();

        list_fresh_ok();

        add_validation_request(NewValidationRequest {
            canister_id: fake_canister1(),
            build_settings: fake_build_params(),
        });
        add_validation_request(NewValidationRequest {
            canister_id: fake_canister2(),
            build_settings: fake_build_params(),
        });

        context.update_caller(mock_principals::bob());

        add_validation_request(NewValidationRequest {
            canister_id: fake_canister2(),
            build_settings: fake_build_params(),
        });

        context.update_caller(mock_principals::alice());
        let list = my_validations();
        assert_eq!(list.len(), 2);

        // john has not added anything yet
        context.update_caller(mock_principals::john());
        let list = my_validations();
        assert_eq!(list.len(), 0);

        context.update_caller(mock_principals::bob());
        let list = my_validations();
        assert_eq!(list.len(), 1);
    }
}
