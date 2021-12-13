use ic_kit::mock_principals;

use crate::service::model::build_config::BuildConfig;
use crate::service::store::test_data::{fake_canister1, fake_canister2, fake_canister3};
use crate::service::time_utils;

pub fn fake_config1() -> BuildConfig {
    BuildConfig {
        user_id: mock_principals::alice(),
        canister_id: fake_canister1(),
        canister_name: "Canister1".into(),
        repo_url: "https://github.com/something1".into(),
        user_repo_token: "kfskfn".into(),
        commit_hash: "12234".into(),
        rust_version: "1.2.3".into(),
        dfx_version: "0.8.3".into(),
        optimize_times: 0,
        created_at: time_utils::now_to_str(),
        updated_at: time_utils::now_to_str(),
    }
}

pub fn fake_config2() -> BuildConfig {
    BuildConfig {
        user_id: mock_principals::bob(),
        canister_id: fake_canister2(),
        canister_name: "Canister2".into(),
        repo_url: "https://github.com/something2".into(),
        user_repo_token: "xvddd".into(),
        commit_hash: "1345".into(),
        rust_version: "".into(),
        dfx_version: "0.8.3".into(),
        optimize_times: 2,
        created_at: time_utils::now_to_str(),
        updated_at: time_utils::now_to_str(),
    }
}

pub fn fake_config3() -> BuildConfig {
    BuildConfig {
        user_id: mock_principals::john(),
        canister_id: fake_canister1(),
        canister_name: "Canister1".into(),
        repo_url: "https://github.com/something1".into(),
        user_repo_token: "fsfbb".into(),
        commit_hash: "12234".into(),
        rust_version: "1.2.3".into(),
        dfx_version: "0.8.3".into(),
        optimize_times: 1,
        created_at: time_utils::now_to_str(),
        updated_at: time_utils::now_to_str(),
    }
}

pub fn fake_config4() -> BuildConfig {
    BuildConfig {
        user_id: mock_principals::alice(),
        canister_id: fake_canister3(),
        canister_name: "Canister3".into(),
        repo_url: "https://github.com/something3".into(),
        user_repo_token: "tryryh".into(),
        commit_hash: "12234".into(),
        rust_version: "1.2.3".into(),
        dfx_version: "0.8.3".into(),
        optimize_times: 4,
        created_at: time_utils::now_to_str(),
        updated_at: time_utils::now_to_str(),
    }
}
