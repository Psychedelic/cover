use crate::service::model::build_config::UpdateBuildConfig;

pub fn fake_update_build_config1() -> UpdateBuildConfig {
    UpdateBuildConfig {
        canister_name: "UpdatedName1".into(),
        repo_url: "https://github.com/Updated1".into(),
        user_repo_token: "thisisupdatedtoken1".into(),
        commit_hash: "123444".into(),
        rust_version: "1.2.3".into(),
        dfx_version: "0.8.4".into(),
        optimize_times: 0,
    }
}

pub fn fake_update_build_config2() -> UpdateBuildConfig {
    UpdateBuildConfig {
        canister_name: "UpdatedName2".into(),
        repo_url: "https://github.com/Updated2".into(),
        user_repo_token: "thisisupdatedtoken2".into(),
        commit_hash: "234555".into(),
        rust_version: "1.2.5".into(),
        dfx_version: "0.8.4".into(),
        optimize_times: 2,
    }
}
