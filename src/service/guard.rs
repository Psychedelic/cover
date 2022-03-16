use crate::service::store::{admin, builder, validator};
use ic_cdk::caller;

pub fn is_admin() -> Result<(), String> {
    admin::admin_existed(&caller())
        .then(|| ())
        .ok_or_else(|| "Caller is not an authorized admin".into())
}

pub fn is_builder() -> Result<(), String> {
    builder::builder_existed(&caller())
        .then(|| ())
        .ok_or_else(|| "Caller is not an authorized builder".into())
}

pub fn is_validator() -> Result<(), String> {
    validator::validator_existed(&caller())
        .then(|| ())
        .ok_or_else(|| "Caller is not an authorized validator".into())
}
