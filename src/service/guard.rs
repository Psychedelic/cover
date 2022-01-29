use crate::service::{admin_store, builder_store, validator_store};
use ic_kit::ic::caller;

pub fn is_admin() -> Result<(), String> {
    admin_store()
        .admin_existed(&caller())
        .then(|| ())
        .ok_or_else(|| "Caller is not an authorized admin".into())
}

pub fn is_builder() -> Result<(), String> {
    builder_store()
        .builder_existed(&caller())
        .then(|| ())
        .ok_or_else(|| "Caller is not an authorized builder".into())
}

pub fn is_validator() -> Result<(), String> {
    validator_store()
        .validator_existed(&caller())
        .then(|| ())
        .ok_or_else(|| "Caller is not an authorized validator".into())
}
