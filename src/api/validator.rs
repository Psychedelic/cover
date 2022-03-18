use crate::common::types::ValidatorId;
use crate::service::guard::is_admin;
use crate::service::store::validator;
use ic_cdk::api::call::ManualReply;
use ic_cdk::export::candid::candid_method;
use ic_cdk_macros::{query, update};

#[update(name = "addValidator", guard = "is_admin")]
#[candid_method(update, rename = "addValidator")]
fn add_validator(validator_id: ValidatorId) {
    validator::add_validator(&validator_id)
}

#[update(name = "deleteValidator", guard = "is_admin")]
#[candid_method(update, rename = "deleteValidator")]
fn delete_validator(validator_id: ValidatorId) {
    validator::delete_validator(&validator_id)
}

#[query(name = "getValidators", guard = "is_admin", manual_reply = true)]
#[candid_method(query, rename = "getValidators")]
fn get_validators() -> ManualReply<Vec<ValidatorId>> {
    validator::get_validators(|result| ManualReply::one(result))
}
