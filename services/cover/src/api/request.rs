use ic_kit::candid::candid_method;
use ic_kit::ic::caller;
use ic_kit::macros::{query, update};

use crate::common::types::ReqId;
use crate::service::model::error::Error;
use crate::service::model::provider::ProviderInfo;
use crate::service::model::request::{CreateRequest, Request};
use crate::service::request;

#[query(name = "getRequestById")]
#[candid_method(query, rename = "getRequestById")]
fn get_request_by_id(request_id: ReqId) -> Option<&'static Request> {
    request::get_request_by_id(request_id)
}

#[query(name = "getAllRequests")]
#[candid_method(query, rename = "getAllRequests")]
fn get_all_requests() -> Vec<&'static Request> {
    request::get_all_requests()
}

#[update(name = "consumeRequests")]
#[candid_method(update, rename = "consumeRequests")]
fn consume_requests(provider_info: ProviderInfo) -> Result<Vec<&'static Request>, Error> {
    request::consume_requests(provider_info)
}

#[update(name = "createRequest")]
#[candid_method(update, rename = "createRequest")]
fn create_request(_create_request: CreateRequest) -> Result<(), Error> {
    request::create_request(caller(), _create_request)
}
