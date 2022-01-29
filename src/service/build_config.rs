use crate::common::types::{CanisterId, CanisterOwnerId};
use crate::service::model::build_config::{BuildConfig, SaveBuildConfig};
use crate::service::{build_config_store, build_config_store_mut};

pub fn get_build_configs(owner_id: &CanisterOwnerId) -> Vec<&'static BuildConfig> {
    build_config_store().get_build_configs(owner_id)
}

pub fn get_build_config_by_id(
    owner_id: &CanisterOwnerId,
    canister_id: &CanisterId,
) -> Option<&'static BuildConfig> {
    build_config_store().get_build_config_by_id(owner_id, canister_id)
}

pub fn delete_build_config(owner_id: &CanisterOwnerId, canister_id: &CanisterId) {
    build_config_store_mut().delete_build_config(owner_id, canister_id)
}

pub fn save_build_config(config: SaveBuildConfig) {
    build_config_store_mut().save_build_config(config)
}
