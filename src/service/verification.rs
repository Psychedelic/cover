use crate::common::types::{CallerId, CanisterId};
use crate::service::model::error::Error;
use crate::service::model::verification::{SubmitVerification, Verification};
use crate::service::{verification_store, verification_store_mut};

pub fn get_verification_by_canister_id(canister_id: &CanisterId) -> Option<&'static Verification> {
    verification_store().get_verification_by_canister_id(canister_id)
}

pub fn get_all_verifications() -> Vec<&'static Verification> {
    verification_store().get_all_verifications()
}

pub fn submit_verification(
    owner_id: &CallerId,
    submit_verification: SubmitVerification,
) -> Result<(), Error> {
    match verification_store().verification_exists(&submit_verification.canister_id) {
        true => verification_store_mut()
            .update_verification(owner_id, submit_verification)
            .map_err(|e| e.into()),
        false => verification_store_mut()
            .add_verification(owner_id, submit_verification)
            .map_err(|e| e.into()),
    }
}
