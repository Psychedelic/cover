use crate::service::model::provider::{AddProvider, Provider, UpdateProvider};
use crate::service::time_utils;

pub fn fake_provider_use_update_model(updated_provider: UpdateProvider) -> Provider {
    Provider {
        id: updated_provider.id,
        name: updated_provider.name,
        memo: updated_provider.memo,
        created_by: updated_provider.id,
        created_at: time_utils::now_to_str(),
        updated_by: updated_provider.id,
        updated_at: time_utils::now_to_str(),
    }
}

pub fn fake_provider_use_add_model(new_provider: AddProvider) -> Provider {
    Provider {
        id: new_provider.id,
        name: new_provider.name,
        memo: new_provider.memo,
        created_by: new_provider.id,
        created_at: time_utils::now_to_str(),
        updated_by: new_provider.id,
        updated_at: time_utils::now_to_str(),
    }
}
