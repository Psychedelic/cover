use ic_kit::macros::{query, update};

use crate::common::types::{CanisterId, ProviderId, ReqId};
use crate::service::cover_service;
use crate::service::types::{
    AddProvider, AddVerification, CreateRequest, Error, Progress, Provider, ProviderInfo, Request,
    UpdateProgress, UpdateProvider, UpdateVerification, Verification,
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
    cover_service::get_all_requests()
}

#[update]
fn consume_requests(provider_info: ProviderInfo) -> Result<Vec<&'static Request>, Error> {
    cover_service::consume_requests(provider_info)
}

#[query]
fn get_progress_by_request_id(request_id: ReqId) -> Option<&'static Progress> {
    cover_service::get_progress_by_request_id(request_id)
}

#[query]
fn get_progresses_by_canister_id(canister_id: CanisterId) -> Vec<&'static Progress> {
    cover_service::get_progresses_by_canister_id(canister_id)
}

#[query]
fn get_all_progresses() -> Vec<&'static Progress> {
    cover_service::get_all_progresses()
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
fn add_verification(_add_verification: AddVerification) -> Result<(), Error> {
    cover_service::add_verification(_add_verification)
}

#[update]
fn update_verification(_update_verification: UpdateVerification) -> Result<(), Error> {
    cover_service::update_verification(_update_verification)
}

#[update]
fn add_provider(_add_provider: AddProvider) -> Result<(), Error> {
    cover_service::add_provider(_add_provider)
}

#[update]
fn update_provider(_update_provider: UpdateProvider) -> Result<(), Error> {
    cover_service::update_provider(_update_provider)
}

#[update]
fn delete_provider(provider_id: ProviderId) -> Result<(), Error> {
    cover_service::delete_provider(&provider_id)
}

#[query]
fn get_provider_by_id(provider_id: ProviderId) -> Option<&'static Provider> {
    cover_service::get_provider_by_id(&provider_id)
}

#[query]
fn get_all_providers() -> Vec<&'static Provider> {
    cover_service::get_all_providers()
}
