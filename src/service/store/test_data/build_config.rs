use crate::common::types::{CallerId, CanisterId};
use crate::service::model::build_config::{AddBuildConfig, BuildConfig, UpdateBuildConfig};
use crate::service::time_utils;

pub fn fake_build_config_use_update_model(
    caller_id: &CallerId,
    canister_id: &CanisterId,
    updated_config: UpdateBuildConfig,
) -> BuildConfig {
    BuildConfig {
        user_id: *caller_id,
        canister_id: *canister_id,
        canister_name: updated_config.canister_name,
        repo_url: updated_config.repo_url,
        user_repo_token: updated_config.user_repo_token,
        commit_hash: updated_config.commit_hash,
        rust_version: updated_config.rust_version,
        dfx_version: updated_config.dfx_version,
        optimize_times: updated_config.optimize_times,
        created_at: time_utils::now_to_str(),
        updated_at: time_utils::now_to_str(),
    }
}

pub fn fake_build_config_use_add_model(
    caller_id: &CallerId,
    new_config: AddBuildConfig,
) -> BuildConfig {
    BuildConfig {
        user_id: *caller_id,
        canister_id: new_config.canister_id,
        canister_name: new_config.canister_name,
        repo_url: new_config.repo_url,
        user_repo_token: new_config.user_repo_token,
        commit_hash: new_config.commit_hash,
        rust_version: new_config.rust_version,
        dfx_version: new_config.dfx_version,
        optimize_times: new_config.optimize_times,
        created_at: time_utils::now_to_str(),
        updated_at: time_utils::now_to_str(),
    }
}
