use crate::common::types::{CallerId, CanisterId};
use crate::service::model::build_config::BuildConfig;
use crate::service::model::error::Error;
use crate::service::{build_config_store, build_config_store_mut};

pub fn get_all_build_configs(caller_id: &CallerId) -> Vec<&'static BuildConfig> {
    build_config_store().get_all_build_configs(caller_id)
}

pub fn get_build_config_by_id(
    caller_id: &CallerId,
    canister_id: &CanisterId,
) -> Result<&'static BuildConfig, Error> {
    build_config_store()
        .get_build_config_by_id(caller_id, canister_id)
        .map_err(|e| e.into())
}

pub fn update_build_config(
    caller_id: &CallerId,
    canister_id: &CanisterId,
    config: BuildConfig,
) -> Result<(), Error> {
    build_config_store_mut()
        .update_build_config(caller_id, canister_id, config)
        .map_err(|e| e.into())
}

pub fn delete_build_config(caller_id: &CallerId, canister_id: &CanisterId) -> Result<(), Error> {
    build_config_store_mut()
        .delete_build_config(caller_id, canister_id)
        .map_err(|e| e.into())
}

pub fn add_build_config(caller_id: &CallerId, config: BuildConfig) -> Result<(), Error> {
    build_config_store_mut()
        .add_build_config(caller_id, config)
        .map_err(|e| e.into())
}
