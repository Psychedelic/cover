use ic_kit::candid::candid_method;
use ic_kit::ic::caller;
use ic_kit::macros::{query, update};

use crate::common::types::ReqId;
use crate::service::model::error::Error;
use crate::service::model::provider::ProviderInfo;
use crate::service::model::request::{AddRequest, Request};
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
fn consume_requests(request: ProviderInfo) -> Result<Vec<&'static Request>, Error> {
    request::consume_requests(request)
}

#[update(name = "addRequest")]
#[candid_method(update, rename = "addRequest")]
fn add_request(request: AddRequest) -> Result<(), Error> {
    request::add_request(&caller(), request)
}
