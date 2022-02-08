use crate::service::model::stats::Stats;
use crate::service::model::verification::{BuildStatus, CanisterType};
use crate::service::verification_store;

pub fn get_stats() -> Stats {
    let verifications = verification_store().get_all();
    let mut stats = Stats {
        total_canisters: verifications.len(),
        motoko_canisters_count: 0,
        rust_canisters_count: 0,
        build_pending_count: 0,
        build_in_progress_count: 0,
        build_error_count: 0,
        build_success_count: 0,
    };
    for v in verifications {
        if let Some(canister_type) = v.canister_type {
            match canister_type {
                CanisterType::Rust => stats.rust_canisters_count += 1,
                CanisterType::Motoko => stats.motoko_canisters_count += 1,
            }
        };

        match v.build_status {
            BuildStatus::Pending => stats.build_pending_count += 1,
            BuildStatus::Building => stats.build_in_progress_count += 1,
            BuildStatus::Error => stats.build_error_count += 1,
            BuildStatus::Success => stats.build_success_count += 1,
        };
    }

    stats
}
