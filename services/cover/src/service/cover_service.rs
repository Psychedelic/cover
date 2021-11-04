use super::{get_validation_registry, get_validation_registry_mut};
use crate::common::types::ReqId;
use crate::service::types::{Error, NewValidationRequest, ProviderInfo, ValidationRequest};
use ic_kit::ic::caller;

pub fn add_validation_request(req: NewValidationRequest) -> Result<(), Error> {
    get_validation_registry_mut().add_request(caller(), req.canister_id, req.build_settings);
    Ok(())
}

pub fn get_pending_request_by_id(id: ReqId) -> Option<&'static ValidationRequest> {
    get_validation_registry().get_pending_request_by_id(id)
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
