use ic_kit::ic::caller;
use ic_kit::macros::{query, update};

use crate::common::types::CanisterId;
use crate::service::model::error::Error;
use crate::service::model::verification::{
    AddVerification, SubmitVerification, UpdateVerification, Verification,
};
use crate::service::verification;

#[query]
fn get_verification_by_canister_id(canister_id: CanisterId) -> Option<&'static Verification> {
    verification::get_verification_by_canister_id(&canister_id)
}

#[query]
fn get_all_verifications() -> Vec<&'static Verification> {
    verification::get_all_verifications()
}

// #[update]
pub fn add_verification(_add_verification: AddVerification) -> Result<(), Error> {
    verification::add_verification(caller(), _add_verification)
}

// #[update]
pub fn update_verification(_update_verification: UpdateVerification) -> Result<(), Error> {
    verification::update_verification(caller(), _update_verification)
}

#[update]
fn submit_verification(_submit_verification: SubmitVerification) -> Result<(), Error> {
    verification::submit_verification(caller(), _submit_verification)
}
