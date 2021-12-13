use ic_kit::ic::caller;
use ic_kit::macros::{query, update};

use crate::common::types::ProviderId;
use crate::service::model::error::Error;
use crate::service::model::provider::{AddProvider, Provider, UpdateProvider};
use crate::service::provider;

#[update]
async fn add_provider(_add_provider: AddProvider) -> Result<(), Error> {
    provider::add_provider(caller(), _add_provider).await
}

#[update]
async fn update_provider(_update_provider: UpdateProvider) -> Result<(), Error> {
    provider::update_provider(caller(), _update_provider).await
}

#[update]
async fn delete_provider(provider_id: ProviderId) -> Result<(), Error> {
    provider::delete_provider(&caller(), &provider_id).await
}

#[query]
fn get_provider_by_id(provider_id: ProviderId) -> Option<&'static Provider> {
    provider::get_provider_by_id(&provider_id)
}

#[query]
fn get_all_providers() -> Vec<&'static Provider> {
    provider::get_all_providers()
}
