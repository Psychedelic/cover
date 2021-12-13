use ic_kit::ic::caller;
use ic_kit::macros::{query, update};

use crate::common::types::CanisterId;
use crate::service::build_config;
use crate::service::model::build_config::BuildConfig;
use crate::service::model::error::Error;

#[query]
fn get_all_build_configs() -> Vec<&'static BuildConfig> {
    build_config::get_all_build_configs(&caller())
}

#[query]
fn get_build_config_by_id(canister_id: CanisterId) -> Result<&'static BuildConfig, Error> {
    build_config::get_build_config_by_id(&caller(), &canister_id)
}

#[update]
fn update_build_config(canister_id: CanisterId, config: BuildConfig) -> Result<(), Error> {
    build_config::update_build_config(&caller(), &canister_id, config)
}

#[update]
fn delete_build_config(canister_id: CanisterId) -> Result<(), Error> {
    build_config::delete_build_config(&caller(), &canister_id)
}

#[update]
fn add_build_config(config: BuildConfig) -> Result<(), Error> {
    build_config::add_build_config(&caller(), config)
}
