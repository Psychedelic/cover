use crate::service::model::activity::Activity;
use crate::service::model::pagination::{Pagination, PaginationInfo};
use crate::service::store::activity;
use ic_cdk::api::call::ManualReply;
use ic_cdk::export::candid::candid_method;
use ic_cdk_macros::query;

#[query(name = "getActivities", manual_reply = true)]
#[candid_method(query, rename = "getActivities")]
fn get_activities(pagination_info: PaginationInfo) -> ManualReply<Pagination<Activity>> {
    activity::get_activities(pagination_info, |result| ManualReply::one(result))
}
