use crate::common::types::{CanisterId, CanisterOwnerId};
use crate::service::model::build_config::{BuildConfig, SaveBuildConfig};
use crate::service::model::error::Error;
use crate::service::{build_config_store, build_config_store_mut};

pub fn get_all_build_configs(owner_id: &CanisterOwnerId) -> Vec<&'static BuildConfig> {
    build_config_store().get_all_build_configs(owner_id)
}

pub fn get_build_config_by_id(
    owner_id: &CanisterOwnerId,
    canister_id: &CanisterId,
) -> Option<&'static BuildConfig> {
    build_config_store().get_build_config_by_id(owner_id, canister_id)
}

pub fn delete_build_config(
    owner_id: &CanisterOwnerId,
    canister_id: &CanisterId,
) -> Result<(), Error> {
    build_config_store_mut()
        .delete_build_config(owner_id, canister_id)
        .map_err(|e| e.into())
}

pub fn save_build_config(config: SaveBuildConfig) -> Result<(), Error> {
    match build_config_store().build_config_exists(&config.owner_id, &config.canister_id) {
        true => build_config_store_mut()
            .update_build_config(config)
            .map_err(|e| e.into()),
        false => build_config_store_mut()
            .add_build_config(config)
            .map_err(|e| e.into()),
    }
}
