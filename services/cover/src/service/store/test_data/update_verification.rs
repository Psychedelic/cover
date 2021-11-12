use crate::common::types::CanisterId;
use crate::service::types::UpdateVerification;

pub fn fake_update_verification1(canister_id: CanisterId) -> UpdateVerification {
    UpdateVerification {
        canister_id,
        git_checksum: "update_git_checksum1".into(),
        canister_checksum: "update_canister_checksum1".into(),
        wasm_checksum: "update_wasm_checksum1".into(),
        build_log_url: "update_build_log_url1".into(),
        source_snapshot_url: "update_build_log_url1".into(),
    }
}

pub fn fake_update_verification2(canister_id: CanisterId) -> UpdateVerification {
    UpdateVerification {
        canister_id,
        git_checksum: "update_git_checksum2".into(),
        canister_checksum: "update_canister_checksum2".into(),
        wasm_checksum: "update_wasm_checksum2".into(),
        build_log_url: "update_build_log_url2".into(),
        source_snapshot_url: "update_build_log_url2".into(),
    }
}

pub fn fake_update_verification3(canister_id: CanisterId) -> UpdateVerification {
    UpdateVerification {
        canister_id,
        git_checksum: "update_git_checksum3".into(),
        canister_checksum: "update_canister_checksum3".into(),
        wasm_checksum: "update_wasm_checksum3".into(),
        build_log_url: "update_build_log_url3".into(),
        source_snapshot_url: "update_build_log_url3".into(),
    }
}
