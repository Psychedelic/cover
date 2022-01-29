use crate::common::types::CanisterId;
use crate::service::model::verification::BuildStatus;

use ic_kit::candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize, PartialEq, Debug)]
pub struct Activity {
    pub canister_id: CanisterId,
    pub build_status: BuildStatus,
    pub create_at: String,
}
