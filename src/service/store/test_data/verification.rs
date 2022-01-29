use crate::service::model::verification::{SubmitVerification, Verification};
use crate::service::time_utils;

pub fn fake_verification(submit_verification: SubmitVerification) -> Verification {
    Verification {
        canister_id: submit_verification.canister_id,
        canister_name: submit_verification.canister_name,
        repo_url: submit_verification.repo_url,
        commit_hash: submit_verification.commit_hash,
        wasm_hash: submit_verification.wasm_hash,
        build_url: Some(submit_verification.build_url),
        build_status: submit_verification.build_status,
        rust_version: submit_verification.rust_version,
        dfx_version: submit_verification.dfx_version,
        optimize_count: submit_verification.optimize_count,
        updated_by: submit_verification.owner_id,
        updated_at: time_utils::now_to_str(),
    }
}
