use crate::common::types::CanisterId;
use crate::service::types::AddVerification;

pub fn fake_add_verification1(canister_id: CanisterId) -> AddVerification {
    AddVerification {
        canister_id,
        git_checksum: "git_checksum1".into(),
        git_ref: "git_ref1".into(),
        wasm_checksum: "wasm_checksum1".into(),
        build_log_url: "build_log_url1".into(),
        source_snapshot_url: "source_snapshot_url1".into(),
    }
}

pub fn fake_add_verification2(canister_id: CanisterId) -> AddVerification {
    AddVerification {
        canister_id,
        git_checksum: "git_checksum2".into(),
        git_ref: "git_ref2".into(),
        wasm_checksum: "wasm_checksum2".into(),
        build_log_url: "build_log_url2".into(),
        source_snapshot_url: "source_snapshot_url2".into(),
    }
}

pub fn fake_add_verification3(canister_id: CanisterId) -> AddVerification {
    AddVerification {
        canister_id,
        git_checksum: "git_checksum3".into(),
        git_ref: "git_ref3".into(),
        wasm_checksum: "wasm_checksum3".into(),
        build_log_url: "build_log_url3".into(),
        source_snapshot_url: "source_snapshot_url3".into(),
    }
}
