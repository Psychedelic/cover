use ic_kit::RejectionCode;

use crate::service::model::error::Error;
use crate::service::store::error::ErrorKindStore;

// ERR_{MODULE}_{LEVEL}_{SEQUENCE}
//      MODULE
//          General               000
//          Request               001
//          Progress              002
//          Verification          003
//          Provider              004
//      LEVEL
//          Api                   001       (inter-canister, external api...)
//          Service               002
//          Store                 003
pub enum ErrorKindApi {
    //============================================================================
    // General
    //============================================================================
    InterCanister((RejectionCode, String)),
    BlackholeCanisterStatus(String),
}

// ERR_{MODULE}_001_{SEQUENCE}
impl From<ErrorKindApi> for Error {
    fn from(kind: ErrorKindApi) -> Self {
        match kind {
            //============================================================================
            // General - ERR_000_001_{SEQUENCE}
            //============================================================================
            ErrorKindApi::InterCanister(e) => Self {
                code: "ERR_000_001_001",
                message: "An error occurred when calling inter-canister",
                debug_log: Some(format!("RejectionCode: {:?}\n{}", e.0, e.1)),
            },
            ErrorKindApi::BlackholeCanisterStatus(e) => Self {
                code: "ERR_000_001_002",
                message: "An error occurred when get status from blackhole canister",
                debug_log: Some(e),
            },
        }
    }
}

pub enum ErrorKindService {
    //============================================================================
    // Verification
    //============================================================================
    InvalidProvider,
    //============================================================================
    // Provider
    //============================================================================
    InvalidCoverController,
}

// ERR_{MODULE}_002_{SEQUENCE}
impl From<ErrorKindService> for Error {
    fn from(kind: ErrorKindService) -> Self {
        match kind {
            //============================================================================
            // Verification - ERR_003_002_{SEQUENCE}
            //============================================================================
            ErrorKindService::InvalidProvider => Self {
                code: "ERR_003_002_001",
                message: "Invalid provider",
                debug_log: None,
            },
            //============================================================================
            // Provider - ERR_004_002_{SEQUENCE}
            //============================================================================
            ErrorKindService::InvalidCoverController => Self {
                code: "ERR_004_002_001",
                message: "Invalid controller",
                debug_log: None,
            },
        }
    }
}

// ERR_{MODULE}_003_{SEQUENCE}
impl From<ErrorKindStore> for Error {
    fn from(kind: ErrorKindStore) -> Self {
        match kind {
            //============================================================================
            // Request - ERR_001_003_{SEQUENCE}
            //============================================================================
            ErrorKindStore::RequestNotFound => Self {
                code: "ERR_001_003_001",
                message: "Request not found",
                debug_log: None,
            },
            //============================================================================
            // Progress - ERR_002_003_{SEQUENCE}
            //============================================================================
            ErrorKindStore::ProgressNotFound => Self {
                code: "ERR_002_003_001",
                message: "Progress not found",
                debug_log: None,
            },
            ErrorKindStore::InitExistedProgress => Self {
                code: "ERR_002_003_002",
                message: "Init existed progress",
                debug_log: None,
            },
            ErrorKindStore::InvalidProgressStatus => Self {
                code: "ERR_002_003_003",
                message: "Invalid progress status",
                debug_log: None,
            },
            //============================================================================
            // Verification - ERR_003_003_{SEQUENCE}
            //============================================================================
            ErrorKindStore::VerificationNotFound => Self {
                code: "ERR_003_003_001",
                message: "Verification not found",
                debug_log: None,
            },
            ErrorKindStore::ExistedVerification => Self {
                code: "ERR_003_003_002",
                message: "Existed verification",
                debug_log: None,
            },
            //============================================================================
            // Provider - ERR_004_003_{SEQUENCE}
            //============================================================================
            ErrorKindStore::ProviderNotFound => Self {
                code: "ERR_004_003_001",
                message: "Provider not found",
                debug_log: None,
            },
            ErrorKindStore::ExistedProvider => Self {
                code: "ERR_004_003_002",
                message: "Existed provider",
                debug_log: None,
            },
            //============================================================================
            // Build Config - ERR_005_003_{SEQUENCE}
            //============================================================================
            ErrorKindStore::BuildConfigNotFound => Self {
                code: "ERR_005_003_001",
                message: "Build Config not found",
                debug_log: None,
            },
            ErrorKindStore::BuildConfigExisted => Self {
                code: "ERR_005_003_002",
                message: "Existed Build Config",
                debug_log: None,
            },
        }
    }
}
