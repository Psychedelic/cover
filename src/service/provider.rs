use crate::common::types::{CallerId, ProviderId};
use crate::service::model::provider::{AddProvider, Provider};
use crate::service::{provider_store, provider_store_mut};

pub fn add_provider(caller_id: &CallerId, add_provider: AddProvider) {
    provider_store_mut().add_provider(caller_id, add_provider)
}

pub fn delete_provider(provider_id: &ProviderId) {
    provider_store_mut().delete_provider(provider_id)
}

pub fn get_provider_by_id(provider_id: &ProviderId) -> Option<&'static Provider> {
    provider_store().get_provider_by_id(provider_id)
}

pub fn get_all_providers() -> Vec<&'static Provider> {
    provider_store().get_all_providers()
}
