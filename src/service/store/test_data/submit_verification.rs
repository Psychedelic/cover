use crate::common::types::CanisterId;
use crate::service::model::verification::SubmitVerification;

pub fn fake_submit_verification1(canister_id: &CanisterId) -> SubmitVerification {
    SubmitVerification {
        canister_id: *canister_id,
        canister_name: "submit_canister_name_1".to_string(),
        repo_url: "submit_repo_1".to_string(),
        commit_hash: "submit_commit_1".to_string(),
        wasm_hash: "submit_wasm1".to_string(),
        rust_version: "1.0.1".to_string(),
        dfx_version: "0.8.4".to_string(),
        optimize_count: 1,
    }
}

pub fn fake_submit_verification2(canister_id: &CanisterId) -> SubmitVerification {
    SubmitVerification {
        canister_id: *canister_id,
        canister_name: "submit_canister_name_2".to_string(),
        repo_url: "submit_repo_2".to_string(),
        commit_hash: "submit_commit_2".to_string(),
        wasm_hash: "submit_wasm2".to_string(),
        rust_version: "2.0.2".to_string(),
        dfx_version: "0.8.4".to_string(),
        optimize_count: 4,
    }
}

pub fn fake_submit_verification3(canister_id: &CanisterId) -> SubmitVerification {
    SubmitVerification {
        canister_id: *canister_id,
        canister_name: "submit_canister_name_3".to_string(),
        repo_url: "submit_repo_3".to_string(),
        commit_hash: "submit_commit_3".to_string(),
        wasm_hash: "submit_wasm3".to_string(),
        rust_version: "".to_string(),
        dfx_version: "0.8.4".to_string(),
        optimize_count: 0,
    }
}
