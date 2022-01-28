use ic_kit::candid::candid_method;
use ic_kit::macros::{query, update};

use crate::common::types::CanisterId;
use crate::service::guard::is_builder;
use crate::service::model::pagination::{Pagination, PaginationInfo};
use crate::service::model::verification::{SubmitVerification, Verification};
use crate::service::verification;

#[query(name = "getVerificationByCanisterId")]
#[candid_method(query, rename = "getVerificationByCanisterId")]
fn get_verification_by_canister_id(canister_id: CanisterId) -> Option<&'static Verification> {
    verification::get_verification_by_canister_id(&canister_id)
}

#[query(name = "getVerifications")]
#[candid_method(query, rename = "getVerifications")]
fn get_verifications(pagination_info: PaginationInfo) -> Pagination<&'static Verification> {
    verification::get_verifications(&pagination_info)
}

#[update(name = "submitVerification", guard = "is_builder")]
#[candid_method(update, rename = "submitVerification")]
fn submit_verification(verification: SubmitVerification) {
    verification::submit_verification(verification)
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

        submit_verification(fake_submit_verification1(
            &mock_principals::alice(),
            &fake_canister1(),
        ));
    }

    #[test]
    fn get_verifications_ok() {
        init_test_data();

        assert_eq!(
            get_verifications(PaginationInfo {
                page_index: 0,
                items_per_page: 2
            }),
            fake_pagination(
                vec![],
                &PaginationInfo {
                    page_index: 0,
                    items_per_page: 2
                },
                1
            )
        );

        assert_eq!(
            get_verifications(PaginationInfo {
                page_index: 1,
                items_per_page: 2
            }),
            fake_pagination(
                vec![&fake_verification(fake_submit_verification1(
                    &mock_principals::alice(),
                    &fake_canister1()
                ))],
                &PaginationInfo {
                    page_index: 1,
                    items_per_page: 2
                },
                1
            )
        );

        assert_eq!(
            get_verifications(PaginationInfo {
                page_index: 2,
                items_per_page: 2
            }),
            fake_pagination(
                vec![],
                &PaginationInfo {
                    page_index: 2,
                    items_per_page: 2
                },
                1
            )
        );
    }

    #[test]
    fn get_verification_by_canister_id_ok() {
        init_test_data();

        assert_eq!(
            get_verification_by_canister_id(fake_canister1()),
            Some(&fake_verification(fake_submit_verification1(
                &mock_principals::alice(),
                &fake_canister1()
            )))
        );

        assert_eq!(get_verification_by_canister_id(fake_canister2()), None);
    }

    #[test]
    fn submit_verification_ok() {
        init_test_data();

        submit_verification(fake_submit_verification1(
            &mock_principals::bob(),
            &fake_canister1(),
        ));

        assert_eq!(
            get_verifications(PaginationInfo {
                page_index: 1,
                items_per_page: 3
            }),
            fake_pagination(
                vec![&fake_verification(fake_submit_verification1(
                    &mock_principals::bob(),
                    &fake_canister1()
                ))],
                &PaginationInfo {
                    page_index: 1,
                    items_per_page: 3
                },
                1
            )
        );

        submit_verification(fake_submit_verification1(
            &mock_principals::bob(),
            &fake_canister2(),
        ));

        assert_eq!(
            get_verifications(PaginationInfo {
                page_index: 1,
                items_per_page: 3
            }),
            fake_pagination(
                vec![
                    &fake_verification(fake_submit_verification1(
                        &mock_principals::bob(),
                        &fake_canister2()
                    )),
                    &fake_verification(fake_submit_verification1(
                        &mock_principals::bob(),
                        &fake_canister1()
                    )),
                ],
                &PaginationInfo {
                    page_index: 1,
                    items_per_page: 3
                },
                2
            )
        );
    }
}
