use crate::common::types::CanisterId;
use crate::service::model::verification::RegisterVerification;
use ic_kit::*;

pub fn fake_register_verification(canister_id: &CanisterId) -> RegisterVerification {
    RegisterVerification {
        owner_id: mock_principals::bob(),
        canister_id: *canister_id,
        canister_name: "Register".to_string(),
        repo_url: "https://register.com".to_string(),
        commit_hash: "register_hash".to_string(),
        rust_version: Some("1.57.3".to_string()),
        dfx_version: "0.8.2".to_string(),
        optimize_count: 2,
    }
}
