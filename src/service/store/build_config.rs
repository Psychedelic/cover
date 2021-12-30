use std::collections::BTreeMap;
use std::ops::Not;

use ic_kit::candid::CandidType;
use serde::Deserialize;

use crate::common::types::{CallerId, CanisterId};
use crate::service::model::build_config::{AddBuildConfig, BuildConfig, UpdateBuildConfig};
use crate::service::store::error::ErrorKindStore;
use crate::service::time_utils;

#[derive(CandidType, Default, Deserialize)]
pub struct BuildConfigStore {
    configs: BTreeMap<(CallerId, CanisterId), BuildConfig>,
}

impl BuildConfigStore {
    pub fn get_all_build_configs(&self, caller_id: &CallerId) -> Vec<&BuildConfig> {
        self.configs
            .iter()
            .filter(|((c, _), _)| c == caller_id)
            .map(|(_, v)| v)
            .collect()
    }

    pub fn get_build_config_by_id(
        &self,
        caller_id: &CallerId,
        canister_id: &CanisterId,
    ) -> Option<&BuildConfig> {
        self.configs.get(&(*caller_id, *canister_id))
    }

    fn build_config_exists(&self, caller_id: &CallerId, canister_id: &CanisterId) -> bool {
        self.configs.contains_key(&(*caller_id, *canister_id))
    }

    pub fn add_build_config(
        &mut self,
        caller_id: &CallerId,
        config: AddBuildConfig,
    ) -> Result<(), ErrorKindStore> {
        self.build_config_exists(caller_id, &config.canister_id)
            .not()
            .then(|| {
                let now = time_utils::now_to_str();
                self.configs.insert(
                    (*caller_id, config.canister_id),
                    BuildConfig {
                        user_id: *caller_id,
                        canister_id: config.canister_id,
                        canister_name: config.canister_name,
                        repo_url: config.repo_url,
                        user_access_token: config.user_access_token,
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

    pub fn update_build_config(
        &mut self,
        caller_id: &CallerId,
        canister_id: &CanisterId,
        config: UpdateBuildConfig,
    ) -> Result<(), ErrorKindStore> {
        self.configs
            .get_mut(&(*caller_id, *canister_id))
            .map(|c| {
                c.canister_name = config.canister_name;
                c.repo_url = config.repo_url;
                c.user_access_token = config.user_access_token;
                c.commit_hash = config.commit_hash;
                c.rust_version = config.rust_version;
                c.dfx_version = config.dfx_version;
                c.optimize_times = config.optimize_times;
                c.updated_at = time_utils::now_to_str()
            })
            .ok_or(ErrorKindStore::BuildConfigNotFound)
    }

    pub fn delete_build_config(
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
    use ic_kit::mock_principals;

    use crate::service::store::test_data::*;

    use super::*;

    fn init_test_data() -> BuildConfigStore {
        let mut store = BuildConfigStore::default();

        store
            .add_build_config(&mock_principals::alice(), fake_add_build_config3())
            .unwrap();

        store
            .add_build_config(&mock_principals::john(), fake_add_build_config2())
            .unwrap();

        store
    }

    #[test]
    fn add_config_ok() {
        let mut store = BuildConfigStore::default();

        assert_eq!(
            store.add_build_config(&mock_principals::bob(), fake_add_build_config1()),
            Ok(())
        );

        assert_eq!(
            store.add_build_config(&mock_principals::bob(), fake_add_build_config2()),
            Ok(())
        );

        assert_eq!(
            store.get_all_build_configs(&mock_principals::bob()),
            vec![
                &fake_build_config_use_add_model(&mock_principals::bob(), fake_add_build_config2()),
                &fake_build_config_use_add_model(&mock_principals::bob(), fake_add_build_config1())
            ]
        );

        assert_eq!(
            store.add_build_config(&mock_principals::bob(), fake_add_build_config1()),
            Err(ErrorKindStore::BuildConfigExisted)
        );
    }

    #[test]
    fn get_all_configs_ok() {
        let store = init_test_data();

        assert_eq!(
            store.get_all_build_configs(&mock_principals::alice()),
            vec![&fake_build_config_use_add_model(
                &mock_principals::alice(),
                fake_add_build_config3()
            )]
        );

        assert_eq!(
            store.get_all_build_configs(&mock_principals::bob()).len(),
            0
        );
    }

    #[test]
    fn get_config_by_id_ok() {
        let store = init_test_data();

        assert_eq!(
            store.get_build_config_by_id(&mock_principals::alice(), &fake_canister3()),
            Some(&fake_build_config_use_add_model(
                &mock_principals::alice(),
                fake_add_build_config3()
            ))
        );

        assert_eq!(
            store.get_build_config_by_id(&mock_principals::alice(), &fake_canister2()),
            None
        );

        assert_eq!(
            store.get_build_config_by_id(&mock_principals::bob(), &fake_canister3()),
            None
        );
    }

    #[test]
    fn update_config_ok() {
        let mut store = init_test_data();

        assert_eq!(
            store.update_build_config(
                &mock_principals::john(),
                &fake_canister2(),
                fake_update_build_config2(),
            ),
            Ok(())
        );

        assert_eq!(
            store.get_build_config_by_id(&mock_principals::john(), &fake_canister2()),
            Some(&fake_build_config_use_update_model(
                &mock_principals::john(),
                &fake_canister2(),
                fake_update_build_config2()
            ))
        );

        assert_eq!(
            store.update_build_config(
                &mock_principals::john(),
                &fake_canister1(),
                fake_update_build_config1(),
            ),
            Err(ErrorKindStore::BuildConfigNotFound)
        )
    }

    #[test]
    fn delete_config_ok() {
        let mut store = init_test_data();

        assert_eq!(
            store.delete_build_config(&mock_principals::alice(), &fake_canister3()),
            Ok(())
        );

        assert_eq!(
            store.delete_build_config(&mock_principals::alice(), &fake_canister3()),
            Err(ErrorKindStore::BuildConfigNotFound)
        );

        assert_eq!(
            store.delete_build_config(&mock_principals::john(), &fake_canister1()),
            Err(ErrorKindStore::BuildConfigNotFound)
        );
    }

    #[test]
    fn config_exists_ok() {
        let store = init_test_data();
        assert!(store.build_config_exists(&mock_principals::john(), &fake_canister2()));

        assert!(store.build_config_exists(&mock_principals::alice(), &fake_canister3()));

        assert!(!store.build_config_exists(&mock_principals::alice(), &fake_canister1()));
    }
}
