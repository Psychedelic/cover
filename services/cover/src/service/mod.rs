use ic_kit::ic::{get, get_mut};
use ic_kit::ic::{stable_restore, stable_store, trap};

use crate::service::store::canister_build_config::CanisterBuildConfigStore;
use crate::service::store::progress::ProgressStore;
use crate::service::store::provider::ProviderStore;
use crate::service::store::request::RequestStore;
use crate::service::store::verification::VerificationStore;

pub mod canister_build_config;
pub mod cover;
pub mod error_handler;
pub mod guard;
pub mod model;
pub mod time_utils;
pub mod types;

mod store;

#[inline]
fn get_request_store_mut() -> &'static mut RequestStore {
    get_mut()
}

#[inline]
fn get_request_store() -> &'static RequestStore {
    get()
}

#[inline]
fn get_progress_store_mut() -> &'static mut ProgressStore {
    get_mut()
}

#[inline]
fn get_progress_store() -> &'static ProgressStore {
    get()
}

#[inline]
fn get_verification_store_mut() -> &'static mut VerificationStore {
    get_mut()
}

#[inline]
fn get_verification_store() -> &'static VerificationStore {
    get()
}

#[inline]
fn get_provider_store_mut() -> &'static mut ProviderStore {
    get_mut()
}

#[inline]
fn get_provider_store() -> &'static ProviderStore {
    get()
}

#[inline]
fn canister_build_config_mut() -> &'static mut CanisterBuildConfigStore {
    get_mut()
}

#[inline]
fn canister_build_config() -> &'static CanisterBuildConfigStore {
    get()
}

/// These steps are atomic: If canister_pre_upgrade or canister_post_upgrade trap, the upgrade has failed, and the canister is reverted to the previous state. Otherwise, the upgrade has succeeded, and the old instance is discarded.
/// fyi: https://sdk.dfinity.org/docs/interface-spec/index.html#system-api

type InternalStableStoreAsRef = (
    &'static RequestStore,
    &'static ProgressStore,
    &'static VerificationStore,
    &'static ProviderStore,
    &'static CanisterBuildConfigStore,
);

pub fn pre_upgrade() {
    if let Err(e) = stable_store::<InternalStableStoreAsRef>((
        get_request_store(),
        get_progress_store(),
        get_verification_store(),
        get_provider_store(),
        canister_build_config(),
    )) {
        trap(&format!(
            "An error occurred when saving to stable memory (pre_upgrade): {:?}",
            e
        ));
    };
}

type InternalStableStore = (
    RequestStore,
    ProgressStore,
    VerificationStore,
    ProviderStore,
    CanisterBuildConfigStore,
);

pub fn post_upgrade() {
    stable_restore::<InternalStableStore>()
        .map(
            |(
                request_store,
                progress_store,
                verification_store,
                provider_store,
                build_config_store,
            )| {
                (*get_request_store_mut()) = request_store;
                (*get_progress_store_mut()) = progress_store;
                (*get_verification_store_mut()) = verification_store;
                (*get_provider_store_mut()) = provider_store;
                (*canister_build_config_mut()) = build_config_store;
            },
        )
        .unwrap_or_else(|e| {
            trap(&format!(
                "An error occurred when loading from stable memory (post_upgrade): {:?}",
                e
            ));
        });
}
