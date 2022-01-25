use ic_kit::ic::{get, get_mut};
use ic_kit::ic::{stable_restore, stable_store, trap};

use crate::service::store::admin::AdminStore;
use crate::service::store::build_config::BuildConfigStore;
use crate::service::store::provider::ProviderStore;
use crate::service::store::verification::VerificationStore;

pub mod admin;
pub mod build_config;
pub mod guard;
pub mod model;
pub mod provider;
pub mod time_utils;
pub mod verification;

#[cfg(not(test))]
mod store;

#[cfg(test)]
pub mod store;

#[inline]
fn verification_store_mut() -> &'static mut VerificationStore {
    get_mut()
}

#[inline]
fn verification_store() -> &'static VerificationStore {
    get()
}

#[inline]
fn provider_store_mut() -> &'static mut ProviderStore {
    get_mut()
}

#[inline]
fn provider_store() -> &'static ProviderStore {
    get()
}

#[inline]
fn build_config_store_mut() -> &'static mut BuildConfigStore {
    get_mut()
}

#[inline]
fn build_config_store() -> &'static BuildConfigStore {
    get()
}

#[inline]
fn admin_store_mut() -> &'static mut AdminStore {
    get_mut()
}

#[inline]
fn admin_store() -> &'static AdminStore {
    get()
}

/// These steps are atomic: If canister_pre_upgrade or canister_post_upgrade trap, the upgrade has failed, and the canister is reverted to the previous state. Otherwise, the upgrade has succeeded, and the old instance is discarded.
/// fyi: https://sdk.dfinity.org/docs/interface-spec/index.html#system-api

type InternalStableStoreAsRef = (
    &'static VerificationStore,
    &'static ProviderStore,
    &'static BuildConfigStore,
    &'static AdminStore,
);

pub fn pre_upgrade() {
    if let Err(e) = stable_store::<InternalStableStoreAsRef>((
        verification_store(),
        provider_store(),
        build_config_store(),
        admin_store(),
    )) {
        trap(&format!(
            "An error occurred when saving to stable memory (pre_upgrade): {:?}",
            e
        ));
    };
}

type InternalStableStore = (
    VerificationStore,
    ProviderStore,
    BuildConfigStore,
    AdminStore,
);

pub fn post_upgrade() {
    stable_restore::<InternalStableStore>()
        .map(
            |(verification_store, provider_store, build_config_store, admin_store)| {
                (*verification_store_mut()) = verification_store;
                (*provider_store_mut()) = provider_store;
                (*build_config_store_mut()) = build_config_store;
                (*admin_store_mut()) = admin_store;
            },
        )
        .unwrap_or_else(|e| {
            trap(&format!(
                "An error occurred when loading from stable memory (post_upgrade): {:?}",
                e
            ));
        });
}
