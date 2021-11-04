use ic_kit::ic::caller;

use crate::common::types::{CanisterId, ReqId};
use crate::service::types::{
    Error, NewValidationRequest, ProviderInfo, UpdateOnGoingProgressStatus, ValidationRequest,
    ValidationResponse,
};

use super::{
    get_progress_tracker, get_progress_tracker_mut, get_validation_registry,
    get_validation_registry_mut,
};

pub fn add_request(new_validation_request: NewValidationRequest) -> Result<(), Error> {
    // TODO: handle canister's owner properly
    get_validation_registry_mut().add_request(
        caller(),
        new_validation_request.canister_id,
        new_validation_request.build_settings,
    );
    Ok(())
}

pub fn get_pending_request_by_id(request_id: ReqId) -> Option<&'static ValidationRequest> {
    get_validation_registry().get_pending_request_by_id(request_id)
}

pub fn get_all_pending_request() -> Vec<&'static ValidationRequest> {
    get_validation_registry().get_all_pending_request()
}

pub fn consume_request(
    provider_info: ProviderInfo,
) -> Result<Vec<&'static ValidationRequest>, Error> {
    //TODO: check allow list
    get_validation_registry_mut()
        .consume_request(provider_info)
        .and_then(|a| {
            for v in a.iter() {
                get_progress_tracker_mut().init_progress(v.request_id, v.canister_id)?;
            }
            Ok(a)
        })
        .map_err(|e| e.into())
}

pub fn get_on_going_progress_by_request_id(
    request_id: ReqId,
) -> Option<&'static ValidationResponse> {
    get_progress_tracker().get_by_request_id(request_id)
}

pub fn get_on_going_progress_by_canister_id(
    canister_id: CanisterId,
) -> Vec<&'static ValidationResponse> {
    get_progress_tracker().get_by_canister_id(canister_id)
}

pub fn update_on_going_progress_status(
    request_validation_id: ReqId,
    canister_id: CanisterId,
    status: UpdateOnGoingProgressStatus,
) -> Result<(), Error> {
    // TODO: check progress owner
    get_progress_tracker_mut()
        .update_status(request_validation_id, canister_id, status)
        .map_err(|e| e.into())
}
