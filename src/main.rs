mod api;
mod common;
mod service;
mod upgrade;

#[cfg(any(target_arch = "wasm32"))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    use crate::common::types::*;
    use crate::service::model::build_config::*;
    use crate::service::model::verification::*;

    ic_kit::candid::export_service!();
    std::print!("{}", __export_service());
}
