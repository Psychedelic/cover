use crate::service::store::error::ErrorKindStore;
use crate::service::types::Error;

// ERR_{MODULE}_{LEVEL}_{SEQUENCE}
//      MODULE
//          Request               001
//          Progress              002
//          Verification          003
//          Provider              004
//      LEVEL
//          Api                   001
//          Service               002
//          Store                 003
pub enum ErrorKindService {
    //============================================================================
    // Verification
    //============================================================================
    InvalidProvider,
}

impl From<ErrorKindService> for Error {
    fn from(kind: ErrorKindService) -> Self {
        match kind {
            //============================================================================
            // Verification
            //============================================================================
            ErrorKindService::InvalidProvider => Self {
                code: "ERR_003_002_001",
                message: "Invalid provider",
                debug_log: None,
            },
        }
    }
}

impl From<ErrorKindStore> for Error {
    fn from(kind: ErrorKindStore) -> Self {
        match kind {
            //============================================================================
            // Request
            //============================================================================
            ErrorKindStore::RequestNotFound => Self {
                code: "ERR_001_001_001",
                message: "Request not found",
                debug_log: None,
            },
            //============================================================================
            // Progress
            //============================================================================
            ErrorKindStore::ProgressNotFound => Self {
                code: "ERR_002_001_001",
                message: "Progress not found",
                debug_log: None,
            },
            ErrorKindStore::InitExistedProgress => Self {
                code: "ERR_002_001_002",
                message: "Init existed progress",
                debug_log: None,
            },
            ErrorKindStore::InvalidProgressStatus => Self {
                code: "ERR_002_001_003",
                message: "Invalid progress status",
                debug_log: None,
            },
            //============================================================================
            // Verification
            //============================================================================
            ErrorKindStore::VerificationNotFound => Self {
                code: "ERR_003_001_001",
                message: "Verification not found",
                debug_log: None,
            },
            ErrorKindStore::ExistedVerification => Self {
                code: "ERR_003_001_002",
                message: "Existed verification",
                debug_log: None,
            },
            //============================================================================
            // Provider
            //============================================================================
            ErrorKindStore::ProviderNotFound => Self {
                code: "ERR_004_001_001",
                message: "Provider not found",
                debug_log: None,
            },
            ErrorKindStore::ExistedProvider => Self {
                code: "ERR_004_001_002",
                message: "Existed provider",
                debug_log: None,
            },
        }
    }
}
