use crate::common::types::AdminId;
use crate::service::model::error::Error;
use crate::service::{admin_store, admin_store_mut};

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
