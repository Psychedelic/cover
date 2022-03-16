use crate::service::model::stats::Stats;
use chrono::{DateTime, Utc};
use ic_cdk::api::call::ManualReply;
use ic_cdk::export::candid::CandidType;
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;

use super::VERIFICATION_STORE;
use crate::common::types::CanisterId;
use crate::service::model::error::Error;
use crate::service::model::pagination::{Pagination, PaginationInfo};
use crate::service::model::verification::{
    BuildStatus, CanisterType, RegisterVerification, SubmitVerification, Verification,
};
use crate::service::pagination::total_pages;
use crate::service::time_utils;

#[derive(CandidType, Deserialize, Default)]
pub struct VerificationStore {
    verifications: HashMap<CanisterId, Verification>,
    records: Vec<CanisterId>,
}

pub fn submit_verification<F: Fn(CanisterId, BuildStatus)>(
    new_verification: SubmitVerification,
    activity_handler: F,
) {
    VERIFICATION_STORE.with(|store| {
        let canister_id = new_verification.canister_id;
        let build_status = new_verification.build_status;
        store.borrow_mut().verifications.insert(
            new_verification.canister_id,
            Verification {
                canister_id: new_verification.canister_id,
                canister_name: new_verification.canister_name,
                repo_url: new_verification.repo_url,
                commit_hash: new_verification.commit_hash,
                wasm_hash: new_verification.wasm_hash,
                build_url: Some(new_verification.build_url),
                build_status: new_verification.build_status,
                rust_version: new_verification.rust_version,
                canister_type: new_verification.canister_type,
                dfx_version: new_verification.dfx_version,
                optimize_count: new_verification.optimize_count,
                repo_visibility: new_verification.repo_visibility,
                updated_by: new_verification.owner_id,
                updated_at: time_utils::now_to_str(),
            },
        );
        activity_handler(canister_id, build_status);
    })
}

pub fn get_verification_by_canister_id<
    F: Fn(Option<&Verification>) -> ManualReply<Option<Verification>>,
>(
    canister_id: &CanisterId,
    manual_reply: F,
) -> ManualReply<Option<Verification>> {
    VERIFICATION_STORE.with(|store| manual_reply(store.borrow().verifications.get(canister_id)))
}

pub fn get_verifications<
    F: Fn(Pagination<&Verification>) -> ManualReply<Pagination<Verification>>,
>(
    pagination_info: &PaginationInfo,
    reply: F,
) -> ManualReply<Pagination<Verification>> {
    VERIFICATION_STORE.with(|store| {
        let store_ref = store.borrow();
        let total_items = store_ref.records.len() as u64;
        let total_pages = total_pages(total_items, pagination_info.items_per_page);

        let mut data: Vec<&Verification> = vec![];
        if 0 < pagination_info.page_index && pagination_info.page_index <= total_pages {
            // check if last page
            let data_length = match pagination_info.page_index == total_pages {
                true => total_items - (pagination_info.items_per_page * (total_pages - 1)),
                false => pagination_info.items_per_page,
            };

            //calculate where the pagination should start and end
            let start = (total_items
                - (pagination_info.page_index - 1) * pagination_info.items_per_page)
                as usize;
            let end = start - data_length as usize;

            //because latest items will get appended to the end of 'records'
            //so in order to get latest data first, we'll iterate 'records' and push verification into 'data' in reverse order
            //end will be included and start will be excluded
            for i in (end..start).rev() {
                data.push(&store_ref.verifications[&store_ref.records[i]])
            }
        }

        reply(Pagination::of(data, pagination_info, total_items))
    })
}

