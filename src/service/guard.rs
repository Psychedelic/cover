use crate::service::admin_store;
use ic_cdk::caller;

use super::provider_store;

pub fn is_admin() -> Result<(), String> {
    admin_store()
        .admin_existed(&caller())
        .then(|| ())
        .ok_or_else(|| "Caller is not an authorized admin".into())
}

pub fn is_provider() -> Result<(), String> {
    provider_store()
        .provider_exists(&caller())
        .then(|| ())
        .ok_or_else(|| "Caller is not an authorized provider".into())
}
