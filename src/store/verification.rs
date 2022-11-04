use std::collections::HashMap;

use ic_cdk::api::call::ManualReply;
use ic_cdk::api::time;
use ic_cdk::export::candid::CandidType;
use serde::Deserialize;

use crate::common::types::CallerId;
use crate::common::types::CanisterId;
use crate::model::error::Error;
use crate::model::pagination::{Pagination, PaginationInfo};
use crate::model::stats::Stats;
use crate::model::verification::{
    BuildStatus, CanisterType, RegisterVerification, SubmitVerification, Verification,
};
use crate::util::pagination::total_pages;

use super::MY_STATS_STORE;
use super::STATS_STORE;
use super::VERIFICATION_STORE;

#[derive(CandidType, Deserialize, Default)]
pub struct VerificationStore {
    verifications: HashMap<CanisterId, Verification>,
    records: Vec<CanisterId>,
}

#[derive(CandidType, Deserialize, Default)]
pub struct StatsStore {
    stats: Stats,
}

#[derive(CandidType, Deserialize, Default)]
pub struct MyStatsStore {
    my_stats: HashMap<CallerId, Stats>,
}

pub fn submit_verification<F: Fn(CanisterId, CallerId, BuildStatus)>(
    verification: SubmitVerification,
    activity_handler: F,
) {
    VERIFICATION_STORE.with(|store| {
        let mut store_ref_mut = store.borrow_mut();
        let old_verification = store_ref_mut.verifications.insert(
            verification.canister_id,
            Verification {
                delegate_canister_id: verification.delegate_canister_id,
                canister_id: verification.canister_id,
                canister_name: verification.canister_name,
                repo_url: verification.repo_url,
                commit_hash: verification.commit_hash,
                wasm_hash: verification.wasm_hash,
                build_url: Some(verification.build_url),
                build_status: verification.build_status,
                rust_version: verification.rust_version,
                canister_type: verification.canister_type,
                dfx_version: verification.dfx_version,
                optimize_count: verification.optimize_count,
                repo_visibility: verification.repo_visibility,
                updated_by: verification.caller_id,
                updated_at: time(),
            },
        );
        calculate_stats(
            old_verification.as_ref(),
            &VerificationStats {
                build_status: verification.build_status,
                canister_type: verification.canister_type,
                updated_by: verification.caller_id,
            },
        );
        activity_handler(
            verification.canister_id,
            verification.caller_id,
            verification.build_status,
        );
    })
}

pub fn get_verification_by_canister_id<
    F: Fn(Option<&Verification>) -> ManualReply<Option<Verification>>,
>(
    canister_id: &CanisterId,
    manual_reply: F,
) -> ManualReply<Option<Verification>> {
    VERIFICATION_STORE.with(|store| manual_reply(store.borrow().verifications.get(canister_id)))
}

pub fn get_verifications<
    F: Fn(Pagination<&Verification>) -> ManualReply<Pagination<Verification>>,
>(
    pagination_info: &PaginationInfo,
    manual_reply: F,
) -> ManualReply<Pagination<Verification>> {
    VERIFICATION_STORE.with(|store| {
        let store_ref = store.borrow();
        let total_items = store_ref.records.len() as u64;
        let total_pages = total_pages(total_items, pagination_info.items_per_page);

        let mut data: Vec<&Verification> = vec![];
        if pagination_info.page_index > 0 && pagination_info.page_index <= total_pages {
            // check if last page
            let data_length = match pagination_info.page_index == total_pages {
                true => total_items - (pagination_info.items_per_page * (total_pages - 1)),
                false => pagination_info.items_per_page,
            };

            // calculate where the pagination should start and end
            let start = (total_items
                - (pagination_info.page_index - 1) * pagination_info.items_per_page)
                as usize;
            let end = start - data_length as usize;

            // because latest items will get appended to the end of 'records'
            // so in order to get latest data first, we'll iterate 'records' and push verification into 'data' in reverse order
            // end will be included and start will be excluded
            for i in (end..start).rev() {
                data.push(&store_ref.verifications[&store_ref.records[i]])
            }
        }

        manual_reply(Pagination::of(data, pagination_info, total_items))
    })
}

pub fn register_verification<F: Fn(CanisterId, CallerId, BuildStatus)>(
    register_verification: RegisterVerification,
    activity_handler: F,
) -> Result<(), Error> {
    VERIFICATION_STORE.with(|store| {
        let mut store_ref_mut = store.borrow_mut();
        let canister_id = register_verification.canister_id;
        store_ref_mut
            .verifications
            .get_mut(&register_verification.canister_id)
            .map(|verification| match verification.build_status {
                BuildStatus::Pending | BuildStatus::Building => {
                    //user have to wait 5 minutes until next register
                    let nanosec_from_last_update = time() - verification.updated_at;
                    if nanosec_from_last_update > 300_000_000_000 {
                        Ok(())
                    } else {
                        Err(Error::BuildInProgress)
                    }
                }
                BuildStatus::Error | BuildStatus::Success => Ok(()),
            })
            .unwrap_or_else(|| Ok(()))
            .map(|_| {
                let old_verification = store_ref_mut.verifications.insert(
                    register_verification.canister_id,
                    Verification {
                        delegate_canister_id: register_verification.delegate_canister_id,
                        canister_id: register_verification.canister_id,
                        canister_name: register_verification.canister_name,
                        repo_url: register_verification.repo_url,
                        commit_hash: register_verification.commit_hash,
                        wasm_hash: None,
                        build_url: None,
                        build_status: BuildStatus::Pending,
                        canister_type: None,
                        rust_version: register_verification.rust_version,
                        dfx_version: register_verification.dfx_version,
                        optimize_count: register_verification.optimize_count,
                        repo_visibility: register_verification.repo_visibility,
                        updated_by: register_verification.caller_id,
                        updated_at: time(),
                    },
                );
                calculate_stats(
                    old_verification.as_ref(),
                    &VerificationStats {
                        build_status: BuildStatus::Pending,
                        canister_type: None,
                        updated_by: register_verification.caller_id,
                    },
                );
                old_verification
                    .is_none()
                    .then(|| store_ref_mut.records.push(canister_id));
                activity_handler(
                    canister_id,
                    register_verification.caller_id,
                    BuildStatus::Pending,
                )
            })
    })
}

