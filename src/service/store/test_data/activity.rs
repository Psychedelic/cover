use crate::common::types::CanisterId;
use crate::service::model::activity::Activity;
use crate::service::model::verification::BuildStatus;
use crate::service::time_utils;

pub fn fake_activity(canister_id: &CanisterId, status: BuildStatus) -> Activity {
    Activity {
        canister_id: *canister_id,
        build_status: status,
        create_at: time_utils::now_to_str(),
    }
}
