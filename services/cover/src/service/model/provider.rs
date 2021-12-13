use ic_kit::candid::CandidType;
use serde::Deserialize;

use crate::common::types::{CallerId, ProviderId};

#[derive(CandidType, Deserialize)]
pub struct AddProvider {
    pub id: ProviderId,
    pub name: String,
    pub memo: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateProvider {
    pub id: ProviderId,
    pub name: String,
    pub memo: Option<String>,
}

#[derive(CandidType, Deserialize, PartialEq)]
pub struct Provider {
    pub id: ProviderId,
    pub name: String,
    pub memo: Option<String>,
    pub created_by: CallerId,
    pub created_at: String,
    pub updated_by: CallerId,
    pub updated_at: String,
}

// TODO: define details
#[derive(CandidType, Deserialize, Debug, PartialEq)]
pub struct ProviderInfo {}
