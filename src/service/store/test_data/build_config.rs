use crate::service::model::build_config::{BuildConfig, BuildConfigRequest};
use crate::service::time_utils;

pub fn fake_build_config(config: BuildConfigRequest) -> BuildConfig {
    BuildConfig {
        user_id: config.user_id,
        canister_id: config.canister_id,
        canister_name: config.canister_name,
        repo_url: config.repo_url,
        commit_hash: config.commit_hash,
        rust_version: config.rust_version,
        dfx_version: config.dfx_version,
        optimize_times: config.optimize_times,
        created_at: time_utils::now_to_str(),
        updated_at: time_utils::now_to_str(),
    }
}
