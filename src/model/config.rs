use ic_cdk::export::candid::CandidType;
use serde::Deserialize;

use crate::common::types::{AdminId, BuilderId, ValidatorId};

#[derive(CandidType, Deserialize)]
pub struct Config {
    pub admin: Option<Vec<AdminId>>,
    pub validator: Option<Vec<ValidatorId>>,
    pub builder: Option<Vec<BuilderId>>,
}
