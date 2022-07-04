mod api;
mod common;
mod service;

use crate::common::types::{AdminId, BuilderId, ValidatorId};
use crate::service::store::{
    activity::ActivityStore, admin, admin::AdminStore, build_config::BuildConfigStore, builder,
    builder::BuilderStore, validator, validator::ValidatorStore, verification::VerificationStore,
};
use ic_cdk::caller;
use ic_cdk::export::candid::candid_method;
use ic_cdk::export::candid::CandidType;
use ic_cdk_macros::init;
use serde::Deserialize;

use std::cell::RefCell;
use std::ops::Deref;

use ic_cdk::storage::{stable_restore, stable_save};
use ic_cdk::trap;

thread_local! {
    static ACTIVITY_STORE: RefCell<ActivityStore> = RefCell::new(ActivityStore::default());
    static ADMIN_STORE: RefCell<AdminStore> = RefCell::new(AdminStore::default());
    static BUILDER_STORE: RefCell<BuilderStore> = RefCell::new(BuilderStore::default());
    static BUILD_CONFIG_STORE: RefCell<BuildConfigStore> = RefCell::new(BuildConfigStore::default());
    static VALIDATOR_STORE: RefCell<ValidatorStore> = RefCell::new(ValidatorStore::default());
    static VERIFICATION_STORE: RefCell<VerificationStore> = RefCell::new(VerificationStore::default());
}

// ========================================================================================================
// INIT
//   - Admin
//   - Validator
//   - Builder
// ========================================================================================================

#[derive(CandidType, Deserialize)]
pub struct Config {
    admin: Option<Vec<AdminId>>,
    validator: Option<Vec<ValidatorId>>,
    builder: Option<Vec<BuilderId>>,
}

#[init]
#[candid_method(init)]
fn init(config: Option<Config>) {
    // default
    admin::add_admin(&caller());

    if let Some(config) = config {
        if let Some(admin) = config.admin {
            admin.iter().for_each(admin::add_admin);
        }
        if let Some(validator) = config.validator {
            validator.iter().for_each(validator::add_validator);
        }
        if let Some(builder) = config.builder {
            builder.iter().for_each(builder::add_builder);
        }
    }
}

type InternalStableStoreAsRef<'a> = (
    &'a AdminStore,
    &'a ActivityStore,
    &'a BuilderStore,
    &'a BuildConfigStore,
    &'a ValidatorStore,
    &'a VerificationStore,
);

pub fn pre_upgrade() {
    ACTIVITY_STORE.with(|activity_store|
            ADMIN_STORE.with(|admin_store|
                BUILDER_STORE.with(|builder_store|
                    BUILD_CONFIG_STORE.with(|build_config_store|
                        VALIDATOR_STORE.with(|validator_store|
                            VERIFICATION_STORE.with(|verification_store| {
                                if let Err(e) = stable_save::<InternalStableStoreAsRef>((
                                    admin_store.borrow().deref(),
                                    activity_store.borrow().deref(),
                                    builder_store.borrow().deref(),
                                    build_config_store.borrow().deref(),
                                    validator_store.borrow().deref(),
                                    verification_store.borrow().deref()
                                )){
                                    trap(&format!(
                                        "An error occurred when saving to stable memory (pre_upgrade): {:?}",
                                        e
                                    ));

                            }}))))))
}

type InternalStableStore = (
    AdminStore,
    ActivityStore,
    BuilderStore,
    BuildConfigStore,
    ValidatorStore,
    VerificationStore,
);

pub fn post_upgrade() {
    stable_restore::<InternalStableStore>()
        .map(
            |(
                admin_store_mut,
                activity_store_mut,
                builder_store_mut,
                build_config_store_mut,
                validator_store_mut,
                verification_store_mut,
            )| {
                ACTIVITY_STORE.with(|activity_store| {
                    ADMIN_STORE.with(|admin_store| {
                        BUILDER_STORE.with(|builder_store| {
                            BUILD_CONFIG_STORE.with(|build_config_store| {
                                VALIDATOR_STORE.with(|validator_store| {
                                    VERIFICATION_STORE.with(|verification_store| {
                                        *verification_store.borrow_mut() = verification_store_mut;
                                        *build_config_store.borrow_mut() = build_config_store_mut;
                                        *builder_store.borrow_mut() = builder_store_mut;
                                        *admin_store.borrow_mut() = admin_store_mut;
                                        *validator_store.borrow_mut() = validator_store_mut;
                                        *activity_store.borrow_mut() = activity_store_mut
                                    })
                                })
                            })
                        })
                    })
                })
            },
        )
        .unwrap_or_else(|e| {
            trap(&format!(
                "An error occurred when loading from stable memory (post_upgrade): {:?}",
                e
            ));
        });
}

#[cfg(any(target_arch = "wasm32"))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    use crate::common::types::*;
    use crate::service::model::activity::*;
    use crate::service::model::build_config::*;
    use crate::service::model::error::*;
    use crate::service::model::pagination::*;
    use crate::service::model::stats::*;
    use crate::service::model::verification::*;
    use ic_cdk::api::call::ManualReply;

    ic_cdk::export::candid::export_service!();
    std::print!("{}", __export_service());
}
