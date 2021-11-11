use ic_cdk::caller;

use crate::common::types::{CanisterId, ProviderId, ReqId};
use crate::service::types::{
    AddProvider, AddVerification, CreateRequest, Error, Progress, Provider, ProviderInfo, Request,
    UpdateProgress, UpdateProvider, UpdateVerification, Verification,
};

use super::{
    get_progress_store, get_progress_store_mut, get_provider_store, get_provider_store_mut,
    get_request_store_mut, get_request_store_registry, get_verification_store,
    get_verification_store_mut,
};

pub fn get_request_by_id(request_id: ReqId) -> Option<&'static Request> {
    get_request_store_registry().get_request_by_id(request_id)
}

pub fn get_all_requests() -> Vec<&'static Request> {
    get_request_store_registry().get_all_requests()
}

pub fn get_progress_by_request_id(request_id: ReqId) -> Option<&'static Progress> {
    get_progress_store().get_progress_by_request_id(request_id)
}

pub fn get_progresses_by_canister_id(canister_id: CanisterId) -> Vec<&'static Progress> {
    get_progress_store().get_progresses_by_canister_id(canister_id)
}

pub fn get_all_progresses() -> Vec<&'static Progress> {
    get_progress_store().get_all_progresses()
}

pub fn get_verification_by_canister_id(canister_id: &CanisterId) -> Option<&'static Verification> {
    get_verification_store().get_verification_by_canister_id(canister_id)
}

pub fn get_all_verifications() -> Vec<&'static Verification> {
    get_verification_store().get_all_verifications()
}

pub fn consume_requests(provider_info: ProviderInfo) -> Result<Vec<&'static Request>, Error> {
    // TODO: check allow list
    get_request_store_mut()
        .consume_requests(provider_info)
        .and_then(|requests| {
            for request in requests.iter() {
                get_progress_store_mut().init_progress(request.request_id, request.canister_id)?;
            }
            Ok(requests)
        })
        .map_err(|e| e.into())
}
pub fn update_progress(update_progress: UpdateProgress) -> Result<(), Error> {
    // TODO: check progress owner
    get_progress_store_mut()
        .update_progress(update_progress)
        .map_err(|e| e.into())
}

pub fn add_verification(add_verification: AddVerification) -> Result<(), Error> {
    get_verification_store_mut()
        .add_verification(caller(), add_verification)
        .map_err(|e| e.into())
}

pub fn update_verification(update_verification: UpdateVerification) -> Result<(), Error> {
    get_verification_store_mut()
        .update_verification(caller(), update_verification)
        .map_err(|e| e.into())
}

pub fn create_request(create_request: CreateRequest) -> Result<(), Error> {
    // TODO: handle canister's owner properly
    get_request_store_mut().create_request(caller(), create_request);
    Ok(())
}

pub fn add_provider(add_provider: AddProvider) -> Result<(), Error> {
    get_provider_store_mut()
        .add_provider(caller(), add_provider)
        .map_err(|e| e.into())
}

pub fn update_provider(update_provider: UpdateProvider) -> Result<(), Error> {
    get_provider_store_mut()
        .update_provider(caller(), update_provider)
        .map_err(|e| e.into())
}

pub fn delete_provider(provider_id: &ProviderId) -> Result<(), Error> {
    get_provider_store_mut()
        .delete_provider(provider_id)
        .map_err(|e| e.into())
}

pub fn get_provider_by_id(provider_id: &ProviderId) -> Option<&'static Provider> {
    get_provider_store().get_provider_by_id(provider_id)
}

pub fn get_all_providers() -> Vec<&'static Provider> {
    get_provider_store().get_all_providers()
}
