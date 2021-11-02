use crate::common::types::CanisterId;

pub fn fake_canister1() -> CanisterId {
    CanisterId::from_text("rkp4c-7iaaa-aaaaa-aaaca-cai").unwrap()
}

pub fn fake_canister2() -> CanisterId {
    CanisterId::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()
}

// pub fn fake_canister3() -> CanisterId {
//     CanisterId::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap()
// }
//
// pub fn fake_canister4() -> CanisterId {
//     CanisterId::from_text("rno2w-sqaaa-aaaaa-aaacq-cai").unwrap()
// }
//
// pub fn fake_canister5() -> CanisterId {
//     CanisterId::from_text("renrk-eyaaa-aaaaa-aaada-cai").unwrap()
// }
