use ic_kit::candid::candid_method;
use ic_kit::ic::caller;
use ic_kit::macros::{query, update};

use crate::common::types::CanisterId;
use crate::service::guard::is_provider;
use crate::service::model::error::Error;
use crate::service::model::verification::{
    AddVerification, SubmitVerification, UpdateVerification, Verification,
};
use crate::service::verification;

#[query(name = "getVerificationByCanisterId", guard = "is_provider")]
#[candid_method(query, rename = "getVerificationByCanisterId")]
fn get_verification_by_canister_id(canister_id: CanisterId) -> Option<&'static Verification> {
    verification::get_verification_by_canister_id(&canister_id)
}

#[query(name = "getAllVerifications", guard = "is_provider")]
#[candid_method(query, rename = "getAllVerifications")]
fn get_all_verifications() -> Vec<&'static Verification> {
    verification::get_all_verifications()
}

#[update(name = "addVerification", guard = "is_provider")]
#[candid_method(update, rename = "addVerification")]
fn add_verification(verification: AddVerification) -> Result<(), Error> {
    verification::add_verification(caller(), verification)
}

#[update(name = "updateVerification", guard = "is_provider")]
#[candid_method(update, rename = "updateVerification")]
fn update_verification(verification: UpdateVerification) -> Result<(), Error> {
    verification::update_verification(caller(), verification)
}

#[update(name = "submitVerification", guard = "is_provider")]
#[candid_method(update, rename = "submitVerification")]
fn submit_verification(verification: SubmitVerification) -> Result<(), Error> {
    verification::submit_verification(caller(), verification)
}
