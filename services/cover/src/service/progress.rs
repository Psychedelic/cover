use crate::service::types::Error;
use crate::service::{get_progress_store, get_progress_store_mut};
use crate::{CanisterId, Progress, ReqId, UpdateProgress};

pub fn get_progress_by_request_id(request_id: ReqId) -> Option<&'static Progress> {
    get_progress_store().get_progress_by_request_id(request_id)
}

pub fn get_progresses_by_canister_id(canister_id: CanisterId) -> Vec<&'static Progress> {
    get_progress_store().get_progresses_by_canister_id(canister_id)
}

pub fn get_all_progresses() -> Vec<&'static Progress> {
    get_progress_store().get_all_progresses()
}

pub fn update_progress(update_progress: UpdateProgress) -> Result<(), Error> {
    // TODO: check progress owner
    get_progress_store_mut()
        .update_progress(update_progress)
        .map_err(|e| e.into())
}
