use ic_cdk::caller;

use crate::common::types::{CanisterId, ReqId};
use crate::service::types::{
    AddVerification, CreateRequest, Error, Progress, ProviderInfo, Request, UpdateProgress,
    UpdateVerification, Verification,
};

use super::{
    get_progress_store, get_progress_store_mut, get_request_store_mut, get_request_store_registry,
    get_verification_store, get_verification_store_mut,
};

pub fn create_request(create_request: CreateRequest) -> Result<(), Error> {
    // TODO: handle canister's owner properly
    get_request_store_mut().create_request(caller(), create_request);
    Ok(())
}

pub fn get_request_by_id(request_id: ReqId) -> Option<&'static Request> {
    get_request_store_registry().get_request_by_id(request_id)
}

pub fn get_all_request() -> Vec<&'static Request> {
    get_request_store_registry().get_all_request()
}

pub fn consume_request(provider_info: ProviderInfo) -> Result<Vec<&'static Request>, Error> {
    // TODO: check allow list
    get_request_store_mut()
        .consume_request(provider_info)
        .and_then(|requests| {
            for request in requests.iter() {
                get_progress_store_mut().init_progress(request.request_id, request.canister_id)?;
            }
            Ok(requests)
        })
        .map_err(|e| e.into())
}

pub fn get_progress_by_request_id(request_id: ReqId) -> Option<&'static Progress> {
    get_progress_store().get_progress_by_request_id(request_id)
}

pub fn get_progress_by_canister_id(canister_id: CanisterId) -> Vec<&'static Progress> {
    get_progress_store().get_progress_by_canister_id(canister_id)
}

pub fn get_all_progress() -> Vec<&'static Progress> {
    get_progress_store().get_all_progress()
}

pub fn update_progress(update_progress: UpdateProgress) -> Result<(), Error> {
    // TODO: check progress owner
    get_progress_store_mut()
        .update_progress(update_progress)
        .map_err(|e| e.into())
}

pub fn get_verification_by_canister_id(canister_id: &CanisterId) -> Option<&'static Verification> {
    get_verification_store().get_verification_by_canister_id(canister_id)
}

pub fn get_all_verification() -> Vec<&'static Verification> {
    get_verification_store().get_all_verification()
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
