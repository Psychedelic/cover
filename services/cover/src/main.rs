mod api;
mod common;
mod service;
mod upgrade;

#[cfg(any(target_arch = "wasm32"))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    use crate::common::types::{CanisterId, ProviderId, ReqId};
    use crate::service::model::build_config::*;
    use crate::service::model::error::Error;
    use crate::service::model::progress::*;
    use crate::service::model::provider::*;
    use crate::service::model::request::*;
    use crate::service::model::verification::*;

    ic_kit::candid::export_service!();
    std::print!("{}", __export_service());
}
