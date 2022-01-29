use ic_kit::candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize, PartialEq, Debug)]
pub enum Error {
    BuildInProgress,
}
