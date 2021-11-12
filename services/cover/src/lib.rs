use ic_kit::macros::{query, update};

use crate::common::types::{CanisterId, ProviderId, ReqId};
use crate::service::cover;
use crate::service::types::{
    AddProvider, AddVerification, CreateRequest, Error, Progress, Provider, ProviderInfo, Request,
    UpdateProgress, UpdateProvider, UpdateVerification, Verification,
};

mod common;
mod service;

// TODO: history api

#[query]
fn get_request_by_id(request_id: ReqId) -> Option<&'static Request> {
    cover::get_request_by_id(request_id)
}

#[query]
fn get_all_requests() -> Vec<&'static Request> {
    cover::get_all_requests()
}

#[query]
fn get_progress_by_request_id(request_id: ReqId) -> Option<&'static Progress> {
    cover::get_progress_by_request_id(request_id)
}

#[query]
fn get_progresses_by_canister_id(canister_id: CanisterId) -> Vec<&'static Progress> {
    cover::get_progresses_by_canister_id(canister_id)
}

#[query]
fn get_all_progresses() -> Vec<&'static Progress> {
    cover::get_all_progresses()
}

#[query]
fn get_verification_by_canister_id(canister_id: CanisterId) -> Option<&'static Verification> {
    cover::get_verification_by_canister_id(&canister_id)
}

#[query]
fn get_all_verifications() -> Vec<&'static Verification> {
    cover::get_all_verifications()
}

#[update]
fn consume_requests(provider_info: ProviderInfo) -> Result<Vec<&'static Request>, Error> {
    cover::consume_requests(provider_info)
}

#[update]
fn update_progress(
    _update_progress: UpdateProgress, // TODO: cdk bug????? param can not be same with fn!!!!!
) -> Result<(), Error> {
    cover::update_progress(_update_progress)
}

#[update]
fn add_verification(_add_verification: AddVerification) -> Result<(), Error> {
    cover::add_verification(_add_verification)
}

#[update]
fn update_verification(_update_verification: UpdateVerification) -> Result<(), Error> {
    cover::update_verification(_update_verification)
}

#[update]
fn create_request(_create_request: CreateRequest) -> Result<(), Error> {
    cover::create_request(_create_request)
}

#[update]
fn add_provider(_add_provider: AddProvider) -> Result<(), Error> {
    cover::add_provider(_add_provider)
}

#[update]
fn update_provider(_update_provider: UpdateProvider) -> Result<(), Error> {
    cover::update_provider(_update_provider)
}

#[update]
fn delete_provider(provider_id: ProviderId) -> Result<(), Error> {
    cover::delete_provider(&provider_id)
}

#[query]
fn get_provider_by_id(provider_id: ProviderId) -> Option<&'static Provider> {
    cover::get_provider_by_id(&provider_id)
}

#[query]
fn get_all_providers() -> Vec<&'static Provider> {
    cover::get_all_providers()
}
