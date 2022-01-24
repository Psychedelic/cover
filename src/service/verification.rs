use crate::common::types::{CanisterId, CanisterOwnerId};
use crate::service::model::verification::{SubmitVerification, Verification};
use crate::service::{verification_store, verification_store_mut};

pub fn get_verification_by_canister_id(canister_id: &CanisterId) -> Option<&'static Verification> {
    verification_store().get_verification_by_canister_id(canister_id)
}

pub fn get_all_verifications() -> Vec<&'static Verification> {
    verification_store().get_all_verifications()
}

pub fn submit_verification(owner_id: &CanisterOwnerId, submit_verification: SubmitVerification) {
    verification_store_mut().submit_verification(owner_id, submit_verification)
}
