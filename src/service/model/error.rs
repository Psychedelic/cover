use ic_kit::candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug, PartialEq)]
pub struct Error {
    pub code: &'static str,
    pub message: &'static str,
    pub debug_log: Option<String>,
}
