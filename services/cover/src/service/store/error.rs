#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    RequestNotFound,
    InitExistedProgress,
    ProgressNotFound,
    InvalidProgressStatus,
    VerificationNotFound,
    ExistedVerification,
}
