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
    pub fn canister_not_exist() -> Self {
        Self {
            message: "This canister does not exist".into(),
            code: "ERR_001_002".into(),
        }
    }
    pub fn invalid_canister_controller() -> Self {
        Self {
            message: "Invalid canister controller".into(),
            code: "ERR_001_003".into(),
        }
    }
    pub fn canister_status_failed() -> Self {
        Self {
            message: "Canister status failed".into(),
            code: "ERR_001_004".into(),
        }
    }
    pub fn canister_update_settings_failed() -> Self {
        Self {
            message: "Update canister settings failed".into(),
            code: "ERR_001_005".into(),
        }
    }
    pub fn add_existing_canister_status_check_failed() -> Self {
        Self {
            message: "Add existing canister status check failed".into(),
            code: "ERR_001_006".into(),
        }
    }
    pub fn create_canister_failed() -> Self {
        Self {
            message: "Create canister failed".into(),
            code: "ERR_001_007".into(),
        }
    }
    pub fn canister_install_code_failed() -> Self {
        Self {
            message: "Install code failed".into(),
            code: "ERR_001_008".into(),
        }
    }
}
