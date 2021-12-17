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
        caller_id: &CallerId,
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
                        created_by: *caller_id,
                        created_at: now.clone(),
                        updated_by: *caller_id,
                        updated_at: now,
                    },
                );
            })
            .ok_or(ErrorKindStore::ExistedVerification)
    }

    pub fn update_verification(
        &mut self,
        caller_id: &CallerId,
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
                verification.updated_by = *caller_id;
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
    use ic_kit::*;

    use crate::service::store::test_data::*;

    use super::*;

    fn init_test_data() -> VerificationStore {
        let mut store = VerificationStore::default();

        store
            .add_verification(
                &mock_principals::alice(),
                fake_add_verification1(&fake_canister1()),
            )
            .unwrap();

        store
            .add_verification(
                &mock_principals::bob(),
                fake_add_verification2(&fake_canister2()),
            )
            .unwrap();

        store
    }

    #[test]
    fn add_verification_ok() {
        let mut store = init_test_data();

        assert_eq!(store.get_all_verifications().len(), 2);

        assert_eq!(
            store.add_verification(
                &mock_principals::alice(),
                fake_add_verification3(&fake_canister3())
            ),
            Ok(())
        );

        assert_eq!(store.get_all_verifications().len(), 3);

        assert_eq!(
            store.add_verification(
                &mock_principals::alice(),
                fake_add_verification2(&fake_canister2())
            ),
            Err(ErrorKindStore::ExistedVerification)
        );

        assert_eq!(store.get_all_verifications().len(), 3);
    }

    #[test]
    fn update_verification_ok() {
        let mut store = init_test_data();

        assert_eq!(store.get_all_verifications().len(), 2);

        assert_eq!(
            store.update_verification(
                &mock_principals::alice(),
                fake_update_verification1(&fake_canister1())
            ),
            Ok(())
        );

        assert_eq!(
            store.get_verification_by_canister_id(&fake_canister1()),
            Some(&fake_verification_use_update_model(
                &mock_principals::alice(),
                fake_update_verification1(&fake_canister1())
            ))
        );

        assert_eq!(store.get_all_verifications().len(), 2);

        assert_eq!(
            store.update_verification(
                &mock_principals::bob(),
                fake_update_verification1(&fake_canister3())
            ),
            Err(ErrorKindStore::VerificationNotFound)
        );

        assert_eq!(store.get_all_verifications().len(), 2);
    }

    #[test]
    fn get_verification_by_canister_id_ok() {
        let store = init_test_data();

        assert_eq!(store.get_all_verifications().len(), 2);

        assert_eq!(
            store.get_verification_by_canister_id(&fake_canister2()),
            Some(&fake_verification_use_add_model(
                &mock_principals::bob(),
                fake_add_verification2(&fake_canister2())
            ))
        );

        assert_eq!(
            store.get_verification_by_canister_id(&fake_canister3()),
            None
        );

        assert_eq!(store.get_all_verifications().len(), 2);
    }

    #[test]
    fn get_all_verification_ok() {
        let store = init_test_data();

        assert_eq!(
            store.get_all_verifications(),
            vec![
                &fake_verification_use_add_model(
                    &mock_principals::bob(),
                    fake_add_verification2(&fake_canister2())
                ),
                &fake_verification_use_add_model(
                    &mock_principals::alice(),
                    fake_add_verification1(&fake_canister1()),
                )
            ]
        );
    }

    #[test]
    fn verification_exists_ok() {
        let store = init_test_data();

        assert_eq!(store.verification_exists(&fake_canister1()), true);

        assert_eq!(store.verification_exists(&fake_canister3()), false);

        assert_eq!(store.get_all_verifications().len(), 2);
    }
}
