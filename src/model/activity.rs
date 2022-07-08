use crate::common::types::CanisterId;
use crate::model::verification::BuildStatus;
use ic_cdk::export::candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct Activity {
    pub canister_id: CanisterId,
    pub build_status: BuildStatus,
    pub create_at: String,
}
