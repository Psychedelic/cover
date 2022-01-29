use std::collections::HashMap;

use ic_kit::candid::CandidType;
use serde::Deserialize;

use crate::common::types::CanisterId;
use crate::service::model::error::Error;
use crate::service::model::pagination::{Pagination, PaginationInfo};
use crate::service::model::verification::{
    BuildStatus, RegisterVerification, SubmitVerification, Verification,
};
use crate::service::pagination::total_pages;
use crate::service::time_utils;

#[derive(CandidType, Deserialize, Default)]
pub struct VerificationStore {
    verifications: HashMap<CanisterId, Verification>,
    records: Vec<CanisterId>,
}

impl VerificationStore {
    pub fn submit_verification<F: Fn(CanisterId, BuildStatus)>(
        &mut self,
        new_verification: SubmitVerification,
        activity_handler: F,
    ) {
        let canister_id = new_verification.canister_id;
        let build_status = new_verification.build_status;
        self.verifications.insert(
            new_verification.canister_id,
            Verification {
                canister_id: new_verification.canister_id,
                canister_name: new_verification.canister_name,
                repo_url: new_verification.repo_url,
                commit_hash: new_verification.commit_hash,
                wasm_hash: new_verification.wasm_hash,
                build_url: Some(new_verification.build_url),
                build_status: new_verification.build_status,
                rust_version: new_verification.rust_version,
                dfx_version: new_verification.dfx_version,
                optimize_count: new_verification.optimize_count,
                updated_by: new_verification.owner_id,
                updated_at: time_utils::now_to_str(),
            },
        );
        activity_handler(canister_id, build_status);
    }

    pub fn get_verification_by_canister_id(
        &self,
        canister_id: &CanisterId,
    ) -> Option<&Verification> {
        self.verifications.get(canister_id)
    }

    pub fn get_verifications(&self, pagination_info: &PaginationInfo) -> Pagination<&Verification> {
        let total_items = self.records.len() as u64;
        let total_pages = total_pages(total_items, pagination_info.items_per_page);

        let mut data: Vec<&Verification> = vec![];
        if 0 < pagination_info.page_index && pagination_info.page_index <= total_pages {
            // check if last page
            let data_length = match pagination_info.page_index == total_pages {
                true => total_items - (pagination_info.items_per_page * (total_pages - 1)),
                false => pagination_info.items_per_page,
            };

            //calculate where the pagination should start and end
            let start = (total_items
                - (pagination_info.page_index - 1) * pagination_info.items_per_page)
                as usize;
            let end = start - data_length as usize;

            //because latest items will get appended to the end of 'records'
            //so in order to get latest data first, we'll iterate 'records' and push verification into 'data' in reverse order
            //end will be included and start will be excluded
            for i in (end..start).rev() {
                data.push(&self.verifications[&self.records[i]])
            }
        }

        Pagination::of(data, pagination_info, total_items)
    }

    pub fn register_verification<F: Fn(CanisterId, BuildStatus)>(
        &mut self,
        register_verification: RegisterVerification,
        activity_handler: F,
    ) -> Result<(), Error> {
        let canister_id = register_verification.canister_id;
        let build_status = BuildStatus::Pending;
        self.verifications
            .get_mut(&register_verification.canister_id)
            .map(|verification| match verification.build_status {
                BuildStatus::Pending | BuildStatus::Building => Err(Error::BuildInProgress),
                BuildStatus::Error | BuildStatus::Success => Ok(()),
            })
            .unwrap_or_else(|| Ok(()))
            .map(|_| {
                self.verifications
                    .insert(
                        register_verification.canister_id,
                        Verification {
                            canister_id: register_verification.canister_id,
                            canister_name: register_verification.canister_name,
                            repo_url: register_verification.repo_url,
                            commit_hash: register_verification.commit_hash,
                            wasm_hash: None,
                            build_url: None,
                            build_status,
                            rust_version: register_verification.rust_version,
                            dfx_version: register_verification.dfx_version,
                            optimize_count: register_verification.optimize_count,
                            updated_by: register_verification.owner_id,
                            updated_at: time_utils::now_to_str(),
                        },
                    )
                    .is_none()
                    .then(|| self.records.push(canister_id));
                activity_handler(canister_id, build_status)
            })
    }
}

#[cfg(test)]
mod test {
    use ic_kit::*;

    use crate::service::store::test_data::*;

    use super::*;

