use ic_kit::candid::candid_method;
use ic_kit::macros::{query, update};

use crate::common::types::AdminId;
use crate::service::admin;
use crate::service::guard::is_authorized;
use crate::service::model::error::Error;

#[query(name = "adminExisted", guard = "is_authorized")]
#[candid_method(query, rename = "adminExisted")]
fn admin_existed(admin_id: AdminId) -> bool {
    admin::admin_existed(&admin_id)
}

#[update(name = "addAdmin", guard = "is_authorized")]
#[candid_method(update, rename = "addAdmin")]
fn add_admin(admin_id: AdminId) -> Result<(), Error> {
    admin::add_admin(&admin_id)
}

#[update(name = "deleteAdmin", guard = "is_authorized")]
#[candid_method(update, rename = "deleteAdmin")]
fn delete_admin(admin_id: AdminId) -> Result<(), Error> {
    admin::delete_admin(&admin_id)
}

#[query(name = "getAllAdmins", guard = "is_authorized")]
#[candid_method(query, rename = "getAllAdmins")]
fn get_all_admins() -> Vec<&'static AdminId> {
    admin::get_all_admins()
}

#[cfg(test)]
mod tests {
    use ic_kit::*;

    use super::*;

    fn init_test_data() {
        MockContext::new()
            .with_caller(mock_principals::john())
            .inject();

        assert_eq!(add_admin(mock_principals::bob()), Ok(()));
    }

    #[test]
    fn add_admin_ok() {
        init_test_data();
        assert_eq!(get_all_admins().len(), 1);

        assert_eq!(add_admin(mock_principals::john()), Ok(()));

        assert_eq!(get_all_admins().len(), 2);

        assert_eq!(
            add_admin(mock_principals::john()),
            Err(Error {
                code: "ERR_0056_003_002",
                message: "Existed admin",
                debug_log: None,
            })
        );
        assert_eq!(get_all_admins().len(), 2);
    }

    #[test]
    fn delete_admin_ok() {
        init_test_data();

        assert_eq!(get_all_admins().len(), 1);

        assert_eq!(delete_admin(mock_principals::bob()), Ok(()));

        assert_eq!(get_all_admins().len(), 0);

        assert_eq!(
            delete_admin(mock_principals::bob()),
            Err(Error {
                code: "ERR_006_003_001",
                message: "Admin not found",
                debug_log: None,
            })
        );
    }

    #[test]
    fn get_all_admins_ok() {
        init_test_data();

        assert_eq!(get_all_admins().len(), 1);
    }

    #[test]
    fn admin_existed_ok() {
        init_test_data();

        assert_eq!(admin_existed(mock_principals::bob()), true);

        assert_eq!(admin_existed(mock_principals::john()), false);
    }
}
