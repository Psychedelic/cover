use ic_kit::ic::caller;
use ic_kit::macros::{query, update};

use crate::common::types::{CallerId, CanisterId, ReqId};
use crate::service::cover_service;
use crate::service::types::{
    Error, NewValidationRequest, ProviderInfo, UpdateOnGoingProgressStatus, ValidationRequest,
    ValidationResponse,
};

mod common;
mod service;

//TODO: validation status

#[query]
fn whoami() -> CallerId {
    caller()
}

#[update]
fn add_request(new_validation_request: NewValidationRequest) -> Result<(), Error> {
    cover_service::add_request(new_validation_request)
}

#[query]
fn get_pending_request_by_id(request_id: ReqId) -> Option<&'static ValidationRequest> {
    cover_service::get_pending_request_by_id(request_id)
}

#[query]
fn get_all_pending_request() -> Vec<&'static ValidationRequest> {
    cover_service::get_all_pending_request()
}

#[update]
fn consume_request(provider_info: ProviderInfo) -> Result<Vec<&'static ValidationRequest>, Error> {
    cover_service::consume_request(provider_info)
}

#[query]
fn get_on_going_progress_by_request_id(request_id: ReqId) -> Option<&'static ValidationResponse> {
    cover_service::get_on_going_progress_by_request_id(request_id)
}

#[query]
fn get_on_going_progress_by_canister_id(
    canister_id: CanisterId,
) -> Vec<&'static ValidationResponse> {
    cover_service::get_on_going_progress_by_canister_id(canister_id)
}

#[update]
fn update_on_going_progress_status(
    request_validation_id: ReqId,
    canister_id: CanisterId,
    status: UpdateOnGoingProgressStatus,
) -> Result<(), Error> {
    cover_service::update_on_going_progress_status(request_validation_id, canister_id, status)
}
