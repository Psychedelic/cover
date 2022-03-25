use crate::common::constants::{MAX_ITEMS_PER_PAGE, MIN_ITEMS_PER_PAGE};
use crate::common::types::CanisterId;
use crate::service::guard::{is_builder, is_validator};
use crate::service::model::error::Error;
use crate::service::model::pagination::{Pagination, PaginationInfo};
use crate::service::model::stats::Stats;
use crate::service::model::verification::{RegisterVerification, SubmitVerification, Verification};
use crate::service::store::activity;
use crate::service::store::verification;
use ic_cdk::api::call::ManualReply;
use ic_cdk::export::candid::candid_method;
use ic_cdk_macros::{query, update};
use std::cmp::{max, min};

#[query(name = "getVerificationByCanisterId", manual_reply = true)]
#[candid_method(query, rename = "getVerificationByCanisterId")]
fn get_verification_by_canister_id(canister_id: CanisterId) -> ManualReply<Option<Verification>> {
    verification::get_verification_by_canister_id(&canister_id, |result| ManualReply::one(result))
}

#[query(name = "getVerifications", manual_reply = true)]
#[candid_method(query, rename = "getVerifications")]
fn get_verifications(mut pagination_info: PaginationInfo) -> ManualReply<Pagination<Verification>> {
    pagination_info.items_per_page = max(MIN_ITEMS_PER_PAGE, pagination_info.items_per_page);
    pagination_info.items_per_page = min(MAX_ITEMS_PER_PAGE, pagination_info.items_per_page);

    verification::get_verifications(&pagination_info, |result| ManualReply::one(result))
}

#[update(name = "submitVerification", guard = "is_builder")]
#[candid_method(update, rename = "submitVerification")]
fn submit_verification(verification: SubmitVerification) {
    verification::submit_verification(verification, activity::add_activity)
}

#[update(name = "registerVerification", guard = "is_validator")]
#[candid_method(update, rename = "registerVerification")]
fn register_verification(verification: RegisterVerification) -> Result<(), Error> {
    verification::register_verification(verification, activity::add_activity)
}

#[query(name = "getVerificationsStats")]
#[candid_method(query, rename = "getVerificationsStats")]
fn get_verifications_stats() -> Stats {
    verification::get_verifications_stats()
}
