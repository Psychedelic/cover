mod build_settings;
mod canister;

pub use canister::*;

pub use build_settings::*;

use crate::common::types::ReqId;
use crate::service::store::registry::ValidationsRegistry;
use ic_kit::*;

pub fn fake_store_with_pending(offset: ReqId, size: usize) -> ValidationsRegistry {
    let mut store = ValidationsRegistry::default();
    store.req_counter = offset;
    store.last_consumed_req_idx = offset;
    for i in 0..size {
        store
            .add_request(
                if i % 2 == 0 {
                    mock_principals::bob()
                } else {
                    mock_principals::alice()
                },
                if i % 2 == 0 {
                    fake_canister1()
                } else {
                    fake_canister2()
                },
                if i % 2 == 0 {
                    fake_build_settings1()
                } else {
                    fake_build_settings2()
                },
            )
            .expect("Should add successfully");
    }
    store
}
