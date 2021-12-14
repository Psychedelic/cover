#[cfg(not(feature = "local_replica"))]
use ic_kit::ic::{call, id};
#[cfg(not(feature = "local_replica"))]
use ic_kit::interfaces::management::CanisterStatusResponse;

#[cfg(not(feature = "local_replica"))]
use crate::common::types::CanisterId;
use crate::common::types::{CallerId, ProviderId};
#[cfg(not(feature = "local_replica"))]
use crate::service::error_handler::ErrorKindApi;
use crate::service::error_handler::ErrorKindService;
use crate::service::model::error::Error;

use super::provider_store;

pub fn is_valid_provider<T, F: FnOnce() -> Result<T, Error>>(
    provider_id: &ProviderId,
    f: F,
) -> Result<T, Error> {
    provider_store()
        .provider_exists(provider_id)
        .then(f)
        .unwrap_or_else(|| Err(ErrorKindService::InvalidProvider.into()))
}

// FOR LOCAL TESTING
// BLACKHOLE_CANISTER_ID: "rrkah-fqaaa-aaaaa-aaaaq-cai"
// #[cfg(not(feature = "local_replica"))]
// const BLACKHOLE_CANISTER_ID: CanisterId = CanisterId::from_slice(&[0, 0, 0, 0, 0, 0, 0, 1, 1, 1]);

/// Fleek's IC Blackhole canister id: "s4jec-wiaaa-aaaah-qch4q-cai"
#[cfg(not(feature = "local_replica"))]
const BLACKHOLE_CANISTER_ID: CanisterId =
    CanisterId::from_slice(&[0, 0, 0, 0, 0, 240, 17, 249, 1, 1]);

#[cfg(not(feature = "local_replica"))]
pub async fn is_cover_owner<T, F: FnOnce() -> Result<T, Error>>(
    caller_id: &CallerId,
    f: F,
) -> Result<T, Error> {
    call(BLACKHOLE_CANISTER_ID, "canister_status", (id(),))
        .await
        .map(|(c,): (Result<CanisterStatusResponse, String>,)| {
            c.map(|s| {
                s.settings
                    .controllers
                    .contains(caller_id)
                    .then(f)
                    .unwrap_or_else(|| Err(ErrorKindService::InvalidCoverController.into()))
            })
            .unwrap_or_else(|e| Err(ErrorKindApi::BlackholeCanisterStatus(e).into()))
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
