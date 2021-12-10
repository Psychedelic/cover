use ic_kit::ic::caller;
use ic_kit::macros::{query, update};

use crate::common::types::{CallerId, CanisterId, ProviderId, ReqId};
use crate::service::canister_build_config;
use crate::service::cover;
use crate::service::model::canister::CanisterBuildConfig;
use crate::service::types::{
    AddProvider, AddVerification, CreateRequest, Error, Progress, Provider, ProviderInfo, Request,
    SubmitVerification, UpdateProgress, UpdateProvider, UpdateVerification, Verification,
};

mod common;
mod service;
mod upgrade;

// TODO: history api

// #[query]
pub fn get_request_by_id(request_id: ReqId) -> Option<&'static Request> {
    cover::get_request_by_id(request_id)
}

// #[query]
pub fn get_all_requests() -> Vec<&'static Request> {
    cover::get_all_requests()
}

// #[query]
pub fn get_progress_by_request_id(request_id: ReqId) -> Option<&'static Progress> {
    cover::get_progress_by_request_id(request_id)
}

// #[query]
pub fn get_progresses_by_canister_id(canister_id: CanisterId) -> Vec<&'static Progress> {
    cover::get_progresses_by_canister_id(canister_id)
}

// #[query]
pub fn get_all_progresses() -> Vec<&'static Progress> {
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

// #[update]
pub fn consume_requests(provider_info: ProviderInfo) -> Result<Vec<&'static Request>, Error> {
    cover::consume_requests(provider_info)
}

// #[update]
pub fn update_progress(
    _update_progress: UpdateProgress, // TODO: cdk bug????? param can not be same with fn!!!!!
) -> Result<(), Error> {
    cover::update_progress(_update_progress)
}

// #[update]
pub fn add_verification(_add_verification: AddVerification) -> Result<(), Error> {
    cover::add_verification(caller(), _add_verification)
}

// #[update]
pub fn update_verification(_update_verification: UpdateVerification) -> Result<(), Error> {
    cover::update_verification(caller(), _update_verification)
}

#[update]
fn submit_verification(_submit_verification: SubmitVerification) -> Result<(), Error> {
    cover::submit_verification(caller(), _submit_verification)
}

// #[update]
pub fn create_request(_create_request: CreateRequest) -> Result<(), Error> {
    cover::create_request(caller(), _create_request)
}

#[update]
async fn add_provider(_add_provider: AddProvider) -> Result<(), Error> {
    cover::add_provider(caller(), _add_provider).await
}

#[update]
async fn update_provider(_update_provider: UpdateProvider) -> Result<(), Error> {
    cover::update_provider(caller(), _update_provider).await
}

#[update]
async fn delete_provider(provider_id: ProviderId) -> Result<(), Error> {
    cover::delete_provider(&caller(), &provider_id).await
}

#[query]
fn get_provider_by_id(provider_id: ProviderId) -> Option<&'static Provider> {
    cover::get_provider_by_id(&provider_id)
}

#[query]
fn get_all_providers() -> Vec<&'static Provider> {
    cover::get_all_providers()
}

#[query]
fn get_all_configs(caller_id: CallerId) -> Vec<&'static CanisterBuildConfig> {
    canister_build_config::get_all_configs(&caller_id)
}

#[query]
fn get_config_by_id(
    caller_id: CallerId,
    canister_id: CanisterId,
) -> Result<&'static CanisterBuildConfig, Error> {
    canister_build_config::get_config_by_id(&caller_id, &canister_id)
}

#[query]
fn update_config(
    caller_id: CallerId,
    canister_id: CanisterId,
    config: CanisterBuildConfig,
) -> Result<(), Error> {
    canister_build_config::update_config(&caller_id, &canister_id, config)
}

#[query]
fn delete_config(caller_id: CallerId, canister_id: CanisterId) -> Result<(), Error> {
    canister_build_config::delete_config(&caller_id, &canister_id)
}

#[query]
fn add_config(config: CanisterBuildConfig) -> Result<(), Error> {
    canister_build_config::add_config(config)
}
