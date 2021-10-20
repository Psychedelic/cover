use crate::common::types::{CallerId, CanisterId, ValidationId};
use crate::service::store::error::{Error, ErrorKind};
use ic_cdk::export::candid::CandidType;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::ops::{Bound::Included, Not};
use crate::service::types::{Validation, BuildParams};

#[derive(CandidType, Deserialize)]
pub struct ValidationsRegistry {
  count: ValidationId,
  // unique counter
  pub fresh: Vec<ValidationId>,
  pub requests: BTreeMap<CanisterId, Vec<ValidationId>>,
  pub validations: BTreeMap<ValidationId, Validation>,
}

impl Default for ValidationsRegistry {
  fn default() -> Self {
    Self {
      count: 0,
      fresh: Vec::new(),
      requests: BTreeMap::new(),
      validations: BTreeMap::new(),
    }
  }
}

/// Internal store implementation of validation requests
impl ValidationsRegistry {
  /// Return list canisters internal information
  pub fn get_requests(&self) -> Vec<(&CanisterId, &ValidationId, &Validation)> {
    self.requests.iter().collect()
  }

  /// Add validation request to internal storage
  ///
  /// Return () when success
  /// and Error when fail
  pub fn add_request(
    &mut self,
    caller_id: CallerId,
    canister_id: CanisterId,
    build_settings: BuildParams,
  ) -> Result<(ValidationId), Error> {
    self.contains_request(canister_id)
      .not()
      .then(|| {
        self.count += 1; // increase counter
        self.validations.insert(self.count, Validation {
          fetched: false,
          caller_id,
          build_settings: build_settings.clone(),
        });
        self.fresh.add(self.count);
        // self.requests.conatins_canister_id(canister_id, self.count).ok_or_else(||  )
      })
      .ok_or_else(|| Error::new(ErrorKind::AddExistedCanister, None))
  }

  /// Get request and mark it as fetched
  ///
  /// Return Validation when success
  /// and Error when fail
  pub fn fetch_request(
    &mut self,
    validation_id: ValidationId,
  ) -> Result<&Validation, Error> {
    self.validations
      .get_mut(&validation_id)
      .map(|v| v.mark_fetched())
      .ok_or_else(|| Error::new(ErrorKind::CanisterNotFound, None))
  }
}

#[cfg(test)]
pub mod test {
  use super::*;
  use ic_kit::*;

