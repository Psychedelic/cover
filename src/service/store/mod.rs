use activity::ActivityStore;
use admin::AdminStore;
use build_config::BuildConfigStore;
use builder::BuilderStore;
use validator::ValidatorStore;
use verification::VerificationStore;

use std::cell::RefCell;

pub mod activity;
pub mod admin;
pub mod build_config;
pub mod builder;
pub mod validator;
pub mod verification;

thread_local! {
    static ACTIVITY_STORE: RefCell<ActivityStore> = RefCell::new(ActivityStore::default());
    static ADMIN_STORE: RefCell<AdminStore> = RefCell::new(AdminStore::default());
    static BUILDER_STORE: RefCell<BuilderStore> = RefCell::new(BuilderStore::default());
    static BUILD_CONFIG_STORE: RefCell<BuildConfigStore> = RefCell::new(BuildConfigStore::default());
    static VALIDATOR_STORE: RefCell<ValidatorStore> = RefCell::new(ValidatorStore::default());
    static VERIFICATION_STORE: RefCell<VerificationStore> = RefCell::new(VerificationStore::default());
}
