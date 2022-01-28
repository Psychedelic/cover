use crate::service::model::pagination::{Pagination, PaginationInfo};

impl<T> Pagination<T> {
    pub fn of(data: Vec<T>, pagination_info: &PaginationInfo, total_items: u64) -> Pagination<T> {
        let total_pages = total_pages(total_items, pagination_info.items_per_page);
        Self {
            data,
            total_items,
            total_pages,
            page_index: pagination_info.page_index,
            items_per_page: pagination_info.items_per_page,
            is_first_page: pagination_info.page_index == 0,
            is_last_page: total_pages == pagination_info.page_index,
        }
    }
}

pub fn total_pages(total_items: u64, items_per_page: u64) -> u64 {
    if items_per_page == 0 {
        0
    } else {
        let is_even = total_items % items_per_page == 0;
        let extra_page = if is_even { 0 } else { 1 };
        (total_items / items_per_page) + extra_page
    }
}
