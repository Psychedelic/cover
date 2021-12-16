use crate::common::types::{CallerId, CanisterId};
use crate::service::model::error::Error;
use crate::service::model::verification::{
    AddVerification, SubmitVerification, UpdateVerification, Verification,
};
use crate::service::{verification_store, verification_store_mut};

pub fn get_verification_by_canister_id(canister_id: &CanisterId) -> Option<&'static Verification> {
    verification_store().get_verification_by_canister_id(canister_id)
}

pub fn get_all_verifications() -> Vec<&'static Verification> {
    verification_store().get_all_verifications()
}

pub fn add_verification(
    caller_id: CallerId,
    add_verification: AddVerification,
) -> Result<(), Error> {
    verification_store_mut()
        .add_verification(caller_id, add_verification)
        .map_err(|e| e.into())
}

pub fn update_verification(
    caller_id: CallerId,
    update_verification: UpdateVerification,
) -> Result<(), Error> {
    verification_store_mut()
        .update_verification(caller_id, update_verification)
        .map_err(|e| e.into())
}

pub fn submit_verification(
    caller_id: CallerId,
    submit_verification: SubmitVerification,
) -> Result<(), Error> {
    match verification_store().verification_exists(&submit_verification.canister_id) {
        true => verification_store_mut()
            .update_verification(
                caller_id,
                UpdateVerification {
                    canister_id: submit_verification.canister_id,
                    git_sha: submit_verification.git_sha,
                    git_ref: submit_verification.git_ref,
                    git_repo: submit_verification.git_repo,
                    wasm_checksum: submit_verification.wasm_checksum,
                    build_log_url: submit_verification.build_log_url,
                    source_snapshot_url: submit_verification.source_snapshot_url,
                },
            )
            .map_err(|e| e.into()),
        false => verification_store_mut()
            .add_verification(
                caller_id,
                AddVerification {
                    canister_id: submit_verification.canister_id,
                    git_sha: submit_verification.git_sha,
                    git_ref: submit_verification.git_ref,
                    git_repo: submit_verification.git_repo,
                    wasm_checksum: submit_verification.wasm_checksum,
                    build_log_url: submit_verification.build_log_url,
                    source_snapshot_url: submit_verification.source_snapshot_url,
                },
            )
            .map_err(|e| e.into()),
    }
}
