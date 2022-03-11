// use crate::service::model::pagination::{Pagination, PaginationInfo};
// use crate::service::pagination::total_pages;
//
// pub fn fake_pagination<T>(data: Vec<T>, info: &PaginationInfo, total: u64) -> Pagination<T> {
//     let total_pages = total_pages(total, info.items_per_page);
//     Pagination {
//         data,
//         total_items: total,
//         total_pages,
//         page_index: info.page_index,
//         items_per_page: info.items_per_page,
//         is_first_page: info.page_index == 0,
//         is_last_page: info.page_index == total_pages,
//     }
// }
