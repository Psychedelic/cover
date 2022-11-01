use crate::common::types::{CallerId, CanisterId};
use crate::model::verification::BuildStatus;
use ic_cdk::export::candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct Activity {
    pub canister_id: CanisterId,
    pub build_status: BuildStatus,
    pub created_at: u64, // Current system time given as nanoseconds since 1970-01-01
}

#[derive(CandidType, Deserialize)]
pub enum MyBuildConfigActivity {
    Save,
    Delete,
}

#[derive(CandidType, Deserialize)]
pub struct MyActivity {
    pub canister_id: CanisterId,
    pub caller_id: CallerId,
    pub build_status: Option<BuildStatus>,
    pub build_config_status: Option<MyBuildConfigActivity>,
    pub created_at: u64, // Current system time given as nanoseconds since 1970-01-01
}
