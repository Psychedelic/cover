use std::collections::BTreeMap;
use std::ops::Not;

use ic_kit::candid::CandidType;
use serde::Deserialize;

use crate::common::types::{CallerId, CanisterId};
use crate::service::model::verification::{AddVerification, UpdateVerification, Verification};
use crate::service::store::error::ErrorKindStore;
use crate::service::time_utils;

#[derive(CandidType, Deserialize, Default)]
pub struct VerificationStore {
    verification: BTreeMap<CanisterId, Verification>,
}

impl VerificationStore {
    pub fn verification_exists(&self, canister_id: &CanisterId) -> bool {
        self.verification.contains_key(canister_id)
    }

    pub fn add_verification(
        &mut self,
        caller_id: CallerId,
        add_verification: AddVerification,
    ) -> Result<(), ErrorKindStore> {
        self.verification_exists(&add_verification.canister_id)
            .not()
            .then(|| {
                let now = time_utils::now_to_str();
                self.verification.insert(
                    add_verification.canister_id,
                    Verification {
                        canister_id: add_verification.canister_id,
                        git_sha: add_verification.git_sha,
                        git_ref: add_verification.git_ref,
                        git_repo: add_verification.git_repo,
                        wasm_checksum: add_verification.wasm_checksum,
                        build_log_url: add_verification.build_log_url,
                        source_snapshot_url: add_verification.source_snapshot_url,
                        created_by: caller_id,
                        created_at: now.clone(),
                        updated_by: caller_id,
                        updated_at: now,
                    },
                );
            })
            .ok_or(ErrorKindStore::ExistedVerification)
    }

    pub fn update_verification(
        &mut self,
        caller_id: CallerId,
        update_verification: UpdateVerification,
    ) -> Result<(), ErrorKindStore> {
        self.verification
            .get_mut(&update_verification.canister_id)
            .map(|verification| {
                let now = time_utils::now_to_str();
                verification.git_sha = update_verification.git_sha;
                verification.git_ref = update_verification.git_ref;
                verification.git_repo = update_verification.git_repo;
                verification.wasm_checksum = update_verification.wasm_checksum;
                verification.build_log_url = update_verification.build_log_url;
                verification.source_snapshot_url = update_verification.source_snapshot_url;
                verification.updated_by = caller_id;
                verification.updated_at = now;
            })
            .ok_or(ErrorKindStore::VerificationNotFound)
    }

    pub fn get_verification_by_canister_id(
        &self,
        canister_id: &CanisterId,
    ) -> Option<&Verification> {
        self.verification.get(canister_id)
    }

