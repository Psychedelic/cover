use ic_kit::ic::caller;
use ic_kit::interfaces::management::{CanisterStatus, CanisterStatusResponse, WithCanisterId};
use ic_kit::interfaces::Method;
use ic_kit::macros::{query, update};
use ic_kit::Principal;

#[query]
fn whoami() -> Principal {
    caller()
}

#[update]
async fn canister_status(canister_id: Principal) -> Result<CanisterStatusResponse, String> {
    CanisterStatus::perform(
        Principal::management_canister(),
        (WithCanisterId { canister_id },),
    )
    .await
    .map(|(status,)| Ok(status))
    .unwrap_or_else(|(code, message)| Err(format!("Code: {:?}, Message: {}", code, message)))
}