    fn init_test_data() -> VerificationStore {
        let mut store = VerificationStore::default();

        assert_eq!(
            store.register_verification(
                fake_register_verification(&fake_canister1()),
                |canister_id, build_status| {
                    assert_eq!(canister_id, fake_canister1());
                    assert_eq!(build_status, BuildStatus::Pending);
                },
            ),
            Ok(())
        );

        assert_eq!(
            store.register_verification(
                fake_register_verification(&fake_canister2()),
                |canister_id, build_status| {
                    assert_eq!(canister_id, fake_canister2());
                    assert_eq!(build_status, BuildStatus::Pending);
                },
            ),
            Ok(())
        );

        store
    }

    #[test]
    fn submit_verification_ok() {
        let mut store = init_test_data();

        store.submit_verification(
            fake_success_verification(&mock_principals::alice(), &fake_canister3()),
            |canister_id, build_status| {
                assert_eq!(canister_id, fake_canister3());
                assert_eq!(build_status, BuildStatus::Success);
            },
        );

        assert_eq!(
            store.get_verification_by_canister_id(&fake_canister3()),
            Some(&fake_verification(fake_success_verification(
                &mock_principals::alice(),
                &fake_canister3()
            )))
        );

        store.submit_verification(
            fake_error_verification(&mock_principals::alice(), &fake_canister1()),
            |canister_id, build_status| {
                assert_eq!(canister_id, fake_canister1());
                assert_eq!(build_status, BuildStatus::Error);
            },
        );

        assert_eq!(
            store.get_verification_by_canister_id(&fake_canister1()),
            Some(&fake_verification(fake_error_verification(
                &mock_principals::alice(),
                &fake_canister1()
            )))
        );
    }

    #[test]
    fn get_verifications_ok() {
        let mut store = init_test_data();

        assert_eq!(
            store.register_verification(
                fake_register_verification(&fake_canister3()),
                |canister_id, build_status| {
                    assert_eq!(canister_id, fake_canister3());
                    assert_eq!(build_status, BuildStatus::Pending);
                },
            ),
            Ok(())
        );

        assert_eq!(
            store.get_verifications(&PaginationInfo {
                page_index: 0,
                items_per_page: 2
            }),
            fake_pagination(
                vec![],
                &PaginationInfo {
                    page_index: 0,
                    items_per_page: 2
                },
                store.verifications.len() as u64
            )
        );

        assert_eq!(
            store.get_verifications(&PaginationInfo {
                page_index: 1,
                items_per_page: 2
            }),
            fake_pagination(
                vec![
                    &fake_verification_use_register_model(fake_register_verification(
                        &fake_canister3()
                    )),
                    &fake_verification_use_register_model(fake_register_verification(
                        &fake_canister2()
                    ))
                ],
                &PaginationInfo {
                    page_index: 1,
                    items_per_page: 2
                },
                store.verifications.len() as u64
            )
        );

        assert_eq!(
            store.get_verifications(&PaginationInfo {
                page_index: 2,
                items_per_page: 2
            }),
            fake_pagination(
                vec![&fake_verification_use_register_model(
                    fake_register_verification(&fake_canister1())
                )],
                &PaginationInfo {
                    page_index: 2,
                    items_per_page: 2
                },
                store.verifications.len() as u64
            )
        );

        assert_eq!(
            store.get_verifications(&PaginationInfo {
                page_index: 3,
                items_per_page: 2
            }),
            fake_pagination(
                vec![],
                &PaginationInfo {
                    page_index: 3,
                    items_per_page: 2
                },
                store.verifications.len() as u64
            )
        );

        assert_eq!(
            store.get_verifications(&PaginationInfo {
                page_index: 3,
                items_per_page: 0
            }),
            fake_pagination(
                vec![],
                &PaginationInfo {
                    page_index: 3,
                    items_per_page: 0
                },
                store.verifications.len() as u64
            )
        );
    }

    #[test]
    fn register_verification_ok() {
        let mut store = init_test_data();

        assert_eq!(
            store.register_verification(
                fake_register_verification(&fake_canister1()),
                |canister_id, build_status| {
                    assert_eq!(canister_id, fake_canister1());
                    assert_eq!(build_status, BuildStatus::Pending);
                },
            ),
            Err(Error::BuildInProgress)
        );

        assert_eq!(
            store.register_verification(
                fake_register_verification(&fake_canister3()),
                |canister_id, build_status| {
                    assert_eq!(canister_id, fake_canister3());
                    assert_eq!(build_status, BuildStatus::Pending);
                },
            ),
            Ok(())
        );

        assert_eq!(
            store.get_verifications(&PaginationInfo {
                page_index: 2,
                items_per_page: 1
            }),
            fake_pagination(
                vec![&fake_verification_use_register_model(
                    fake_register_verification(&fake_canister2())
                )],
                &PaginationInfo {
                    page_index: 2,
                    items_per_page: 1
                },
                store.verifications.len() as u64
            )
        );
    }
}
