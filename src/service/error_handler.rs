use crate::service::model::error::Error;
use crate::service::store::error::ErrorKindStore;

// ERR_{MODULE}_{LEVEL}_{SEQUENCE}
//      MODULE
//          General               000
//          Verification          001
//          Provider              002
//      LEVEL
//          Api                   001       (inter-canister, external api...)
//          Service               002
//          Store                 003
// pub enum ErrorKindApi {
// //============================================================================
// // General
// //============================================================================
// }

// ERR_{MODULE}_001_{SEQUENCE}
// impl From<ErrorKindApi> for Error {
//     fn from(kind: ErrorKindApi) -> Self {
//         match kind {
//             //============================================================================
//             // General - ERR_000_001_{SEQUENCE}
//             //============================================================================
//
//         }
//     }
// }

// pub enum ErrorKindService {
// //============================================================================
// // Verification
// //============================================================================
// //
// //============================================================================
// // Provider
// //============================================================================
// //
// }

// ERR_{MODULE}_002_{SEQUENCE}
// impl From<ErrorKindService> for Error {
//     fn from(kind: ErrorKindService) -> Self {
//         match kind {
//             //============================================================================
//             // Verification - ERR_003_002_{SEQUENCE}
//             //============================================================================
//             //
//             //============================================================================
//             // Provider - ERR_004_002_{SEQUENCE}
//             //============================================================================
//             //
//         }
//     }
// }

// ERR_{MODULE}_003_{SEQUENCE}
impl From<ErrorKindStore> for Error {
    fn from(kind: ErrorKindStore) -> Self {
        match kind {
            //============================================================================
            // Verification - ERR_001_003_{SEQUENCE}
            //============================================================================
            ErrorKindStore::VerificationNotFound => Self {
                code: "ERR_001_003_001",
                message: "Verification not found",
                debug_log: None,
            },
            ErrorKindStore::ExistedVerification => Self {
                code: "ERR_001_003_002",
                message: "Existed verification",
                debug_log: None,
            },
            //============================================================================
            // Provider - ERR_002_003_{SEQUENCE}
            //============================================================================
            ErrorKindStore::ProviderNotFound => Self {
                code: "ERR_002_003_001",
                message: "Provider not found",
                debug_log: None,
            },
            ErrorKindStore::ExistedProvider => Self {
                code: "ERR_002_003_002",
                message: "Existed provider",
                debug_log: None,
            },
            //============================================================================
            // Build Config - ERR_003_003_{SEQUENCE}
            //============================================================================
            ErrorKindStore::BuildConfigNotFound => Self {
                code: "ERR_003_003_001",
                message: "Build config not found",
                debug_log: None,
            },
            ErrorKindStore::ExistedBuildConfig => Self {
                code: "ERR_003_003_002",
                message: "Existed build config",
                debug_log: None,
            },
            //============================================================================
            // Admin - ERR_004_003_{SEQUENCE}
            //============================================================================
            ErrorKindStore::AdminNotFound => Self {
                code: "ERR_004_003_001",
                message: "Admin not found",
                debug_log: None,
            },
            ErrorKindStore::ExistedAdmin => Self {
                code: "ERR_004_003_002",
                message: "Existed admin",
                debug_log: None,
            },
        }
    }
}
