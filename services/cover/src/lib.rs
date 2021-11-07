use ic_kit::macros::{query, update};

use crate::common::types::{CanisterId, ReqId};
use crate::service::cover_service;
use crate::service::types::{
    Error, ProviderInfo, RequestValidation, UpdateProgress, ValidationProgress, ValidationRequest,
};

mod common;
mod service;

//TODO: history api

#[update]
fn add_request(new_validation_request: RequestValidation) -> Result<(), Error> {
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
fn get_progress_by_request_id(request_id: ReqId) -> Option<&'static ValidationProgress> {
    cover_service::get_progress_by_request_id(request_id)
}

#[query]
fn get_progress_by_canister_id(canister_id: CanisterId) -> Vec<&'static ValidationProgress> {
    cover_service::get_progress_by_canister_id(canister_id)
}

#[query]
fn get_all_progress() -> Vec<&'static ValidationProgress> {
    cover_service::get_all_progress()
}

#[update]
fn update_progress(
    request_id: ReqId,
    canister_id: CanisterId,
    _update_progress: UpdateProgress, // TODO: cdk bug????? param can not same with fn!!!!!
) -> Result<(), Error> {
    cover_service::update_progress(request_id, canister_id, _update_progress)
}
