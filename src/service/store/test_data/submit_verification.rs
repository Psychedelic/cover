use crate::common::types::{CanisterId, CanisterOwnerId};
use crate::service::model::verification::BuildStatus;
use crate::service::model::verification::SubmitVerification;

pub fn fake_success_verification(
    owner_id: &CanisterOwnerId,
    canister_id: &CanisterId,
) -> SubmitVerification {
    SubmitVerification {
        owner_id: *owner_id,
        canister_id: *canister_id,
        canister_name: "submit_canister_name_1".to_string(),
        repo_url: "submit_repo_1".to_string(),
        commit_hash: "submit_commit_1".to_string(),
        wasm_hash: Some("hash1".to_string()),
        build_url: "https://build1".to_string(),
        build_status: BuildStatus::Success,
        rust_version: Some("1.0.1".to_string()),
        dfx_version: "0.8.4".to_string(),
        optimize_count: 1,
    }
}

pub fn fake_error_verification(
    owner_id: &CanisterOwnerId,
    canister_id: &CanisterId,
) -> SubmitVerification {
    SubmitVerification {
        owner_id: *owner_id,
        canister_id: *canister_id,
        canister_name: "submit_canister_name_2".to_string(),
        repo_url: "submit_repo_2".to_string(),
        commit_hash: "submit_commit_2".to_string(),
        wasm_hash: None,
        build_url: "https://build2".to_string(),
        build_status: BuildStatus::Error,
        rust_version: Some("2.0.2".to_string()),
        dfx_version: "0.8.4".to_string(),
        optimize_count: 4,
    }
}

pub fn fake_pending_verification(
    owner_id: &CanisterOwnerId,
    canister_id: &CanisterId,
) -> SubmitVerification {
    SubmitVerification {
        owner_id: *owner_id,
        canister_id: *canister_id,
        canister_name: "submit_canister_name_3".to_string(),
        repo_url: "submit_repo_3".to_string(),
        commit_hash: "submit_commit_3".to_string(),
        build_url: "".to_string(),
        wasm_hash: None,
        rust_version: None,
        dfx_version: "0.8.4".to_string(),
        optimize_count: 0,
        build_status: BuildStatus::Pending,
    }
}

pub fn fake_building_verification(
    owner_id: &CanisterOwnerId,
    canister_id: &CanisterId,
) -> SubmitVerification {
    SubmitVerification {
        owner_id: *owner_id,
        canister_id: *canister_id,
        canister_name: "submit_canister_name_4".to_string(),
        repo_url: "submit_repo_4".to_string(),
        commit_hash: "submit_commit_4".to_string(),
        build_url: "https://build4".to_string(),
        wasm_hash: None,
        rust_version: Some("1.57.8".to_string()),
        dfx_version: "0.8.4".to_string(),
        optimize_count: 3,
        build_status: BuildStatus::Building,
    }
}
