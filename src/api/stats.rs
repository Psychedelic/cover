use ic_kit::candid::candid_method;
use ic_kit::macros::query;

use crate::service::model::stats::Stats;
use crate::service::stats;

#[query(name = "getStats")]
#[candid_method(query, rename = "getStats")]
fn get_stats() -> Stats {
    stats::get_stats()
}
