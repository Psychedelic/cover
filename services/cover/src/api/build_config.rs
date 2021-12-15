use ic_kit::candid::candid_method;
use ic_kit::ic::caller;
use ic_kit::macros::{query, update};

use crate::common::types::CanisterId;
use crate::service::build_config;
use crate::service::model::build_config::{AddBuildConfig, BuildConfig, UpdateBuildConfig};
use crate::service::model::error::Error;

#[query(name = "getAllBuildConfigs")]
#[candid_method(query, rename = "getAllBuildConfigs")]
fn get_all_build_configs() -> Vec<&'static BuildConfig> {
    build_config::get_all_build_configs(&caller())
}

#[query(name = "getBuildConfigById")]
#[candid_method(query, rename = "getBuildConfigById")]
fn get_build_config_by_id(canister_id: CanisterId) -> Result<&'static BuildConfig, Error> {
    build_config::get_build_config_by_id(&caller(), &canister_id)
}

#[update(name = "updateBuildConfig")]
#[candid_method(update, rename = "updateBuildConfig")]
fn update_build_config(canister_id: CanisterId, config: UpdateBuildConfig) -> Result<(), Error> {
    build_config::update_build_config(&caller(), &canister_id, config)
}

#[update(name = "deleteBuildConfig")]
#[candid_method(update, rename = "deleteBuildConfig")]
fn delete_build_config(canister_id: CanisterId) -> Result<(), Error> {
    build_config::delete_build_config(&caller(), &canister_id)
}

#[update(name = "addBuildConfig")]
#[candid_method(update, rename = "addBuildConfig")]
fn add_build_config(config: AddBuildConfig) -> Result<(), Error> {
    build_config::add_build_config(&caller(), config)
}

// #[cfg(test)]
// mod tests {
//     use ic_kit::*;
//
//     use crate::service::store::test_data::*;
//
//     use super::*;
//
//     #[test]
//     fn add_build_config_ok() {
//         MockContext::new()
//             .with_caller(mock_principals::john())
//             .inject();
//
//         assert_eq!(add_build_config(fake_add_build_config1()), Ok(()));
//
//         println!("{:?}", get_all_build_configs());
//         assert_eq!(
//             add_build_config(fake_add_build_config1()),
//             Err(Error {
//                 code: "ERR_005_003_002",
//                 message: "Existed Build Config",
//                 debug_log: None,
//             })
//         );
//     }
//
//     #[test]
//     fn delete_build_config_ok() {}
// }
