use ic_kit::candid::candid_method;
use ic_kit::macros::{query, update};

use crate::common::types::ValidatorId;
use crate::service::guard::is_admin;
use crate::service::validator;

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

#[query(name = "getValidators", guard = "is_admin")]
#[candid_method(query, rename = "getValidators")]
fn get_validators() -> Vec<&'static ValidatorId> {
    validator::get_validators()
}

#[cfg(test)]
mod test {
    use super::*;
    use ic_kit::*;

    fn init_test_data() {
        MockContext::new()
            .with_caller(mock_principals::bob())
            .inject();

        add_validator(mock_principals::alice())
    }

    #[test]
    fn get_validators_ok() {
        init_test_data();

        assert_eq!(get_validators(), vec![&mock_principals::alice()])
    }

    #[test]
    fn delete_validator_ok() {
        init_test_data();

        delete_validator(mock_principals::alice());

        assert_eq!(get_validators().len(), 0);

        delete_validator(mock_principals::john());

        assert_eq!(get_validators().len(), 0);
    }

    #[test]
    fn add_validator_ok() {
        init_test_data();

        assert_eq!(get_validators().len(), 1);

        add_validator(mock_principals::john());

        assert_eq!(get_validators().len(), 2);

        add_validator(mock_principals::john());

        assert_eq!(get_validators().len(), 2);
    }
}
