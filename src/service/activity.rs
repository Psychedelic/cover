use crate::service::activity_store;
use crate::service::model::activity::Activity;
use crate::service::model::pagination::{Pagination, PaginationInfo};

pub fn get_activities(pagination_info: &PaginationInfo) -> Pagination<&'static Activity> {
    activity_store().get_activities(pagination_info)
}
