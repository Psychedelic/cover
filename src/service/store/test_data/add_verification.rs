use crate::common::types::CanisterId;
use crate::service::model::verification::AddVerification;

pub fn fake_add_verification1(canister_id: &CanisterId) -> AddVerification {
    AddVerification {
        canister_id: *canister_id,
        canister_name: "add_canister_name_1".to_string(),
        repo_url: "add_repo_1".to_string(),
        commit_hash: "add_commit_1".to_string(),
        wasm_hash: "add_wasm1".to_string(),
        rust_version: "1.0.0".to_string(),
        dfx_version: "0.8.4".to_string(),
        optimize_count: 1,
    }
}

pub fn fake_add_verification2(canister_id: &CanisterId) -> AddVerification {
    AddVerification {
        canister_id: *canister_id,
        canister_name: "add_canister_name_2".to_string(),
        repo_url: "add_repo_2".to_string(),
        commit_hash: "add_commit_2".to_string(),
        wasm_hash: "add_wasm2".to_string(),
        rust_version: "2.0.0".to_string(),
        dfx_version: "0.8.4".to_string(),
        optimize_count: 2,
    }
}

pub fn fake_add_verification3(canister_id: &CanisterId) -> AddVerification {
    AddVerification {
        canister_id: *canister_id,
        canister_name: "add_canister_name_3".to_string(),
        repo_url: "add_repo_3".to_string(),
        commit_hash: "add_commit_3".to_string(),
        wasm_hash: "add_wasm3".to_string(),
        rust_version: "3.0.0".to_string(),
        dfx_version: "0.8.4".to_string(),
        optimize_count: 0,
    }
}
