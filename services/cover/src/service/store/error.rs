#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    PendingRequestNotFound,
    InitExistedProgress,
    ProgressNotFound,
}
