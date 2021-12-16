use ic_kit::candid::candid_method;
use ic_kit::ic::caller;
use ic_kit::macros::{query, update};

use crate::common::types::ProviderId;
use crate::service::guard::is_admin;
use crate::service::model::error::Error;
use crate::service::model::provider::{AddProvider, Provider, UpdateProvider};
use crate::service::provider;

#[update(name = "addProvider", guard = "is_admin")]
#[candid_method(update, rename = "addProvider")]
fn add_provider(provider: AddProvider) -> Result<(), Error> {
    provider::add_provider(&caller(), provider)
}

#[update(name = "updateProvider", guard = "is_admin")]
#[candid_method(update, rename = "updateProvider")]
fn update_provider(provider: UpdateProvider) -> Result<(), Error> {
    provider::update_provider(&caller(), provider)
}

#[update(name = "deleteProvider", guard = "is_admin")]
#[candid_method(update, rename = "deleteProvider")]
fn delete_provider(provider_id: ProviderId) -> Result<(), Error> {
    provider::delete_provider(&provider_id)
}

#[query(name = "getProviderById", guard = "is_admin")]
#[candid_method(query, rename = "getProviderById")]
fn get_provider_by_id(provider_id: ProviderId) -> Option<&'static Provider> {
    provider::get_provider_by_id(&provider_id)
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

        assert_eq!(
            add_provider(fake_add_provider1(mock_principals::bob())),
            Ok(())
        );
    }

    #[test]
    fn add_provider_ok() {
        init_test_data();

        assert_eq!(get_all_providers().len(), 1);

        assert_eq!(
            add_provider(fake_add_provider1(mock_principals::alice())),
            Ok(())
        );

        assert_eq!(get_all_providers().len(), 2);

        assert_eq!(
            add_provider(fake_add_provider1(mock_principals::alice())),
            Err(Error {
                code: "ERR_004_003_002",
                message: "Existed provider",
                debug_log: None,
            })
        );

        assert_eq!(get_all_providers().len(), 2);

        assert_eq!(
            add_provider(fake_add_provider1(mock_principals::john())),
            Ok(())
        );

        assert_eq!(get_all_providers().len(), 3);
    }

    #[test]
    fn get_all_providers_ok() {
        init_test_data();

        assert_eq!(get_all_providers(), vec![&fake_provider1()]);
    }

    #[test]
    fn get_provider_by_id_ok() {
        init_test_data();

        assert_eq!(
            get_provider_by_id(mock_principals::bob()),
            Some(&fake_provider1())
        );

        assert_eq!(get_provider_by_id(mock_principals::alice()), None);
    }

    #[test]
    fn delete_provider_ok() {
        init_test_data();

        assert_eq!(get_all_providers().len(), 1);

        assert_eq!(delete_provider(mock_principals::bob()), Ok(()));

        assert_eq!(get_all_providers().len(), 0);

        assert_eq!(
            delete_provider(mock_principals::bob()),
            Err(Error {
                code: "ERR_004_003_001",
                message: "Provider not found",
                debug_log: None,
            })
        );

        assert_eq!(get_all_providers().len(), 0);
    }

    #[test]
    fn update_provider_ok() {
        init_test_data();

        assert_eq!(get_all_providers().len(), 1);

        assert_eq!(
            update_provider(fake_update_provider2(mock_principals::bob())),
            Ok(())
        );

        assert_eq!(
            get_provider_by_id(mock_principals::bob()).unwrap().name
                == fake_update_provider2(mock_principals::bob()).name,
            true
        );

        assert_eq!(get_all_providers().len(), 1);

        assert_eq!(
            update_provider(fake_update_provider2(mock_principals::alice())),
            Err(Error {
                code: "ERR_004_003_001",
                message: "Provider not found",
                debug_log: None,
            })
        );
    }
}
