use crate::common::types::BuilderId;
use crate::service::{builder_store, builder_store_mut};

pub fn add_builder(builder_id: &BuilderId) {
    builder_store_mut().add_builder(builder_id);
}

pub fn delete_builder(builder_id: &BuilderId) {
    builder_store_mut().delete_builder(builder_id)
}

pub fn get_all_builders() -> Vec<&'static BuilderId> {
    builder_store().get_all_builders()
}
