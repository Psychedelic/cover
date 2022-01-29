use crate::common::types::CanisterId;
use crate::service::model::error::Error;
use crate::service::model::pagination::{Pagination, PaginationInfo};
use crate::service::model::verification::{
    BuildStatus, RegisterVerification, SubmitVerification, Verification,
};
use crate::service::{activity_store_mut, verification_store, verification_store_mut};

pub fn get_verification_by_canister_id(canister_id: &CanisterId) -> Option<&'static Verification> {
    verification_store().get_verification_by_canister_id(canister_id)
}

pub fn get_verifications(pagination_info: &PaginationInfo) -> Pagination<&'static Verification> {
    verification_store().get_verifications(pagination_info)
}

pub fn submit_verification(submit_verification: SubmitVerification) {
    activity_store_mut().add_activity(
        &submit_verification.canister_id,
        &submit_verification.build_status,
    );

    verification_store_mut().submit_verification(submit_verification)
}

pub fn register_verification(register_verification: RegisterVerification) -> Result<(), Error> {
    let canister_id = register_verification.canister_id;
    let result = verification_store_mut().register_verification(register_verification);

    result
        .is_ok()
        .then(|| (activity_store_mut().add_activity(&canister_id, &BuildStatus::Pending)));

    result
}
