use ic_cdk::caller;

use crate::store::{admin, builder, validator};

pub fn is_admin() -> Result<(), String> {
    admin::admin_existed(&caller())
        .then_some(())
        .ok_or_else(|| "Caller is not an authorized admin".into())
}

pub fn is_builder() -> Result<(), String> {
    builder::builder_existed(&caller())
        .then_some(())
        .ok_or_else(|| "Caller is not an authorized builder".into())
}

pub fn is_validator() -> Result<(), String> {
    validator::validator_existed(&caller())
        .then_some(())
        .ok_or_else(|| "Caller is not an authorized validator".into())
}
