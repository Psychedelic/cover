use ic_cdk::export::candid::CandidType;

#[derive(CandidType)]
pub struct Stats {
    pub total_canisters: usize,
    pub motoko_canisters_count: usize,
    pub rust_canisters_count: usize,
    pub custom_canisters_count: usize,
    pub build_pending_count: usize,
    pub build_in_progress_count: usize,
    pub build_error_count: usize,
    pub build_success_count: usize,
}
