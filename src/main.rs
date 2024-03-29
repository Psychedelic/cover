use std::cmp::{max, min};

use compile_time_run::run_command_str;
use ic_cdk::api::call::ManualReply;
use ic_cdk::caller;
use ic_cdk::export::candid::candid_method;
use ic_cdk_macros::{init, query, update};

use crate::common::constants::{MAX_ITEMS_PER_PAGE, MIN_ITEMS_PER_PAGE};
use crate::common::types::{AdminId, BuilderId, CallerId, CanisterId, ValidatorId};
use crate::model::activity::{Activity, MyActivity, MyBuildConfigActivity};
use crate::model::build_config::{BuildConfig, BuildConfigInfo, SaveBuildConfig};
use crate::model::config::Config;
use crate::model::cover_metadata::CoverMetadata;
use crate::model::error::Error;
use crate::model::pagination::{Pagination, PaginationInfo};
use crate::model::stats::Stats;
use crate::model::verification::{
    BuildStatus, RegisterVerification, SubmitVerification, Verification,
};
use crate::store::{activity, admin, build_config, builder, validator, verification};
use crate::util::guard::{is_admin, is_builder, is_validator};

mod common;
mod model;
mod store;
mod util;

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
// CoverMetadata
// =====================================================================================================
#[query(name = "coverMetadata")]
#[candid_method(query, rename = "coverMetadata")]
fn cover_metadata() -> CoverMetadata {
    CoverMetadata {
        canister_name: "cover_test",
        repo_url: "psychedelic/cover",
        commit_hash: run_command_str!("git", "rev-parse", "HEAD"),
        dfx_version: "0.11.2",
        rust_version: Some("1.65.0"),
        optimize_count: 0,
        controller: "j3dqd-46f74-s45g5-yt6qa-c5vyq-4zv7t-y4iie-omikc-cjngg-olpgg-rqe",
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

#[query(name = "getMyActivities", manual_reply = true)]
#[candid_method(query, rename = "getMyActivities")]
fn get_my_activities(mut pagination_info: PaginationInfo) -> ManualReply<Pagination<MyActivity>> {
    pagination_info.items_per_page = max(MIN_ITEMS_PER_PAGE, pagination_info.items_per_page);
    pagination_info.items_per_page = min(MAX_ITEMS_PER_PAGE, pagination_info.items_per_page);
    activity::get_my_activities(caller(), pagination_info, |result| ManualReply::one(result))
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
#[query(name = "getMyBuildConfigs", manual_reply = true)]
#[candid_method(query, rename = "getMyBuildConfigs")]
fn get_my_build_configs() -> ManualReply<Vec<BuildConfig>> {
    build_config::get_build_configs(&caller(), |result| ManualReply::one(result))
}

#[query(name = "getMyBuildConfigById", manual_reply = true)]
#[candid_method(query, rename = "getMyBuildConfigById")]
fn get_my_build_config_by_id(canister_id: CanisterId) -> ManualReply<Option<BuildConfig>> {
    build_config::get_build_config_by_id(&caller(), &canister_id, |result| ManualReply::one(result))
}

#[update(name = "deleteMyBuildConfig")]
#[candid_method(update, rename = "deleteMyBuildConfig")]
fn delete_my_build_config(canister_id: CanisterId) {
    build_config::delete_build_config(&caller(), &canister_id);
    activity::add_my_activity(
        canister_id,
        caller(),
        None,
        Some(MyBuildConfigActivity::Delete),
    );
}

#[update(name = "saveBuildConfig", guard = "is_validator")]
#[candid_method(update, rename = "saveBuildConfig")]
fn save_build_config(config: SaveBuildConfig) {
    let canister_id = config.canister_id;
    let caller_id = config.caller_id;
    build_config::save_build_config(config);
    activity::add_my_activity(
        canister_id,
        caller_id,
        None,
        Some(MyBuildConfigActivity::Save),
    );
}

#[query(
    name = "getBuildConfigValidator",
    guard = "is_validator",
    manual_reply = true
)]
#[candid_method(query, rename = "getBuildConfigValidator")]
fn get_build_config_validator(info: BuildConfigInfo) -> ManualReply<Option<BuildConfig>> {
    build_config::get_build_config_by_id(&info.caller_id, &info.canister_id, |result| {
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

fn activity_handler(canister_id: CanisterId, caller_id: CallerId, build_status: BuildStatus) {
    activity::add_activity(canister_id, build_status);
    activity::add_my_activity(canister_id, caller_id, Some(build_status), None);
}

#[update(name = "submitVerification", guard = "is_builder")]
#[candid_method(update, rename = "submitVerification")]
fn submit_verification(verification: SubmitVerification) {
    verification::submit_verification(verification, activity_handler)
}

#[update(name = "registerVerification", guard = "is_validator")]
#[candid_method(update, rename = "registerVerification")]
fn register_verification(verification: RegisterVerification) -> Result<(), Error> {
    verification::register_verification(verification, activity_handler)
}

#[query(name = "getVerificationStats", manual_reply = true)]
#[candid_method(query, rename = "getVerificationStats")]
fn get_verification_stats() -> ManualReply<Stats> {
    verification::get_verification_stats(|stats| ManualReply::one(stats))
}

#[query(name = "getMyVerificationStats", manual_reply = true)]
#[candid_method(query, rename = "getMyVerificationStats")]
fn get_my_verification_stats() -> ManualReply<Stats> {
    verification::get_my_verification_stats(caller(), |stats| ManualReply::one(stats))
}

#[cfg(any(target_arch = "wasm32"))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    ic_cdk::export::candid::export_service!();
    std::print!("{}", __export_service());
}
