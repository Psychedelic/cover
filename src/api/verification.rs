use crate::common::types::CanisterId;
use crate::service::guard::{is_builder, is_validator};
use crate::service::model::error::Error;
use crate::service::model::pagination::{Pagination, PaginationInfo};
use crate::service::model::stats::Stats;
use crate::service::model::verification::{RegisterVerification, SubmitVerification, Verification};
use crate::service::store::activity;
use crate::service::store::verification;
use ic_cdk::api::call::ManualReply;
use ic_cdk::export::candid::candid_method;
use ic_cdk_macros::{query, update};

#[query(name = "getVerificationByCanisterId", manual_reply = true)]
#[candid_method(query, rename = "getVerificationByCanisterId")]
fn get_verification_by_canister_id(canister_id: CanisterId) -> ManualReply<Option<Verification>> {
    verification::get_verification_by_canister_id(&canister_id, |result| ManualReply::one(result))
}

#[query(name = "getVerifications", manual_reply = true)]
#[candid_method(query, rename = "getVerifications")]
fn get_verifications(pagination_info: PaginationInfo) -> ManualReply<Pagination<Verification>> {
    verification::get_verifications(&pagination_info, |result| ManualReply::one(result))
}

#[update(name = "submitVerification", guard = "is_builder")]
#[candid_method(update, rename = "submitVerification")]
fn submit_verification(verification: SubmitVerification) {
    verification::submit_verification(verification, |canister_id, build_status| {
        activity::add_activity(canister_id, build_status)
    })
}

#[update(name = "registerVerification", guard = "is_validator")]
#[candid_method(update, rename = "registerVerification")]
fn register_verification(verification: RegisterVerification) -> Result<(), Error> {
    verification::register_verification(verification, |canister_id, build_status| {
        activity::add_activity(canister_id, build_status)
    })
}

#[query(name = "getVerificationsStats")]
#[candid_method(query, rename = "getVerificationsStats")]
fn get_verifications_stats() -> Stats {
    verification::get_verifications_stats()
}

#[cfg(test)]
mod tests {
    // use crate::service::activity::*;
    // use crate::service::model::verification::BuildStatus;
    // use ic_kit::*;
    //
    // use crate::service::store::test_data::*;
    //
    // use super::*;
    //
    // fn init_test_data() {
    //     MockContext::new()
    //         .with_caller(mock_principals::bob())
    //         .inject();
    //
    //     assert_eq!(
    //         register_verification(fake_register_verification(&fake_canister1())),
    //         Ok(())
    //     );
    // }
    //
    // #[test]
    // fn get_verifications_ok() {
    //     init_test_data();
    //
    //     assert_eq!(
    //         get_verifications(PaginationInfo {
    //             page_index: 0,
    //             items_per_page: 2
    //         }),
    //         fake_pagination(
    //             vec![],
    //             &PaginationInfo {
    //                 page_index: 0,
    //                 items_per_page: 2
    //             },
    //             1
    //         )
    //     );
    //
    //     assert_eq!(
    //         get_verifications(PaginationInfo {
    //             page_index: 1,
    //             items_per_page: 2
    //         }),
    //         fake_pagination(
    //             vec![&fake_verification_use_register_model(
    //                 fake_register_verification(&fake_canister1())
    //             )],
    //             &PaginationInfo {
    //                 page_index: 1,
    //                 items_per_page: 2
    //             },
    //             1
    //         )
    //     );
    //
    //     assert_eq!(
    //         get_verifications(PaginationInfo {
    //             page_index: 2,
    //             items_per_page: 2
    //         }),
    //         fake_pagination(
    //             vec![],
    //             &PaginationInfo {
    //                 page_index: 2,
    //                 items_per_page: 2
    //             },
    //             1
    //         )
    //     );
    // }
    //
    // #[test]
    // fn get_verification_by_canister_id_ok() {
    //     init_test_data();
    //
    //     assert_eq!(
    //         get_verification_by_canister_id(fake_canister1()),
    //         Some(&fake_verification_use_register_model(
    //             fake_register_verification(&fake_canister1())
    //         ))
    //     );
    //
    //     assert_eq!(get_verification_by_canister_id(fake_canister2()), None);
    // }
    //
    // #[test]
    // fn submit_verification_ok() {
    //     init_test_data();
    //
    //     submit_verification(fake_success_verification(
    //         &mock_principals::bob(),
    //         &fake_canister1(),
    //     ));
    //
    //     assert_eq!(
    //         get_activities(&PaginationInfo {
    //             page_index: 2,
    //             items_per_page: 1
    //         }),
    //         fake_pagination(
    //             vec![&fake_activity(fake_canister1(), BuildStatus::Pending)],
    //             &PaginationInfo {
    //                 page_index: 2,
    //                 items_per_page: 1
    //             },
    //             2
    //         )
    //     );
    //
    //     assert_eq!(
    //         get_verification_by_canister_id(fake_canister1()),
    //         Some(&fake_verification(fake_success_verification(
    //             &mock_principals::bob(),
    //             &fake_canister1()
    //         )))
    //     );
    //
    //     submit_verification(fake_error_verification(
    //         &mock_principals::bob(),
    //         &fake_canister2(),
    //     ));
    //
    //     assert_eq!(
    //         get_activities(&PaginationInfo {
    //             page_index: 2,
    //             items_per_page: 2
    //         }),
    //         fake_pagination(
    //             vec![&fake_activity(fake_canister1(), BuildStatus::Pending)],
    //             &PaginationInfo {
    //                 page_index: 2,
    //                 items_per_page: 2
    //             },
    //             3
    //         )
    //     );
    // }
    //
    // #[test]
    // fn register_verification_ok() {
    //     init_test_data();
    //
    //     assert_eq!(
    //         register_verification(fake_register_verification(&fake_canister2())),
    //         Ok(())
    //     );
    //
    //     assert_eq!(
    //         get_activities(&PaginationInfo {
    //             page_index: 1,
    //             items_per_page: 4
    //         }),
    //         fake_pagination(
    //             vec![
    //                 &fake_activity(fake_canister2(), BuildStatus::Pending),
    //                 &fake_activity(fake_canister1(), BuildStatus::Pending)
    //             ],
    //             &PaginationInfo {
    //                 page_index: 1,
    //                 items_per_page: 4
    //             },
    //             2
    //         )
    //     );
    //
    //     assert_eq!(
    //         register_verification(fake_register_verification(&fake_canister1())),
    //         Err(Error::BuildInProgress)
    //     );
    //
    //     assert_eq!(
    //         get_activities(&PaginationInfo {
    //             page_index: 1,
    //             items_per_page: 100
    //         }),
    //         fake_pagination(
    //             vec![
    //                 &fake_activity(fake_canister2(), BuildStatus::Pending),
    //                 &fake_activity(fake_canister1(), BuildStatus::Pending)
    //             ],
    //             &PaginationInfo {
    //                 page_index: 1,
    //                 items_per_page: 100
    //             },
    //             2
    //         )
    //     );
    //
    //     assert_eq!(
    //         get_verifications(PaginationInfo {
    //             page_index: 1,
    //             items_per_page: 20
    //         }),
    //         fake_pagination(
    //             vec![
    //                 &fake_verification_use_register_model(fake_register_verification(
    //                     &fake_canister2()
    //                 )),
    //                 &fake_verification_use_register_model(fake_register_verification(
    //                     &fake_canister1()
    //                 ))
    //             ],
    //             &PaginationInfo {
    //                 page_index: 1,
    //                 items_per_page: 20
    //             },
    //             2
    //         )
    //     );
    // }
}
