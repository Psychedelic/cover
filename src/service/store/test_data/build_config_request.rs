use crate::common::types::{CallerId, CanisterId};
use crate::service::model::build_config::BuildConfigRequest;

pub fn fake_build_config_request1(
    owner_id: &CallerId,
    canister_id: &CanisterId,
) -> BuildConfigRequest {
    BuildConfigRequest {
        user_id: *owner_id,
        canister_id: *canister_id,
        canister_name: "Canister1".into(),
        repo_url: "https://github.com/something1".into(),
        commit_hash: "1234".into(),
        rust_version: Some("1.4.4".into()),
        dfx_version: "0.8.4".into(),
        optimize_times: 0,
    }
}

pub fn fake_build_config_request2(
    owner_id: &CallerId,
    canister_id: &CanisterId,
) -> BuildConfigRequest {
    BuildConfigRequest {
        user_id: *owner_id,
        canister_id: *canister_id,
        canister_name: "Canister2".into(),
        repo_url: "https://github.com/something2".into(),
        commit_hash: "2345".into(),
        rust_version: Some("1.5.1".into()),
        dfx_version: "0.8.3".into(),
        optimize_times: 1,
    }
}

pub fn fake_build_config_request3(
    owner_id: &CallerId,
    canister_id: &CanisterId,
) -> BuildConfigRequest {
    BuildConfigRequest {
        user_id: *owner_id,
        canister_id: *canister_id,
        canister_name: "Canister3".into(),
        repo_url: "https://github.com/something3".into(),
        commit_hash: "3456".into(),
        rust_version: Some("1.0.0".into()),
        dfx_version: "0.8.0".into(),
        optimize_times: 3,
    }
}
