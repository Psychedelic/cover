use std::collections::BTreeMap;
use std::ops::Not;

use serde::Deserialize;

use crate::common::types::CallerId;
use crate::service::model::canister_build_config::CanisterBuildConfig;
use crate::service::store::error::ErrorKindStore;
use crate::service::time_utils;
use crate::CanisterId;
use ic_kit::candid::CandidType;

#[derive(CandidType, Default, Deserialize)]
pub struct CanisterBuildConfigStore {
    configs: BTreeMap<(CallerId, CanisterId), CanisterBuildConfig>,
}

impl CanisterBuildConfigStore {
    pub fn get_all_configs(&self, caller_id: &CallerId) -> Vec<&CanisterBuildConfig> {
        self.configs
            .iter()
            .filter(|((c, _), _)| c == caller_id)
            .map(|(_, v)| v)
            .collect()
    }

    pub fn get_config_by_id(
        &self,
        caller_id: &CallerId,
        canister_id: &CanisterId,
    ) -> Result<&CanisterBuildConfig, ErrorKindStore> {
        self.configs
            .get(&(*caller_id, *canister_id))
            .ok_or(ErrorKindStore::BuildConfigNotFound)
    }

    fn config_exists(&self, caller_id: &CallerId, canister_id: &CanisterId) -> bool {
        self.configs.contains_key(&(*caller_id, *canister_id))
    }

    pub fn add_config(&mut self, config: CanisterBuildConfig) -> Result<(), ErrorKindStore> {
        self.config_exists(&config.user_id, &config.canister_id)
            .not()
            .then(|| {
                let now = time_utils::now_to_str();
                self.configs.insert(
                    (config.user_id, config.canister_id),
                    CanisterBuildConfig {
                        user_id: config.user_id,
                        canister_id: config.canister_id,
                        canister_name: config.canister_name,
                        repo_url: config.repo_url,
                        user_repo_token: config.user_repo_token,
                        commit_hash: config.commit_hash,
                        rust_version: config.rust_version,
                        dfx_version: config.dfx_version,
                        optimize_times: config.optimize_times,
                        created_at: now.clone(),
                        updated_at: now,
                    },
                );
            })
            .ok_or(ErrorKindStore::BuildConfigExisted)
    }

    pub fn update_config(
        &mut self,
        caller_id: &CallerId,
        canister_id: &CanisterId,
        config: CanisterBuildConfig,
    ) -> Result<(), ErrorKindStore> {
        self.configs
            .get_mut(&(*caller_id, *canister_id))
            .map(|c| {
                c.canister_name = config.canister_name;
                c.repo_url = config.repo_url;
                c.user_repo_token = config.user_repo_token;
                c.commit_hash = config.commit_hash;
                c.rust_version = config.rust_version;
                c.dfx_version = config.dfx_version;
                c.optimize_times = config.optimize_times;
                c.updated_at = time_utils::now_to_str()
            })
            .ok_or(ErrorKindStore::BuildConfigNotFound)
    }

    pub fn delete_config(
        &mut self,
        caller_id: &CallerId,
        canister_id: &CanisterId,
    ) -> Result<(), ErrorKindStore> {
        self.configs
            .remove(&(*caller_id, *canister_id))
            .map(|_| ())
            .ok_or(ErrorKindStore::BuildConfigNotFound)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::service::store::test_data::*;

    fn init_test_data() -> CanisterBuildConfigStore {
        let mut store = CanisterBuildConfigStore::default();
        store.add_config(fake_config1()).unwrap();
        store.add_config(fake_config4()).unwrap();
        store
    }

    #[test]
    fn add_config_ok() {
        let mut store = CanisterBuildConfigStore::default();

        assert_eq!(store.add_config(fake_config1()), Ok(()));
        assert_eq!(store.add_config(fake_config2()), Ok(()));
        assert_eq!(
            store.add_config(fake_config1()),
            Err(ErrorKindStore::BuildConfigExisted)
        );
    }

    #[test]
    fn get_all_configs_ok() {
        let store = init_test_data();
        assert_eq!(
            store.get_all_configs(&fake_config1().user_id),
            vec![&fake_config4(), &fake_config1()]
        );
        assert_eq!(store.get_all_configs(&fake_config2().user_id).len(), 0);
    }

    #[test]
    fn get_config_by_id_ok() {
        let store = init_test_data();
        assert_eq!(
            store.get_config_by_id(&fake_config1().user_id, &fake_config1().canister_id),
            Ok(&fake_config1())
        );
        assert_eq!(
            store.get_config_by_id(&fake_config2().user_id, &fake_config2().canister_id),
            Err(ErrorKindStore::BuildConfigNotFound)
        );
        assert_eq!(
            store.get_config_by_id(&fake_config3().user_id, &fake_config3().canister_id),
            Err(ErrorKindStore::BuildConfigNotFound)
        );
        assert_eq!(
            store.get_config_by_id(&fake_config4().user_id, &fake_config2().canister_id),
            Err(ErrorKindStore::BuildConfigNotFound)
        );
    }

    #[test]
    fn update_config_ok() {
        let mut store = init_test_data();

        assert_eq!(
            store.update_config(
                &fake_config4().user_id,
                &fake_config4().canister_id,
                fake_config2()
            ),
            Ok(())
        );

        assert_eq!(
            store
                .get_config_by_id(&fake_config4().user_id, &fake_config4().canister_id,)
                .unwrap()
                .canister_name
                == fake_config2().canister_name,
            true
        );

        assert_eq!(
            store.update_config(
                &fake_config2().user_id,
                &fake_config2().canister_id,
                fake_config1(),
            ),
            Err(ErrorKindStore::BuildConfigNotFound)
        )
    }

    #[test]
    fn delete_config_ok() {
        let mut store = init_test_data();
        assert_eq!(
            store.delete_config(&fake_config1().user_id, &fake_config1().canister_id),
            Ok(())
        );
        assert_eq!(
            store.delete_config(&fake_config1().user_id, &fake_config1().canister_id),
            Err(ErrorKindStore::BuildConfigNotFound)
        );
        assert_eq!(
            store.delete_config(&fake_config2().user_id, &fake_config2().canister_id),
            Err(ErrorKindStore::BuildConfigNotFound)
        );
    }

    #[test]
    fn config_exists_ok() {
        let store = init_test_data();
        assert_eq!(
            store.config_exists(&fake_config1().user_id, &fake_config1().canister_id),
            true
        );

        assert_eq!(
            store.config_exists(&fake_config4().user_id, &fake_config4().canister_id),
            true
        );

        assert_eq!(
            store.config_exists(&fake_config3().user_id, &fake_config3().canister_id),
            false
        );
    }
}
