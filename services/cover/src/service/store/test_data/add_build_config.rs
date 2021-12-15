use crate::service::model::build_config::AddBuildConfig;
use crate::service::store::test_data::{fake_canister1, fake_canister2, fake_canister3};

pub fn fake_add_build_config1() -> AddBuildConfig {
    AddBuildConfig {
        canister_id: fake_canister1(),
        canister_name: "Canister1".into(),
        repo_url: "https://github.com/something1".into(),
        user_repo_token: "thisistoken1".into(),
        commit_hash: "1234".into(),
        rust_version: "1.2.3".into(),
        dfx_version: "0.8.4".into(),
        optimize_times: 0,
    }
}

pub fn fake_add_build_config2() -> AddBuildConfig {
    AddBuildConfig {
        canister_id: fake_canister2(),
        canister_name: "Canister2".into(),
        repo_url: "https://github.com/something2".into(),
        user_repo_token: "thisistoke2".into(),
        commit_hash: "2345".into(),
        rust_version: "".into(),
        dfx_version: "0.8.4".into(),
        optimize_times: 2,
    }
}

pub fn fake_add_build_config3() -> AddBuildConfig {
    AddBuildConfig {
        canister_id: fake_canister3(),
        canister_name: "Canister3".into(),
        repo_url: "https://github.com/something3".into(),
        user_repo_token: "thisistoken3".into(),
        commit_hash: "3456".into(),
        rust_version: "5.0.0".into(),
        dfx_version: "0.8.4".into(),
        optimize_times: 5,
    }
}
