use std::borrow::BorrowMut;
use std::collections::BTreeMap;
use std::ops::Bound::Included;

use chrono::{SecondsFormat, Utc};

use crate::common::types::{CanisterId, ReqId};
use crate::service::store::error::ErrorKind;
use crate::service::types::{ProgressStatus, UpdateProgress, ValidationProgress};

pub struct ProgressTracker {
    /// Request id is unique => single entry
    progress: BTreeMap<(ReqId, CanisterId), ValidationProgress>,
}

impl Default for ProgressTracker {
    fn default() -> Self {
        Self {
            progress: BTreeMap::default(),
        }
    }
}

impl ProgressTracker {
    pub fn get_progress_by_request_id(
        &self,
        request_validation_id: ReqId,
    ) -> Option<&ValidationProgress> {
        let start = (request_validation_id, CanisterId::management_canister()); // [0; 29],
        let end = (request_validation_id, CanisterId::from_slice(&[255; 29]));
        self.progress
            .range((Included(start), Included(end)))
            .map(|(_, v)| v)
            .next()
    }

    pub fn get_progress_by_canister_id(
        &self,
        request_validation_id: CanisterId,
    ) -> Vec<&ValidationProgress> {
        let start = (ReqId::min_value(), request_validation_id);
        let end = (ReqId::max_value(), request_validation_id);
        self.progress
            .range((Included(start), Included(end)))
            .map(|(_, v)| v)
            .collect()
    }

    pub fn get_all_progress(&self) -> Vec<&ValidationProgress> {
        self.progress.iter().map(|(_, v)| v).collect()
    }

    pub fn init_progress(
        &mut self,
        request_validation_id: ReqId,
        canister_id: CanisterId,
    ) -> Result<(), ErrorKind> {
        self.progress
            .get(&(request_validation_id, canister_id))
            .map(|_| Err(ErrorKind::InitExistedProgress))
            .unwrap_or(Ok(()))?;
        self.progress.insert(
            (request_validation_id, canister_id),
            ValidationProgress {
                request_id: request_validation_id,
                validation_started_at: Utc::now().to_rfc3339_opts(SecondsFormat::Millis, false),
                validation_updated_at: None,
                validation_completed_at: None,
                git_checksum: None,
                canister_checksum: None,
                wasm_checksum: None,
                build_log_url: None,
                source_snapshot_url: None,
                percentage: None,
                status: ProgressStatus::Init,
            },
        );
        Ok(())
    }

    pub fn update_progress(
        &mut self,
        request_validation_id: ReqId,
        canister_id: CanisterId,
        status: UpdateProgress,
    ) -> Result<(), ErrorKind> {
        let validation_response = self
            .progress
            .get_mut(&(request_validation_id, canister_id))
            .ok_or(ErrorKind::ProgressNotFound)?
            .borrow_mut();
        if status.status == ProgressStatus::Init {
            return Err(ErrorKind::InvalidProgressStatus);
        }
        let now = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, false);
        validation_response.validation_updated_at = Some(now.clone());
        validation_response.git_checksum = status.git_checksum;
        validation_response.canister_checksum = status.canister_checksum;
        validation_response.wasm_checksum = status.wasm_checksum;
        validation_response.build_log_url = status.build_log_url;
        validation_response.source_snapshot_url = status.source_snapshot_url;
        validation_response.percentage = status.percentage;
        validation_response.status = status.status;
        if validation_response.status == ProgressStatus::Finished
            || validation_response.status == ProgressStatus::Error
        {
            validation_response.validation_completed_at = Some(now)
            // TODO: remove entry and push to history
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::service::store::test_data;

    use super::*;

    fn assert_progress_utils(left: &ValidationProgress, right: &UpdateProgress) {
        assert_eq!(left.git_checksum, right.git_checksum);
        assert_eq!(left.canister_checksum, right.canister_checksum);
        assert_eq!(left.wasm_checksum, right.wasm_checksum);
        assert_eq!(left.build_log_url, right.build_log_url);
        assert_eq!(left.source_snapshot_url, right.source_snapshot_url);
        assert_eq!(left.percentage, right.percentage);
        assert_eq!(left.status, right.status);
    }

    impl ProgressTracker {
        fn assert_progress(&self) {
            self.progress
                .iter()
                .enumerate()
                .for_each(|(index, (_, p))| {
                    let request_id = index + 1;
                    assert_eq!(p.request_id, request_id as ReqId);
                    assert_eq!(p.validation_started_at.is_empty(), false);
                    println!("{:?}", p);
                    if request_id % 4 == 0 {
                        assert_eq!(p.validation_updated_at.is_some(), false);
                        assert_eq!(p.validation_completed_at.is_some(), false);
                        assert_progress_utils(p, &test_data::fake_update_progress_default());
                    } else if request_id % 4 == 1 {
                        assert_eq!(p.validation_updated_at.is_some(), true);
                        assert_eq!(p.validation_completed_at.is_some(), false);
                        assert_progress_utils(p, &test_data::fake_update_progress_in_progress());
                    } else if request_id % 4 == 2 {
                        assert_eq!(p.validation_updated_at.is_some(), true);
                        assert_eq!(p.validation_completed_at.is_some(), true);
                        assert_progress_utils(p, &test_data::fake_update_progress_finished());
                    } else {
                        assert_eq!(p.validation_updated_at.is_some(), true);
                        assert_eq!(p.validation_completed_at.is_some(), true);
                        assert_progress_utils(p, &test_data::fake_update_progress_error());
                    }
                });
        }
    }

    #[test]
    fn init_progress_ok() {
        let len = 15;
        let mut store = ProgressTracker::default();
        for i in 1..len + 1 {
            let result = store.init_progress(i, test_data::fake_canister1());
            assert_eq!(result, Ok(()));
            store
                .progress
                .iter()
                .enumerate()
                .for_each(|(index, (_, p))| {
                    assert_eq!(p.request_id, (index + 1) as ReqId);
                    assert_eq!(p.validation_started_at.is_empty(), false);
                    assert_eq!(p.validation_updated_at, None);
                    assert_eq!(p.validation_completed_at, None);
                    assert_eq!(p.git_checksum, None);
                    assert_eq!(p.canister_checksum, None);
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
        let mut store = ProgressTracker::default();
        for i in 1..len + 1 {
            let result = store.init_progress(i, test_data::fake_canister1());
            assert_eq!(result, Ok(()));
        }
        assert_eq!(store.progress.len(), len as usize);
        for i in 1..len + 1 {
            let result = store.update_progress(
                i,
                test_data::fake_canister2(),
                test_data::fake_update_progress_default(),
            );
            assert_eq!(result, Err(ErrorKind::ProgressNotFound));
            let update_progress = if i % 4 == 0 {
                test_data::fake_update_progress_init()
            } else if i % 4 == 1 {
                test_data::fake_update_progress_in_progress()
            } else if i % 4 == 2 {
                test_data::fake_update_progress_finished()
            } else {
                test_data::fake_update_progress_error()
            };
            println!("{:?}", update_progress);
            let result = store.update_progress(i, test_data::fake_canister1(), update_progress);
            assert_eq!(
                result,
                if i % 4 == 0 {
                    Err(ErrorKind::InvalidProgressStatus)
                } else {
                    Ok(())
                }
            );
        }
        store.assert_progress();
        assert_eq!(store.progress.len(), len as usize);
    }
}
