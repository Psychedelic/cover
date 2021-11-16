use std::collections::BTreeMap;
use std::ops::Bound::Included;
use std::ops::Not;

use crate::common::types::{CanisterId, ReqId};
use crate::service::store::error::ErrorKind;
use crate::service::time_utils;
use crate::service::types::{Progress, ProgressStatus, UpdateProgress};

pub struct ProgressStore {
    /// Request id is unique => single entry
    progress: BTreeMap<(ReqId, CanisterId), Progress>,
}

impl Default for ProgressStore {
    fn default() -> Self {
        Self {
            progress: BTreeMap::default(),
        }
    }
}

impl ProgressStore {
    pub fn get_progress_by_request_id(&self, request_id: ReqId) -> Option<&Progress> {
        self.progress
            .range((
                Included((request_id, CanisterId::management_canister())),
                Included((request_id, CanisterId::from_slice(&[255; 29]))),
            ))
            .map(|(_, v)| v)
            .next()
    }

    pub fn get_progresses_by_canister_id(&self, canister_id: CanisterId) -> Vec<&Progress> {
        self.progress
            .range((
                Included((ReqId::MIN, canister_id)),
                Included((ReqId::MAX, canister_id)),
            ))
            .map(|(_, v)| v)
            .collect()
    }

    pub fn get_all_progresses(&self) -> Vec<&Progress> {
        self.progress.iter().map(|(_, v)| v).collect()
    }

    pub fn init_progress(
        &mut self,
        request_id: ReqId,
        canister_id: CanisterId,
    ) -> Result<(), ErrorKind> {
        self.progress
            .get(&(request_id, canister_id))
            .is_some()
            .not()
            .then(|| {
                self.progress.insert(
                    (request_id, canister_id),
                    Progress {
                        request_id,
                        canister_id,
                        started_at: time_utils::now_to_str(),
                        updated_at: None,
                        git_checksum: None,
                        git_ref: None,
                        wasm_checksum: None,
                        build_log_url: None,
                        source_snapshot_url: None,
                        percentage: None,
                        status: ProgressStatus::Init,
                    },
                );
            })
            .ok_or(ErrorKind::InitExistedProgress)
    }

    pub fn update_progress(&mut self, update_progress: UpdateProgress) -> Result<(), ErrorKind> {
        self.progress
            .get_mut(&(update_progress.request_id, update_progress.canister_id))
            .ok_or(ErrorKind::ProgressNotFound)
            .and_then(|progress| {
                ProgressStatus::Init
                    .ne(&update_progress.status)
                    .then(|| progress)
                    .ok_or(ErrorKind::InvalidProgressStatus)
            })
            .map(|progress| {
                progress.updated_at = Some(time_utils::now_to_str());
                progress.git_checksum = update_progress.git_checksum;
                progress.git_ref = update_progress.git_ref;
                progress.wasm_checksum = update_progress.wasm_checksum;
                progress.build_log_url = update_progress.build_log_url;
                progress.source_snapshot_url = update_progress.source_snapshot_url;
                progress.percentage = update_progress.percentage;
                progress.status = update_progress.status;
                if progress.status == ProgressStatus::Finished
                    || progress.status == ProgressStatus::Error
                {
                    // TODO: remove entry and push to history
                }
            })
    }
}

#[cfg(test)]
mod test {
    use crate::service::store::test_data;

    use super::*;

    fn assert_progress_utils(left: &Progress, right: &UpdateProgress) {
        assert_eq!(left.request_id, right.request_id);
        assert_eq!(left.canister_id, right.canister_id);
        assert_eq!(left.git_checksum, right.git_checksum);
        assert_eq!(left.git_ref, right.git_ref);
        assert_eq!(left.wasm_checksum, right.wasm_checksum);
        assert_eq!(left.build_log_url, right.build_log_url);
        assert_eq!(left.source_snapshot_url, right.source_snapshot_url);
        assert_eq!(left.percentage, right.percentage);
        assert_eq!(left.status, right.status);
    }

