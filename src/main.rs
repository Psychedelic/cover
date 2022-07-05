mod common;
mod service;

use crate::common::types::{AdminId, BuilderId, ValidatorId};
use crate::service::store::{
    activity::ActivityStore, admin, admin::AdminStore, build_config::BuildConfigStore, builder,
    builder::BuilderStore, validator, validator::ValidatorStore, verification,
    verification::VerificationStore,
};
use ic_cdk::caller;
use ic_cdk::export::candid::CandidType;
use serde::Deserialize;

use std::cell::RefCell;
use std::ops::Deref;

use crate::common::constants::{MAX_ITEMS_PER_PAGE, MIN_ITEMS_PER_PAGE};
use crate::service::model::activity::Activity;
use crate::service::model::pagination::{Pagination, PaginationInfo};
use crate::service::model::stats::Stats;
use crate::service::model::verification::{RegisterVerification, SubmitVerification, Verification};
use crate::service::store::activity;
use ic_cdk::api::call::ManualReply;
use ic_cdk::export::candid::candid_method;
use ic_cdk::storage::{stable_restore, stable_save};
use ic_cdk::trap;
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use std::cmp::{max, min};

use crate::common::types::CanisterId;
use crate::service::guard::{is_admin, is_builder, is_validator};
use crate::service::model::build_config::{BuildConfig, BuildConfigInfo, SaveBuildConfig};
use crate::service::model::error::Error;
use crate::service::store::build_config;

thread_local! {
    static ACTIVITY_STORE: RefCell<ActivityStore> = RefCell::new(ActivityStore::default());
    static ADMIN_STORE: RefCell<AdminStore> = RefCell::new(AdminStore::default());
    static BUILDER_STORE: RefCell<BuilderStore> = RefCell::new(BuilderStore::default());
    static BUILD_CONFIG_STORE: RefCell<BuildConfigStore> = RefCell::new(BuildConfigStore::default());
    static VALIDATOR_STORE: RefCell<ValidatorStore> = RefCell::new(ValidatorStore::default());
    static VERIFICATION_STORE: RefCell<VerificationStore> = RefCell::new(VerificationStore::default());
}

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

// =====================================================================================================
// Activity
// =====================================================================================================
#[query(name = "getActivities", manual_reply = true)]
#[candid_method(query, rename = "getActivities")]
fn get_activities(mut pagination_info: PaginationInfo) -> ManualReply<Pagination<Activity>> {
    pagination_info.items_per_page = max(MIN_ITEMS_PER_PAGE, pagination_info.items_per_page);
    pagination_info.items_per_page = min(MAX_ITEMS_PER_PAGE, pagination_info.items_per_page);

    activity::get_activities(pagination_info, |result| ManualReply::one(result))
}

// =====================================================================================================
// Admin
// =====================================================================================================
#[update(name = "addAdmin", guard = "is_admin")]
#[candid_method(update, rename = "addAdmin")]
fn add_admin(admin_id: AdminId) {
    admin::add_admin(&admin_id)
}

#[update(name = "deleteAdmin", guard = "is_admin")]
#[candid_method(update, rename = "deleteAdmin")]
fn delete_admin(admin_id: AdminId) {
    admin::delete_admin(&admin_id)
}

#[query(name = "getAdmins", guard = "is_admin", manual_reply = true)]
#[candid_method(query, rename = "getAdmins")]
fn get_admins() -> ManualReply<Vec<AdminId>> {
    admin::get_admins(|result| ManualReply::one(result))
}

// =====================================================================================================
// Build Config
// =====================================================================================================
#[query(name = "getBuildConfigs", manual_reply = true)]
#[candid_method(query, rename = "getBuildConfigs")]
fn get_build_configs() -> ManualReply<Vec<BuildConfig>> {
    build_config::get_build_configs(&caller(), |result| ManualReply::one(result))
}

#[query(name = "getBuildConfigById", manual_reply = true)]
#[candid_method(query, rename = "getBuildConfigById")]
fn get_build_config_by_id(canister_id: CanisterId) -> ManualReply<Option<BuildConfig>> {
    build_config::get_build_config_by_id(&caller(), &canister_id, |result| ManualReply::one(result))
}

