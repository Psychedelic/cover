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
  fresh: Vec<(CanisterId, ValidationId)>,
  canister_reqs: BTreeMap<CanisterId, Vec<ValidationId>>,
  validation_reqs: BTreeMap<ValidationId, ValidationRequest>,
}

impl Default for ValidationsRegistry {
  fn default() -> Self {
    Self {
      count: 0,
      fresh: Vec::new(),
      canister_reqs: BTreeMap::new(),
      validation_reqs: BTreeMap::new(),
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
  ) -> Result<(), Error> {
    self.contains_request(canister_id)
      .not()
      .then(|| {
        self.count += 1; // increase counter
        self.validation_reqs.insert(self.count, ValidationRequest {
          canister_id: canister_id.clone(),
          fetched: false,
          caller_id: caller_id.clone(),
          build_settings: build_settings.clone(),
        });
        self.fresh.push((canister_id.clone(), self.count));
        // self.canister_requests.conatins_canister_id(canister_id, self.count).ok_or_else(||
        //   request.add(canister_id, self.count)
        // );
      })
      .ok_or_else(|| Error::new(ErrorKind::FetchRequestNotFound, None))
  }

  /// Get request and mark it as fetched
  ///
  /// Return Validation when success
  /// and Error when fail
  pub fn fetch_request(
    &mut self,
    canister_id: &CanisterId,
  ) -> Result<&ValidationRequest, Error> {
    let index = self.fresh.iter().position(|(c_id, _vid)| c_id == canister_id).unwrap();
    let (_cid, request_id) = self.fresh.remove(index);

    self.validation_reqs
      .get_mut(&request_id)
      .map(|v| {
        v.mark_fetched()
      })
      .ok_or_else(|| Error::new(ErrorKind::FetchRequestNotFound, None))
  }

  pub fn list_all(&self) -> Vec<&ValidationRequest> {
    self.validation_reqs.keys().map(|req_id|
      self.validation_reqs.get(req_id).unwrap()
    ).collect()
  }

  /// Return list of fresh canister ids
  pub fn list_fresh(&self) -> Vec<&CanisterId> {
    self.fresh.iter().map(|(canister_id, _req_id)|
      canister_id
    ).collect()
  }


  // pub fn update_request(&mut self, validation_id: ValidationId, params: UpdateRequestParams) {
  //   let & mut validation = validations.get(params.validation_id);
  //   validation.setValidationStatus(params);
  // }

  pub fn contains_request(&self, canister_id: &CanisterId) -> bool {
    self.canister_reqs.contains_key(canister_id)
  }
}


#[cfg(test)]
pub mod test {
  use super::*;
  use ic_kit::*;
  use crate::service::store::registry::ValidationsRegistry;
  use std::collections::BTreeMap;
  use crate::common::types::CallerId;

  impl ValidationsRegistry {
    pub fn count_all(&self) -> usize {
      self.validation_reqs.len()
    }
    pub fn count_fresh(&self) -> usize {
      self.fresh.len()
    }
  }

  pub fn fake_registry() -> ValidationsRegistry { ValidationsRegistry::default() }

  pub fn fake_caller1() -> CallerId { CallerId::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap() }

  pub fn fake_canister1() -> CanisterId { CanisterId::from_text("rkp4c-7iaaa-aaaaa-aaaca-cai").unwrap() }

  pub fn fake_canister2() -> CanisterId { CanisterId::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap() }

  pub fn fake_canister3() -> CanisterId { CanisterId::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap() }

  pub fn fake_build_params() -> BuildParams {
    BuildParams {
      git_ref: "git@github.com/Psychedelic/cover".into(),
      git_sha: "".into(),
    }
  }

  #[test]
  fn adding_request_ok() {
    let mut registry = fake_registry();
    assert_eq!(registry.count_fresh(), 0);
    let r = registry.add_request(&fake_caller1(), &fake_canister1(), &fake_build_params());
    assert_eq!(r, Ok(()));
    assert_eq!(registry.count_fresh(), 1);
    assert_eq!(registry.count_all(), 1);

    let r = registry.add_request(&fake_caller1(), &fake_canister2(), &fake_build_params());
    assert_eq!(r, Ok(()));
    assert_eq!(registry.count_fresh(), 2);
    assert_eq!(registry.count_all(), 2);

    let r = registry.add_request(&fake_caller1(), &fake_canister3(), &fake_build_params());
    assert_eq!(r, Ok(()));
    assert_eq!(registry.count_fresh(), 3);
    assert_eq!(registry.count_all(), 3);

    assert_eq!(registry.list_fresh(), vec![
      &fake_canister1(),
      &fake_canister2(),
      &fake_canister3(),
    ]);
  }

  #[test]
  fn fetching_request_ok() {
    let mut registry = fake_registry();
    registry.add_request(&fake_caller1(), &fake_canister1(), &fake_build_params());
    registry.add_request(&fake_caller1(), &fake_canister2(), &fake_build_params());
    registry.add_request(&fake_caller1(), &fake_canister3(), &fake_build_params());

    assert_eq!(registry.list_fresh(), vec![
      &fake_canister1(),
      &fake_canister2(),
      &fake_canister3(),
    ]);

    {
      let req1 = registry.fetch_request(&fake_canister1()).unwrap();
      assert_eq!(req1.caller_id, fake_caller1());
      assert_eq!(req1.canister_id, fake_canister1());
      assert_eq!(req1.fetched, true);
    }

    assert_eq!(registry.list_fresh(), vec![
      &fake_canister2(),
      &fake_canister3(),
    ]);
    assert_eq!(registry.list_fresh().len(), 2); // removed from fresh
    assert_eq!(registry.list_all(), vec![
      &ValidationRequest {
        canister_id: fake_canister1(),
        caller_id: fake_caller1(),
        build_settings: fake_build_params(),
        fetched: true,
      },
      &ValidationRequest {
        canister_id: fake_canister2(),
        caller_id: fake_caller1(),
        build_settings: fake_build_params(),
        fetched: false,
      },
      &ValidationRequest {
        canister_id: fake_canister3(),
        caller_id: fake_caller1(),
        build_settings: fake_build_params(),
        fetched: false,
      },
    ]);
  }
}