pub fn register_verification<F: Fn(CanisterId, BuildStatus)>(
    register_verification: RegisterVerification,
    activity_handler: F,
) -> Result<(), Error> {
    VERIFICATION_STORE.with(|store| {
        let mut store_ref_mut = store.borrow_mut();
        let canister_id = register_verification.canister_id;
        let build_status = BuildStatus::Pending;
        store_ref_mut
            .verifications
            .get_mut(&register_verification.canister_id)
            .map(|verification| match verification.build_status {
                BuildStatus::Pending | BuildStatus::Building => {
                    //user have to wait 5 minutes until next register
                    let time_update: DateTime<Utc> =
                        DateTime::from_str(&*verification.updated_at).unwrap();
                    let minutes_from_last_update = time_utils::get_now()
                        .signed_duration_since(time_update)
                        .num_minutes();
                    if minutes_from_last_update > 5 {
                        Ok(())
                    } else {
                        Err(Error::BuildInProgress)
                    }
                }
                BuildStatus::Error | BuildStatus::Success => Ok(()),
            })
            .unwrap_or_else(|| Ok(()))
            .map(|_| {
                store_ref_mut
                    .verifications
                    .insert(
                        register_verification.canister_id,
                        Verification {
                            canister_id: register_verification.canister_id,
                            canister_name: register_verification.canister_name,
                            repo_url: register_verification.repo_url,
                            commit_hash: register_verification.commit_hash,
                            wasm_hash: None,
                            build_url: None,
                            build_status,
                            canister_type: None,
                            rust_version: register_verification.rust_version,
                            dfx_version: register_verification.dfx_version,
                            optimize_count: register_verification.optimize_count,
                            repo_visibility: None,
                            updated_by: register_verification.owner_id,
                            updated_at: time_utils::now_to_str(),
                        },
                    )
                    .is_none()
                    .then(|| store_ref_mut.records.push(canister_id));
                activity_handler(canister_id, build_status)
            })
    })
}

pub fn get_verifications_stats() -> Stats {
    VERIFICATION_STORE.with(|store| {
        let store_ref = store.borrow();
        let verifications = store_ref
            .verifications
            .iter()
            .map(|(_, verification)| verification)
            .collect::<Vec<&Verification>>();

        let mut stats = Stats {
            total_canisters: verifications.len(),
            motoko_canisters_count: 0,
            rust_canisters_count: 0,
            build_pending_count: 0,
            build_in_progress_count: 0,
            build_error_count: 0,
            build_success_count: 0,
        };

        for v in verifications {
            if let Some(canister_type) = v.canister_type {
                match canister_type {
                    CanisterType::Rust => stats.rust_canisters_count += 1,
                    CanisterType::Motoko => stats.motoko_canisters_count += 1,
                }
            };

            match v.build_status {
                BuildStatus::Pending => stats.build_pending_count += 1,
                BuildStatus::Building => stats.build_in_progress_count += 1,
                BuildStatus::Error => stats.build_error_count += 1,
                BuildStatus::Success => stats.build_success_count += 1,
            };
        }

        stats
    })
}

