use crate::common::types::{CanisterId, ReqId};
use crate::service::model::error::Error;
use crate::service::model::progress::{Progress, UpdateProgress};
use crate::service::{progress_store, progress_store_mut};

pub fn get_progress_by_request_id(request_id: ReqId) -> Option<&'static Progress> {
    progress_store().get_progress_by_request_id(request_id)
}

pub fn get_progresses_by_canister_id(canister_id: CanisterId) -> Vec<&'static Progress> {
    progress_store().get_progresses_by_canister_id(canister_id)
}

pub fn get_all_progresses() -> Vec<&'static Progress> {
    progress_store().get_all_progresses()
}

pub fn update_progress(update_progress: UpdateProgress) -> Result<(), Error> {
    // TODO: check progress owner
    progress_store_mut()
        .update_progress(update_progress)
        .map_err(|e| e.into())
}
