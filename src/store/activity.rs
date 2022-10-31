use super::ACTIVITY_STORE;
use crate::common::types::CanisterId;
use crate::model::activity::Activity;
use crate::model::pagination::{Pagination, PaginationInfo};
use crate::model::verification::BuildStatus;
use crate::util::pagination::total_pages;
use ic_cdk::api::call::ManualReply;
use ic_cdk::api::time;
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
        if store_ref_mut.activities.len() >= MAX_ACTIVITIES_NUMBER {
            store_ref_mut.activities.pop_back();
        }

        store_ref_mut.activities.push_front(Activity {
            canister_id,
            build_status,
            created_at: time(),
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
