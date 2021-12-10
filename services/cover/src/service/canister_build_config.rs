use super::{canister_build_config, canister_build_config_mut};
use crate::common::types::CallerId;
use crate::service::model::canister::CanisterBuildConfig;
use crate::{CanisterId, Error};

pub fn get_all_configs(caller_id: &CallerId) -> Vec<&'static CanisterBuildConfig> {
    canister_build_config().get_all_configs(caller_id)
}

pub fn get_config_by_id(
    caller_id: &CallerId,
    canister_id: &CanisterId,
) -> Result<&'static CanisterBuildConfig, Error> {
    canister_build_config()
        .get_config_by_id(caller_id, canister_id)
        .map_err(|e| e.into())
}
pub fn update_config(
    caller_id: &CallerId,
    canister_id: &CanisterId,
    config: CanisterBuildConfig,
) -> Result<(), Error> {
    canister_build_config_mut()
        .update_config(caller_id, canister_id, config)
        .map_err(|e| e.into())
}

pub fn delete_config(caller_id: &CallerId, canister_id: &CanisterId) -> Result<(), Error> {
    canister_build_config_mut()
        .delete_config(caller_id, canister_id)
        .map_err(|e| e.into())
}

pub fn add_config(config: CanisterBuildConfig) -> Result<(), Error> {
    canister_build_config_mut()
        .add_config(config)
        .map_err(|e| e.into())
}