#[update(name = "deleteBuildConfig")]
#[candid_method(update, rename = "deleteBuildConfig")]
fn delete_build_config(canister_id: CanisterId) {
    build_config::delete_build_config(&caller(), &canister_id)
}

#[update(name = "saveBuildConfig", guard = "is_validator")]
#[candid_method(update, rename = "saveBuildConfig")]
fn save_build_config(config: SaveBuildConfig) {
    build_config::save_build_config(config)
}

#[query(
    name = "getBuildConfigValidator",
    guard = "is_validator",
    manual_reply = true
)]
#[candid_method(query, rename = "getBuildConfigValidator")]
fn get_build_config_validator(info: BuildConfigInfo) -> ManualReply<Option<BuildConfig>> {
    build_config::get_build_config_by_id(&info.owner_id, &info.canister_id, |result| {
        ManualReply::one(result)
    })
}

// =====================================================================================================
// Builder
// =====================================================================================================
#[update(name = "addBuilder", guard = "is_admin")]
#[candid_method(update, rename = "addBuilder")]
fn add_builder(builder_id: BuilderId) {
    builder::add_builder(&builder_id)
}

#[update(name = "deleteBuilder", guard = "is_admin")]
#[candid_method(update, rename = "deleteBuilder")]
fn delete_builder(builder_id: BuilderId) {
    builder::delete_builder(&builder_id)
}

#[query(name = "getBuilders", guard = "is_admin", manual_reply = true)]
#[candid_method(query, rename = "getBuilders")]
fn get_builders() -> ManualReply<Vec<BuilderId>> {
    builder::get_builders(|result| ManualReply::one(result))
}

// =====================================================================================================
// Validator
// =====================================================================================================
#[update(name = "addValidator", guard = "is_admin")]
#[candid_method(update, rename = "addValidator")]
fn add_validator(validator_id: ValidatorId) {
    validator::add_validator(&validator_id)
}

#[update(name = "deleteValidator", guard = "is_admin")]
#[candid_method(update, rename = "deleteValidator")]
fn delete_validator(validator_id: ValidatorId) {
    validator::delete_validator(&validator_id)
}

#[query(name = "getValidators", guard = "is_admin", manual_reply = true)]
#[candid_method(query, rename = "getValidators")]
fn get_validators() -> ManualReply<Vec<ValidatorId>> {
    validator::get_validators(|result| ManualReply::one(result))
}

// =====================================================================================================
// Verification
// =====================================================================================================
#[query(name = "getVerificationByCanisterId", manual_reply = true)]
#[candid_method(query, rename = "getVerificationByCanisterId")]
fn get_verification_by_canister_id(canister_id: CanisterId) -> ManualReply<Option<Verification>> {
    verification::get_verification_by_canister_id(&canister_id, |result| ManualReply::one(result))
}

#[query(name = "getVerifications", manual_reply = true)]
#[candid_method(query, rename = "getVerifications")]
fn get_verifications(mut pagination_info: PaginationInfo) -> ManualReply<Pagination<Verification>> {
    pagination_info.items_per_page = max(MIN_ITEMS_PER_PAGE, pagination_info.items_per_page);
    pagination_info.items_per_page = min(MAX_ITEMS_PER_PAGE, pagination_info.items_per_page);

    verification::get_verifications(&pagination_info, |result| ManualReply::one(result))
}

#[update(name = "submitVerification", guard = "is_builder")]
#[candid_method(update, rename = "submitVerification")]
fn submit_verification(verification: SubmitVerification) {
    verification::submit_verification(verification, activity::add_activity)
}

#[update(name = "registerVerification", guard = "is_validator")]
#[candid_method(update, rename = "registerVerification")]
fn register_verification(verification: RegisterVerification) -> Result<(), Error> {
    verification::register_verification(verification, activity::add_activity)
}

#[query(name = "getVerificationsStats")]
#[candid_method(query, rename = "getVerificationsStats")]
fn get_verifications_stats() -> Stats {
    verification::get_verifications_stats()
}

type InternalStableStoreAsRef<'a> = (
    &'a AdminStore,
    &'a ActivityStore,
    &'a BuilderStore,
    &'a BuildConfigStore,
    &'a ValidatorStore,
    &'a VerificationStore,
);

#[pre_upgrade]
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

#[post_upgrade]
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

    ic_cdk::export::candid::export_service!();
    std::print!("{}", __export_service());
}
