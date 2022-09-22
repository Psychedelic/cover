pub mod activity;
pub mod admin;
pub mod build_config;
pub mod builder;
pub mod validator;
pub mod verification;

use activity::ActivityStore;
use admin::AdminStore;
use build_config::BuildConfigStore;
use builder::BuilderStore;
use ic_cdk::storage::{stable_restore, stable_save};
use ic_cdk::trap;
use ic_cdk_macros::{post_upgrade, pre_upgrade};
use std::cell::RefCell;
use std::ops::Deref;
use validator::ValidatorStore;
use verification::VerificationStore;

thread_local! {
    static ACTIVITY_STORE: RefCell<ActivityStore> = RefCell::new(ActivityStore::default());
    static ADMIN_STORE: RefCell<AdminStore> = RefCell::new(AdminStore::default());
    static BUILDER_STORE: RefCell<BuilderStore> = RefCell::new(BuilderStore::default());
    static BUILD_CONFIG_STORE: RefCell<BuildConfigStore> = RefCell::new(BuildConfigStore::default());
    static VALIDATOR_STORE: RefCell<ValidatorStore> = RefCell::new(ValidatorStore::default());
    static VERIFICATION_STORE: RefCell<VerificationStore> = RefCell::new(VerificationStore::default());
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

// =================================================================================================
// Legacy
// =================================================================================================
use crate::common::types::*;
use crate::model::verification::*;
use crate::{Activity, BuildConfig};
use chrono::{DateTime, Utc};
use ic_cdk::export::candid::CandidType;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap, LinkedList};
use std::str::FromStr;

#[derive(Default, CandidType, Deserialize)]
pub struct ActivityStoreLegacy {
    pub activities: LinkedList<ActivityLegacy>,
}

#[derive(CandidType, Deserialize)]
pub struct ActivityLegacy {
    pub canister_id: CanisterId,
    pub build_status: BuildStatus,
    pub create_at: String,
}

#[derive(CandidType, Default, Deserialize)]
pub struct BuildConfigStoreLegacy {
    pub configs: BTreeMap<(CanisterOwnerId, CanisterId), BuildConfigLegacy>,
}

#[derive(CandidType, Deserialize)]
pub struct BuildConfigLegacy {
    pub owner_id: CanisterOwnerId,
    pub delegate_canister_id: Option<CanisterId>,
    pub canister_id: CanisterId,
    pub canister_name: String,
    pub repo_url: String,
    pub commit_hash: String,
    pub rust_version: Option<String>,
    pub dfx_version: String,
    pub optimize_count: u8,
    pub updated_at: String,
}

#[derive(CandidType, Deserialize, Default)]
pub struct VerificationStoreLegacy {
    pub verifications: HashMap<CanisterId, VerificationLegacy>,
    pub records: Vec<CanisterId>,
}

#[derive(CandidType, Deserialize)]
pub struct VerificationLegacy {
    pub delegate_canister_id: Option<CanisterId>,
    pub canister_id: CanisterId,
    pub canister_name: String,
    pub repo_url: String,
    pub commit_hash: String,
    pub wasm_hash: Option<String>,
    pub build_url: Option<String>,
    pub build_status: BuildStatus,
    pub canister_type: Option<CanisterType>,
    pub rust_version: Option<String>,
    pub dfx_version: String,
    pub optimize_count: u8,
    pub repo_visibility: String,
    pub updated_by: CanisterOwnerId,
    pub updated_at: String,
}

