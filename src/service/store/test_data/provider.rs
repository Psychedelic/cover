use crate::common::types::CallerId;
use crate::service::model::provider::{AddProvider, Provider};
use crate::service::time_utils;

pub fn fake_provider(caller_id: &CallerId, new_provider: AddProvider) -> Provider {
    Provider {
        id: new_provider.id,
        name: new_provider.name,
        memo: new_provider.memo,
        created_by: *caller_id,
        created_at: time_utils::now_to_str(),
        updated_by: *caller_id,
        updated_at: time_utils::now_to_str(),
    }
}
