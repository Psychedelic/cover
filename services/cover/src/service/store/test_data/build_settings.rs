use crate::service::types::BuildSettings;

pub fn fake_build_settings1() -> BuildSettings {
    BuildSettings {
        git_ref: "dummy git_ref1".to_string(),
        git_tag: "dummy git_tag1".to_string(),
    }
}

pub fn fake_build_settings2() -> BuildSettings {
    BuildSettings {
        git_ref: "dummy git_ref2".to_string(),
        git_tag: "dummy git_tag2".to_string(),
    }
}
