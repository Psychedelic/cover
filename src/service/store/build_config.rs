use std::collections::BTreeMap;
use std::ops::Not;

use ic_kit::candid::CandidType;
use serde::Deserialize;

use crate::common::types::{CallerId, CanisterId};
use crate::service::model::build_config::{BuildConfig, BuildConfigRequest};
use crate::service::store::error::ErrorKindStore;
use crate::service::time_utils;

#[derive(CandidType, Default, Deserialize)]
pub struct BuildConfigStore {
    configs: BTreeMap<(CallerId, CanisterId), BuildConfig>,
}

impl BuildConfigStore {
    pub fn get_all_build_configs(&self, owner_id: &CallerId) -> Vec<&BuildConfig> {
        self.configs
            .iter()
            .filter(|((c, _), _)| c == owner_id)
            .map(|(_, v)| v)
            .collect()
    }

    pub fn get_build_config_by_id(
        &self,
        owner_id: &CallerId,
        canister_id: &CanisterId,
    ) -> Option<&BuildConfig> {
        self.configs.get(&(*owner_id, *canister_id))
    }

    fn build_config_exists(&self, owner_id: &CallerId, canister_id: &CanisterId) -> bool {
        self.configs.contains_key(&(*owner_id, *canister_id))
    }

    pub fn add_build_config(&mut self, config: BuildConfigRequest) -> Result<(), ErrorKindStore> {
        self.build_config_exists(&config.user_id, &config.canister_id)
            .not()
            .then(|| {
                let now = time_utils::now_to_str();
                self.configs.insert(
                    (config.user_id, config.canister_id),
                    BuildConfig {
                        user_id: config.user_id,
                        canister_id: config.canister_id,
                        canister_name: config.canister_name,
                        repo_url: config.repo_url,
                        commit_hash: config.commit_hash,
                        rust_version: config.rust_version,
                        dfx_version: config.dfx_version,
                        optimize_times: config.optimize_times,
                        created_at: now.clone(),
                        updated_at: now,
                    },
                );
            })
            .ok_or(ErrorKindStore::ExistedBuildConfig)
    }

    pub fn update_build_config(
        &mut self,
        config: BuildConfigRequest,
    ) -> Result<(), ErrorKindStore> {
        self.configs
            .get_mut(&(config.user_id, config.canister_id))
            .map(|c| {
                c.canister_name = config.canister_name;
                c.repo_url = config.repo_url;
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
        owner_id: &CallerId,
        canister_id: &CanisterId,
    ) -> Result<(), ErrorKindStore> {
        self.configs
            .remove(&(*owner_id, *canister_id))
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
            .add_build_config(fake_build_config_request1(
                &mock_principals::bob(),
                &fake_canister1(),
            ))
            .unwrap();

        store
            .add_build_config(fake_build_config_request2(
                &mock_principals::bob(),
                &fake_canister2(),
            ))
            .unwrap();

        store
    }

    #[test]
    fn add_config_ok() {
        let mut store = BuildConfigStore::default();

        assert_eq!(
            store.add_build_config(fake_build_config_request1(
                &mock_principals::alice(),
                &fake_canister1()
            )),
            Ok(())
        );

        assert_eq!(
            store.add_build_config(fake_build_config_request1(
                &mock_principals::john(),
                &fake_canister1()
            )),
            Ok(())
        );

        assert_eq!(
            store.get_all_build_configs(&mock_principals::alice()),
            vec![&fake_build_config(fake_build_config_request1(
                &mock_principals::alice(),
                &fake_canister1()
            ))]
        );

        assert_eq!(
            store.add_build_config(fake_build_config_request1(
                &mock_principals::john(),
                &fake_canister1()
            )),
            Err(ErrorKindStore::ExistedBuildConfig)
        );
    }

    #[test]
    fn get_all_configs_ok() {
        let store = init_test_data();

        assert_eq!(
            store.get_all_build_configs(&mock_principals::bob()),
            vec![
                &fake_build_config(fake_build_config_request2(
                    &mock_principals::bob(),
                    &fake_canister2()
                )),
                &fake_build_config(fake_build_config_request1(
                    &mock_principals::bob(),
                    &fake_canister1()
                ))
            ]
        );

        assert_eq!(
            store.get_all_build_configs(&mock_principals::john()).len(),
            0
        );
    }

    #[test]
    fn get_config_by_id_ok() {
        let store = init_test_data();

        assert_eq!(
            store.get_build_config_by_id(&mock_principals::bob(), &fake_canister2()),
            Some(&fake_build_config(fake_build_config_request2(
                &mock_principals::bob(),
                &fake_canister2()
            )))
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
            store.update_build_config(fake_build_config_request3(
                &mock_principals::bob(),
                &fake_canister2()
            )),
            Ok(())
        );

        assert_eq!(
            store.get_build_config_by_id(&mock_principals::bob(), &fake_canister2()),
            Some(&fake_build_config(fake_build_config_request3(
                &mock_principals::bob(),
                &fake_canister2()
            )))
        );

        assert_eq!(
            store.update_build_config(fake_build_config_request3(
                &mock_principals::alice(),
                &fake_canister1()
            ),),
            Err(ErrorKindStore::BuildConfigNotFound)
        )
    }

    #[test]
    fn delete_config_ok() {
        let mut store = init_test_data();

        assert_eq!(
            store.delete_build_config(&mock_principals::bob(), &fake_canister2()),
            Ok(())
        );

        assert_eq!(
            store.get_all_build_configs(&mock_principals::bob()),
            vec![&fake_build_config(fake_build_config_request1(
                &mock_principals::bob(),
                &fake_canister1()
            ))]
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
        assert!(store.build_config_exists(&mock_principals::bob(), &fake_canister2()));

        assert!(!store.build_config_exists(&mock_principals::alice(), &fake_canister1()));
    }
}
