use super::{get_validation_registry, get_validation_registry_mut};
use crate::common::types::{CanisterId, RequestId, CallerId};
use crate::service::constants::{Error, Ok};
use crate::service::types::{NewValidationRequest, ValidationRequest, BuildParams, ValidationResponse};
use crate::service::utils::ValidationResult;
use ic_kit::ic::{caller, id};
use ic_kit::interfaces::Method;
use std::ops::Not;
use crate::service::store::error::ErrorKind;

/// Adds a new validation request to registry
pub fn add_validation_request(req: NewValidationRequest) -> ValidationResult<()> {
    let caller = caller();

    check_called_by_owner(&caller, &req.canister_id);

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

/// Return list of all requests
pub fn list_requests(caller: Option<&CallerId>) -> Vec<ValidationRequest> {
    let reg = get_validation_registry();
    reg.list_all_requests(caller)
}

/// Fetch request from fresh FIFO list
/// returns JSON
pub fn fetch_next_request() -> ValidationResult<ValidationRequest> {
    check_caller_whitelisted();
    let req = get_validation_registry_mut().fetch_next_request();
    match req {
        Err(msg) => ValidationResult::fail(Error::request_not_found()),
        Ok(val) => ValidationResult::data(val)
    }
}

pub fn get_request(request_id: RequestId) -> ValidationResult<ValidationRequest> {
    let req = get_validation_registry().get_request(request_id).unwrap();
    ValidationResult::data(req.clone())
}

/// Fetch request from fresh list, mark it as fetched
/// returns ValidationRequest
pub fn fetch_request_by_canister_id(canister_id: &CanisterId) -> ValidationResult<ValidationRequest> {
    check_caller_whitelisted();
    let val = get_validation_registry_mut().fetch_request(&canister_id).unwrap();
    ValidationResult::data(val)
}

/// Get a validation request - part of public api
/// return ValidationRequest with provided request_id
pub fn get_validation_request(request_id: RequestId) -> ValidationResult<ValidationRequest> {
    let val = get_validation_registry_mut().get_request(request_id).unwrap();
    ValidationResult::data(val.clone())
}

/// Adds a new validation request to registry
pub fn add_response(res: &ValidationResponse) -> ValidationResult<()> {
    check_caller_whitelisted();

    let caller = caller();
    get_validation_registry_mut()
        .add_response(
            &caller,
            &res,
        )
        .map(|_| ValidationResult::success(Ok::validation_request_added()))
        .unwrap_or_else(|_| ValidationResult::fail(Error::validation_requested()))
}


/// Check if caller is allowed to call the ValidateAPI endpoints
fn check_caller_whitelisted() {
    // TODO finish implementation
}

// only canister owner should be allowed to request validations
fn check_called_by_owner(caller: &CallerId, canister_id: &CanisterId) {
    // TODO finish implementation
}
