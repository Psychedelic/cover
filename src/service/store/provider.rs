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
        caller_id: &CallerId,
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
                        created_by: *caller_id,
                        created_at: now.clone(),
                        updated_by: *caller_id,
                        updated_at: now,
                    },
                );
            })
            .ok_or(ErrorKindStore::ExistedProvider)
    }

    pub fn update_provider(
        &mut self,
        caller_id: &CallerId,
        update_provider: UpdateProvider,
    ) -> Result<(), ErrorKindStore> {
        self.provider
            .get_mut(&update_provider.id)
            .map(|provider| {
                provider.name = update_provider.name;
                provider.memo = update_provider.memo;
                provider.updated_by = *caller_id;
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
    use ic_kit::*;

    use crate::service::store::test_data::*;

    use super::*;

    fn init_test_data() -> ProviderStore {
        let mut store = ProviderStore::default();

        store
            .add_provider(
                &mock_principals::bob(),
                fake_add_provider1(&mock_principals::alice()),
            )
            .unwrap();

        store
    }

    #[test]
    fn add_provider_ok() {
        let mut store = init_test_data();

        assert_eq!(store.get_all_providers().len(), 1);

        assert_eq!(
            store.add_provider(
                &mock_principals::alice(),
                fake_add_provider1(&mock_principals::john()),
            ),
            Ok(())
        );

        assert_eq!(
            store.add_provider(
                &mock_principals::bob(),
                fake_add_provider1(&mock_principals::john())
            ),
            Err(ErrorKindStore::ExistedProvider)
        );

        assert_eq!(store.get_all_providers().len(), 2);
    }

    #[test]
    fn update_provider_ok() {
        let mut store = init_test_data();

        assert_eq!(store.get_all_providers().len(), 1);

        assert_eq!(
            store.update_provider(
                &mock_principals::bob(),
                fake_update_provider1(&mock_principals::alice())
            ),
            Ok(())
        );

        assert_eq!(
            store.get_provider_by_id(&mock_principals::alice()),
            Some(&fake_provider_use_update_model(
                &mock_principals::bob(),
                fake_update_provider1(&mock_principals::alice())
            ))
        );

        assert_eq!(
            store.update_provider(
                &mock_principals::bob(),
                fake_update_provider2(&mock_principals::john())
            ),
            Err(ErrorKindStore::ProviderNotFound)
        );
    }

    #[test]
    fn get_provider_by_canister_id_ok() {
        let store = init_test_data();

        assert_eq!(
            store.get_provider_by_id(&mock_principals::alice()),
            Some(&fake_provider_use_add_model(
                &mock_principals::bob(),
                fake_add_provider1(&mock_principals::alice()),
            ))
        );

        assert_eq!(store.get_provider_by_id(&mock_principals::john()), None);

        assert_eq!(store.get_all_providers().len(), 1);
    }

    #[test]
    fn get_all_provider_ok() {
        let store = init_test_data();

        assert_eq!(
            store.get_all_providers(),
            vec![&fake_provider_use_add_model(
                &mock_principals::bob(),
                fake_add_provider1(&mock_principals::alice())
            )]
        );
    }
}