    #[test]
    fn init_progress_ok() {
        let len = 15;
        let mut store = ProgressStore::default();
        for i in 1..len + 1 {
            let result = store.init_progress(i, test_data::fake_canister1());
            assert_eq!(result, Ok(()));
            store
                .progress
                .iter()
                .enumerate()
                .for_each(|(index, (_, p))| {
                    assert_eq!(p.request_id, (index + 1) as ReqId);
                    assert_eq!(p.canister_id, test_data::fake_canister1());
                    assert_eq!(p.started_at.is_empty(), false);
                    assert_eq!(p.updated_at, None);
                    assert_eq!(p.git_checksum, None);
                    assert_eq!(p.git_ref, None);
                    assert_eq!(p.wasm_checksum, None);
                    assert_eq!(p.build_log_url, None);
                    assert_eq!(p.source_snapshot_url, None);
                    assert_eq!(p.percentage, None);
                    assert_eq!(p.status, ProgressStatus::Init);
                });
        }
        assert_eq!(store.progress.len(), len as usize);
        for i in 1..len + 1 {
            let result = store.init_progress(i, test_data::fake_canister1());
            assert_eq!(result, Err(ErrorKind::InitExistedProgress));
        }
        assert_eq!(store.progress.len(), len as usize);
    }

    #[test]
    fn update_progress_ok() {
        let len = 15;
        let mut store = ProgressStore::default();
        for i in 1..len + 1 {
            let result = store.init_progress(i, test_data::fake_canister1());
            assert_eq!(result, Ok(()));
        }
        assert_eq!(store.progress.len(), len as usize);
        for i in 1..len + 1 {
            let result = store.update_progress(test_data::fake_update_progress_default(
                i,
                test_data::fake_canister2(),
            ));
            assert_eq!(result, Err(ErrorKind::ProgressNotFound));
            let update_progress = if i % 4 == 0 {
                test_data::fake_update_progress_init(i, test_data::fake_canister1())
            } else if i % 4 == 1 {
                test_data::fake_update_progress_in_progress(i, test_data::fake_canister1())
            } else if i % 4 == 2 {
                test_data::fake_update_progress_finished(i, test_data::fake_canister1())
            } else {
                test_data::fake_update_progress_error(i, test_data::fake_canister1())
            };
            let result = store.update_progress(update_progress);
            assert_eq!(
                result,
                if i % 4 == 0 {
                    Err(ErrorKind::InvalidProgressStatus)
                } else {
                    Ok(())
                }
            );
        }
        store
            .progress
            .iter()
            .enumerate()
            .for_each(|(index, (_, p))| {
                let request_id = index + 1;
                assert_eq!(p.started_at.is_empty(), false);
                if request_id % 4 == 0 {
                    assert_eq!(p.updated_at.is_some(), false);
                    assert_progress_utils(
                        p,
                        &test_data::fake_update_progress_default(
                            request_id as ReqId,
                            test_data::fake_canister1(),
                        ),
                    );
                } else if request_id % 4 == 1 {
                    assert_eq!(p.updated_at.is_some(), true);
                    assert_progress_utils(
                        p,
                        &test_data::fake_update_progress_in_progress(
                            request_id as ReqId,
                            test_data::fake_canister1(),
                        ),
                    );
                } else if request_id % 4 == 2 {
                    assert_eq!(p.updated_at.is_some(), true);
                    assert_progress_utils(
                        p,
                        &test_data::fake_update_progress_finished(
                            request_id as ReqId,
                            test_data::fake_canister1(),
                        ),
                    );
                } else {
                    assert_eq!(p.updated_at.is_some(), true);
                    assert_progress_utils(
                        p,
                        &test_data::fake_update_progress_error(
                            request_id as ReqId,
                            test_data::fake_canister1(),
                        ),
                    );
                }
            });
        assert_eq!(store.progress.len(), len as usize);
    }

    // TODO: test get_progress_by_request_id
    // TODO: test get_progresses_by_canister_id
    // TODO: test get_all_progresses
}
