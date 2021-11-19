use crate::common::types::CanisterId;
use crate::service::types::SubmitVerification;

pub fn fake_submit_verification1(canister_id: CanisterId) -> SubmitVerification {
    SubmitVerification {
        canister_id,
        git_checksum: "update_git_checksum1".into(),
        git_ref: "update_git_ref1".into(),
        wasm_checksum: "update_wasm_checksum1".into(),
        build_log_url: "update_build_log_url1".into(),
        source_snapshot_url: "source_snapshot_url1".into(),
    }
}

pub fn fake_submit_verification2(canister_id: CanisterId) -> SubmitVerification {
    SubmitVerification {
        canister_id,
        git_checksum: "update_git_checksum2".into(),
        git_ref: "update_git_ref2".into(),
        wasm_checksum: "update_wasm_checksum2".into(),
        build_log_url: "update_build_log_url2".into(),
        source_snapshot_url: "source_snapshot_url2".into(),
    }
}

pub fn fake_submit_verification3(canister_id: CanisterId) -> SubmitVerification {
    SubmitVerification {
        canister_id,
        git_checksum: "update_git_checksum3".into(),
        git_ref: "update_git_ref3".into(),
        wasm_checksum: "update_wasm_checksum3".into(),
        build_log_url: "update_build_log_url3".into(),
        source_snapshot_url: "source_snapshot_url3".into(),
    }
}
