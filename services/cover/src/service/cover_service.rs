use ic_kit::ic::caller;

use crate::common::types::ReqId;
use crate::service::types::{Error, NewValidationRequest, ProviderInfo, ValidationRequest};

use super::{get_validation_registry, get_validation_registry_mut};

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
    get_validation_registry_mut()
        .consume_request(provider_info)
        .map_err(|e| e.into())
}
