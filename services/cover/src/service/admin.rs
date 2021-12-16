use crate::common::types::AdminId;
use crate::service::model::error::Error;
use crate::service::{admin_store, admin_store_mut};

pub fn admin_existed(admin_id: &AdminId) -> bool {
    admin_store().admin_existed(admin_id)
}

pub fn add_admin(admin_id: &AdminId) -> Result<(), Error> {
    admin_store_mut().add_admin(admin_id).map_err(|e| e.into())
}

pub fn delete_admin(admin_id: &AdminId) -> Result<(), Error> {
    admin_store_mut()
        .delete_admin(admin_id)
        .map_err(|e| e.into())
}

pub fn get_all_admins() -> Vec<&'static AdminId> {
    admin_store().get_all_admins()
}
