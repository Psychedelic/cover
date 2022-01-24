use ic_kit::candid::candid_method;
use ic_kit::macros::{query, update};

use crate::common::types::{CallerId, CanisterId};
use crate::service::guard::is_provider;
use crate::service::model::verification::{SubmitVerification, Verification};
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

#[update(name = "submitVerification", guard = "is_provider")]
#[candid_method(update, rename = "submitVerification")]
fn submit_verification(owner_id: CallerId, verification: SubmitVerification) {
    verification::submit_verification(&owner_id, verification)
}

#[cfg(test)]
mod tests {
    use ic_kit::*;

    use crate::service::store::test_data::*;

    use super::*;

    fn init_test_data() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        submit_verification(
            mock_principals::alice(),
            fake_submit_verification1(&fake_canister1()),
        );
    }

    #[test]
    fn get_all_verifications_ok() {
        init_test_data();

        assert_eq!(
            get_all_verifications(),
            vec![&fake_verification(
                &mock_principals::alice(),
                fake_submit_verification1(&fake_canister1())
            )]
        );
    }

    #[test]
    fn get_verification_by_canister_id_ok() {
        init_test_data();

        assert_eq!(
            get_verification_by_canister_id(fake_canister1()),
            Some(&fake_verification(
                &mock_principals::alice(),
                fake_submit_verification1(&fake_canister1())
            ))
        );

        assert_eq!(get_verification_by_canister_id(fake_canister2()), None);
    }

    #[test]
    fn submit_verification_ok() {
        init_test_data();

        get_all_verifications_ok();

        submit_verification(
            mock_principals::bob(),
            fake_submit_verification1(&fake_canister1()),
        );

        assert_eq!(
            get_all_verifications(),
            vec![&fake_verification(
                &mock_principals::bob(),
                fake_submit_verification1(&fake_canister1())
            )]
        );

        submit_verification(
            mock_principals::bob(),
            fake_submit_verification1(&fake_canister2()),
        );

        assert_eq!(
            get_all_verifications(),
            vec![
                &fake_verification(
                    &mock_principals::bob(),
                    fake_submit_verification1(&fake_canister2())
                ),
                &fake_verification(
                    &mock_principals::bob(),
                    fake_submit_verification1(&fake_canister1())
                )
            ]
        );
    }
}
