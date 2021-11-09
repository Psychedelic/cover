use crate::common::types::{CanisterId, ReqId};
use crate::service::types::{ProgressStatus, UpdateProgress};

pub fn fake_update_progress_default(request_id: ReqId, canister_id: CanisterId) -> UpdateProgress {
    UpdateProgress {
        request_id,
        canister_id,
        git_checksum: None,
        canister_checksum: None,
        wasm_checksum: None,
        build_log_url: None,
        source_snapshot_url: None,
        percentage: None,
        status: ProgressStatus::Init,
    }
}

pub fn fake_update_progress_init(request_id: ReqId, canister_id: CanisterId) -> UpdateProgress {
    UpdateProgress {
        request_id,
        canister_id,
        git_checksum: Some("git_checksum0".into()),
        canister_checksum: Some("canister_checksum0".into()),
        wasm_checksum: Some("wasm_checksum0".into()),
        build_log_url: Some("build_log_url0".into()),
        source_snapshot_url: Some("source_snapshot_url0".into()),
        percentage: Some(0.0),
        status: ProgressStatus::Init,
    }
}

pub fn fake_update_progress_in_progress(
    request_id: ReqId,
    canister_id: CanisterId,
) -> UpdateProgress {
    UpdateProgress {
        request_id,
        canister_id,
        git_checksum: Some("git_checksum1".into()),
        canister_checksum: Some("canister_checksum1".into()),
        wasm_checksum: Some("wasm_checksum1".into()),
        build_log_url: Some("build_log_url1".into()),
        source_snapshot_url: Some("source_snapshot_url1".into()),
        percentage: Some(1.1),
        status: ProgressStatus::InProgress,
    }
}

pub fn fake_update_progress_finished(request_id: ReqId, canister_id: CanisterId) -> UpdateProgress {
    UpdateProgress {
        request_id,
        canister_id,
        git_checksum: Some("git_checksum2".into()),
        canister_checksum: Some("canister_checksum2".into()),
        wasm_checksum: Some("wasm_checksum2".into()),
        build_log_url: Some("build_log_url2".into()),
        source_snapshot_url: Some("source_snapshot_url2".into()),
        percentage: Some(2.2),
        status: ProgressStatus::Finished,
    }
}

pub fn fake_update_progress_error(request_id: ReqId, canister_id: CanisterId) -> UpdateProgress {
    UpdateProgress {
        request_id,
        canister_id,
        git_checksum: Some("git_checksum3".into()),
        canister_checksum: Some("canister_checksum3".into()),
        wasm_checksum: Some("wasm_checksum3".into()),
        build_log_url: Some("build_log_url3".into()),
        source_snapshot_url: Some("source_snapshot_url3".into()),
        percentage: Some(3.3),
        status: ProgressStatus::Error,
    }
}
