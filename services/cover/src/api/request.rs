use ic_kit::ic::caller;

use crate::common::types::ReqId;
use crate::service::model::error::Error;
use crate::service::model::provider::ProviderInfo;
use crate::service::model::request::{CreateRequest, Request};
use crate::service::request;

// #[query]
pub fn get_request_by_id(request_id: ReqId) -> Option<&'static Request> {
    request::get_request_by_id(request_id)
}

// #[query]
pub fn get_all_requests() -> Vec<&'static Request> {
    request::get_all_requests()
}

// #[update]
pub fn consume_requests(provider_info: ProviderInfo) -> Result<Vec<&'static Request>, Error> {
    request::consume_requests(provider_info)
}

// #[update]
pub fn create_request(_create_request: CreateRequest) -> Result<(), Error> {
    request::create_request(caller(), _create_request)
}
