use crate::common::types::AdminId;
use crate::service::{admin_store, admin_store_mut};

pub fn add_admin(admin_id: &AdminId) {
    admin_store_mut().add_admin(admin_id)
}

pub fn delete_admin(admin_id: &AdminId) {
    admin_store_mut().delete_admin(admin_id)
}

pub fn get_admins() -> Vec<&'static AdminId> {
    admin_store().get_admins()
}
