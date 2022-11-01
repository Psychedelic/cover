use super::{ACTIVITY_STORE, MY_ACTIVITY_STORE};
use crate::common::types::{CallerId, CanisterId};
use crate::model::activity::{Activity, MyActivity, MyBuildConfigActivity};
use crate::model::pagination::{Pagination, PaginationInfo};
use crate::model::verification::BuildStatus;
use crate::util::pagination::total_pages;
use ic_cdk::api::call::ManualReply;
use ic_cdk::api::time;
use ic_cdk::export::candid::CandidType;
use serde::Deserialize;
use std::collections::{BTreeMap, LinkedList};

const MAX_ACTIVITIES_NUMBER: usize = 12_000;
const MAX_MY_ACTIVITIES_NUMBER: usize = 120;

#[derive(Default, CandidType, Deserialize)]
pub struct ActivityStore {
    activities: LinkedList<Activity>,
}

#[derive(Default, CandidType, Deserialize)]
pub struct MyActivityStore {
    my_activities: BTreeMap<CallerId, LinkedList<MyActivity>>,
}

pub fn add_activity(canister_id: CanisterId, build_status: BuildStatus) {
    ACTIVITY_STORE.with(|store| {
        let mut store_ref_mut = store.borrow_mut();
        if store_ref_mut.activities.len() >= MAX_ACTIVITIES_NUMBER {
            store_ref_mut.activities.pop_back();
        }

        store_ref_mut.activities.push_front(Activity {
            canister_id,
            build_status,
            created_at: time(),
        })
    });
}

pub fn add_my_activity(
    canister_id: CanisterId,
    caller_id: CallerId,
    build_status: Option<BuildStatus>,
    build_config_status: Option<MyBuildConfigActivity>,
) {
    MY_ACTIVITY_STORE.with(|store| {
        let mut store_ref_mut = store.borrow_mut();
        let my_activities = store_ref_mut
            .my_activities
            .entry(caller_id)
            .or_insert_with(LinkedList::default);

        if my_activities.len() >= MAX_MY_ACTIVITIES_NUMBER {
            my_activities.pop_back();
        }

        my_activities.push_front(MyActivity {
            canister_id,
            caller_id,
            build_status,
            build_config_status,
            created_at: time(),
        })
    });
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
        if pagination_info.page_index > 0 && pagination_info.page_index <= total_pages {
            // check if last page
            let data_length = match pagination_info.page_index == total_pages {
                true => total_items - (pagination_info.items_per_page * (total_pages - 1)),
                false => pagination_info.items_per_page,
            };

            // calculate where the pagination should start and end
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

pub fn get_my_activities<F: Fn(&Pagination<&MyActivity>) -> ManualReply<Pagination<MyActivity>>>(
    caller_id: CallerId,
    pagination_info: PaginationInfo,
    manual_reply: F,
) -> ManualReply<Pagination<MyActivity>> {
    MY_ACTIVITY_STORE.with(|store| {
        let store_ref = store.borrow();
        let default_my_activities = LinkedList::default();
        let my_activities = store_ref
            .my_activities
            .get(&caller_id)
            .unwrap_or(&default_my_activities);

        let total_items = my_activities.len() as u64;
        let total_pages = total_pages(total_items, pagination_info.items_per_page);

        let mut data: Vec<&MyActivity> = vec![];
        if pagination_info.page_index > 0 && pagination_info.page_index <= total_pages {
            // check if last page
            let data_length = match pagination_info.page_index == total_pages {
                true => total_items - (pagination_info.items_per_page * (total_pages - 1)),
                false => pagination_info.items_per_page,
            };

            // calculate where the pagination should start and end
            let start =
                ((pagination_info.page_index - 1) * pagination_info.items_per_page) as usize;
            let end = start + data_length as usize;

            for (index, my_activity) in my_activities.iter().enumerate() {
                if index == end {
                    break;
                }

                if start <= index && index < end {
                    data.push(my_activity)
                }
            }
        }

        manual_reply(&Pagination::of(data, &pagination_info, total_items))
    })
}
