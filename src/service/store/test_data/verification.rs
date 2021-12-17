use crate::common::types::CallerId;
use crate::service::model::verification::Verification;
use crate::service::store::test_data::fake_canister1;
use crate::service::time_utils;

pub fn fake_verification1(caller_id: &CallerId) -> Verification {
    Verification {
        canister_id: fake_canister1(),
        git_sha: "git_sha1".into(),
        git_ref: "git_ref1".into(),
        git_repo: "user/repo1".to_string(),
        wasm_checksum: "wasm_checksum1".into(),
        build_log_url: "build_log_url1".into(),
        source_snapshot_url: "source_snapshot_url1".into(),
        created_by: *caller_id,
        created_at: time_utils::now_to_str(),
        updated_by: *caller_id,
        updated_at: time_utils::now_to_str(),
    }
}
