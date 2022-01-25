use std::collections::BTreeMap;

use ic_kit::candid::CandidType;
use serde::Deserialize;

use crate::common::types::{CallerId, ProviderId};
use crate::service::model::provider::{AddProvider, Provider};
use crate::service::time_utils;

#[derive(CandidType, Deserialize, Default)]
pub struct ProviderStore {
    provider: BTreeMap<ProviderId, Provider>,
}

impl ProviderStore {
    pub fn provider_exists(&self, provider_id: &ProviderId) -> bool {
        self.provider.contains_key(provider_id)
    }

    pub fn add_provider(&mut self, caller_id: &CallerId, add_provider: AddProvider) {
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
    }

    pub fn delete_provider(&mut self, provider_id: &ProviderId) {
        self.provider.remove(provider_id);
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

        store.add_provider(
            &mock_principals::bob(),
            fake_add_provider1(&mock_principals::alice()),
        );

        store
    }

    #[test]
    fn get_all_providers_ok() {
        let store = init_test_data();

        assert_eq!(
            store.get_all_providers(),
            vec![&fake_provider(
                &mock_principals::bob(),
                fake_add_provider1(&mock_principals::alice())
            )]
        );
    }

    #[test]
    fn add_provider_ok() {
        let mut store = init_test_data();

        get_all_providers_ok();

        store.add_provider(
            &mock_principals::alice(),
            fake_add_provider1(&mock_principals::john()),
        );

        assert_eq!(
            store.get_all_providers(),
            vec![
                &fake_provider(
                    &mock_principals::alice(),
                    fake_add_provider1(&mock_principals::john())
                ),
                &fake_provider(
                    &mock_principals::bob(),
                    fake_add_provider1(&mock_principals::alice())
                )
            ]
        );

        store.add_provider(
            &mock_principals::bob(),
            fake_add_provider1(&mock_principals::john()),
        );

        assert_eq!(
            store.get_all_providers(),
            vec![
                &fake_provider(
                    &mock_principals::bob(),
                    fake_add_provider1(&mock_principals::john())
                ),
                &fake_provider(
                    &mock_principals::bob(),
                    fake_add_provider1(&mock_principals::alice())
                )
            ]
        );
    }

    #[test]
    fn get_provider_by_canister_id_ok() {
        let store = init_test_data();

        assert_eq!(
            store.get_provider_by_id(&mock_principals::alice()),
            Some(&fake_provider(
                &mock_principals::bob(),
                fake_add_provider1(&mock_principals::alice()),
            ))
        );

        assert_eq!(store.get_provider_by_id(&mock_principals::john()), None);
    }

    #[test]
    fn delete_provider_ok() {
        let mut store = init_test_data();

        store.delete_provider(&mock_principals::alice());
        assert_eq!(store.get_all_providers().len(), 0);

        store.delete_provider(&mock_principals::alice());
        assert_eq!(store.get_all_providers().len(), 0);
    }
}
