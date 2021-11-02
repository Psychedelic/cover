mod common;
mod service;
//
// use crate::common::types::{CallerId, RequestId, CanisterId};
// use crate::service::cover_service;
// use crate::service::types::{NewValidationRequest, ValidationRequest, BuildParams, ValidationResponse};
// use crate::service::utils::ValidationResult;
// use ic_kit::ic::caller;
// use ic_kit::macros::{query, update};
// use serde::{Deserialize, Serialize};
// use crate::service::cover_service::get_response;
//
// #[query]
// fn whoami() -> CallerId {
//     caller()
// }
//
// /*
//     Developer/Public API
// */
//
// /*
//     Allow creation of validation requests
//     add_validation_request
//     add_validation_request_json
//  */
// #[update]
// fn add_request(request: NewValidationRequest) -> ValidationResult<()> {
//     cover_service::add_validation_request(request)
// }
//
// #[update]
// fn add_request_json(str: String) -> ValidationResult<()> {
//     let request: NewValidationRequest = serde_json::from_str(str.as_ref()).unwrap();
//     cover_service::add_validation_request(request)
// }
//
// #[query]
// fn get_request(request_id: RequestId) -> ValidationResult<ValidationRequest> {
//     cover_service::get_request(request_id)
// }
//
// #[query]
// fn get_request_json(request_id: RequestId) -> String {
//     let res = get_request(request_id);
//     match res.data {
//         Some(value) => serde_json::to_string_pretty(&value).unwrap(),
//         _ => "".to_string(),
//     }
// }
//
// #[query]
// fn get_validation(request_id: RequestId) -> ValidationResult<ValidationResponse> {
//     cover_service::get_response(request_id)
// }
//
// #[query]
// fn get_validation_json(request_id: RequestId) -> String {
//     let res = get_response(request_id);
//     match res.data {
//         Some(value) => serde_json::to_string_pretty(&value).unwrap(),
//         _ => "".to_string(),
//     }
// }
//
// #[query]
// fn my_requests() -> Vec<ValidationRequest> {
//     let caller = caller();
//     cover_service::list_requests(Some(&caller))
// }
//
// #[query]
// fn all_requests() -> Vec<ValidationRequest> {
//     cover_service::list_requests(None)
// }
//
// #[query]
// fn all_validations() -> Vec<ValidationResponse> {
//     cover_service::list_responses(None)
// }
//
//
// /*
//    Validator API
// */
// #[update]
// fn fetch_request() -> ValidationResult<ValidationRequest> {
//     cover_service::fetch_next_request()
// }
//
// // returns json with validation params or empty string if none found
// #[update]
// fn fetch_request_json() -> String {
//     let res = fetch_request();
//     match res.data {
//         Some(value) => serde_json::to_string_pretty(&value).unwrap(),
//         _ => "".to_string(),
//     }
// }
//
// #[update]
// fn add_response(response: ValidationResponse) -> ValidationResult<()> {
//     cover_service::add_response(&response)
// }
//
// #[update]
// fn add_response_json(json: String) -> ValidationResult<()> {
//     let response: ValidationResponse = serde_json::from_str(json.as_ref()).unwrap();
//     add_response(response)
// }
//
// /*
//     Admin API
// */
// #[update]
// fn add_validator(id: CallerId) -> ValidationResult<()> {
//     unimplemented!()
// }
// #[update]
// fn remove_validator(id: CallerId) -> ValidationResult<()> {
//     unimplemented!()
// }
// #[query]
// fn list_validators() -> Vec<CallerId> {
//     unimplemented!()
// }
//
// #[query]
// fn fresh_requests() -> Vec<CanisterId> {
//     cover_service::fresh_validation_requests()
// }
//
// #[cfg(test)]
// mod tests {
//     // okay to inherit modules from parent
//     use super::*;
//     use crate::service::constants::*;
//     use crate::service::test::*;
//     use crate::service::CanisterInternalStoreTest;
//     use ic_kit::interfaces::management::*;
//     use ic_kit::*;
//     use crate::service::cover_service::fetch_next_request;
//     use serde_json::to_string;
//
//     #[test]
//     fn whoami_success() {
//         MockContext::new()
//             .with_caller(mock_principals::bob())
//             .inject();
//         assert_eq!(whoami(), mock_principals::bob());
//     }
//
//     #[test]
//     fn list_fresh_ok() {
//         MockContext::new()
//             .with_caller(mock_principals::alice())
//             .with_data(fake_registry())
//             .inject();
//         let fresh = fresh_requests();
//         assert_eq!(fresh.len(), 0);
//     }
//
//     #[test]
//     fn list_add_request_ok() {
//         MockContext::new()
//             .with_caller(mock_principals::alice())
//             .with_data(fake_registry())
//             .inject();
//         list_fresh_ok();
//         add_request(NewValidationRequest {
//             canister_id: fake_canister1(),
//             build_settings: fake_build_params(),
//         });
//         let fresh = fresh_requests();
//         assert_eq!(fresh.len(), 1);
//
//         add_request(NewValidationRequest {
//             canister_id: fake_canister2(),
//             build_settings: fake_build_params(),
//         });
//         let fresh = fresh_requests();
//         assert_eq!(fresh.len(), 2);
//     }
//
//     #[test]
//     fn list_fetch_validation_ok() {
//         list_add_request_ok();
//         let fresh = fresh_requests();
//         assert_eq!(fresh.len(), 2);
//
//         let json = fetch_request_json();
//         assert_eq!(fresh_requests().len(), 1);
//
//         let json = fetch_request_json();
//         assert_eq!(fresh_requests().len(), 0);
//
//         let json = fetch_request_json();
//         assert_eq!(fresh_requests().len(), 0);
//         assert_eq!(json, ""); // "{ \"error \": \"Not found\" }")
//     }
//
//     #[test]
//     fn list_add_request_json_ok() {
//         list_fresh_ok();
//         let str = r#"{
//       "canister_id": "rrkah-fqaaa-aaaaa-aaaaq-cai",
//       "build_settings": {
//         "git_ref": "REF",
//         "git_tag": "SHA",
//         "additionalParam": "TO BE SKIPPED"
//       }
//     }"#;
//         add_request_json(str.to_string());
//         let fresh = fresh_requests();
//         assert_eq!(fresh.len(), 1);
//     }
//
//     #[test]
//     fn list_add_response_json_ok() {
//         list_add_request_json_ok();
//         let req = fetch_request();
//         let data = req.data.unwrap();
//         let str = r#"{
//             "request_id": {REQ_ID},
//             "canister_id": "{CAN_ID}",
//             "validation_started_at": "String",
//             "validation_completed_at": "String",
//             "git_checksum": "String",
//             "canister_checksum": "String",
//             "wasm_checksum": "String",
//             "build_log_url": "String",
//             "source_snapshot_url": "String",
//             "status": "test"
//         }"#.to_string();
//
//         // substitute
//         let str = str::replace(str.as_str(), "{REQ_ID}", &data.request_id.unwrap().to_string());
//         let str = str::replace(str.as_str(), "{CAN_ID}", &data.canister_id.to_string());
//         assert_eq!(str, r#"{
//             "request_id": 1,
//             "canister_id": "rrkah-fqaaa-aaaaa-aaaaq-cai",
//             "validation_started_at": "String",
//             "validation_completed_at": "String",
//             "git_checksum": "String",
//             "canister_checksum": "String",
//             "wasm_checksum": "String",
//             "build_log_url": "String",
//             "source_snapshot_url": "String",
//             "status": "test"
//         }"#);
//
//         let result = add_response_json(str.to_string());
//         assert_eq!(result, ValidationResult::success(Ok::validation_request_added()));
//     }
//
//     #[test]
//     fn list_my_validations() {
//         let mut reg = fake_registry();
//
//         let mut context = MockContext::new()
//             .with_caller(mock_principals::alice())
//             .with_data(reg)
//             .inject();
//
//         list_fresh_ok();
//
//         add_request(NewValidationRequest {
//             canister_id: fake_canister1(),
//             build_settings: fake_build_params(),
//         });
//         add_request(NewValidationRequest {
//             canister_id: fake_canister2(),
//             build_settings: fake_build_params(),
//         });
//
//         context.update_caller(mock_principals::bob());
//
//         add_request(NewValidationRequest {
//             canister_id: fake_canister2(),
//             build_settings: fake_build_params(),
//         });
//
//         context.update_caller(mock_principals::alice());
//         let list = my_requests();
//         assert_eq!(list.len(), 2);
//
//         // john has not added anything yet
//         context.update_caller(mock_principals::john());
//         let list = my_requests();
//         assert_eq!(list.len(), 0);
//
//         context.update_caller(mock_principals::bob());
//         let list = my_requests();
//         assert_eq!(list.len(), 1);
//     }
// }
