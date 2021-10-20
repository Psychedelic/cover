use crate::service::constants::{Error, Ok};
use ic_cdk::export::candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct ValidationResult<T> {
    pub data: Option<T>,
    pub is_error: bool,
    pub message: Option<String>,
    pub code: Option<String>,
}

impl<T> ValidationResult<T> {
    pub fn data(data: T) -> Self {
        Self {
            data: Some(data),
            ..ValidationResult::empty_result()
        }
    }
    pub fn success(ok: Ok) -> Self {
        Self {
            message: Some(ok.message),
            code: Some(ok.code),
            ..ValidationResult::empty_result()
        }
    }
    pub fn data_with_message(data: T, ok: Ok) -> Self {
        Self {
            data: Some(data),
            message: Some(ok.message),
            code: Some(ok.code),
            ..ValidationResult::empty_result()
        }
    }
    pub fn fail(error: Error) -> Self {
        Self {
            data: None,
            is_error: true,
            message: Some(error.message),
            code: Some(error.code),
        }
    }
    fn empty_result() -> Self {
        Self {
            data: None,
            is_error: false,
            message: None,
            code: None,
        }
    }
}
