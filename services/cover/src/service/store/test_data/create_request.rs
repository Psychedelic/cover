use crate::common::types::CanisterId;
use crate::service::types::{BuildSettings, CreateRequest};

pub fn fake_create_request(
    canister_id: CanisterId,
    build_settings: BuildSettings,
) -> CreateRequest {
    CreateRequest {
        canister_id,
        build_settings,
    }
}
