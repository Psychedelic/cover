use ic_kit::candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug, PartialEq)]
pub struct PaginationInfo {
    pub page_index: u64,
    pub items_per_page: u64,
}

#[derive(CandidType, Deserialize, Debug, PartialEq)]
pub struct Pagination<T> {
    pub data: Vec<T>,
    pub total_items: u64,
    pub total_pages: u64,
    pub page_index: u64,
    pub items_per_page: u64,
    pub is_first_page: bool,
    pub is_last_page: bool,
}
