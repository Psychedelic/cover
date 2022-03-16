// use crate::common::types::{CanisterId, CanisterOwnerId};
// use crate::service::model::build_config::SaveBuildConfig;
//
// pub fn fake_save_build_config1(
//     owner_id: &CanisterOwnerId,
//     canister_id: &CanisterId,
// ) -> SaveBuildConfig {
//     SaveBuildConfig {
//         owner_id: *owner_id,
//         canister_id: *canister_id,
//         canister_name: "Canister1".into(),
//         repo_url: "https://github.com/something1".into(),
//         commit_hash: "1234".into(),
//         rust_version: Some("1.4.4".into()),
//         dfx_version: "0.8.4".into(),
//         optimize_count: 0,
//     }
// }
//
// pub fn fake_save_build_config2(
//     owner_id: &CanisterOwnerId,
//     canister_id: &CanisterId,
// ) -> SaveBuildConfig {
//     SaveBuildConfig {
//         owner_id: *owner_id,
//         canister_id: *canister_id,
//         canister_name: "Canister2".into(),
//         repo_url: "https://github.com/something2".into(),
//         commit_hash: "2345".into(),
//         rust_version: Some("1.2.45".into()),
//         dfx_version: "0.8.4".into(),
//         optimize_count: 2,
//     }
// }
//
// pub fn fake_save_build_config3(
//     owner_id: &CanisterOwnerId,
//     canister_id: &CanisterId,
// ) -> SaveBuildConfig {
//     SaveBuildConfig {
//         owner_id: *owner_id,
//         canister_id: *canister_id,
//         canister_name: "Canister3".into(),
//         repo_url: "https://github.com/something3".into(),
//         commit_hash: "3456".into(),
//         rust_version: Some("2.2.4".into()),
//         dfx_version: "0.8.4".into(),
//         optimize_count: 5,
//     }
// }
