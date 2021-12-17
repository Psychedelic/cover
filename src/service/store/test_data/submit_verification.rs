use crate::common::types::CanisterId;
use crate::service::model::verification::SubmitVerification;

pub fn fake_submit_verification1(canister_id: &CanisterId) -> SubmitVerification {
    SubmitVerification {
        canister_id: *canister_id,
        git_sha: "submit_git_sha1".into(),
        git_ref: "submit_git_ref1".into(),
        git_repo: "submit_user/repo1".to_string(),
        wasm_checksum: "submit_wasm_checksum1".into(),
        build_log_url: "submit_build_log_url1".into(),
        source_snapshot_url: "submit_source_snapshot_url1".into(),
    }
}
