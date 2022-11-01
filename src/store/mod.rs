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
use verification::VerificationStore;

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
}

type InternalStableStoreAsRef<'a> = (
    &'a AdminStore,
    &'a ValidatorStore,
    &'a BuilderStore,
    &'a ActivityStore,
    &'a MyActivityStore,
    &'a BuildConfigStore,
    &'a VerificationStore,
);

#[pre_upgrade]
pub fn pre_upgrade() {
    ADMIN_STORE.with(|admin_store|
        VALIDATOR_STORE.with(|validator_store|
            BUILDER_STORE.with(|builder_store|
                ACTIVITY_STORE.with(|activity_store|
                    MY_ACTIVITY_STORE.with(|my_activity_store|
                        BUILD_CONFIG_STORE.with(|build_config_store|
                            VERIFICATION_STORE.with(|verification_store| {
                                if let Err(e) = stable_save::<InternalStableStoreAsRef>((
                                    admin_store.borrow().deref(),
                                    validator_store.borrow().deref(),
                                    builder_store.borrow().deref(),
                                    activity_store.borrow().deref(),
                                    my_activity_store.borrow().deref(),
                                    build_config_store.borrow().deref(),
                                    verification_store.borrow().deref()
                                )) {
                                    trap(&format!(
                                        "An error occurred when saving to stable memory (pre_upgrade): {:?}",
                                        e
                                    ));
                                }
                            })))))))
}

type InternalStableStore = (
    AdminStore,
    ValidatorStore,
    BuilderStore,
    ActivityStore,
    MyActivityStore,
    BuildConfigStore,
    VerificationStore,
);

#[post_upgrade]
pub fn post_upgrade() {
    stable_restore::<InternalStableStore>()
        .map(
            |(
                admin_store_mut,
                validator_store_mut,
                builder_store_mut,
                activity_store_mut,
                my_activity_store_mut,
                build_config_store_mut,
                verification_store_mut,
            )| {
                ADMIN_STORE.with(|admin_store| {
                    VALIDATOR_STORE.with(|validator_store| {
                        BUILDER_STORE.with(|builder_store| {
                            ACTIVITY_STORE.with(|activity_store| {
                                MY_ACTIVITY_STORE.with(|my_activity_store| {
                                    BUILD_CONFIG_STORE.with(|build_config_store| {
                                        VERIFICATION_STORE.with(|verification_store| {
                                            *admin_store.borrow_mut() = admin_store_mut;
                                            *validator_store.borrow_mut() = validator_store_mut;
                                            *builder_store.borrow_mut() = builder_store_mut;
                                            *activity_store.borrow_mut() = activity_store_mut;
                                            *my_activity_store.borrow_mut() = my_activity_store_mut;
                                            *build_config_store.borrow_mut() =
                                                build_config_store_mut;
                                            *verification_store.borrow_mut() =
                                                verification_store_mut;
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