pub fn get_verification_stats<F: Fn(&Stats) -> ManualReply<Stats>>(
    manual_reply: F,
) -> ManualReply<Stats> {
    STATS_STORE.with(|store| manual_reply(&store.borrow().stats))
}

pub fn get_my_verification_stats<F: Fn(&Stats) -> ManualReply<Stats>>(
    caller_id: CallerId,
    manual_reply: F,
) -> ManualReply<Stats> {
    let default_stats = Stats::default();
    MY_STATS_STORE.with(|store| {
        manual_reply(
            store
                .borrow()
                .my_stats
                .get(&caller_id)
                .unwrap_or(&default_stats),
        )
    })
}

struct VerificationStats {
    canister_type: Option<CanisterType>,
    build_status: BuildStatus,
    updated_by: CallerId,
}

fn calculate_stats(old: Option<&Verification>, new: &VerificationStats) {
    STATS_STORE.with(|store| {
        let mut store_ref_mut = store.borrow_mut();

        // remove old verification stats
        if let Some(old) = old {
            if let Some(canister_type) = old.canister_type {
                match canister_type {
                    CanisterType::Rust => store_ref_mut.stats.rust_canisters_count -= 1,
                    CanisterType::Motoko => store_ref_mut.stats.motoko_canisters_count -= 1,
                    CanisterType::Custom => store_ref_mut.stats.custom_canisters_count -= 1,
                    CanisterType::Assets => store_ref_mut.stats.assets_canisters_count -= 1,
                }
            } else {
                store_ref_mut.stats.unknown_canisters_count -= 1;
            }
            match old.build_status {
                BuildStatus::Pending => store_ref_mut.stats.build_pending_count -= 1,
                BuildStatus::Building => store_ref_mut.stats.build_in_progress_count -= 1,
                BuildStatus::Error => store_ref_mut.stats.build_error_count -= 1,
                BuildStatus::Success => store_ref_mut.stats.build_success_count -= 1,
            };
            store_ref_mut.stats.total_canisters -= 1;
        }

        // update new verification stats
        if let Some(canister_type) = new.canister_type {
            match canister_type {
                CanisterType::Rust => store_ref_mut.stats.rust_canisters_count += 1,
                CanisterType::Motoko => store_ref_mut.stats.motoko_canisters_count += 1,
                CanisterType::Custom => store_ref_mut.stats.custom_canisters_count += 1,
                CanisterType::Assets => store_ref_mut.stats.assets_canisters_count += 1,
            }
        } else {
            store_ref_mut.stats.unknown_canisters_count += 1;
        }
        match new.build_status {
            BuildStatus::Pending => store_ref_mut.stats.build_pending_count += 1,
            BuildStatus::Building => store_ref_mut.stats.build_in_progress_count += 1,
            BuildStatus::Error => store_ref_mut.stats.build_error_count += 1,
            BuildStatus::Success => store_ref_mut.stats.build_success_count += 1,
        };
        store_ref_mut.stats.total_canisters += 1;
    });

    MY_STATS_STORE.with(|store| {
        let mut store_ref_mut = store.borrow_mut();

        // remove old verification stats
        if let Some(old) = old {
            let old_stats = store_ref_mut
                .my_stats
                .get_mut(&old.updated_by)
                .unwrap_or_else(|| panic!("No stats found for: {:?}", old.updated_by.to_text()));

            if let Some(canister_type) = old.canister_type {
                match canister_type {
                    CanisterType::Rust => old_stats.rust_canisters_count -= 1,
                    CanisterType::Motoko => old_stats.motoko_canisters_count -= 1,
                    CanisterType::Custom => old_stats.custom_canisters_count -= 1,
                    CanisterType::Assets => old_stats.assets_canisters_count -= 1,
                }
            } else {
                old_stats.unknown_canisters_count -= 1;
            }
            match old.build_status {
                BuildStatus::Pending => old_stats.build_pending_count -= 1,
                BuildStatus::Building => old_stats.build_in_progress_count -= 1,
                BuildStatus::Error => old_stats.build_error_count -= 1,
                BuildStatus::Success => old_stats.build_success_count -= 1,
            };
            old_stats.total_canisters -= 1;
        }

        let new_stats = store_ref_mut
            .my_stats
            .entry(new.updated_by)
            .or_insert_with(Stats::default);

        // update new verification stats
        if let Some(canister_type) = new.canister_type {
            match canister_type {
                CanisterType::Rust => new_stats.rust_canisters_count += 1,
                CanisterType::Motoko => new_stats.motoko_canisters_count += 1,
                CanisterType::Custom => new_stats.custom_canisters_count += 1,
                CanisterType::Assets => new_stats.assets_canisters_count += 1,
            }
        } else {
            new_stats.unknown_canisters_count += 1;
        }
        match new.build_status {
            BuildStatus::Pending => new_stats.build_pending_count += 1,
            BuildStatus::Building => new_stats.build_in_progress_count += 1,
            BuildStatus::Error => new_stats.build_error_count += 1,
            BuildStatus::Success => new_stats.build_success_count += 1,
        };
        new_stats.total_canisters += 1;
    })
}
