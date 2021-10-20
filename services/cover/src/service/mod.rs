pub mod cover_service;
pub mod constants;
pub mod types;
pub mod utils;

mod store;

#[cfg(test)]
pub use store::registry::{test, ValidationsRegistry as CanisterInternalStoreTest};

use crate::service::store::registry::ValidationsRegistry;
use ic_kit::ic::{get, get_mut};

#[inline]
fn get_validation_registry_mut() -> &'static mut ValidationsRegistry {
    get_mut()
}

#[inline]
fn get_validation_registry() -> &'static ValidationsRegistry {
    get()
}