type InternalStableStore = (
    AdminStore,
    ActivityStoreLegacy,
    BuilderStore,
    BuildConfigStoreLegacy,
    ValidatorStore,
    VerificationStoreLegacy,
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
                                        *verification_store.borrow_mut() = VerificationStore {
                                            verifications: verification_store_mut
                                                .verifications
                                                .into_iter()
                                                .map(|(k, verification_legacy)| {
                                                    (
                                                        k,
                                                        Verification {
                                                            delegate_canister_id:
                                                                verification_legacy
                                                                    .delegate_canister_id,
                                                            canister_id: verification_legacy
                                                                .canister_id,
                                                            canister_name: verification_legacy
                                                                .canister_name,
                                                            repo_url: verification_legacy.repo_url,
                                                            commit_hash: verification_legacy
                                                                .commit_hash,
                                                            wasm_hash: verification_legacy
                                                                .wasm_hash,
                                                            build_url: verification_legacy
                                                                .build_url,
                                                            build_status: verification_legacy
                                                                .build_status,
                                                            canister_type: verification_legacy
                                                                .canister_type,
                                                            rust_version: verification_legacy
                                                                .rust_version,
                                                            dfx_version: verification_legacy
                                                                .dfx_version,
                                                            optimize_count: verification_legacy
                                                                .optimize_count,
                                                            repo_visibility: verification_legacy
                                                                .repo_visibility,
                                                            updated_by: verification_legacy
                                                                .updated_by,
                                                            updated_at: DateTime::<Utc>::from_str(
                                                                &verification_legacy.updated_at,
                                                            )
                                                            .unwrap()
                                                            .timestamp_nanos()
                                                                as u64,
                                                        },
                                                    )
                                                })
                                                .collect(),
                                            records: verification_store_mut.records,
                                        };
                                        *build_config_store.borrow_mut() = BuildConfigStore {
                                            configs: build_config_store_mut
                                                .configs
                                                .into_iter()
                                                .map(|(k, build_config_legacy)| {
                                                    (
                                                        k,
                                                        BuildConfig {
                                                            owner_id: build_config_legacy.owner_id,
                                                            delegate_canister_id:
                                                                build_config_legacy
                                                                    .delegate_canister_id,
                                                            canister_id: build_config_legacy
                                                                .canister_id,
                                                            canister_name: build_config_legacy
                                                                .canister_name,
                                                            repo_url: build_config_legacy.repo_url,
                                                            commit_hash: build_config_legacy
                                                                .commit_hash,
                                                            rust_version: build_config_legacy
                                                                .rust_version,
                                                            dfx_version: build_config_legacy
                                                                .dfx_version,
                                                            optimize_count: build_config_legacy
                                                                .optimize_count,
                                                            updated_at: DateTime::<Utc>::from_str(
                                                                &build_config_legacy.updated_at,
                                                            )
                                                            .unwrap()
                                                            .timestamp_nanos()
                                                                as u64,
                                                        },
                                                    )
                                                })
                                                .collect(),
                                        };
                                        *builder_store.borrow_mut() = builder_store_mut;
                                        *admin_store.borrow_mut() = admin_store_mut;
                                        *validator_store.borrow_mut() = validator_store_mut;
                                        *activity_store.borrow_mut() = ActivityStore {
                                            activities: activity_store_mut
                                                .activities
                                                .into_iter()
                                                .map(|activity_legacy| Activity {
                                                    canister_id: activity_legacy.canister_id,
                                                    build_status: activity_legacy.build_status,
                                                    created_at: DateTime::<Utc>::from_str(
                                                        &activity_legacy.create_at,
                                                    )
                                                    .unwrap()
                                                    .timestamp_nanos()
                                                        as u64,
                                                })
                                                .collect(),
                                        };
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
// =================================================================================================
// Legacy
// =================================================================================================

// type InternalStableStore = (
//     AdminStore,
//     ActivityStore,
//     BuilderStore,
//     BuildConfigStore,
//     ValidatorStore,
//     VerificationStore,
// );
//
// #[post_upgrade]
// pub fn post_upgrade() {
//     stable_restore::<InternalStableStore>()
//         .map(
//             |(
//                 admin_store_mut,
//                 activity_store_mut,
//                 builder_store_mut,
//                 build_config_store_mut,
//                 validator_store_mut,
//                 verification_store_mut,
//             )| {
//                 ACTIVITY_STORE.with(|activity_store| {
//                     ADMIN_STORE.with(|admin_store| {
//                         BUILDER_STORE.with(|builder_store| {
//                             BUILD_CONFIG_STORE.with(|build_config_store| {
//                                 VALIDATOR_STORE.with(|validator_store| {
//                                     VERIFICATION_STORE.with(|verification_store| {
//                                         *verification_store.borrow_mut() = verification_store_mut;
//                                         *build_config_store.borrow_mut() = build_config_store_mut;
//                                         *builder_store.borrow_mut() = builder_store_mut;
//                                         *admin_store.borrow_mut() = admin_store_mut;
//                                         *validator_store.borrow_mut() = validator_store_mut;
//                                         *activity_store.borrow_mut() = activity_store_mut
//                                     })
//                                 })
//                             })
//                         })
//                     })
//                 })
//             },
//         )
//         .unwrap_or_else(|e| {
//             trap(&format!(
//                 "An error occurred when loading from stable memory (post_upgrade): {:?}",
//                 e
//             ));
//         });
// }
