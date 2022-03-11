use ic_cdk::export::candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize, PartialEq, Debug)]
pub enum Error {
    BuildInProgress,
}
