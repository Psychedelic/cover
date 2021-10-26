pub struct Ok {
    pub message: String,
    pub code: String,
}

// OK CODE following pattern:
// OK_{context_code}_{code}
// CONTEXT CODE:
//   - 01: Canister
//   - ...
impl Ok {
    pub fn validation_request_added() -> Self {
        Self {
            message: "Added validation request for the canister".into(),
            code: "OK_100_001".into(),
        }
    }
}

pub struct Error {
    pub message: String,
    pub code: String,
}

// ERROR CODE following pattern:
// ERR_{context_code}_{code}
// CONTEXT CODE:
//   - 01: Canister
//   - ...
impl Error {
    pub fn validation_requested() -> Self {
        Self {
            message: "Validation request for the canister already exists".into(),
            code: "ERR_001_001".into(),
        }
    }
    pub fn request_not_found() -> Self {
        Self {
            message: "Fetch request failed. Request not found".into(),
            code: "ERR_100_008".into(),
        }
    }
}
