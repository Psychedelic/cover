use crate::common::types::ProviderId;
use crate::service::model::provider::AddProvider;

pub fn fake_add_provider1(id: &ProviderId) -> AddProvider {
    AddProvider {
        id: *id,
        name: "name1".into(),
        memo: Some("memo1".into()),
    }
}
