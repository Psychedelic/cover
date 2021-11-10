use ic_kit::macros::{query, update};

use crate::common::types::{CanisterId, ReqId};
use crate::service::cover_service;
use crate::service::types::{
    CreateRequest, Error, Progress, ProviderInfo, Request, UpdateProgress, UpdateVerification,
    Verification,
};

mod common;
mod service;

// TODO: history api

#[update]
fn create_request(_create_request: CreateRequest) -> Result<(), Error> {
    cover_service::create_request(_create_request)
}

#[query]
fn get_request_by_id(request_id: ReqId) -> Option<&'static Request> {
    cover_service::get_request_by_id(request_id)
}

#[query]
fn get_all_requests() -> Vec<&'static Request> {
    cover_service::get_all_request()
}

#[update]
fn consume_requests(provider_info: ProviderInfo) -> Result<Vec<&'static Request>, Error> {
    cover_service::consume_request(provider_info)
}

#[query]
fn get_progress_by_request_id(request_id: ReqId) -> Option<&'static Progress> {
    cover_service::get_progress_by_request_id(request_id)
}

#[query]
fn get_progresses_by_canister_id(canister_id: CanisterId) -> Vec<&'static Progress> {
    cover_service::get_progress_by_canister_id(canister_id)
}

#[query]
fn get_all_progresses() -> Vec<&'static Progress> {
    cover_service::get_all_progress()
}

#[update]
fn update_progress(
    _update_progress: UpdateProgress, // TODO: cdk bug????? param can not be same with fn!!!!!
) -> Result<(), Error> {
    cover_service::update_progress(_update_progress)
}

#[query]
fn get_verification_by_canister_id(canister_id: CanisterId) -> Option<&'static Verification> {
    cover_service::get_verification_by_canister_id(&canister_id)
}

#[query]
fn get_all_verifications() -> Vec<&'static Verification> {
    cover_service::get_all_verification()
}

#[update]
fn update_verification(_update_verification: UpdateVerification) {
    cover_service::update_verification(_update_verification)
}
