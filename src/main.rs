mod api;
mod common;
mod service;
mod upgrade;

#[cfg(any(target_arch = "wasm32"))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    use crate::common::types::*;
    use crate::service::model::activity::*;
    use crate::service::model::build_config::*;
    use crate::service::model::error::*;
    use crate::service::model::pagination::*;
    use crate::service::model::verification::*;

    ic_kit::candid::export_service!();
    std::print!("{}", __export_service());
}

// ========================================================================================================
// INIT
//   - Admin
//   - Validator
//   - Builder
// ========================================================================================================

use crate::common::types::{AdminId, BuilderId, ValidatorId};
use crate::service::{admin, builder, validator};
use ic_kit::candid::candid_method;
use ic_kit::candid::CandidType;
use ic_kit::ic::caller;
use ic_kit::macros::init;
use serde::Deserialize;

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
