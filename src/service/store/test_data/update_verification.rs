use crate::common::types::CanisterId;
use crate::service::model::verification::UpdateVerification;

pub fn fake_update_verification1(canister_id: &CanisterId) -> UpdateVerification {
    UpdateVerification {
        canister_id: *canister_id,
        canister_name: "updated_canister_name_1".to_string(),
        repo_url: "updated_repo_1".to_string(),
        commit_hash: "updated_commit_1".to_string(),
        wasm_hash: "updated_wasm1".to_string(),
        rust_version: "1.1.1".to_string(),
        dfx_version: "0.8.4".to_string(),
        optimize_count: 0,
    }
}
