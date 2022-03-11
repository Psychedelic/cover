use ic_cdk::api::call::ManualReply;
use ic_cdk::caller;
use ic_cdk::export::candid::candid_method;
use ic_cdk_macros::{query, update};

use crate::common::types::CanisterId;
use crate::service::guard::is_validator;
use crate::service::model::build_config::{BuildConfig, BuildConfigInfo, SaveBuildConfig};
use crate::service::store::build_config;

#[query(name = "getBuildConfigs", manual_reply = true)]
#[candid_method(query, rename = "getBuildConfigs")]
fn get_build_configs() -> ManualReply<Vec<BuildConfig>> {
    build_config::get_build_configs(&caller(), |result| ManualReply::one(result))
}

#[query(name = "getBuildConfigById", manual_reply = true)]
#[candid_method(query, rename = "getBuildConfigById")]
fn get_build_config_by_id(canister_id: CanisterId) -> ManualReply<Option<BuildConfig>> {
    build_config::get_build_config_by_id(&caller(), &canister_id, |result| ManualReply::one(result))
}

#[update(name = "deleteBuildConfig")]
#[candid_method(update, rename = "deleteBuildConfig")]
fn delete_build_config(canister_id: CanisterId) {
    build_config::delete_build_config(&caller(), &canister_id)
}

#[update(name = "saveBuildConfig", guard = "is_validator")]
#[candid_method(update, rename = "saveBuildConfig")]
fn save_build_config(config: SaveBuildConfig) {
    build_config::save_build_config(config)
}

#[query(
    name = "getBuildConfigValidator",
    guard = "is_validator",
    manual_reply = true
)]
#[candid_method(query, rename = "getBuildConfigValidator")]
fn get_build_config_validator(info: BuildConfigInfo) -> ManualReply<Option<BuildConfig>> {
    build_config::get_build_config_by_id(&info.owner_id, &info.canister_id, |result| {
        ManualReply::one(result)
    })
}

#[cfg(test)]
mod tests {
    // use ic_kit::*;
    //
    // use crate::service::store::test_data::*;
    //
    // use super::*;
    //
    // fn init_test_data() {
    //     MockContext::new()
    //         .with_caller(mock_principals::bob())
    //         .inject();
    //
    //     save_build_config(fake_save_build_config1(
    //         &mock_principals::bob(),
    //         &fake_canister1(),
    //     ));
    //
    //     save_build_config(fake_save_build_config2(
    //         &mock_principals::bob(),
    //         &fake_canister2(),
    //     ));
    // }
    //
    // #[test]
    // fn save_build_config_ok() {
    //     MockContext::new()
    //         .with_caller(mock_principals::john())
    //         .inject();
    //
    //     assert_eq!(get_build_configs().len(), 0);
    //
    //     save_build_config(fake_save_build_config1(
    //         &mock_principals::john(),
    //         &fake_canister1(),
    //     ));
    //
    //     assert_eq!(
    //         get_build_configs(),
    //         vec![&fake_build_config_from(fake_save_build_config1(
    //             &mock_principals::john(),
    //             &fake_canister1()
    //         ))]
    //     );
    //
    //     save_build_config(fake_save_build_config3(
    //         &mock_principals::john(),
    //         &fake_canister1(),
    //     ));
    //
    //     assert_eq!(
    //         get_build_configs(),
    //         vec![&fake_build_config_from(fake_save_build_config3(
    //             &mock_principals::john(),
    //             &fake_canister1()
    //         ))]
    //     );
    // }
    //
    // #[test]
    // fn delete_build_config_ok() {
    //     init_test_data();
    //
    //     get_build_configs_ok();
    //
    //     delete_build_config(fake_canister1());
    //
    //     assert_eq!(
    //         get_build_configs(),
    //         vec![&fake_build_config_from(fake_save_build_config2(
    //             &mock_principals::bob(),
    //             &fake_canister2()
    //         ))]
    //     );
    //
    //     delete_build_config(fake_canister1());
    //
    //     assert_eq!(
    //         get_build_configs(),
    //         vec![&fake_build_config_from(fake_save_build_config2(
    //             &mock_principals::bob(),
    //             &fake_canister2()
    //         ))]
    //     );
    //
    //     delete_build_config(fake_canister2());
    //
    //     assert_eq!(get_build_configs().len(), 0);
    // }
    //
    // #[test]
    // fn get_build_configs_ok() {
    //     init_test_data();
    //
    //     assert_eq!(
    //         get_build_configs(),
    //         vec![
    //             &fake_build_config_from(fake_save_build_config2(
    //                 &mock_principals::bob(),
    //                 &fake_canister2()
    //             )),
    //             &fake_build_config_from(fake_save_build_config1(
    //                 &mock_principals::bob(),
    //                 &fake_canister1()
    //             ))
    //         ]
    //     );
    // }
    //
    // #[test]
    // fn get_build_config_by_id_ok() {
    //     init_test_data();
    //
    //     assert_eq!(
    //         get_build_config_by_id(fake_canister1()),
    //         Some(&fake_build_config_from(fake_save_build_config1(
    //             &mock_principals::bob(),
    //             &fake_canister1()
    //         )))
    //     );
    //
    //     assert_eq!(get_build_config_by_id(fake_canister3()), None);
    // }
    //
    // #[test]
    // fn get_build_config_validator_ok() {
    //     init_test_data();
    //
    //     assert_eq!(
    //         get_build_config_validator(BuildConfigInfo {
    //             owner_id: mock_principals::bob(),
    //             canister_id: fake_canister1()
    //         }),
    //         Some(&fake_build_config_from(fake_save_build_config1(
    //             &mock_principals::bob(),
    //             &fake_canister1()
    //         )))
    //     );
    // }
}
