use crate::common::types::CanisterId;

pub fn fake_canister1() -> CanisterId {
    CanisterId::from_text("rkp4c-7iaaa-aaaaa-aaaca-cai").unwrap()
}

pub fn fake_canister2() -> CanisterId {
    CanisterId::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()
}
