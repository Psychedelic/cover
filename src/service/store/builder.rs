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

#[cfg(test)]
mod test {
    // use super::*;
    // use ic_kit::*;
    //
    // fn init_test_data() -> BuilderStore {
    //     let mut store = BuilderStore::default();
    //
    //     store.add_builder(&mock_principals::bob());
    //
    //     store
    // }
    //
    // #[test]
    // fn get_builder_ok() {
    //     let store = init_test_data();
    //
    //     assert_eq!(store.get_builders(), vec![&mock_principals::bob()]);
    // }
    //
    // #[test]
    // fn add_builder_ok() {
    //     let mut store = init_test_data();
    //
    //     assert_eq!(store.get_builders().len(), 1);
    //
    //     store.add_builder(&mock_principals::john());
    //
    //     assert_eq!(store.get_builders().len(), 2);
    //
    //     assert!(store.builder_existed(&mock_principals::john()));
    //
    //     store.add_builder(&mock_principals::alice());
    //
    //     assert_eq!(store.get_builders().len(), 3);
    //
    //     assert!(store.builder_existed(&mock_principals::alice()));
    //
    //     store.add_builder(&mock_principals::alice());
    //
    //     assert_eq!(store.get_builders().len(), 3);
    // }
    //
    // #[test]
    // fn delete_builder_ok() {
    //     let mut store = init_test_data();
    //
    //     store.delete_builder(&mock_principals::alice());
    //     assert_eq!(store.get_builders().len(), 1);
    //
    //     store.delete_builder(&mock_principals::bob());
    //     assert_eq!(store.get_builders().len(), 0);
    // }
}
