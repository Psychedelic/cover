use ic_kit::ic::{get, get_mut};
use ic_kit::ic::{stable_restore, stable_store, trap};

use crate::service::store::admin::AdminStore;
use crate::service::store::build_config::BuildConfigStore;
use crate::service::store::builder::BuilderStore;
use crate::service::store::validator::ValidatorStore;
use crate::service::store::verification::VerificationStore;

pub mod admin;
pub mod build_config;
pub mod builder;
pub mod guard;
pub mod model;
pub mod pagination;
pub mod time_utils;
pub mod validator;
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
fn builder_store_mut() -> &'static mut BuilderStore {
    get_mut()
}

#[inline]
fn builder_store() -> &'static BuilderStore {
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

#[inline]
fn validator_store_mut() -> &'static mut ValidatorStore {
    get_mut()
}

#[inline]
fn validator_store() -> &'static ValidatorStore {
    get()
}

/// These steps are atomic: If canister_pre_upgrade or canister_post_upgrade trap, the upgrade has failed, and the canister is reverted to the previous state. Otherwise, the upgrade has succeeded, and the old instance is discarded.
/// fyi: https://sdk.dfinity.org/docs/interface-spec/index.html#system-api

type InternalStableStoreAsRef = (
    &'static VerificationStore,
    &'static BuilderStore,
    &'static BuildConfigStore,
    &'static AdminStore,
    &'static ValidatorStore,
);

pub fn pre_upgrade() {
    if let Err(e) = stable_store::<InternalStableStoreAsRef>((
        verification_store(),
        builder_store(),
        build_config_store(),
        admin_store(),
        validator_store(),
    )) {
        trap(&format!(
            "An error occurred when saving to stable memory (pre_upgrade): {:?}",
            e
        ));
    };
}

type InternalStableStore = (
    VerificationStore,
    BuilderStore,
    BuildConfigStore,
    AdminStore,
    ValidatorStore,
);

pub fn post_upgrade() {
    stable_restore::<InternalStableStore>()
        .map(
            |(
                verification_store,
                builder_store,
                build_config_store,
                admin_store,
                validator_store,
            )| {
                (*verification_store_mut()) = verification_store;
                (*builder_store_mut()) = builder_store;
                (*build_config_store_mut()) = build_config_store;
                (*admin_store_mut()) = admin_store;
                (*validator_store_mut()) = validator_store;
            },
        )
        .unwrap_or_else(|e| {
            trap(&format!(
                "An error occurred when loading from stable memory (post_upgrade): {:?}",
                e
            ));
        });
}
