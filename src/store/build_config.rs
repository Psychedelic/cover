use super::BUILD_CONFIG_STORE;
use crate::common::types::{CanisterId, CanisterOwnerId};
use crate::model::build_config::{BuildConfig, SaveBuildConfig};
use ic_cdk::api::call::ManualReply;
use ic_cdk::api::time;
use ic_cdk::export::candid::CandidType;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(CandidType, Default, Deserialize)]
pub struct BuildConfigStore {
    pub configs: BTreeMap<(CanisterOwnerId, CanisterId), BuildConfig>,
}

pub fn get_build_configs<F: Fn(Vec<&BuildConfig>) -> ManualReply<Vec<BuildConfig>>>(
    owner_id: &CanisterOwnerId,
    manual_reply: F,
) -> ManualReply<Vec<BuildConfig>> {
    BUILD_CONFIG_STORE.with(|store| {
        manual_reply(
            store
                .borrow()
                .configs
                .iter()
                .filter(|((c, _), _)| c == owner_id)
                .map(|(_, v)| v)
                .collect(),
        )
    })
}

pub fn save_build_config(config: SaveBuildConfig) {
    BUILD_CONFIG_STORE.with(|store| {
        let now = time();
        store.borrow_mut().configs.insert(
            (config.owner_id, config.canister_id),
            BuildConfig {
                owner_id: config.owner_id,
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
    owner_id: &CanisterOwnerId,
    canister_id: &CanisterId,
    manual_reply: F,
) -> ManualReply<Option<BuildConfig>> {
    BUILD_CONFIG_STORE
        .with(|store| manual_reply(store.borrow().configs.get(&(*owner_id, *canister_id))))
}

pub fn delete_build_config(owner_id: &CanisterOwnerId, canister_id: &CanisterId) {
    BUILD_CONFIG_STORE.with(|store| {
        store
            .borrow_mut()
            .configs
            .remove(&(*owner_id, *canister_id));
    })
}
