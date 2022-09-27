use super::BUILD_CONFIG_STORE;
use crate::common::types::{CallerId, CanisterId};
use crate::model::build_config::{BuildConfig, SaveBuildConfig};
use ic_cdk::api::call::ManualReply;
use ic_cdk::api::time;
use ic_cdk::export::candid::CandidType;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(CandidType, Default, Deserialize)]
pub struct BuildConfigStore {
    configs: BTreeMap<(CallerId, CanisterId), BuildConfig>,
}

pub fn get_build_configs<F: Fn(Vec<&BuildConfig>) -> ManualReply<Vec<BuildConfig>>>(
    caller_id: &CallerId,
    manual_reply: F,
) -> ManualReply<Vec<BuildConfig>> {
    BUILD_CONFIG_STORE.with(|store| {
        manual_reply(
            store
                .borrow()
                .configs
                .iter()
                .filter(|((c, _), _)| c == caller_id)
                .map(|(_, v)| v)
                .collect(),
        )
    })
}

pub fn save_build_config(config: SaveBuildConfig) {
    BUILD_CONFIG_STORE.with(|store| {
        let now = time();
        store.borrow_mut().configs.insert(
            (config.caller_id, config.canister_id),
            BuildConfig {
                caller_id: config.caller_id,
                delegate_canister_id: config.delegate_canister_id,
                canister_id: config.canister_id,
                canister_name: config.canister_name,
                repo_url: config.repo_url,
                commit_hash: config.commit_hash,
                rust_version: config.rust_version,
                dfx_version: config.dfx_version,
                optimize_count: config.optimize_count,
                updated_at: now,
            },
        );
    })
}

pub fn get_build_config_by_id<F: Fn(Option<&BuildConfig>) -> ManualReply<Option<BuildConfig>>>(
    caller_id: &CallerId,
    canister_id: &CanisterId,
    manual_reply: F,
) -> ManualReply<Option<BuildConfig>> {
    BUILD_CONFIG_STORE
        .with(|store| manual_reply(store.borrow().configs.get(&(*caller_id, *canister_id))))
}

pub fn delete_build_config(caller_id: &CallerId, canister_id: &CanisterId) {
    BUILD_CONFIG_STORE.with(|store| {
        store
            .borrow_mut()
            .configs
            .remove(&(*caller_id, *canister_id));
    })
}
