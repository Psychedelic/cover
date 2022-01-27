use std::collections::HashSet;

use ic_kit::candid::CandidType;
use serde::Deserialize;

use crate::common::types::BuilderId;

#[derive(CandidType, Deserialize, Default)]
pub struct BuilderStore {
    builder: HashSet<BuilderId>,
}

impl BuilderStore {
    pub fn builder_existed(&self, builder_id: &BuilderId) -> bool {
        self.builder.contains(builder_id)
    }

    pub fn add_builder(&mut self, builder_id: &BuilderId) {
        self.builder.insert(*builder_id);
    }

    pub fn delete_builder(&mut self, builder_id: &BuilderId) {
        self.builder.remove(builder_id);
    }

    pub fn get_all_builders(&self) -> Vec<&BuilderId> {
        self.builder.iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ic_kit::*;

    fn init_test_data() -> BuilderStore {
        let mut store = BuilderStore::default();

        store.add_builder(&mock_principals::bob());

        store
    }

    #[test]
    fn get_all_builder_ok() {
        let store = init_test_data();

        assert_eq!(store.get_all_builders(), vec![&mock_principals::bob()]);
    }

    #[test]
    fn add_builder_ok() {
        let mut store = init_test_data();

        assert_eq!(store.get_all_builders().len(), 1);

        store.add_builder(&mock_principals::john());

        assert_eq!(store.get_all_builders().len(), 2);

        assert!(store.builder_existed(&mock_principals::john()));

        store.add_builder(&mock_principals::alice());

        assert_eq!(store.get_all_builders().len(), 3);

        assert!(store.builder_existed(&mock_principals::alice()));

        store.add_builder(&mock_principals::alice());

        assert_eq!(store.get_all_builders().len(), 3);
    }

    #[test]
    fn delete_builder_ok() {
        let mut store = init_test_data();

        store.delete_builder(&mock_principals::alice());
        assert_eq!(store.get_all_builders().len(), 1);

        store.delete_builder(&mock_principals::bob());
        assert_eq!(store.get_all_builders().len(), 0);
    }
}
