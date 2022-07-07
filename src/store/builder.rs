use std::collections::HashSet;

use ic_cdk::api::call::ManualReply;
use ic_cdk::export::candid::CandidType;

use serde::Deserialize;

use super::BUILDER_STORE;
use crate::common::types::BuilderId;

#[derive(CandidType, Deserialize, Default)]
pub struct BuilderStore {
    builder: HashSet<BuilderId>,
}

pub fn builder_existed(builder_id: &BuilderId) -> bool {
    BUILDER_STORE.with(|store| store.borrow().builder.contains(builder_id))
}

pub fn add_builder(builder_id: &BuilderId) {
    BUILDER_STORE.with(|store| {
        store.borrow_mut().builder.insert(*builder_id);
    })
}

pub fn delete_builder(builder_id: &BuilderId) {
    BUILDER_STORE.with(|store| {
        store.borrow_mut().builder.remove(builder_id);
    })
}

pub fn get_builders<F: Fn(Vec<&BuilderId>) -> ManualReply<Vec<BuilderId>>>(
    manual_reply: F,
) -> ManualReply<Vec<BuilderId>> {
    BUILDER_STORE.with(|store| manual_reply(store.borrow().builder.iter().collect()))
}
