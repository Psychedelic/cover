use crate::common::types::CanisterId;
use crate::service::model::request::{AddRequest, BuildSettings};

pub fn fake_create_request(canister_id: CanisterId, build_settings: BuildSettings) -> AddRequest {
    AddRequest {
        canister_id,
        build_settings,
    }
}
