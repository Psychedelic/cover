use crate::common::types::{CallerId, CanisterId};
use crate::service::time_utils;
use crate::service::types::{UpdateVerification, Verification};
use std::collections::BTreeMap;

pub struct VerificationStore {
    verification: BTreeMap<CanisterId, Verification>,
}

impl Default for VerificationStore {
    fn default() -> Self {
        Self {
            verification: BTreeMap::default(),
        }
    }
}

impl VerificationStore {
    pub fn update_verification(
        &mut self,
        caller_id: CallerId,
        update_verification: UpdateVerification,
    ) {
        let now = time_utils::now_to_str();
        match self.verification.get_mut(&update_verification.canister_id) {
            Some(verification) => {
                verification.git_checksum = update_verification.git_checksum;
                verification.canister_checksum = update_verification.canister_checksum;
                verification.wasm_checksum = update_verification.wasm_checksum;
                verification.build_log_url = update_verification.build_log_url;
                verification.source_snapshot_url = update_verification.source_snapshot_url;
                verification.updated_by = caller_id;
                verification.updated_at = now;
            }
            None => {
                let verification = Verification {
                    canister_id: update_verification.canister_id,
                    git_checksum: update_verification.git_checksum,
                    canister_checksum: update_verification.canister_checksum,
                    wasm_checksum: update_verification.wasm_checksum,
                    build_log_url: update_verification.build_log_url,
                    source_snapshot_url: update_verification.source_snapshot_url,
                    created_by: caller_id,
                    created_at: now.clone(),
                    updated_by: caller_id,
                    updated_at: now,
                };
                self.verification
                    .insert(update_verification.canister_id, verification);
            }
        }
    }

    pub fn get_verification_by_canister_id(&self, canister_id: &CanisterId) -> Option<&Verification> {
        self.verification.get(canister_id)
    }

    pub fn get_all_verification(&self) -> Vec<&Verification> {
        self.verification.iter().map(|(_, v)| v).collect()
    }
}
