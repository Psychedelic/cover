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
