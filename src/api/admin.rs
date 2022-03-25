use ic_cdk::api::call::ManualReply;
use ic_cdk::export::candid::candid_method;
use ic_cdk_macros::{query, update};

use crate::common::types::AdminId;
use crate::service::guard::is_admin;
use crate::service::store::admin;

#[update(name = "addAdmin", guard = "is_admin")]
#[candid_method(update, rename = "addAdmin")]
fn add_admin(admin_id: AdminId) {
    admin::add_admin(&admin_id)
}

#[update(name = "deleteAdmin", guard = "is_admin")]
#[candid_method(update, rename = "deleteAdmin")]
fn delete_admin(admin_id: AdminId) {
    admin::delete_admin(&admin_id)
}

#[query(name = "getAdmins", guard = "is_admin", manual_reply = true)]
#[candid_method(query, rename = "getAdmins")]
fn get_admins() -> ManualReply<Vec<AdminId>> {
    admin::get_admins(|result| ManualReply::one(result))
}
