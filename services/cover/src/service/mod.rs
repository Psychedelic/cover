use ic_kit::ic::{get, get_mut};

use crate::service::store::registry::ValidationsRegistry;

pub mod cover_service;
pub mod types;

mod store;

#[inline]
fn get_validation_registry_mut() -> &'static mut ValidationsRegistry {
    get_mut()
}

#[inline]
fn get_validation_registry() -> &'static ValidationsRegistry {
    get()
}
