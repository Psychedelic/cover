use crate::common::types::{CallerId, ReqId};
use crate::service::model::error::Error;
use crate::service::model::provider::ProviderInfo;
use crate::service::model::request::{CreateRequest, Request};
use crate::service::{get_progress_store_mut, get_request_store, get_request_store_mut};

pub fn get_request_by_id(request_id: ReqId) -> Option<&'static Request> {
    get_request_store().get_request_by_id(request_id)
}

pub fn get_all_requests() -> Vec<&'static Request> {
    get_request_store().get_all_requests()
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

pub fn create_request(caller_id: CallerId, create_request: CreateRequest) -> Result<(), Error> {
    // TODO: handle canister's owner properly
    get_request_store_mut().create_request(caller_id, create_request);
    Ok(())
}
