use std::collections::HashSet;

use ic_kit::candid::CandidType;
use serde::Deserialize;

use crate::common::types::ValidatorId;

#[derive(Default, Deserialize, CandidType)]
pub struct ValidatorStore {
    validators: HashSet<ValidatorId>,
}

impl ValidatorStore {
    pub fn validator_existed(&self, validator_id: &ValidatorId) -> bool {
        self.validators.contains(validator_id)
    }

    pub fn add_validator(&mut self, validator_id: &ValidatorId) {
        self.validators.insert(*validator_id);
    }

    pub fn delete_validator(&mut self, validator_id: &ValidatorId) {
        self.validators.remove(validator_id);
    }

    pub fn get_all_validators(&self) -> Vec<&ValidatorId> {
        self.validators.iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ic_kit::*;

    fn init_test_data() -> ValidatorStore {
        let mut store = ValidatorStore::default();

        store.add_validator(&mock_principals::bob());

        store
    }

    #[test]
    fn validator_existed_ok() {
        let store = init_test_data();

        assert!(store.validator_existed(&mock_principals::bob()))
    }

    #[test]
    fn get_all_validators_ok() {
        let store = init_test_data();

        assert_eq!(store.get_all_validators(), vec![&mock_principals::bob()])
    }

    #[test]
    fn add_validator_ok() {
        let mut store = init_test_data();

        store.add_validator(&mock_principals::john());

        assert_eq!(store.get_all_validators().len(), 2);

        store.add_validator(&mock_principals::john());

        assert_eq!(store.get_all_validators().len(), 2);

        assert!(store.validator_existed(&mock_principals::john()))
    }

    #[test]
    fn delete_validator_ok() {
        let mut store = init_test_data();

        store.delete_validator(&mock_principals::bob());

        assert_eq!(store.get_all_validators().len(), 0);
    }
}
