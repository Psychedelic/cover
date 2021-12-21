use std::collections::HashSet;

use ic_kit::candid::CandidType;
use serde::Deserialize;

use crate::common::types::AdminId;
use crate::service::store::error::ErrorKindStore;

#[derive(Default, CandidType, Deserialize)]
pub struct AdminStore {
    admins: HashSet<AdminId>,
}

impl AdminStore {
    pub fn admin_existed(&self, admin_id: &AdminId) -> bool {
        self.admins.contains(admin_id)
    }

    pub fn add_admin(&mut self, admin_id: &AdminId) -> Result<(), ErrorKindStore> {
        self.admins
            .insert(*admin_id)
            .then(|| ())
            .ok_or(ErrorKindStore::AdminExisted)
    }

    pub fn delete_admin(&mut self, admin_id: &AdminId) -> Result<(), ErrorKindStore> {
        self.admins
            .remove(admin_id)
            .then(|| ())
            .ok_or(ErrorKindStore::AdminNotFound)
    }

    pub fn get_all_admins(&self) -> Vec<&AdminId> {
        self.admins.iter().collect()
    }
}

#[cfg(test)]
mod test {
    use ic_kit::*;

    use super::*;

    fn init_test_data() -> AdminStore {
        let mut store = AdminStore::default();

        assert_eq!(store.add_admin(&mock_principals::alice()), Ok(()));

        assert_eq!(store.add_admin(&mock_principals::bob()), Ok(()));

        store
    }

    #[test]
    fn add_admin_ok() {
        let mut store = AdminStore::default();

        assert_eq!(store.get_all_admins().len(), 0);

        assert_eq!(store.add_admin(&mock_principals::alice()), Ok(()));

        assert_eq!(store.get_all_admins().len(), 1);

        assert_eq!(
            store.add_admin(&mock_principals::alice()),
            Err(ErrorKindStore::AdminExisted)
        );

        assert_eq!(store.get_all_admins().len(), 1);

        assert_eq!(store.add_admin(&mock_principals::bob()), Ok(()));

        assert_eq!(store.get_all_admins().len(), 2);
    }

    #[test]
    fn delete_admin_ok() {
        let mut store = init_test_data();

        assert_eq!(store.get_all_admins().len(), 2);

        assert_eq!(store.delete_admin(&mock_principals::bob()), Ok(()));

        assert_eq!(store.get_all_admins().len(), 1);

        assert_eq!(
            store.delete_admin(&mock_principals::bob()),
            Err(ErrorKindStore::AdminNotFound)
        );

        assert_eq!(store.get_all_admins().len(), 1);

        assert_eq!(store.delete_admin(&mock_principals::alice()), Ok(()));

        assert_eq!(store.get_all_admins().len(), 0);
    }

    #[test]
    fn admin_existed_ok() {
        let store = init_test_data();

        assert!(store.admin_existed(&mock_principals::alice()));

        assert!(!store.admin_existed(&mock_principals::john()));
    }

    #[test]
    fn get_all_admins_ok() {
        let store = init_test_data();

        assert_eq!(store.get_all_admins().len(), 2);
    }
}
