use crate::common::types::CanisterId;
use crate::service::model::verification::UpdateVerification;

pub fn fake_update_verification1(canister_id: &CanisterId) -> UpdateVerification {
    UpdateVerification {
        canister_id: *canister_id,
        git_sha: "update_git_sha1".into(),
        git_ref: "update_git_ref1".into(),
        git_repo: "Psychedelic/cover1".into(),
        wasm_checksum: "update_wasm_checksum1".into(),
        build_log_url: "update_build_log_url1".into(),
        source_snapshot_url: "source_snapshot_url1".into(),
    }
}
