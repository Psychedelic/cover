use std::collections::BTreeMap;
use std::ops::Not;

use ic_kit::candid::CandidType;
use serde::Deserialize;

use crate::common::types::{CallerId, ProviderId};
use crate::service::model::provider::{AddProvider, Provider, UpdateProvider};
use crate::service::store::error::ErrorKindStore;
use crate::service::time_utils;

#[derive(CandidType, Deserialize, Default)]
pub struct ProviderStore {
    provider: BTreeMap<ProviderId, Provider>,
}

impl ProviderStore {
    pub fn provider_exists(&self, provider_id: &ProviderId) -> bool {
        self.provider.contains_key(provider_id)
    }

    pub fn add_provider(
        &mut self,
        caller_id: CallerId,
        add_provider: AddProvider,
    ) -> Result<(), ErrorKindStore> {
        self.provider_exists(&add_provider.id)
            .not()
            .then(|| {
                let now = time_utils::now_to_str();
                self.provider.insert(
                    add_provider.id,
                    Provider {
                        id: add_provider.id,
                        name: add_provider.name,
                        memo: add_provider.memo,
                        created_by: caller_id,
                        created_at: now.clone(),
                        updated_by: caller_id,
                        updated_at: now,
                    },
                );
            })
            .ok_or(ErrorKindStore::ExistedProvider)
    }

    pub fn update_provider(
        &mut self,
        caller_id: CallerId,
        update_provider: UpdateProvider,
    ) -> Result<(), ErrorKindStore> {
        self.provider
            .get_mut(&update_provider.id)
            .map(|provider| {
                provider.name = update_provider.name;
                provider.memo = update_provider.memo;
                provider.updated_by = caller_id;
                provider.updated_at = time_utils::now_to_str();
            })
            .ok_or(ErrorKindStore::ProviderNotFound)
    }

    pub fn delete_provider(&mut self, provider_id: &ProviderId) -> Result<(), ErrorKindStore> {
        self.provider
            .remove(provider_id)
            .map(|_| ())
            .ok_or(ErrorKindStore::ProviderNotFound)
    }

    pub fn get_provider_by_id(&self, provider_id: &ProviderId) -> Option<&Provider> {
        self.provider.get(provider_id)
    }

    pub fn get_all_providers(&self) -> Vec<&Provider> {
        self.provider.iter().map(|(_, v)| v).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::service::store::test_data;

    use super::*;

    fn update_provider_gen(seed: u8) -> UpdateProvider {
        if seed % 3 == 0 {
            test_data::fake_update_provider1(test_data::fake_canister1())
        } else if seed % 3 == 1 {
            test_data::fake_update_provider2(test_data::fake_canister2())
        } else {
            test_data::fake_update_provider3(test_data::fake_canister3())
        }
    }

    fn add_provider_gen(seed: u8) -> AddProvider {
        if seed % 3 == 0 {
            test_data::fake_add_provider1(test_data::fake_canister1())
        } else if seed % 3 == 1 {
            test_data::fake_add_provider2(test_data::fake_canister2())
        } else {
            test_data::fake_add_provider3(test_data::fake_canister3())
        }
    }

    fn init_test_data(len: u8) -> ProviderStore {
        let mut store = ProviderStore::default();
        for i in 0..len {
            let result = store.add_provider(test_data::caller_gen(i), add_provider_gen(i));
            assert_eq!(result, Ok(()));
        }
        store
    }

    #[test]
    fn add_provider_ok() {
        let mut store = init_test_data(3);
        assert_eq!(store.provider.len(), 3);
        for i in 0..store.provider.len() {
            let result =
                store.add_provider(test_data::caller_gen(i as u8), add_provider_gen(i as u8));
            assert_eq!(result, Err(ErrorKindStore::ExistedProvider));
        }
    }

    #[test]
    fn update_provider_ok() {
        let mut store = ProviderStore::default();
        for i in 0..3 {
            let update_provider = update_provider_gen(i as u8);
            let caller_id = test_data::caller_gen(i as u8);
            let result = store.update_provider(caller_id, update_provider);
            assert_eq!(result, Err(ErrorKindStore::ProviderNotFound));
        }
        let mut store = init_test_data(3);
        for i in 0..store.provider.len() {
            let update_provider = update_provider_gen(i as u8);
            let caller_id = test_data::caller_gen(i as u8);
            let result = store.update_provider(caller_id, update_provider);
            assert_eq!(result, Ok(()));
        }
        for i in 0..store.provider.len() {
            let update_provider = update_provider_gen(i as u8);
            let provider = store.provider.get(&update_provider.id).unwrap();
            let caller_id = test_data::caller_gen(i as u8);
            let now = time_utils::now_to_str();
            assert_eq!(provider.id, update_provider.id);
            assert_eq!(provider.name, update_provider.name);
            assert_eq!(provider.memo, update_provider.memo);
            assert_eq!(provider.created_at, now);
            assert_eq!(provider.created_by, caller_id);
            assert_eq!(provider.updated_at, now);
            assert_eq!(provider.updated_by, caller_id);
        }
    }

    #[test]
    fn delete_provider_ok() {
        let mut store = ProviderStore::default();
        for i in 0..3 {
            let provider = add_provider_gen(i as u8);
            let result = store.delete_provider(&provider.id);
            assert_eq!(result, Err(ErrorKindStore::ProviderNotFound));
        }
        let mut store = init_test_data(3);
        for i in 0..store.provider.len() {
            let provider = add_provider_gen(i as u8);
            let result = store.delete_provider(&provider.id);
            assert_eq!(result, Ok(()));
        }
        assert_eq!(store.provider.len(), 0);
    }

    #[test]
    fn get_provider_by_canister_id_ok() {
        let store = init_test_data(3);
        for i in 0..store.provider.len() {
            let provider = add_provider_gen(i as u8);
            let provider = store.get_provider_by_id(&provider.id).unwrap();
            let caller_id = test_data::caller_gen(i as u8);
            let now = time_utils::now_to_str();
            assert_eq!(provider.id, provider.id);
            assert_eq!(provider.name, provider.name);
            assert_eq!(provider.memo, provider.memo);
            assert_eq!(provider.created_at, now);
            assert_eq!(provider.created_by, caller_id);
            assert_eq!(provider.updated_at, now);
            assert_eq!(provider.updated_by, caller_id);
        }
    }

    #[test]
    fn get_all_provider_ok() {
        let store = init_test_data(3);
        let providers = store.get_all_providers();
        for i in 0..providers.len() {
            let provider = add_provider_gen(i as u8);
            let caller_id = test_data::caller_gen(i as u8);
            let now = time_utils::now_to_str();
            assert_eq!(
                providers.contains(&&Provider {
                    id: provider.id,
                    name: provider.name,
                    memo: provider.memo,
                    created_at: now.clone(),
                    created_by: caller_id,
                    updated_at: now,
                    updated_by: caller_id,
                }),
                true
            );
        }
    }
}
