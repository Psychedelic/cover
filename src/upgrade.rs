use ic_cdk_macros::{post_upgrade, pre_upgrade};

use crate::service;

#[pre_upgrade]
fn pre_upgrade() {
    service::store::pre_upgrade();
}

#[post_upgrade]
fn post_upgrade() {
    service::store::post_upgrade();
}
