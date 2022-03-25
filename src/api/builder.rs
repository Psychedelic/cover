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
