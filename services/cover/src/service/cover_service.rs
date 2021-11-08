use crate::common::types::{CanisterId, ReqId, CallerId};
use crate::service::types::{
    BuildSettings, Error, ProviderInfo, UpdateProgress, ValidationProgress, ValidationRequest,
};

use super::{
    get_progress_tracker, get_progress_tracker_mut, get_validation_registry,
    get_validation_registry_mut,
};

pub fn add_request(caller_id: CallerId, canister_id: CanisterId, build_settings: BuildSettings) -> Result<(), Error> {
    // TODO: handle canister's owner properly
    get_validation_registry_mut().add_request(caller_id, canister_id, build_settings);
    Ok(())
}

pub fn get_request_by_id(request_id: ReqId) -> Option<&'static ValidationRequest> {
    get_validation_registry().get_request_by_id(request_id)
}

pub fn get_all_request() -> Vec<&'static ValidationRequest> {
    get_validation_registry().get_all_request()
}

pub fn consume_request(
    provider_info: ProviderInfo,
) -> Result<Vec<&'static ValidationRequest>, Error> {
    //TODO: check allow list
    get_validation_registry_mut()
        .consume_request(provider_info)
        .and_then(|requests| {
            for request in requests.iter() {
                get_progress_tracker_mut()
                    .init_progress(request.request_id, request.canister_id)?;
            }
            Ok(requests)
        })
        .map_err(|e| e.into())
}

pub fn get_progress_by_request_id(request_id: ReqId) -> Option<&'static ValidationProgress> {
    get_progress_tracker().get_progress_by_request_id(request_id)
}

pub fn get_progress_by_canister_id(canister_id: CanisterId) -> Vec<&'static ValidationProgress> {
    get_progress_tracker().get_progress_by_canister_id(canister_id)
}

pub fn get_all_progress() -> Vec<&'static ValidationProgress> {
    get_progress_tracker().get_all_progress()
}

pub fn update_progress(
    request_id: ReqId,
    canister_id: CanisterId,
    update_progress: UpdateProgress,
) -> Result<(), Error> {
    // TODO: check progress owner
    get_progress_tracker_mut()
        .update_progress(request_id, canister_id, update_progress)
        .map_err(|e| e.into())
}
