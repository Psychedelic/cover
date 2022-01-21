use ic_kit::candid::candid_method;
use ic_kit::ic::caller;
use ic_kit::macros::{query, update};

use crate::common::types::{CallerId, CanisterId};
use crate::service::build_config;
use crate::service::model::build_config::{BuildConfig, BuildConfigRequest};
use crate::service::model::error::Error;

#[query(name = "getAllBuildConfigs")]
#[candid_method(query, rename = "getAllBuildConfigs")]
fn get_all_build_configs() -> Vec<&'static BuildConfig> {
    build_config::get_all_build_configs(&caller())
}

#[query(name = "getBuildConfigById")]
#[candid_method(query, rename = "getBuildConfigById")]
fn get_build_config_by_id(canister_id: CanisterId) -> Option<&'static BuildConfig> {
    build_config::get_build_config_by_id(&caller(), &canister_id)
}

#[update(name = "deleteBuildConfig")]
#[candid_method(update, rename = "deleteBuildConfig")]
fn delete_build_config(canister_id: CanisterId) -> Result<(), Error> {
    build_config::delete_build_config(&caller(), &canister_id)
}

#[update(name = "updateBuildConfig")]
#[candid_method(update, rename = "updateBuildConfig")]
fn update_build_config(config: BuildConfigRequest) -> Result<(), Error> {
    build_config::update_build_config(config)
}

#[update(name = "addBuildConfig")]
#[candid_method(update, rename = "addBuildConfig")]
fn add_build_config(config: BuildConfigRequest) -> Result<(), Error> {
    build_config::add_build_config(config)
}

#[query(name = "getBuildConfigProvider")]
#[candid_method(query, rename = "getBuildConfigProvider")]
fn get_build_config_provider(
    canister_owner: CallerId,
    canister_id: CanisterId,
) -> Option<&'static BuildConfig> {
    build_config::get_build_config_by_id(&canister_owner, &canister_id)
}

#[cfg(test)]
mod tests {
    use crate::service::store::error::ErrorKindStore;
    use ic_kit::*;

    use crate::service::store::test_data::*;

    use super::*;

    fn init_test_data() {
        MockContext::new()
            .with_caller(mock_principals::bob())
            .inject();

        assert_eq!(
            add_build_config(fake_build_config_request1(
                &mock_principals::bob(),
                &fake_canister1()
            )),
            Ok(())
        );

        assert_eq!(
            add_build_config(fake_build_config_request2(
                &mock_principals::bob(),
                &fake_canister2()
            )),
            Ok(())
        );
    }

    #[test]
    fn add_build_config_ok() {
        MockContext::new()
            .with_caller(mock_principals::john())
            .inject();

        assert_eq!(get_all_build_configs().len(), 0);

        assert_eq!(
            add_build_config(fake_build_config_request3(
                &mock_principals::john(),
                &fake_canister1()
            )),
            Ok(())
        );

        assert_eq!(
            get_all_build_configs(),
            vec![&fake_build_config(fake_build_config_request3(
                &mock_principals::john(),
                &fake_canister1()
            ))]
        );

        assert_eq!(
            add_build_config(fake_build_config_request3(
                &mock_principals::john(),
                &fake_canister1()
            )),
            Err(Error::from(ErrorKindStore::ExistedBuildConfig))
        );

        assert_eq!(
            get_all_build_configs(),
            vec![&fake_build_config(fake_build_config_request3(
                &mock_principals::john(),
                &fake_canister1()
            ))]
        );

        assert_eq!(
            get_all_build_configs(),
            vec![&fake_build_config(fake_build_config_request3(
                &mock_principals::john(),
                &fake_canister1()
            ))]
        );
    }

    #[test]
    fn delete_build_config_ok() {
        init_test_data();

        assert_eq!(get_all_build_configs().len(), 2);

        assert_eq!(delete_build_config(fake_canister1()), Ok(()));

        assert_eq!(
            get_all_build_configs(),
            vec![&fake_build_config(fake_build_config_request2(
                &mock_principals::bob(),
                &fake_canister2()
            ))]
        );

        assert_eq!(
            delete_build_config(fake_canister1()),
            Err(Error::from(ErrorKindStore::BuildConfigNotFound))
        );
        assert_eq!(get_all_build_configs().len(), 1);

        assert_eq!(delete_build_config(fake_canister2()), Ok(()));

        assert_eq!(get_all_build_configs().len(), 0);
    }

    #[test]
    fn get_all_build_configs_ok() {
        init_test_data();

        assert_eq!(
            get_all_build_configs(),
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
    }

    #[test]
    fn get_build_config_by_id_ok() {
        init_test_data();

        assert_eq!(
            get_build_config_by_id(fake_canister1()),
            Some(&fake_build_config(fake_build_config_request1(
                &mock_principals::bob(),
                &fake_canister1()
            )))
        );

        assert_eq!(get_build_config_by_id(fake_canister3()), None);
    }

    #[test]
    fn update_build_config_ok() {
        init_test_data();

        assert_eq!(get_all_build_configs().len(), 2);

        assert_eq!(
            update_build_config(fake_build_config_request3(
                &mock_principals::bob(),
                &fake_canister2()
            )),
            Ok(())
        );

        assert_eq!(
            get_build_config_by_id(fake_canister2()),
            Some(&fake_build_config(fake_build_config_request3(
                &mock_principals::bob(),
                &fake_canister2()
            )))
        );

        assert_eq!(
            get_all_build_configs(),
            vec![
                &fake_build_config(fake_build_config_request3(
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
            update_build_config(fake_build_config_request1(
                &mock_principals::john(),
                &fake_canister1()
            )),
            Err(Error::from(ErrorKindStore::BuildConfigNotFound))
        );
    }

    #[test]
    fn get_build_config_provider_ok() {
        init_test_data();

        assert_eq!(
            get_build_config_provider(mock_principals::bob(), fake_canister1()),
            Some(&fake_build_config(fake_build_config_request1(
                &mock_principals::bob(),
                &fake_canister1()
            )))
        );
    }
}
