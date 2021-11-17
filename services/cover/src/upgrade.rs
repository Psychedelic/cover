use ic_kit::macros::{post_upgrade, pre_upgrade};

use crate::service;

#[pre_upgrade]
fn pre_upgrade() {
    service::pre_upgrade();
}

#[post_upgrade]
fn post_upgrade() {
    service::post_upgrade();
}
