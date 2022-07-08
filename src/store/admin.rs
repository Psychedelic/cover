use super::ADMIN_STORE;
use crate::common::types::AdminId;
use ic_cdk::api::call::ManualReply;
use ic_cdk::export::candid::CandidType;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Default, CandidType, Deserialize)]
pub struct AdminStore {
    admins: HashSet<AdminId>,
}

pub fn admin_existed(admin_id: &AdminId) -> bool {
    ADMIN_STORE.with(|store| store.borrow().admins.contains(admin_id))
}

pub fn add_admin(admin_id: &AdminId) {
    ADMIN_STORE.with(|store| {
        store.borrow_mut().admins.insert(*admin_id);
    })
}

pub fn delete_admin(admin_id: &AdminId) {
    ADMIN_STORE.with(|store| {
        store.borrow_mut().admins.remove(admin_id);
    })
}

pub fn get_admins<F: Fn(Vec<&AdminId>) -> ManualReply<Vec<AdminId>>>(
    manual_reply: F,
) -> ManualReply<Vec<AdminId>> {
    ADMIN_STORE.with(|store| manual_reply(store.borrow().admins.iter().collect()))
}
