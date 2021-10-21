use crate::common::types::{CallerId, CanisterId, ValidationId};
use crate::service::store::error::{Error, ErrorKind};
use ic_cdk::export::candid::CandidType;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::ops::{Bound::Included, Not};
use crate::service::types::{ValidationRequest, BuildParams};

#[derive(CandidType, Deserialize)]
pub struct ValidationsRegistry {
  count: ValidationId,
  pub fresh: Vec<ValidationId>,
  pub canister_requests: BTreeMap<CanisterId, Vec<ValidationId>>,
  pub requests: BTreeMap<ValidationId, ValidationRequest>,
}

impl Default for ValidationsRegistry {
  fn default() -> Self {
    Self {
      count: 0,
      fresh: Vec::new(),
      canister_requests: BTreeMap::new(),
      requests: BTreeMap::new(),
    }
  }
}

/// Internal store implementation of validation requests
impl ValidationsRegistry {
  /// Add validation request to internal storage
  ///
  /// Return () when success
  /// and Error when fail
  pub fn add_request(
    &mut self,
    caller_id: &CallerId,
    canister_id: &CanisterId,
    build_settings: &BuildParams,
  ) -> Result<ValidationId, Error> {
    self.contains_request(canister_id)
      .not()
      .then(|| {
        self.count += 1; // increase counter
        self.requests.insert(self.count, ValidationRequest {
          canister_id: canister_id.clone(),
          fetched: false,
          caller_id: caller_id.clone(),
          build_settings: build_settings.clone(),
        });
        self.fresh.push(self.count);
        // self.canister_requests.conatins_canister_id(canister_id, self.count).ok_or_else(||
        //   request.add(canister_id, self.count)
        // );
        self.count
      })
      .ok_or_else(|| Error::new(ErrorKind::AddExistedCanister, None))
  }

  /// Get request and mark it as fetched
  ///
  /// Return Validation when success
  /// and Error when fail
  pub fn fetch_request(
    &mut self,
    request_id: ValidationId,
  ) -> Result<&ValidationRequest, Error> {
    let index = self.fresh.iter().position(|x| *x == request_id).unwrap();
    self.fresh.remove(index);

    self.requests
      .get_mut(&request_id)
      .map(|v| {
        v.mark_fetched()
      })
      .ok_or_else(|| Error::new(ErrorKind::CanisterNotFound, None))
  }

  pub fn list_all(&self) -> Vec<&ValidationRequest> {
    self.requests.keys().map(|req_id|
      self.requests.get(req_id).unwrap()
    ).collect()
  }

  pub fn list_fresh(&self) -> Vec<&ValidationRequest> {
    self.fresh.iter().map(|req_id|
      self.requests.get(req_id).unwrap()
    ).collect()
  }


  // pub fn update_request(&mut self, validation_id: ValidationId, params: UpdateRequestParams) {
  //   let & mut validation = validations.get(params.validation_id);
  //   validation.setValidationStatus(params);
  // }

  pub fn contains_request(&self, canister_id: &CanisterId) -> bool {
    self.canister_requests.contains_key(canister_id)
  }
}

#[cfg(test)]
pub mod test {
  use super::*;
  use ic_kit::*;

  impl ValidationsRegistry {
    pub fn count(&self) -> usize {
      self.requests.len()
    }
  }

  pub fn fake_canister1() -> CanisterId {
    CanisterId::from_text("rkp4c-7iaaa-aaaaa-aaaca-cai").unwrap()
  }

  pub fn fake_canister2() -> CanisterId {
    CanisterId::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()
  }

  pub fn fake_canister3() -> CanisterId {
    CanisterId::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap()
  }

  pub fn fake_canister4() -> CanisterId {
    CanisterId::from_text("rno2w-sqaaa-aaaaa-aaacq-cai").unwrap()
  }

  pub fn fake_canister5() -> CanisterId {
    CanisterId::from_text("renrk-eyaaa-aaaaa-aaada-cai").unwrap()
  }

  pub fn fake_canister6() -> CanisterId {
    CanisterId::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap()
  }
}