  impl ValidationsRegistry {
    pub fn count(&self) -> usize {
      self.validations.len()
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

  pub fn fake_store(store: Option<BTreeMap<RequestKey, CanisterInfoInternal>>) -> ValidationsRegistry {
    ValidationsRegistry {
      requests: store.unwrap_or(BTreeMap::new()),
    }
  }

  pub fn fake_data() -> BTreeMap<RequestKey, CanisterInfoInternal> {
    let mut store = BTreeMap::new();
    store.insert(
      (mock_principals::bob(), fake_canister1()),
      CanisterInfoInternal {
        name: "Bob canister 1".into(),
        canister_type: "".into(),
      },
    );
    store.insert(
      (mock_principals::bob(), fake_canister2()),
      CanisterInfoInternal {
        name: "Bob canister 2".into(),
        canister_type: "".into(),
      },
    );
    store.insert(
      (mock_principals::bob(), fake_canister3()),
      CanisterInfoInternal {
        name: "Bob canister 3".into(),
        canister_type: "".into(),
      },
    );
    store.insert(
      (mock_principals::alice(), fake_canister4()),
      CanisterInfoInternal {
        name: "Alice canister 1".into(),
        canister_type: "".into(),
      },
    );
    store.insert(
      (mock_principals::alice(), fake_canister5()),
      CanisterInfoInternal {
        name: "Alice canister 2".into(),
        canister_type: "".into(),
      },
    );
    store
  }

  #[test]
  fn initial_state_ok() {
    let store = fake_store(None);
    assert_eq!(store.get_all(CallerId::anonymous()).len(), 0);
  }

  #[test]
  fn get_all_ok() {
    let store = fake_store(Some(fake_data()));
    assert_eq!(
      store.get_all(mock_principals::alice()),
      vec![
        (
          (&fake_canister4()),
          &CanisterInfoInternal {
            name: "Alice canister 1".into(),
            canister_type: "".into(),
          },
        ),
        (
          (&fake_canister5()),
          &CanisterInfoInternal {
            name: "Alice canister 2".into(),
            canister_type: "".into(),
          }
        ),
      ]
    )
  }

  #[test]
  fn get_by_id_return_some() {
    let store = fake_store(Some(fake_data()));
    assert_eq!(
      store.get_canister(mock_principals::alice(), fake_canister5()),
      Some(&CanisterInfoInternal {
        name: "Alice canister 2".into(),
        canister_type: "".into(),
      })
    )
  }

  #[test]
  fn get_by_id_return_none() {
    let store = fake_store(Some(fake_data()));
    assert_eq!(
      store.get_canister(mock_principals::bob(), fake_canister5()),
      None
    )
  }

  #[test]
  fn contains_canister_true() {
    let store = fake_store(Some(fake_data()));
    assert!(store.contains_request(mock_principals::alice(), fake_canister5()))
  }

  #[test]
  fn contains_canister_false() {
    let store = fake_store(Some(fake_data()));
    assert!(!store.contains_request(mock_principals::bob(), fake_canister5()))
  }

  #[test]
  fn add_canister_ok() {
    let mut store = fake_store(Some(fake_data()));
    assert_eq!(
      store.add_canister(
        mock_principals::alice(),
        fake_canister6(),
        CanisterInfoInternal {
          name: "Alice canister 3".into(),
          canister_type: "".into(),
        },
      ),
      Ok(())
    );
    assert_eq!(store.validations.len(), 6);
    let canister = store
      .validations
      .get(&(mock_principals::alice(), fake_canister6()))
      .unwrap();
    assert_eq!(
      canister,
      &CanisterInfoInternal {
        name: "Alice canister 3".into(),
        canister_type: "".into(),
      }
    );
  }

  #[test]
  fn add_canister_error() {
    let mut store = fake_store(Some(fake_data()));
    assert_eq!(
      store.add_request(
        mock_principals::alice(),
        fake_canister5(),
        CanisterInfoInternal {
          name: "Alice canister 2".into(),
          canister_type: "".into(),
        },
      ),
      Err(Error::new(ErrorKind::AddExistedCanister, None))
    );
    assert_eq!(store.validations.len(), 5);
  }

  #[test]
  fn remove_canister_ok() {
    let mut store = fake_store(Some(fake_data()));
    assert_eq!(
      store.remove_canister(mock_principals::alice(), fake_canister5()),
      Ok(CanisterInfoInternal {
        name: "Alice canister 2".into(),
        canister_type: "".into(),
      })
    );
    assert_eq!(store.validations.len(), 4);
    let canister = store
      .validations
      .get(&(mock_principals::alice(), fake_canister5()));
    assert_eq!(canister, None);
  }

  #[test]
  fn remove_canister_error() {
    let mut store = fake_store(Some(fake_data()));
    assert_eq!(
      store.remove_canister(mock_principals::bob(), fake_canister5()),
      Err(Error::new(ErrorKind::CanisterNotFound, None))
    );
    assert_eq!(store.validations.len(), 5);
  }

  #[test]
  fn update_canister_type_ok() {
    let mut store = fake_store(Some(fake_data()));
    assert_eq!(
      store.update_canister_type(
        mock_principals::alice(),
        fake_canister5(),
        "new type".into(),
      ),
      Ok(())
    );
    let canister = store
      .validations
      .get(&(mock_principals::alice(), fake_canister5()))
      .unwrap();
    assert_eq!(canister.canister_type, "new type");
  }

  #[test]
  fn update_canister_type_error() {
    let mut store = fake_store(Some(fake_data()));
    assert_eq!(
      store.update_canister_type(mock_principals::bob(), fake_canister5(), "new type".into()),
      Err(Error::new(ErrorKind::CanisterNotFound, None))
    );
  }

  #[test]
  fn update_canister_name_ok() {
    let mut store = fake_store(Some(fake_data()));
    assert_eq!(
      store.update_canister_name(
        mock_principals::alice(),
        fake_canister5(),
        "new name".into(),
      ),
      Ok(())
    );
    let canister = store
      .validations
      .get(&(mock_principals::alice(), fake_canister5()))
      .unwrap();
    assert_eq!(canister.name, "new name");
  }

  #[test]
  fn update_canister_name_error() {
    let mut store = fake_store(Some(fake_data()));
    assert_eq!(
      store.update_canister_name(mock_principals::bob(), fake_canister5(), "new new".into()),
      Err(Error::new(ErrorKind::CanisterNotFound, None))
    );
  }

  #[test]
  fn archive_ok() {
    let mut store = fake_store(Some(fake_data()));
    assert_eq!(
      store.archive(),
      fake_data()
        .into_iter()
        .collect::<Vec<(RequestKey, CanisterInfoInternal)>>()
    )
  }

  #[test]
  fn load() {
    let mut store = fake_store(None);
    store.load(
      fake_data()
        .into_iter()
        .collect::<Vec<(RequestKey, CanisterInfoInternal)>>(),
    );
    assert_eq!(fake_data(), store.validations)
  }
}
