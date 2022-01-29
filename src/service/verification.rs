use crate::common::types::CanisterId;
use crate::service::model::error::Error;
use crate::service::model::pagination::{Pagination, PaginationInfo};
use crate::service::model::verification::{RegisterVerification, SubmitVerification, Verification};
use crate::service::{activity_store_mut, verification_store, verification_store_mut};

pub fn get_verification_by_canister_id(canister_id: &CanisterId) -> Option<&'static Verification> {
    verification_store().get_verification_by_canister_id(canister_id)
}

pub fn get_verifications(pagination_info: &PaginationInfo) -> Pagination<&'static Verification> {
    verification_store().get_verifications(pagination_info)
}

pub fn submit_verification(submit_verification: SubmitVerification) {
    verification_store_mut()
        .submit_verification(submit_verification, |canister_id, build_status| {
            activity_store_mut().add_activity(canister_id, build_status)
        })
}

pub fn register_verification(register_verification: RegisterVerification) -> Result<(), Error> {
    verification_store_mut()
        .register_verification(register_verification, |canister_id, build_status| {
            activity_store_mut().add_activity(canister_id, build_status)
        })
}
