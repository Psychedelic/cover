use crate::service::model::build_config::UpdateBuildConfig;

pub fn fake_update_build_config1() -> UpdateBuildConfig {
    UpdateBuildConfig {
        canister_name: "AnotherName1".into(),
        repo_url: "https://github.com/another1".into(),
        user_repo_token: "thisistoken1".into(),
        commit_hash: "123444".into(),
        rust_version: "1.2.3".into(),
        dfx_version: "0.8.4".into(),
        optimize_times: 0,
    }
}

pub fn fake_update_build_config2() -> UpdateBuildConfig {
    UpdateBuildConfig {
        canister_name: "AnotherName2".into(),
        repo_url: "https://github.com/another2".into(),
        user_repo_token: "thisistoken2".into(),
        commit_hash: "234555".into(),
        rust_version: "1.2.3".into(),
        dfx_version: "0.8.4".into(),
        optimize_times: 2,
    }
}

pub fn fake_update_build_config3() -> UpdateBuildConfig {
    UpdateBuildConfig {
        canister_name: "AnotherName3".into(),
        repo_url: "https://github.com/another3".into(),
        user_repo_token: "thisistoken3".into(),
        commit_hash: "345666".into(),
        rust_version: "1.2.3".into(),
        dfx_version: "0.8.4".into(),
        optimize_times: 3,
    }
}
