#[derive(Debug, PartialEq)]
pub enum ErrorKindStore {
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
    ExistedBuildConfig,
    //============================================================================
    // Admin
    //============================================================================
    AdminNotFound,
    ExistedAdmin,
}
