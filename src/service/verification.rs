use crate::common::types::CanisterId;
use crate::service::model::pagination::{Pagination, PaginationInfo};
use crate::service::model::verification::{SubmitVerification, Verification};
use crate::service::{verification_store, verification_store_mut};

pub fn get_verification_by_canister_id(canister_id: &CanisterId) -> Option<&'static Verification> {
    verification_store().get_verification_by_canister_id(canister_id)
}

pub fn get_verifications(pagination_info: &PaginationInfo) -> Pagination<&'static Verification> {
    verification_store().get_verifications(pagination_info)
}

pub fn submit_verification(submit_verification: SubmitVerification) {
    verification_store_mut().submit_verification(submit_verification)
}
