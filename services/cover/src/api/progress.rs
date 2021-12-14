use ic_kit::candid::candid_method;
use ic_kit::macros::{query, update};

use crate::common::types::{CanisterId, ReqId};
use crate::service::model::error::Error;
use crate::service::model::progress::{Progress, UpdateProgress};
use crate::service::progress;

#[query(name = "getProgressByCanisterId")]
#[candid_method(query, rename = "getProgressByCanisterId")]
fn get_progresses_by_canister_id(canister_id: CanisterId) -> Vec<&'static Progress> {
    progress::get_progresses_by_canister_id(canister_id)
}

#[query(name = "getAllProgresses")]
#[candid_method(query, rename = "getAllProgresses")]
fn get_all_progresses() -> Vec<&'static Progress> {
    progress::get_all_progresses()
}

#[update(name = "updateProgress")]
#[candid_method(update, rename = "updateProgress")]
fn update_progress(
    _update_progress: UpdateProgress, // TODO: cdk bug????? param can not be same with fn!!!!!
) -> Result<(), Error> {
    progress::update_progress(_update_progress)
}

#[query(name = "getProgressByRequestId")]
#[candid_method(query, rename = "getProgressByRequestId")]
fn get_progress_by_request_id(request_id: ReqId) -> Option<&'static Progress> {
    progress::get_progress_by_request_id(request_id)
}
