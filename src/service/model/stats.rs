use ic_kit::candid::CandidType;

#[derive(CandidType, Debug, PartialEq)]
pub struct Stats {
    pub total_canisters: usize,
    pub motoko_canisters_count: usize,
    pub rust_canisters_count: usize,
    pub build_pending_count: usize,
    pub build_in_progress_count: usize,
    pub build_error_count: usize,
    pub build_success_count: usize,
}