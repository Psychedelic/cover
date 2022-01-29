use crate::service::model::activity::Activity;
use crate::service::model::pagination::{Pagination, PaginationInfo};
use crate::service::{activity_store, activity_store_mut};

pub fn add_activity(new_activity: Activity) {
    activity_store_mut().add_activity(new_activity)
}

pub fn get_activities(pagination_info: &PaginationInfo) -> Pagination<&'static Activity> {
    activity_store().get_activities(pagination_info)
}
