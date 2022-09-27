use crate::common::types::CanisterId;
use crate::model::verification::BuildStatus;
use ic_cdk::export::candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct Activity {
    pub canister_id: CanisterId,
    pub build_status: BuildStatus,
    pub created_at: u64, // Current system time given as nanoseconds since 1970-01-01
}
