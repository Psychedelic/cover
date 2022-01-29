use crate::service::model::error::Error;
use ic_kit::candid::candid_method;
use ic_kit::macros::{query, update};

use crate::common::types::CanisterId;
use crate::service::guard::{is_builder, is_validator};
use crate::service::model::pagination::{Pagination, PaginationInfo};
use crate::service::model::verification::{RegisterVerification, SubmitVerification, Verification};
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

#[update(name = "registerVerification", guard = "is_validator")]
#[candid_method(update, rename = "registerVerification")]
fn register_verification(verification: RegisterVerification) -> Result<(), Error> {
    verification::register_verification(verification)
}

#[cfg(test)]
mod tests {
    use crate::service::activity::*;
    use crate::service::model::verification::BuildStatus;
    use ic_kit::*;

    use crate::service::store::test_data::*;

    use super::*;

    fn init_test_data() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        submit_verification(fake_success_verification(
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
                vec![&fake_verification(fake_success_verification(
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
            Some(&fake_verification(fake_success_verification(
                &mock_principals::alice(),
                &fake_canister1()
            )))
        );

        assert_eq!(get_verification_by_canister_id(fake_canister2()), None);
    }

    #[test]
    fn submit_verification_ok() {
        init_test_data();

        assert_eq!(
            get_activities(&PaginationInfo {
                page_index: 1,
                items_per_page: 2
            }),
            fake_pagination(
                vec![&fake_activity(fake_canister1(), BuildStatus::Success)],
                &PaginationInfo {
                    page_index: 1,
                    items_per_page: 2
                },
                1
            )
        );

        submit_verification(fake_success_verification(
            &mock_principals::bob(),
            &fake_canister1(),
        ));

        assert_eq!(
            get_activities(&PaginationInfo {
                page_index: 2,
                items_per_page: 1
            }),
            fake_pagination(
                vec![&fake_activity(fake_canister1(), BuildStatus::Success)],
                &PaginationInfo {
                    page_index: 2,
                    items_per_page: 1
                },
                2
            )
        );

        assert_eq!(
            get_verifications(PaginationInfo {
                page_index: 1,
                items_per_page: 3
            }),
            fake_pagination(
                vec![&fake_verification(fake_success_verification(
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

        submit_verification(fake_error_verification(
            &mock_principals::bob(),
            &fake_canister2(),
        ));

        assert_eq!(
            get_activities(&PaginationInfo {
                page_index: 2,
                items_per_page: 2
            }),
            fake_pagination(
                vec![&fake_activity(fake_canister1(), BuildStatus::Success)],
                &PaginationInfo {
                    page_index: 2,
                    items_per_page: 2
                },
                3
            )
        );

        assert_eq!(
            get_verifications(PaginationInfo {
                page_index: 1,
                items_per_page: 3
            }),
            fake_pagination(
                vec![
                    &fake_verification(fake_error_verification(
                        &mock_principals::bob(),
                        &fake_canister2()
                    )),
                    &fake_verification(fake_success_verification(
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

    #[test]
    fn register_verification_ok() {
        init_test_data();

        assert_eq!(
            register_verification(fake_register_verification(
                &mock_principals::alice(),
                &fake_canister1()
            )),
            Ok(())
        );

        assert_eq!(
            get_activities(&PaginationInfo {
                page_index: 1,
                items_per_page: 4
            }),
            fake_pagination(
                vec![
                    &fake_activity(fake_canister1(), BuildStatus::Pending),
                    &fake_activity(fake_canister1(), BuildStatus::Success)
                ],
                &PaginationInfo {
                    page_index: 1,
                    items_per_page: 4
                },
                2
            )
        );

        assert_eq!(
            register_verification(fake_register_verification(
                &mock_principals::john(),
                &fake_canister1()
            )),
            Err(Error::BuildInProgress)
        );

        assert_eq!(
            get_activities(&PaginationInfo {
                page_index: 1,
                items_per_page: 100
            }),
            fake_pagination(
                vec![
                    &fake_activity(fake_canister1(), BuildStatus::Pending),
                    &fake_activity(fake_canister1(), BuildStatus::Success)
                ],
                &PaginationInfo {
                    page_index: 1,
                    items_per_page: 100
                },
                2
            )
        );
    }
}
