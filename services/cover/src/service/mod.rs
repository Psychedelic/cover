use ic_kit::ic::{get, get_mut};

use crate::service::store::progress::ProgressStore;
use crate::service::store::provider::ProviderStore;
use crate::service::store::request::RequestStore;
use crate::service::store::verification::VerificationStore;

pub mod cover;
pub mod time_utils;
pub mod types;

mod store;

#[inline]
fn get_request_store_mut() -> &'static mut RequestStore {
    get_mut()
}

#[inline]
fn get_request_store_registry() -> &'static RequestStore {
    get()
}

#[inline]
fn get_progress_store_mut() -> &'static mut ProgressStore {
    get_mut()
}

#[inline]
fn get_progress_store() -> &'static ProgressStore {
    get()
}

#[inline]
fn get_verification_store_mut() -> &'static mut VerificationStore {
    get_mut()
}

#[inline]
fn get_verification_store() -> &'static VerificationStore {
    get()
}

#[inline]
fn get_provider_store_mut() -> &'static mut ProviderStore {
    get_mut()
}

#[inline]
fn get_provider_store() -> &'static ProviderStore {
    get()
}