#[cfg(test)]
mod test {
    // use ic_kit::*;
    //
    // use crate::service::store::test_data::*;
    //
    // use super::*;
    //
    // fn init_test_data() -> VerificationStore {
    //     let mut store = VerificationStore::default();
    //
    //     assert_eq!(
    //         store.register_verification(
    //             fake_register_verification(&fake_canister1()),
    //             |canister_id, build_status| {
    //                 assert_eq!(canister_id, fake_canister1());
    //                 assert_eq!(build_status, BuildStatus::Pending);
    //             },
    //         ),
    //         Ok(())
    //     );
    //
    //     assert_eq!(
    //         store.register_verification(
    //             fake_register_verification(&fake_canister2()),
    //             |canister_id, build_status| {
    //                 assert_eq!(canister_id, fake_canister2());
    //                 assert_eq!(build_status, BuildStatus::Pending);
    //             },
    //         ),
    //         Ok(())
    //     );
    //
    //     store
    // }
    //
    // #[test]
    // fn submit_verification_ok() {
    //     let mut store = init_test_data();
    //
    //     store.submit_verification(
    //         fake_success_verification(&mock_principals::alice(), &fake_canister3()),
    //         |canister_id, build_status| {
    //             assert_eq!(canister_id, fake_canister3());
    //             assert_eq!(build_status, BuildStatus::Success);
    //         },
    //     );
    //
    //     assert_eq!(
    //         store.get_verification_by_canister_id(&fake_canister3()),
    //         Some(&fake_verification(fake_success_verification(
    //             &mock_principals::alice(),
    //             &fake_canister3()
    //         )))
    //     );
    //
    //     store.submit_verification(
    //         fake_error_verification(&mock_principals::alice(), &fake_canister1()),
    //         |canister_id, build_status| {
    //             assert_eq!(canister_id, fake_canister1());
    //             assert_eq!(build_status, BuildStatus::Error);
    //         },
    //     );
    //
    //     assert_eq!(
    //         store.get_verification_by_canister_id(&fake_canister1()),
    //         Some(&fake_verification(fake_error_verification(
    //             &mock_principals::alice(),
    //             &fake_canister1()
    //         )))
    //     );
    // }
    //
    // #[test]
    // fn get_verifications_ok() {
    //     let mut store = init_test_data();
    //
    //     assert_eq!(
    //         store.register_verification(
    //             fake_register_verification(&fake_canister3()),
    //             |canister_id, build_status| {
    //                 assert_eq!(canister_id, fake_canister3());
    //                 assert_eq!(build_status, BuildStatus::Pending);
    //             },
    //         ),
    //         Ok(())
    //     );
    //
    //     assert_eq!(
    //         store.get_verifications(&PaginationInfo {
    //             page_index: 0,
    //             items_per_page: 2
    //         }),
    //         fake_pagination(
    //             vec![],
    //             &PaginationInfo {
    //                 page_index: 0,
    //                 items_per_page: 2
    //             },
    //             store.verifications.len() as u64
    //         )
    //     );
    //
    //     assert_eq!(
    //         store.get_verifications(&PaginationInfo {
    //             page_index: 1,
    //             items_per_page: 2
    //         }),
    //         fake_pagination(
    //             vec![
    //                 &fake_verification_use_register_model(fake_register_verification(
    //                     &fake_canister3()
    //                 )),
    //                 &fake_verification_use_register_model(fake_register_verification(
    //                     &fake_canister2()
    //                 ))
    //             ],
    //             &PaginationInfo {
    //                 page_index: 1,
    //                 items_per_page: 2
    //             },
    //             store.verifications.len() as u64
    //         )
    //     );
    //
    //     assert_eq!(
    //         store.get_verifications(&PaginationInfo {
    //             page_index: 2,
    //             items_per_page: 2
    //         }),
    //         fake_pagination(
    //             vec![&fake_verification_use_register_model(
    //                 fake_register_verification(&fake_canister1())
    //             )],
    //             &PaginationInfo {
    //                 page_index: 2,
    //                 items_per_page: 2
    //             },
    //             store.verifications.len() as u64
    //         )
    //     );
    //
    //     assert_eq!(
    //         store.get_verifications(&PaginationInfo {
    //             page_index: 3,
    //             items_per_page: 2
    //         }),
    //         fake_pagination(
    //             vec![],
    //             &PaginationInfo {
    //                 page_index: 3,
    //                 items_per_page: 2
    //             },
    //             store.verifications.len() as u64
    //         )
    //     );
    //
    //     assert_eq!(
    //         store.get_verifications(&PaginationInfo {
    //             page_index: 3,
    //             items_per_page: 0
    //         }),
    //         fake_pagination(
    //             vec![],
    //             &PaginationInfo {
    //                 page_index: 3,
    //                 items_per_page: 0
    //             },
    //             store.verifications.len() as u64
    //         )
    //     );
    // }
    //
    // #[test]
    // fn register_verification_ok() {
    //     let mut store = init_test_data();
    //
    //     assert_eq!(
    //         store.register_verification(
    //             fake_register_verification(&fake_canister1()),
    //             |canister_id, build_status| {
    //                 assert_eq!(canister_id, fake_canister1());
    //                 assert_eq!(build_status, BuildStatus::Pending);
    //             },
    //         ),
    //         Err(Error::BuildInProgress)
    //     );
    //
    //     assert_eq!(
    //         store.register_verification(
    //             fake_register_verification(&fake_canister3()),
    //             |canister_id, build_status| {
    //                 assert_eq!(canister_id, fake_canister3());
    //                 assert_eq!(build_status, BuildStatus::Pending);
    //             },
    //         ),
    //         Ok(())
    //     );
    //
    //     assert_eq!(
    //         store.get_verifications(&PaginationInfo {
    //             page_index: 2,
    //             items_per_page: 1
    //         }),
    //         fake_pagination(
    //             vec![&fake_verification_use_register_model(
    //                 fake_register_verification(&fake_canister2())
    //             )],
    //             &PaginationInfo {
    //                 page_index: 2,
    //                 items_per_page: 1
    //             },
    //             store.verifications.len() as u64
    //         )
    //     );
    // }
}
