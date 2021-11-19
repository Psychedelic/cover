use crate::service::types::BuildSettings;

pub fn fake_build_settings1() -> BuildSettings {
    BuildSettings {
        git_ref: "dummy git_ref1".to_string(),
        git_repo: "user/repo1".to_string(),
        git_sha: "dummyChecksum1".to_string(),
    }
}

pub fn fake_build_settings2() -> BuildSettings {
    BuildSettings {
        git_ref: "dummy git_ref2".to_string(),
        git_repo: "user/repo2".to_string(),
        git_sha: "dummy checskum2".to_string(),
    }
}
