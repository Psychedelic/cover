use ic_cdk::api::call::ManualReply;
use ic_cdk::export::candid::candid_method;
use ic_cdk_macros::{query, update};

use crate::common::types::BuilderId;
use crate::service::guard::is_admin;
use crate::service::store::builder;

#[update(name = "addBuilder", guard = "is_admin")]
#[candid_method(update, rename = "addBuilder")]
fn add_builder(builder_id: BuilderId) {
    builder::add_builder(&builder_id)
}

#[update(name = "deleteBuilder", guard = "is_admin")]
#[candid_method(update, rename = "deleteBuilder")]
fn delete_builder(builder_id: BuilderId) {
    builder::delete_builder(&builder_id)
}

#[query(name = "getBuilders", guard = "is_admin", manual_reply = true)]
#[candid_method(query, rename = "getBuilders")]
fn get_builders() -> ManualReply<Vec<BuilderId>> {
    builder::get_builders(|result| ManualReply::one(result))
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use ic_kit::*;
    //
    // fn init_test_data() {
    //     MockContext::new()
    //         .with_caller(mock_principals::bob())
    //         .inject();
    //
    //     add_builder(mock_principals::alice());
    // }
    //
    // #[test]
    // fn add_builder_ok() {
    //     init_test_data();
    //
    //     get_builders_ok();
    //
    //     assert_eq!(get_builders().len(), 1);
    //
    //     add_builder(mock_principals::john());
    //
    //     assert_eq!(get_builders().len(), 2);
    // }
    //
    // #[test]
    // fn get_builders_ok() {
    //     init_test_data();
    //
    //     assert_eq!(get_builders(), vec![&mock_principals::alice()]);
    // }
    //
    // #[test]
    // fn delete_builder_ok() {
    //     init_test_data();
    //
    //     get_builders_ok();
    //
    //     delete_builder(mock_principals::alice());
    //
    //     assert_eq!(get_builders().len(), 0);
    //
    //     delete_builder(mock_principals::john());
    //
    //     assert_eq!(get_builders().len(), 0);
    // }
}
