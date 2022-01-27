use ic_kit::candid::candid_method;
use ic_kit::ic::caller;
use ic_kit::macros::{query, update};

use crate::common::types::ProviderId;
use crate::service::guard::is_admin;
use crate::service::model::provider::{AddProvider, Provider};
use crate::service::provider;

#[update(name = "addProvider", guard = "is_admin")]
#[candid_method(update, rename = "addProvider")]
fn add_provider(provider: AddProvider) {
    provider::add_provider(&caller(), provider)
}

#[update(name = "deleteProvider", guard = "is_admin")]
#[candid_method(update, rename = "deleteProvider")]
fn delete_provider(provider_id: ProviderId) {
    provider::delete_provider(&provider_id)
}

#[query(name = "getAllProviders", guard = "is_admin")]
#[candid_method(query, rename = "getAllProviders")]
fn get_all_providers() -> Vec<&'static Provider> {
    provider::get_all_providers()
}

#[cfg(test)]
mod tests {
    use ic_kit::*;

    use crate::service::store::test_data::*;

    use super::*;

    fn init_test_data() {
        MockContext::new()
            .with_caller(mock_principals::bob())
            .inject();

        add_provider(fake_add_provider1(&mock_principals::alice()));
    }

    #[test]
    fn add_provider_ok() {
        init_test_data();

        get_all_providers_ok();

        add_provider(fake_add_provider1(&mock_principals::john()));

        assert_eq!(
            get_all_providers(),
            vec![
                &fake_provider(
                    &mock_principals::bob(),
                    fake_add_provider1(&mock_principals::john())
                ),
                &fake_provider(
                    &mock_principals::bob(),
                    fake_add_provider1(&mock_principals::alice())
                )
            ]
        );
    }

    #[test]
    fn get_all_providers_ok() {
        init_test_data();

        assert_eq!(
            get_all_providers(),
            vec![&fake_provider(
                &mock_principals::bob(),
                fake_add_provider1(&mock_principals::alice())
            )]
        );
    }

    #[test]
    fn delete_provider_ok() {
        init_test_data();

        get_all_providers_ok();

        delete_provider(mock_principals::alice());

        assert_eq!(get_all_providers().len(), 0);

        delete_provider(mock_principals::alice());

        assert_eq!(get_all_providers().len(), 0);
    }
}