    pub fn get_all_verifications(&self) -> Vec<&Verification> {
        self.verification.iter().map(|(_, v)| v).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::service::store::test_data;

    use super::*;

    fn update_verification_gen(seed: u8) -> UpdateVerification {
        if seed % 3 == 0 {
            test_data::fake_update_verification1(test_data::fake_canister1())
        } else if seed % 3 == 1 {
            test_data::fake_update_verification2(test_data::fake_canister2())
        } else {
            test_data::fake_update_verification3(test_data::fake_canister3())
        }
    }

    fn add_verification_gen(seed: u8) -> AddVerification {
        if seed % 3 == 0 {
            test_data::fake_add_verification1(test_data::fake_canister1())
        } else if seed % 3 == 1 {
            test_data::fake_add_verification2(test_data::fake_canister2())
        } else {
            test_data::fake_add_verification3(test_data::fake_canister3())
        }
    }

    fn init_test_data(len: u8) -> VerificationStore {
        let mut store = VerificationStore::default();
        for i in 0..len {
            let result = store.add_verification(test_data::caller_gen(i), add_verification_gen(i));
            assert_eq!(result, Ok(()));
        }
        store
    }

    #[test]
    fn add_verification_ok() {
        let mut store = init_test_data(3);
        assert_eq!(store.verification.len(), 3);
        for i in 0..store.verification.len() {
            let result = store.add_verification(
                test_data::caller_gen(i as u8),
                add_verification_gen(i as u8),
            );
            assert_eq!(result, Err(ErrorKindStore::ExistedVerification));
        }
    }

    #[test]
    fn update_verification_ok() {
        let mut store = VerificationStore::default();
        for i in 0..3 {
            let update_verification = update_verification_gen(i as u8);
            let caller_id = test_data::caller_gen(i as u8);
            let result = store.update_verification(caller_id, update_verification);
            assert_eq!(result, Err(ErrorKindStore::VerificationNotFound));
        }
        let mut store = init_test_data(3);
        for i in 0..store.verification.len() {
            let update_verification = update_verification_gen(i as u8);
            let caller_id = test_data::caller_gen(i as u8);
            let result = store.update_verification(caller_id, update_verification);
            assert_eq!(result, Ok(()));
        }
        for i in 0..store.verification.len() {
            let update_verification = update_verification_gen(i as u8);
            let verification = store
                .verification
                .get(&update_verification.canister_id)
                .unwrap();
            let caller_id = test_data::caller_gen(i as u8);
            let now = time_utils::now_to_str();
            assert_eq!(verification.canister_id, update_verification.canister_id);
            assert_eq!(verification.git_sha, update_verification.git_sha);
            assert_eq!(verification.git_ref, update_verification.git_ref);
            assert_eq!(verification.git_repo, update_verification.git_repo);
            assert_eq!(
                verification.wasm_checksum,
                update_verification.wasm_checksum
            );
            assert_eq!(
                verification.build_log_url,
                update_verification.build_log_url
            );
            assert_eq!(
                verification.source_snapshot_url,
                update_verification.source_snapshot_url
            );
            assert_eq!(verification.created_at, now);
            assert_eq!(verification.created_by, caller_id);
            assert_eq!(verification.updated_at, now);
            assert_eq!(verification.updated_by, caller_id);
        }
    }

    #[test]
    fn get_verification_by_canister_id_ok() {
        let store = init_test_data(3);
        for i in 0..store.verification.len() {
            let update_verification = add_verification_gen(i as u8);
            let verification = store
                .get_verification_by_canister_id(&update_verification.canister_id)
                .unwrap();
            let caller_id = test_data::caller_gen(i as u8);
            let now = time_utils::now_to_str();
            assert_eq!(verification.canister_id, update_verification.canister_id);
            assert_eq!(verification.git_sha, update_verification.git_sha);
            assert_eq!(verification.git_ref, update_verification.git_ref);
            assert_eq!(verification.git_repo, update_verification.git_repo);
            assert_eq!(
                verification.wasm_checksum,
                update_verification.wasm_checksum
            );
            assert_eq!(
                verification.build_log_url,
                update_verification.build_log_url
            );
            assert_eq!(
                verification.source_snapshot_url,
                update_verification.source_snapshot_url
            );
            assert_eq!(verification.created_at, now);
            assert_eq!(verification.created_by, caller_id);
            assert_eq!(verification.updated_at, now);
            assert_eq!(verification.updated_by, caller_id);
        }
    }

    #[test]
    fn get_all_verification_ok() {
        let store = init_test_data(3);
        let verifications = store.get_all_verifications();
        for i in 0..verifications.len() {
            let update_verification = add_verification_gen(i as u8);
            let caller_id = test_data::caller_gen(i as u8);
            let now = time_utils::now_to_str();
            assert_eq!(
                verifications.contains(&&Verification {
                    canister_id: update_verification.canister_id,
                    git_sha: update_verification.git_sha,
                    git_ref: update_verification.git_ref,
                    git_repo: update_verification.git_repo,
                    wasm_checksum: update_verification.wasm_checksum,
                    build_log_url: update_verification.build_log_url,
                    source_snapshot_url: update_verification.source_snapshot_url,
                    created_at: now.clone(),
                    created_by: caller_id,
                    updated_at: now,
                    updated_by: caller_id,
                }),
                true
            );
        }
    }
}
