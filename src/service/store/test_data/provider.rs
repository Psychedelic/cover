use ic_kit::mock_principals;

use crate::service::model::provider::Provider;
use crate::service::time_utils;

pub fn fake_provider1() -> Provider {
    Provider {
        id: mock_principals::bob(),
        name: "name1".into(),
        memo: Some("memo1".into()),
        created_by: mock_principals::bob(),
        created_at: time_utils::now_to_str(),
        updated_by: mock_principals::bob(),
        updated_at: time_utils::now_to_str(),
    }
}
