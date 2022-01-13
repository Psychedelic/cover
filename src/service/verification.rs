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
    owner_id: &CallerId,
    add_verification: AddVerification,
) -> Result<(), Error> {
    verification_store_mut()
        .add_verification(owner_id, add_verification)
        .map_err(|e| e.into())
}

pub fn update_verification(
    owner_id: &CallerId,
    update_verification: UpdateVerification,
) -> Result<(), Error> {
    verification_store_mut()
        .update_verification(owner_id, update_verification)
        .map_err(|e| e.into())
}

pub fn submit_verification(
    owner_id: &CallerId,
    submit_verification: SubmitVerification,
) -> Result<(), Error> {
    match verification_store().verification_exists(&submit_verification.canister_id) {
        true => verification_store_mut()
            .update_verification(
                owner_id,
                UpdateVerification {
                    canister_id: submit_verification.canister_id,
                    canister_name: submit_verification.canister_name,
                    repo_url: submit_verification.repo_url,
                    commit_hash: submit_verification.commit_hash,
                    wasm_hash: submit_verification.wasm_hash,
                    rust_version: submit_verification.rust_version,
                    dfx_version: submit_verification.dfx_version,
                    optimize_count: submit_verification.optimize_count,
                },
            )
            .map_err(|e| e.into()),
        false => verification_store_mut()
            .add_verification(
                owner_id,
                AddVerification {
                    canister_id: submit_verification.canister_id,
                    canister_name: submit_verification.canister_name,
                    repo_url: submit_verification.repo_url,
                    commit_hash: submit_verification.commit_hash,
                    wasm_hash: submit_verification.wasm_hash,
                    rust_version: submit_verification.rust_version,
                    dfx_version: submit_verification.dfx_version,
                    optimize_count: submit_verification.optimize_count,
                },
            )
            .map_err(|e| e.into()),
    }
}
