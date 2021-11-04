mod common;
mod service;

use crate::common::types::{CallerId, ReqId};
use crate::service::cover_service;
use crate::service::types::{Error, NewValidationRequest, ProviderInfo, ValidationRequest};
use ic_kit::ic::caller;
use ic_kit::macros::{query, update};

//TODO: validation status

#[query]
fn whoami() -> CallerId {
    caller()
}

#[update]
fn add_request(request: NewValidationRequest) -> Result<(), Error> {
    cover_service::add_request(request)
}

#[query]
fn get_pending_request_by_id(id: ReqId) -> Option<&'static ValidationRequest> {
    cover_service::get_pending_request_by_id(id)
}

#[query]
fn get_all_pending_request() -> Vec<&'static ValidationRequest> {
    cover_service::get_all_pending_request()
}

#[update]
fn consume_request(provider_info: ProviderInfo) -> Result<Vec<&'static ValidationRequest>, Error> {
    cover_service::consume_request(provider_info)
}
