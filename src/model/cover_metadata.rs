use ic_cdk::export::candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct CoverMetadata {
    pub canister_name: &'static str,
    pub repo_url: &'static str,
    pub commit_hash: &'static str,
    pub rust_version: Option<&'static str>,
    pub dfx_version: &'static str,
    pub optimize_count: u8,
    pub controller: Option<&'static str>, // "myActivity" indexing
}
