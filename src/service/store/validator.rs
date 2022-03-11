use ic_cdk::api::call::ManualReply;
use std::collections::HashSet;

use ic_cdk::export::candid::CandidType;
use serde::Deserialize;

use super::VALIDATOR_STORE;
use crate::common::types::ValidatorId;

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

#[cfg(test)]
mod test {
    // use super::*;
    // use ic_kit::*;
    //
    // fn init_test_data() -> ValidatorStore {
    //     let mut store = ValidatorStore::default();
    //
    //     store.add_validator(&mock_principals::bob());
    //
    //     store
    // }
    //
    // #[test]
    // fn validator_existed_ok() {
    //     let store = init_test_data();
    //
    //     assert!(store.validator_existed(&mock_principals::bob()))
    // }
    //
    // #[test]
    // fn get_validators_ok() {
    //     let store = init_test_data();
    //
    //     assert_eq!(store.get_validators(), vec![&mock_principals::bob()])
    // }
    //
    // #[test]
    // fn add_validator_ok() {
    //     let mut store = init_test_data();
    //
    //     store.add_validator(&mock_principals::john());
    //
    //     assert_eq!(store.get_validators().len(), 2);
    //
    //     store.add_validator(&mock_principals::john());
    //
    //     assert_eq!(store.get_validators().len(), 2);
    //
    //     assert!(store.validator_existed(&mock_principals::john()))
    // }
    //
    // #[test]
    // fn delete_validator_ok() {
    //     let mut store = init_test_data();
    //
    //     store.delete_validator(&mock_principals::bob());
    //
    //     assert_eq!(store.get_validators().len(), 0);
    // }
}
