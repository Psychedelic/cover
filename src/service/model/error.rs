use ic_cdk::export::candid::CandidType;

#[derive(CandidType)]
pub enum Error {
    BuildInProgress,
}
