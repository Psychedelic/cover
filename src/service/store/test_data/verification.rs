use crate::common::types::CallerId;
use crate::service::model::verification::{
    AddVerification, SubmitVerification, UpdateVerification, Verification,
};
use crate::service::time_utils;

pub fn fake_verification_use_add_model(
    owner_id: &CallerId,
    add_verification: AddVerification,
) -> Verification {
    Verification {
        canister_id: add_verification.canister_id,
        canister_name: add_verification.canister_name,
        repo_url: add_verification.repo_url,
        commit_hash: add_verification.commit_hash,
        wasm_hash: add_verification.wasm_hash,
        rust_version: add_verification.rust_version,
        dfx_version: add_verification.dfx_version,
        optimize_count: add_verification.optimize_count,
        created_by: *owner_id,
        created_at: time_utils::now_to_str(),
        updated_by: *owner_id,
        updated_at: time_utils::now_to_str(),
    }
}

pub fn fake_verification_use_update_model(
    creator_id: &CallerId,
    updater_id: &CallerId,
    updated_verification: UpdateVerification,
) -> Verification {
    Verification {
        canister_id: updated_verification.canister_id,
        canister_name: updated_verification.canister_name,
        repo_url: updated_verification.repo_url,
        commit_hash: updated_verification.commit_hash,
        wasm_hash: updated_verification.wasm_hash,
        rust_version: updated_verification.rust_version,
        dfx_version: updated_verification.dfx_version,
        optimize_count: updated_verification.optimize_count,
        created_by: *creator_id,
        created_at: time_utils::now_to_str(),
        updated_by: *updater_id,
        updated_at: time_utils::now_to_str(),
    }
}

pub fn fake_verification_use_submit_model(
    creator_id: &CallerId,
    updater_id: &CallerId,
    submit_verification: SubmitVerification,
) -> Verification {
    Verification {
        canister_id: submit_verification.canister_id,
        canister_name: submit_verification.canister_name,
        repo_url: submit_verification.repo_url,
        commit_hash: submit_verification.commit_hash,
        wasm_hash: submit_verification.wasm_hash,
        rust_version: submit_verification.rust_version,
        dfx_version: submit_verification.dfx_version,
        optimize_count: submit_verification.optimize_count,
        created_by: *creator_id,
        created_at: time_utils::now_to_str(),
        updated_by: *updater_id,
        updated_at: time_utils::now_to_str(),
    }
}
