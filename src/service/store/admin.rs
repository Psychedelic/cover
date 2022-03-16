use super::ADMIN_STORE;
use ic_cdk::api::call::ManualReply;
use ic_cdk::export::candid::CandidType;
use serde::Deserialize;
use std::collections::HashSet;

use crate::common::types::AdminId;

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

#[cfg(test)]
mod test {
    // use ic_kit::*;

    // use super::*;

    // fn init_test_data() -> AdminStore {
    //     let mut store = AdminStore::default();
    //
    //     store.add_admin(&mock_principals::alice());
    //
    //     store.add_admin(&mock_principals::bob());
    //
    //     store
    // }
    //
    // #[test]
    // fn add_admin_ok() {
    //     let mut store = AdminStore::default();
    //
    //     assert_eq!(store.get_admins().len(), 0);
    //
    //     store.add_admin(&mock_principals::alice());
    //
    //     assert_eq!(store.get_admins(), vec![&mock_principals::alice()]);
    //
    //     store.add_admin(&mock_principals::alice());
    //
    //     assert_eq!(store.get_admins(), vec![&mock_principals::alice()]);
    //
    //     store.add_admin(&mock_principals::john());
    //
    //     assert_eq!(store.get_admins().len(), 2);
    //
    //     assert!(store.admin_existed(&mock_principals::alice()));
    //
    //     assert!(store.admin_existed(&mock_principals::john()));
    // }
    //
    // #[test]
    // fn delete_admin_ok() {
    //     let mut store = init_test_data();
    //
    //     get_admins_ok();
    //
    //     store.delete_admin(&mock_principals::bob());
    //
    //     assert_eq!(store.get_admins(), vec![&mock_principals::alice()]);
    //
    //     store.delete_admin(&mock_principals::bob());
    //
    //     assert_eq!(store.get_admins(), vec![&mock_principals::alice()]);
    //
    //     store.delete_admin(&mock_principals::alice());
    //
    //     assert_eq!(store.get_admins().len(), 0);
    // }
    //
    // #[test]
    // fn admin_existed_ok() {
    //     let store = init_test_data();
    //
    //     assert!(store.admin_existed(&mock_principals::alice()));
    //
    //     assert!(!store.admin_existed(&mock_principals::john()));
    // }
    //
    // #[test]
    // fn get_admins_ok() {
    //     let store = init_test_data();
    //
    //     assert_eq!(store.get_admins().len(), 2);
    //
    //     assert!(store.admin_existed(&mock_principals::alice()));
    //
    //     assert!(store.admin_existed(&mock_principals::bob()));
    // }
}
