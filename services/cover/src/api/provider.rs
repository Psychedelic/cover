use ic_kit::candid::candid_method;
use ic_kit::ic::caller;
use ic_kit::macros::{query, update};

use crate::common::types::ProviderId;
use crate::service::model::error::Error;
use crate::service::model::provider::{AddProvider, Provider, UpdateProvider};
use crate::service::provider;

#[update(name = "addProvider")]
#[candid_method(update, rename = "addProvider")]
async fn add_provider(provider: AddProvider) -> Result<(), Error> {
    provider::add_provider(caller(), provider).await
}

#[update(name = "updateProvider")]
#[candid_method(update, rename = "updateProvider")]
async fn update_provider(provider: UpdateProvider) -> Result<(), Error> {
    provider::update_provider(caller(), provider).await
}

#[update(name = "deleteProvider")]
#[candid_method(update, rename = "deleteProvider")]
async fn delete_provider(provider_id: ProviderId) -> Result<(), Error> {
    provider::delete_provider(&caller(), &provider_id).await
}

#[query(name = "getProviderById")]
#[candid_method(query, rename = "getProviderById")]
fn get_provider_by_id(provider_id: ProviderId) -> Option<&'static Provider> {
    provider::get_provider_by_id(&provider_id)
}

#[query(name = "getAllProviders")]
#[candid_method(query, rename = "getAllProviders")]
fn get_all_providers() -> Vec<&'static Provider> {
    provider::get_all_providers()
}
