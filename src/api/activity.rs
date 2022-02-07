use ic_kit::candid::candid_method;
use ic_kit::macros::query;

use crate::service::activity;
use crate::service::model::activity::Activity;
use crate::service::model::pagination::{Pagination, PaginationInfo};

#[query(name = "getActivities")]
#[candid_method(query, rename = "getActivities")]
fn get_activities(pagination_info: PaginationInfo) -> Pagination<&'static Activity> {
    activity::get_activities(&pagination_info)
}
