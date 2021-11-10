use crate::common::types::ProviderId;
use crate::service::types::AddProvider;

pub fn fake_add_provider1(id: ProviderId) -> AddProvider {
    AddProvider {
        id,
        name: "name1".into(),
        memo: "memo1".into(),
    }
}

pub fn fake_add_provider2(id: ProviderId) -> AddProvider {
    AddProvider {
        id,
        name: "name2".into(),
        memo: "memo2".into(),
    }
}

pub fn fake_add_provider3(id: ProviderId) -> AddProvider {
    AddProvider {
        id,
        name: "name3".into(),
        memo: "memo3".into(),
    }
}
