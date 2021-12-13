use crate::common::types::{CanisterId, ReqId};
use crate::service::model::error::Error;
use crate::service::model::progress::{Progress, UpdateProgress};
use crate::service::progress;

// #[query]
pub fn get_progresses_by_canister_id(canister_id: CanisterId) -> Vec<&'static Progress> {
    progress::get_progresses_by_canister_id(canister_id)
}

// #[query]
pub fn get_all_progresses() -> Vec<&'static Progress> {
    progress::get_all_progresses()
}

// #[update]
pub fn update_progress(
    _update_progress: UpdateProgress, // TODO: cdk bug????? param can not be same with fn!!!!!
) -> Result<(), Error> {
    progress::update_progress(_update_progress)
}

// #[query]
pub fn get_progress_by_request_id(request_id: ReqId) -> Option<&'static Progress> {
    progress::get_progress_by_request_id(request_id)
}
