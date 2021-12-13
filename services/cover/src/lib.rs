use ic_kit::ic::caller;
use ic_kit::macros::{query, update};

use crate::common::types::{CanisterId, ProviderId, ReqId};
use crate::service::model::build_config::BuildConfig;
use crate::service::model::error::Error;
use crate::service::model::progress::{Progress, UpdateProgress};
use crate::service::model::provider::{AddProvider, Provider, ProviderInfo, UpdateProvider};
use crate::service::model::request::{CreateRequest, Request};
use crate::service::model::verification::{
    AddVerification, SubmitVerification, UpdateVerification, Verification,
};
use crate::service::{build_config, progress, provider, request, verification};

mod common;
mod service;
mod upgrade;

// TODO: history api

// #[query]
pub fn get_request_by_id(request_id: ReqId) -> Option<&'static Request> {
    request::get_request_by_id(request_id)
}

// #[query]
pub fn get_all_requests() -> Vec<&'static Request> {
    request::get_all_requests()
}

// #[query]
pub fn get_progress_by_request_id(request_id: ReqId) -> Option<&'static Progress> {
    progress::get_progress_by_request_id(request_id)
}

// #[query]
pub fn get_progresses_by_canister_id(canister_id: CanisterId) -> Vec<&'static Progress> {
    progress::get_progresses_by_canister_id(canister_id)
}

// #[query]
pub fn get_all_progresses() -> Vec<&'static Progress> {
    progress::get_all_progresses()
}

#[query]
fn get_verification_by_canister_id(canister_id: CanisterId) -> Option<&'static Verification> {
    verification::get_verification_by_canister_id(&canister_id)
}

#[query]
fn get_all_verifications() -> Vec<&'static Verification> {
    verification::get_all_verifications()
}

// #[update]
pub fn consume_requests(provider_info: ProviderInfo) -> Result<Vec<&'static Request>, Error> {
    request::consume_requests(provider_info)
}

// #[update]
pub fn update_progress(
    _update_progress: UpdateProgress, // TODO: cdk bug????? param can not be same with fn!!!!!
) -> Result<(), Error> {
    progress::update_progress(_update_progress)
}

// #[update]
pub fn add_verification(_add_verification: AddVerification) -> Result<(), Error> {
    verification::add_verification(caller(), _add_verification)
}

// #[update]
pub fn update_verification(_update_verification: UpdateVerification) -> Result<(), Error> {
    verification::update_verification(caller(), _update_verification)
}

#[update]
fn submit_verification(_submit_verification: SubmitVerification) -> Result<(), Error> {
    verification::submit_verification(caller(), _submit_verification)
}

// #[update]
pub fn create_request(_create_request: CreateRequest) -> Result<(), Error> {
    request::create_request(caller(), _create_request)
}

#[update]
async fn add_provider(_add_provider: AddProvider) -> Result<(), Error> {
    provider::add_provider(caller(), _add_provider).await
}

#[update]
async fn update_provider(_update_provider: UpdateProvider) -> Result<(), Error> {
    provider::update_provider(caller(), _update_provider).await
}

#[update]
async fn delete_provider(provider_id: ProviderId) -> Result<(), Error> {
    provider::delete_provider(&caller(), &provider_id).await
}

#[query]
fn get_provider_by_id(provider_id: ProviderId) -> Option<&'static Provider> {
    provider::get_provider_by_id(&provider_id)
}

#[query]
fn get_all_providers() -> Vec<&'static Provider> {
    provider::get_all_providers()
}

#[query]
fn get_all_build_configs() -> Vec<&'static BuildConfig> {
    build_config::get_all_build_configs(&caller())
}

#[query]
fn get_build_config_by_id(canister_id: CanisterId) -> Result<&'static BuildConfig, Error> {
    build_config::get_build_config_by_id(&caller(), &canister_id)
}

#[update]
fn update_build_config(canister_id: CanisterId, config: BuildConfig) -> Result<(), Error> {
    build_config::update_build_config(&caller(), &canister_id, config)
}

#[update]
fn delete_build_config(canister_id: CanisterId) -> Result<(), Error> {
    build_config::delete_build_config(&caller(), &canister_id)
}

#[update]
fn add_build_config(config: BuildConfig) -> Result<(), Error> {
    build_config::add_build_config(&caller(), config)
}
