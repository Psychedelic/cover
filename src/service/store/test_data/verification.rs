use crate::common::types::CallerId;
use crate::service::model::verification::{
    AddVerification, SubmitVerification, UpdateVerification, Verification,
};
use crate::service::time_utils;

pub fn fake_verification_use_add_model(
    caller_id: &CallerId,
    add_verification: AddVerification,
) -> Verification {
    Verification {
        canister_id: add_verification.canister_id,
        git_sha: add_verification.git_sha,
        git_ref: add_verification.git_ref,
        git_repo: add_verification.git_repo,
        wasm_checksum: add_verification.wasm_checksum,
        build_log_url: add_verification.build_log_url,
        source_snapshot_url: add_verification.source_snapshot_url,
        created_by: *caller_id,
        created_at: time_utils::now_to_str(),
        updated_by: *caller_id,
        updated_at: time_utils::now_to_str(),
    }
}

pub fn fake_verification_use_update_model(
    caller_id: &CallerId,
    updated_verification: UpdateVerification,
) -> Verification {
    Verification {
        canister_id: updated_verification.canister_id,
        git_sha: updated_verification.git_sha,
        git_ref: updated_verification.git_ref,
        git_repo: updated_verification.git_repo,
        wasm_checksum: updated_verification.wasm_checksum,
        build_log_url: updated_verification.build_log_url,
        source_snapshot_url: updated_verification.source_snapshot_url,
        created_by: *caller_id,
        created_at: time_utils::now_to_str(),
        updated_by: *caller_id,
        updated_at: time_utils::now_to_str(),
    }
}

pub fn fake_verification_use_submit_model(
    caller_id: &CallerId,
    submit_verification: SubmitVerification,
) -> Verification {
    Verification {
        canister_id: submit_verification.canister_id,
        git_sha: submit_verification.git_sha,
        git_ref: submit_verification.git_ref,
        git_repo: submit_verification.git_repo,
        wasm_checksum: submit_verification.wasm_checksum,
        build_log_url: submit_verification.build_log_url,
        source_snapshot_url: submit_verification.source_snapshot_url,
        created_by: *caller_id,
        created_at: time_utils::now_to_str(),
        updated_by: *caller_id,
        updated_at: time_utils::now_to_str(),
    }
}
