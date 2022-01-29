use crate::common::types::ValidatorId;
use crate::service::{validator_store, validator_store_mut};

pub fn add_validator(validator_id: &ValidatorId) {
    validator_store_mut().add_validator(validator_id)
}

pub fn delete_validator(validator_id: &ValidatorId) {
    validator_store_mut().delete_validator(validator_id)
}

pub fn get_validators() -> Vec<&'static ValidatorId> {
    validator_store().get_validators()
}
