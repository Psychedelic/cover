use std::borrow::BorrowMut;
use std::collections::BTreeMap;
use std::ops::Bound::Included;

use chrono::{SecondsFormat, Utc};

use crate::common::types::{CanisterId, ReqId};
use crate::service::store::error::ErrorKind;
use crate::service::types::{ProgressStatus, UpdateOnGoingProgressStatus, ValidationResponse};

pub struct ProgressTracker {
    /// Request id is unique => single entry
    store: BTreeMap<(ReqId, CanisterId), ValidationResponse>,
}

impl Default for ProgressTracker {
    fn default() -> Self {
        Self {
            store: BTreeMap::default(),
        }
    }
}

impl ProgressTracker {
    pub fn get_by_request_id(&self, request_validation_id: ReqId) -> Option<&ValidationResponse> {
        let start = (request_validation_id, CanisterId::management_canister()); // [0; 29],
        let end = (request_validation_id, CanisterId::from_slice(&[255; 29]));
        self.store
            .range((Included(start), Included(end)))
            .map(|(_, v)| v)
            .next()
    }

    pub fn get_by_canister_id(
        &self,
        request_validation_id: CanisterId,
    ) -> Vec<&ValidationResponse> {
        let start = (ReqId::min_value(), request_validation_id);
        let end = (ReqId::max_value(), request_validation_id);
        self.store
            .range((Included(start), Included(end)))
            .map(|(_, v)| v)
            .collect()
    }

    pub fn init_progress(
        &mut self,
        request_validation_id: ReqId,
        canister_id: CanisterId,
    ) -> Result<(), ErrorKind> {
        self.store
            .get(&(request_validation_id, canister_id))
            .map(|_| Err(ErrorKind::InitExistedProgress))
            .unwrap_or(Ok(()))?;
        self.store.insert(
            (request_validation_id, canister_id),
            ValidationResponse {
                request_validation_id,
                validation_started_at: Utc::now().to_rfc3339_opts(SecondsFormat::Millis, false),
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

    pub fn update_status(
        &mut self,
        request_validation_id: ReqId,
        canister_id: CanisterId,
        status: UpdateOnGoingProgressStatus,
    ) -> Result<(), ErrorKind> {
        let validation_response = self
            .store
            .get_mut(&(request_validation_id, canister_id))
            .ok_or(ErrorKind::ProgressNotFound)?
            .borrow_mut();
        validation_response.git_checksum = status.git_checksum;
        validation_response.canister_checksum = status.canister_checksum;
        validation_response.wasm_checksum = status.wasm_checksum;
        validation_response.build_log_url = status.build_log_url;
        validation_response.source_snapshot_url = status.source_snapshot_url;
        validation_response.percentage = status.percentage;
        validation_response.status = status.status;
        if validation_response.status == ProgressStatus::Finished {
            validation_response.validation_completed_at =
                Some(Utc::now().to_rfc3339_opts(SecondsFormat::Millis, false))
            // TODO: remove entry and push to history
        }
        Ok(())
    }
}
