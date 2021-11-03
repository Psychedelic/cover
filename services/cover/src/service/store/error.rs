#[derive(Debug, PartialEq)]
pub struct Error {
    code: &'static str,
    message: &'static str,
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    /// 001
    PendingRequestNotFound(Error),
}

impl ErrorKind {
    pub fn pending_request_not_found() -> ErrorKind {
        ErrorKind::PendingRequestNotFound(Error {
            code: "ERR_001_001",
            message: "Pending request not found.",
        })
    }
}
