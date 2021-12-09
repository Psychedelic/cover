#[derive(Debug, PartialEq)]
pub enum ErrorKindStore {
    //============================================================================
    // Request
    //============================================================================
    RequestNotFound,
    //============================================================================
    // Progress
    //============================================================================
    InitExistedProgress,
    ProgressNotFound,
    InvalidProgressStatus,
    //============================================================================
    // Verification
    //============================================================================
    VerificationNotFound,
    ExistedVerification,
    //============================================================================
    // Provider
    //============================================================================
    ProviderNotFound,
    ExistedProvider,
    //============================================================================
    // Build Config
    //============================================================================
    BuildConfigNotFound,
    BuildConfigExisted,
}
