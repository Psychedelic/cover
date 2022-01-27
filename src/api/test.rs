use ic_kit::candid::candid_method;
use ic_kit::macros::{query, update};

type COUNTER = u32;

#[update(name = "test")]
#[candid_method(update, rename = "test")]
fn test() {
    *(ic_kit::ic::get_mut::<COUNTER>()) += 1;
}

#[query(name = "check")]
#[candid_method(query, rename = "check")]
fn check() -> u32 {
    *ic_kit::ic::get::<COUNTER>()
}
