use std::cell::RefCell;
use std::ops::Deref;

use ic_cdk::storage::{stable_restore, stable_save};
use ic_cdk::trap;
use ic_cdk_macros::{post_upgrade, pre_upgrade};

use activity::{ActivityStore, MyActivityStore};
use admin::AdminStore;
use build_config::BuildConfigStore;
use builder::BuilderStore;
use validator::ValidatorStore;
use verification::{MyStatsStore, StatsStore, VerificationStore};

pub mod activity;
pub mod admin;
pub mod build_config;
pub mod builder;
pub mod validator;
pub mod verification;

thread_local! {
    static ADMIN_STORE: RefCell<AdminStore> = RefCell::new(AdminStore::default());
    static VALIDATOR_STORE: RefCell<ValidatorStore> = RefCell::new(ValidatorStore::default());
    static BUILDER_STORE: RefCell<BuilderStore> = RefCell::new(BuilderStore::default());
    static ACTIVITY_STORE: RefCell<ActivityStore> = RefCell::new(ActivityStore::default());
    static MY_ACTIVITY_STORE: RefCell<MyActivityStore> = RefCell::new(MyActivityStore::default());
    static BUILD_CONFIG_STORE: RefCell<BuildConfigStore> = RefCell::new(BuildConfigStore::default());
    static VERIFICATION_STORE: RefCell<VerificationStore> = RefCell::new(VerificationStore::default());
    static STATS_STORE: RefCell<StatsStore> = RefCell::new(StatsStore::default());
    static MY_STATS_STORE: RefCell<MyStatsStore> = RefCell::new(MyStatsStore::default());
}

type InternalStableStoreAsRef<'a> = (
    &'a AdminStore,
    &'a ValidatorStore,
    &'a BuilderStore,
    &'a ActivityStore,
    &'a MyActivityStore,
    &'a BuildConfigStore,
    &'a VerificationStore,
    &'a StatsStore,
    &'a MyStatsStore,
);

#[pre_upgrade]
pub fn pre_upgrade() {
    ADMIN_STORE.with(|admin_store|
        VALIDATOR_STORE.with(|validator_store|
            BUILDER_STORE.with(|builder_store|
                ACTIVITY_STORE.with(|activity_store|
                    MY_ACTIVITY_STORE.with(|my_activity_store|
                        BUILD_CONFIG_STORE.with(|build_config_store|
                            VERIFICATION_STORE.with(|verification_store|
                                STATS_STORE.with(|stats_store|
                                    MY_STATS_STORE.with(|my_stats_store| {
                                        if let Err(e) = stable_save::<InternalStableStoreAsRef>((
                                            admin_store.borrow().deref(),
                                            validator_store.borrow().deref(),
                                            builder_store.borrow().deref(),
                                            activity_store.borrow().deref(),
                                            my_activity_store.borrow().deref(),
                                            build_config_store.borrow().deref(),
                                            verification_store.borrow().deref(),
                                            stats_store.borrow().deref(),
                                            my_stats_store.borrow().deref()
                                        )) {
                                            trap(&format!(
                                                "An error occurred when saving to stable memory (pre_upgrade): {:?}",
                                                e
                                            ));
                                        }
                                    })))))))))
}

type InternalStableStore = (
    AdminStore,
    ValidatorStore,
    BuilderStore,
    ActivityStore,
    MyActivityStore,
    BuildConfigStore,
    VerificationStore,
    StatsStore,
    MyStatsStore,
);

#[post_upgrade]
pub fn post_upgrade() {
    stable_restore::<InternalStableStore>()
        .map(
            |(
                admin,
                validator,
                builder,
                activity,
                my_activity,
                build_config,
                verification,
                stats,
                my_stats,
            )| {
                ADMIN_STORE.with(|admin_store| {
                    VALIDATOR_STORE.with(|validator_store| {
                        BUILDER_STORE.with(|builder_store| {
                            ACTIVITY_STORE.with(|activity_store| {
                                MY_ACTIVITY_STORE.with(|my_activity_store| {
                                    BUILD_CONFIG_STORE.with(|build_config_store| {
                                        VERIFICATION_STORE.with(|verification_store| {
                                            STATS_STORE.with(|stats_store| {
                                                MY_STATS_STORE.with(|my_stats_store| {
                                                    *admin_store.borrow_mut() = admin;
                                                    *validator_store.borrow_mut() = validator;
                                                    *builder_store.borrow_mut() = builder;
                                                    *activity_store.borrow_mut() = activity;
                                                    *my_activity_store.borrow_mut() = my_activity;
                                                    *build_config_store.borrow_mut() = build_config;
                                                    *verification_store.borrow_mut() = verification;
                                                    *stats_store.borrow_mut() = stats;
                                                    *my_stats_store.borrow_mut() = my_stats;
                                                })
                                            })
                                        })
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
