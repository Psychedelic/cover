use crate::common::types::{CallerId, ProviderId};
use crate::service::guard::is_cover_owner;
use crate::service::model::error::Error;
use crate::service::model::provider::{AddProvider, Provider, UpdateProvider};
use crate::service::{provider_store, provider_store_mut};

pub async fn add_provider(caller_id: CallerId, add_provider: AddProvider) -> Result<(), Error> {
    is_cover_owner(&caller_id, || {
        provider_store_mut()
            .add_provider(caller_id, add_provider)
            .map_err(|e| e.into())
    })
    .await
}

pub async fn update_provider(
    caller_id: CallerId,
    update_provider: UpdateProvider,
) -> Result<(), Error> {
    is_cover_owner(&caller_id, || {
        provider_store_mut()
            .update_provider(caller_id, update_provider)
            .map_err(|e| e.into())
    })
    .await
}

pub async fn delete_provider(caller_id: &CallerId, provider_id: &ProviderId) -> Result<(), Error> {
    is_cover_owner(caller_id, || {
        provider_store_mut()
            .delete_provider(provider_id)
            .map_err(|e| e.into())
    })
    .await
}

pub fn get_provider_by_id(provider_id: &ProviderId) -> Option<&'static Provider> {
    provider_store().get_provider_by_id(provider_id)
}

pub fn get_all_providers() -> Vec<&'static Provider> {
    provider_store().get_all_providers()
}