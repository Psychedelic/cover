use crate::common::constants::{MAX_ITEMS_PER_PAGE, MIN_ITEMS_PER_PAGE};
use crate::service::model::activity::Activity;
use crate::service::model::pagination::{Pagination, PaginationInfo};
use crate::service::store::activity;
use ic_cdk::api::call::ManualReply;
use ic_cdk::export::candid::candid_method;
use ic_cdk_macros::query;
use std::cmp::{max, min};

#[query(name = "getActivities", manual_reply = true)]
#[candid_method(query, rename = "getActivities")]
fn get_activities(mut pagination_info: PaginationInfo) -> ManualReply<Pagination<Activity>> {
    pagination_info.items_per_page = max(MIN_ITEMS_PER_PAGE, pagination_info.items_per_page);
    pagination_info.items_per_page = min(MAX_ITEMS_PER_PAGE, pagination_info.items_per_page);

    activity::get_activities(pagination_info, |result| ManualReply::one(result))
}
