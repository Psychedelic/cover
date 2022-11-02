use std::collections::HashSet;

use ic_cdk::api::call::ManualReply;
use ic_cdk::export::candid::CandidType;
use serde::Deserialize;

use crate::common::types::ValidatorId;

use super::VALIDATOR_STORE;

#[derive(Default, Deserialize, CandidType)]
pub struct ValidatorStore {
    validators: HashSet<ValidatorId>,
}

pub fn validator_existed(validator_id: &ValidatorId) -> bool {
    VALIDATOR_STORE.with(|store| store.borrow().validators.contains(validator_id))
}

pub fn add_validator(validator_id: &ValidatorId) {
    VALIDATOR_STORE.with(|store| {
        store.borrow_mut().validators.insert(*validator_id);
    })
}

pub fn delete_validator(validator_id: &ValidatorId) {
    VALIDATOR_STORE.with(|store| {
        store.borrow_mut().validators.remove(validator_id);
    })
}

pub fn get_validators<F: Fn(Vec<&ValidatorId>) -> ManualReply<Vec<ValidatorId>>>(
    manual_reply: F,
) -> ManualReply<Vec<ValidatorId>> {
    VALIDATOR_STORE.with(|store| manual_reply(store.borrow().validators.iter().collect()))
}
