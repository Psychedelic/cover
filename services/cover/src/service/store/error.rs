#[derive(Debug, PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: Option<String>,
}

// impl Error {
//   fn new(kind: ErrorKind, message: Option<String>) -> Self {
//     Self { kind, message }
//   }
// }
//
#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    //   PendingRequestNotFound,
}
