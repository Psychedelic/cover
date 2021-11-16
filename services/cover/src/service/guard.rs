use crate::common::types::{CallerId, ProviderId};
use crate::service::error_handler::ErrorKindService;
use crate::service::types::Error;

use super::get_provider_store;

#[cfg(not(feature = "local_replica"))]
use crate::common::types::CanisterId;
#[cfg(not(feature = "local_replica"))]
use crate::service::error_handler::ErrorKindApi;
#[cfg(not(feature = "local_replica"))]
use ic_kit::ic::id;

#[cfg(not(feature = "local_replica"))]
use ic_kit::interfaces::management::{CanisterStatus, WithCanisterId};
#[cfg(not(feature = "local_replica"))]
use ic_kit::interfaces::Method;

pub fn is_valid_provider<T, F: FnOnce() -> Result<T, Error>>(
    provider_id: &ProviderId,
    f: F,
) -> Result<T, Error> {
    get_provider_store()
        .is_provider_exists(provider_id)
        .then(|| f())
        .unwrap_or_else(|| Err(ErrorKindService::InvalidProvider.into()))
}

/// FIXME: Production Fleek's canister id
#[cfg(not(feature = "local_replica"))]
const BLACKHOLE_CANISTER_ID: CanisterId = CanisterId::from_slice(&[]);

#[cfg(not(feature = "local_replica"))]
pub async fn is_cover_owner<T, F: FnOnce() -> Result<T, Error>>(
    caller_id: &CallerId,
    f: F,
) -> Result<T, Error> {
    CanisterStatus::perform(
        BLACKHOLE_CANISTER_ID,
        (WithCanisterId { canister_id: id() },),
    )
    .await
    .map(|(c,)| {
        c.settings
            .controllers
            .contains(caller_id)
            .then(|| f())
            .unwrap_or_else(|| Err(ErrorKindService::InvalidCoverController.into()))
    })
    .unwrap_or_else(|(code, err)| Err(ErrorKindApi::InterCanister((code, err)).into()))
}

#[cfg(feature = "local_replica")]
pub async fn is_cover_owner<T, F: FnOnce() -> Result<T, Error>>(
    _caller_id: &CallerId,
    f: F,
) -> Result<T, Error> {
    f()
}
