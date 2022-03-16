use super::ACTIVITY_STORE;
use crate::common::types::CanisterId;
use crate::service::model::activity::Activity;
use crate::service::model::pagination::{Pagination, PaginationInfo};
use crate::service::model::verification::BuildStatus;
use crate::service::pagination::total_pages;
use crate::service::time_utils;
use ic_cdk::api::call::ManualReply;
use ic_cdk::export::candid::CandidType;
use serde::Deserialize;
use std::collections::LinkedList;
const MAX_ACTIVITIES_NUMBER: usize = 120;

#[derive(Default, CandidType, Deserialize)]
pub struct ActivityStore {
    activities: LinkedList<Activity>,
}

pub fn add_activity(canister_id: CanisterId, build_status: BuildStatus) {
    ACTIVITY_STORE.with(|store| {
        let mut store_ref_mut = store.borrow_mut();
        if store.borrow().activities.len() >= MAX_ACTIVITIES_NUMBER {
            store_ref_mut.activities.pop_back();
        }

        store_ref_mut.activities.push_front(Activity {
            canister_id,
            build_status,
            create_at: time_utils::now_to_str(),
        })
    })
}

pub fn get_activities<F: Fn(&Pagination<&Activity>) -> ManualReply<Pagination<Activity>>>(
    pagination_info: PaginationInfo,
    manual_reply: F,
) -> ManualReply<Pagination<Activity>> {
    ACTIVITY_STORE.with(|store| {
        let store_ref = store.borrow();
        let total_items = store_ref.activities.len() as u64;
        let total_pages = total_pages(total_items, pagination_info.items_per_page);

        let mut data: Vec<&Activity> = vec![];
        if 0 < pagination_info.page_index && pagination_info.page_index <= total_pages {
            // check if last page
            let data_length = match pagination_info.page_index == total_pages {
                true => total_items - (pagination_info.items_per_page * (total_pages - 1)),
                false => pagination_info.items_per_page,
            };

            //calculate where the pagination should start and end
            let start =
                ((pagination_info.page_index - 1) * pagination_info.items_per_page) as usize;
            let end = start + data_length as usize;

            for (index, activity) in store_ref.activities.iter().enumerate() {
                if index == end {
                    break;
                }

                if start <= index && index < end {
                    data.push(activity)
                }
            }
        }

        manual_reply(&Pagination::of(data, &pagination_info, total_items))
    })
}

#[cfg(test)]
mod test {
    // use crate::service::model::verification::BuildStatus;
    // use crate::service::store::test_data::*;
    //
    // use super::*;

    // fn init_test_data() -> ActivityStore {
    //     let mut store = ActivityStore::default();
    //
    //     store.add_activity(fake_canister1(), BuildStatus::Success);
    //
    //     store.add_activity(fake_canister2(), BuildStatus::Error);
    //
    //     store.add_activity(fake_canister3(), BuildStatus::Pending);
    //
    //     store
    // }
    //
    // fn init_max_activities() -> ActivityStore {
    //     let mut store = ActivityStore::default();
    //
    //     for _ in 0..MAX_ACTIVITIES_NUMBER {
    //         store.add_activity(fake_canister1(), BuildStatus::Pending)
    //     }
    //
    //     store
    // }
    //
    // #[test]
    // fn get_activities_ok() {
    //     let store = init_test_data();
    //
    //     assert_eq!(
    //         store.get_activities(&PaginationInfo {
    //             page_index: 2,
    //             items_per_page: 2
    //         }),
    //         fake_pagination(
    //             vec![&fake_activity(fake_canister1(), BuildStatus::Success)],
    //             &PaginationInfo {
    //                 page_index: 2,
    //                 items_per_page: 2
    //             },
    //             store.activities.len() as u64
    //         )
    //     );
    //
    //     assert_eq!(
    //         store.get_activities(&PaginationInfo {
    //             page_index: 1,
    //             items_per_page: 100
    //         }),
    //         fake_pagination(
    //             vec![
    //                 &fake_activity(fake_canister3(), BuildStatus::Pending),
    //                 &fake_activity(fake_canister2(), BuildStatus::Error),
    //                 &fake_activity(fake_canister1(), BuildStatus::Success)
    //             ],
    //             &PaginationInfo {
    //                 page_index: 1,
    //                 items_per_page: 100
    //             },
    //             store.activities.len() as u64
    //         )
    //     );
    //
    //     assert_eq!(
    //         store.get_activities(&PaginationInfo {
    //             page_index: 0,
    //             items_per_page: 5
    //         }),
    //         fake_pagination(
    //             vec![],
    //             &PaginationInfo {
    //                 page_index: 0,
    //                 items_per_page: 5
    //             },
    //             store.activities.len() as u64
    //         )
    //     );
    //
    //     assert_eq!(
    //         store.get_activities(&PaginationInfo {
    //             page_index: 2,
    //             items_per_page: 50
    //         }),
    //         fake_pagination(
    //             vec![],
    //             &PaginationInfo {
    //                 page_index: 2,
    //                 items_per_page: 50
    //             },
    //             store.activities.len() as u64
    //         )
    //     );
    //
    //     assert_eq!(
    //         store.get_activities(&PaginationInfo {
    //             page_index: 1,
    //             items_per_page: 1
    //         }),
    //         fake_pagination(
    //             vec![&fake_activity(fake_canister3(), BuildStatus::Pending)],
    //             &PaginationInfo {
    //                 page_index: 1,
    //                 items_per_page: 1
    //             },
    //             store.activities.len() as u64
    //         )
    //     );
    //
    //     assert_eq!(
    //         store.get_activities(&PaginationInfo {
    //             page_index: 0,
    //             items_per_page: 0
    //         }),
    //         fake_pagination(
    //             vec![],
    //             &PaginationInfo {
    //                 page_index: 0,
    //                 items_per_page: 0
    //             },
    //             store.activities.len() as u64
    //         )
    //     );
    // }
    //
    // #[test]
    // fn add_activity_ok() {
    //     let mut store = init_max_activities();
    //
    //     assert_eq!(
    //         store.activities.front(),
    //         Some(&fake_activity(fake_canister1(), BuildStatus::Pending))
    //     );
    //
    //     assert_eq!(store.activities.len(), MAX_ACTIVITIES_NUMBER);
    //
    //     store.add_activity(fake_canister3(), BuildStatus::Success);
    //
    //     assert_eq!(store.activities.len(), MAX_ACTIVITIES_NUMBER);
    //
    //     assert_eq!(
    //         store.activities.front(),
    //         Some(&fake_activity(fake_canister3(), BuildStatus::Success))
    //     );
    //
    //     assert_eq!(
    //         store.get_activities(&PaginationInfo {
    //             page_index: 18,
    //             items_per_page: 7,
    //         }),
    //         fake_pagination(
    //             vec![&fake_activity(fake_canister1(), BuildStatus::Pending)],
    //             &PaginationInfo {
    //                 page_index: 18,
    //                 items_per_page: 7,
    //             },
    //             MAX_ACTIVITIES_NUMBER as u64,
    //         )
    //     );
    // }
}
