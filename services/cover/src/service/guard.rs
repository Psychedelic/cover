use crate::common::types::ProviderId;
use crate::service::error_handler::ErrorKindService;
use crate::service::types::Error;

use super::get_provider_store;

pub fn is_valid_provider<T, F: FnOnce() -> Result<T, Error>>(
    provider_id: &ProviderId,
    f: F,
) -> Result<T, Error> {
    get_provider_store()
        .is_provider_exists(provider_id)
        .then(|| f())
        .unwrap_or_else(|| Err(ErrorKindService::InvalidProvider.into()))
}
