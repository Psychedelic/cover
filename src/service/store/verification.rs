use std::collections::BTreeMap;

use ic_kit::candid::CandidType;
use serde::Deserialize;

use crate::common::types::CanisterId;
use crate::service::model::verification::{SubmitVerification, Verification};
use crate::service::time_utils;

#[derive(CandidType, Deserialize, Default)]
pub struct VerificationStore {
    verification: BTreeMap<CanisterId, Verification>,
}

impl VerificationStore {
    pub fn submit_verification(&mut self, new_verification: SubmitVerification) {
        let now = time_utils::now_to_str();
        self.verification.insert(
            new_verification.canister_id,
            Verification {
                canister_id: new_verification.canister_id,
                canister_name: new_verification.canister_name,
                repo_url: new_verification.repo_url,
                commit_hash: new_verification.commit_hash,
                wasm_hash: new_verification.wasm_hash,
                build_url: new_verification.build_url,
                build_status: new_verification.build_status,
                rust_version: new_verification.rust_version,
                dfx_version: new_verification.dfx_version,
                optimize_count: new_verification.optimize_count,
                updated_by: new_verification.owner_id,
                updated_at: now,
            },
        );
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

        store.submit_verification(fake_submit_verification1(
            &mock_principals::alice(),
            &fake_canister1(),
        ));

        store.submit_verification(fake_submit_verification2(
            &mock_principals::bob(),
            &fake_canister2(),
        ));

        store
    }

    #[test]
    fn submit_verification_ok() {
        let mut store = init_test_data();

        get_all_verification_ok();

        store.submit_verification(fake_submit_verification3(
            &mock_principals::alice(),
            &fake_canister3(),
        ));

        assert_eq!(
            store.get_all_verifications(),
            vec![
                &fake_verification(fake_submit_verification3(
                    &mock_principals::alice(),
                    &fake_canister3()
                )),
                &fake_verification(fake_submit_verification2(
                    &mock_principals::bob(),
                    &fake_canister2()
                )),
                &fake_verification(fake_submit_verification1(
                    &mock_principals::alice(),
                    &fake_canister1()
                ),)
            ]
        );

        store.submit_verification(fake_submit_verification2(
            &mock_principals::john(),
            &fake_canister1(),
        ));

        assert_eq!(
            store.get_all_verifications(),
            vec![
                &fake_verification(fake_submit_verification3(
                    &mock_principals::alice(),
                    &fake_canister3()
                )),
                &fake_verification(fake_submit_verification2(
                    &mock_principals::bob(),
                    &fake_canister2()
                )),
                &fake_verification(fake_submit_verification2(
                    &mock_principals::john(),
                    &fake_canister1()
                ),)
            ]
        );
    }

    #[test]
    fn get_verification_by_canister_id_ok() {
        let store = init_test_data();

        assert_eq!(
            store.get_verification_by_canister_id(&fake_canister2()),
            Some(&fake_verification(fake_submit_verification2(
                &mock_principals::bob(),
                &fake_canister2()
            )))
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
                &fake_verification(fake_submit_verification2(
                    &mock_principals::bob(),
                    &fake_canister2()
                )),
                &fake_verification(fake_submit_verification1(
                    &mock_principals::alice(),
                    &fake_canister1()
                ),)
            ]
        );
    }
}
