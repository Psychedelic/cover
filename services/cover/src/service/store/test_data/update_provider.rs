use crate::common::types::ProviderId;
use crate::service::types::UpdateProvider;

pub fn fake_update_provider1(id: ProviderId) -> UpdateProvider {
    UpdateProvider {
        id,
        name: "update name1".into(),
        memo: "update memo1".into(),
    }
}

pub fn fake_update_provider2(id: ProviderId) -> UpdateProvider {
    UpdateProvider {
        id,
        name: "update name2".into(),
        memo: "update memo2".into(),
    }
}

pub fn fake_update_provider3(id: ProviderId) -> UpdateProvider {
    UpdateProvider {
        id,
        name: "update name3".into(),
        memo: "update memo3".into(),
    }
}
