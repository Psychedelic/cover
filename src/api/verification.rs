use ic_kit::candid::candid_method;
use ic_kit::macros::{query, update};

use crate::common::types::{CallerId, CanisterId};
use crate::service::guard::is_provider;
use crate::service::model::error::Error;
use crate::service::model::verification::{
    AddVerification, SubmitVerification, UpdateVerification, Verification,
};
use crate::service::verification;

#[query(name = "getVerificationByCanisterId")]
#[candid_method(query, rename = "getVerificationByCanisterId")]
fn get_verification_by_canister_id(canister_id: CanisterId) -> Option<&'static Verification> {
    verification::get_verification_by_canister_id(&canister_id)
}

#[query(name = "getAllVerifications")]
#[candid_method(query, rename = "getAllVerifications")]
fn get_all_verifications() -> Vec<&'static Verification> {
    verification::get_all_verifications()
}

#[update(name = "addVerification", guard = "is_provider")]
#[candid_method(update, rename = "addVerification")]
fn add_verification(owner_id: CallerId, verification: AddVerification) -> Result<(), Error> {
    verification::add_verification(&owner_id, verification)
}

#[update(name = "updateVerification", guard = "is_provider")]
#[candid_method(update, rename = "updateVerification")]
fn update_verification(owner_id: CallerId, verification: UpdateVerification) -> Result<(), Error> {
    verification::update_verification(&owner_id, verification)
}

#[update(name = "submitVerification", guard = "is_provider")]
#[candid_method(update, rename = "submitVerification")]
fn submit_verification(owner_id: CallerId, verification: SubmitVerification) -> Result<(), Error> {
    verification::submit_verification(&owner_id, verification)
}

#[cfg(test)]
mod tests {
    use ic_kit::*;

    use crate::service::store::test_data::*;

    use super::*;

    fn init_test_data() {
        MockContext::new()
            .with_caller(mock_principals::john())
            .inject();

        assert_eq!(
            add_verification(
                mock_principals::alice(),
                fake_add_verification1(&fake_canister1())
            ),
            Ok(())
        );
    }

    #[test]
    fn add_verification_ok() {
        init_test_data();

        assert_eq!(get_all_verifications().len(), 1);

        assert_eq!(
            add_verification(
                mock_principals::bob(),
                fake_add_verification2(&fake_canister2())
            ),
            Ok(())
        );

        assert_eq!(get_all_verifications().len(), 2);

        assert_eq!(
            add_verification(
                mock_principals::alice(),
                fake_add_verification2(&fake_canister2())
            ),
            Err(Error {
                code: "ERR_003_003_002",
                message: "Existed verification",
                debug_log: None,
            })
        );

        assert_eq!(get_all_verifications().len(), 2);
    }

    #[test]
    fn get_all_verifications_ok() {
        init_test_data();

        assert_eq!(
            get_all_verifications(),
            vec![&fake_verification_use_add_model(
                &mock_principals::alice(),
                fake_add_verification1(&fake_canister1())
            )]
        );
    }

    #[test]
    fn get_verification_by_canister_id_ok() {
        init_test_data();

        assert_eq!(
            get_verification_by_canister_id(fake_canister1()),
            Some(&fake_verification_use_add_model(
                &mock_principals::alice(),
                fake_add_verification1(&fake_canister1())
            ))
        );

        assert_eq!(get_verification_by_canister_id(fake_canister2()), None);
    }

    #[test]
    fn update_verification_ok() {
        init_test_data();

        assert_eq!(get_all_verifications().len(), 1);

        assert_eq!(
            update_verification(
                mock_principals::bob(),
                fake_update_verification1(&fake_canister1())
            ),
            Ok(())
        );

        assert_eq!(
            get_verification_by_canister_id(fake_canister1()),
            Some(&fake_verification_use_update_model(
                &mock_principals::alice(),
                &mock_principals::bob(),
                fake_update_verification1(&fake_canister1())
            ))
        );

        assert_eq!(
            update_verification(
                mock_principals::alice(),
                fake_update_verification1(&fake_canister2())
            ),
            Err(Error {
                code: "ERR_003_003_001",
                message: "Verification not found",
                debug_log: None,
            })
        );
    }

    #[test]
    fn submit_verification_ok() {
        init_test_data();

        assert_eq!(get_all_verifications().len(), 1);

        assert_eq!(
            submit_verification(
                mock_principals::bob(),
                fake_submit_verification1(&fake_canister1())
            ),
            Ok(())
        );

        assert_eq!(get_all_verifications().len(), 1);

        assert_eq!(
            get_verification_by_canister_id(fake_canister1()),
            Some(&fake_verification_use_submit_model(
                &mock_principals::alice(),
                &mock_principals::bob(),
                fake_submit_verification1(&fake_canister1())
            ))
        );

        assert_eq!(
            submit_verification(
                mock_principals::bob(),
                fake_submit_verification1(&fake_canister2())
            ),
            Ok(())
        );

        assert_eq!(get_all_verifications().len(), 2);
    }
}
