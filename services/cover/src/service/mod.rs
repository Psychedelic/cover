use ic_kit::ic::{get, get_mut};

use crate::service::store::progress::ProgressTracker;
use crate::service::store::request::ValidationsRegistry;

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

#[inline]
fn get_progress_tracker_mut() -> &'static mut ProgressTracker {
    get_mut()
}

#[inline]
fn get_progress_tracker() -> &'static ProgressTracker {
    get()
}
