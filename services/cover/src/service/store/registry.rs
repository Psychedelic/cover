use crate::common::types::{CallerId, CanisterId, RequestId};
use crate::service::store::error::{Error, ErrorKind};
use ic_cdk::export::candid::CandidType;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::ops::{Bound::Included, Not};
use crate::service::types::{ValidationRequest, BuildParams, ValidationResponse};

#[derive(CandidType, Deserialize)]
pub struct ValidationsRegistry {
    request_counter: RequestId,

    requests: BTreeMap<RequestId, ValidationRequest>,
    fresh_requests: Vec<(CanisterId, RequestId)>,
    request_by_canister_id: BTreeMap<CanisterId, Vec<RequestId>>, // lookup helper

    responses: BTreeMap<RequestId, ValidationResponse>,
}

impl Default for ValidationsRegistry {
    fn default() -> Self {
        Self {
            request_counter: 0,
            requests: BTreeMap::new(),
            request_by_canister_id: BTreeMap::new(),
            fresh_requests: Vec::new(),
            responses: BTreeMap::new(),
        }
    }
}

/// Internal store implementation of validation requests
impl ValidationsRegistry {
    /// Add validation request to internal storage
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
                self.request_counter += 1; // increase counter
                self.requests.insert(self.request_counter, ValidationRequest {
                    request_id: None,
                    canister_id: canister_id.clone(),
                    fetched: false,
                    caller_id: caller_id.clone(),
                    build_settings: build_settings.clone(),
                });
                self.fresh_requests.push((canister_id.clone(), self.request_counter));
                // self.canister_requests.conatins_canister_id(canister_id, self.count).ok_or_else(||
                //   request.add(canister_id, self.count)
                // );
            })
            .ok_or_else(|| Error::new(ErrorKind::FetchRequestNotFound, None))
    }

    pub fn get_request(&self, request_id: RequestId) -> Result<&ValidationRequest, Error> {
        Ok(self.requests.get(&request_id).unwrap())
    }

    /// Get validation request and mark it as fetched
    ///
    /// Return Validation when success
    /// and Error when fail
    pub fn fetch_request(
        &mut self,
        canister_id: &CanisterId,
    ) -> Result<ValidationRequest, Error> {
        let index = self.fresh_requests.iter().position(|(c_id, _vid)| c_id == canister_id).unwrap();
        let (_cid, request_id) = self.fresh_requests.swap_remove(index); // use faster swap_remove
        self.pull_request(request_id)
    }

    pub fn fetch_next_request(&mut self) -> Result<ValidationRequest, Error> {
        if self.fresh_requests.is_empty() {
            return Result::Err(Error::new(ErrorKind::RequestNotFound, None));
        }

        match self.fresh_requests.remove(0) {
            (_canister_id, request_id) => self.pull_request(request_id),
            _ => Result::Err(Error::new(ErrorKind::RequestNotFound, None))
        }
    }

    /// pull requests from fresh list (mark fetched)
    fn pull_request(&mut self, request_id: RequestId) -> Result<ValidationRequest, Error> {
        self.requests
            .get_mut(&request_id)
            .map(|v| {
                v.mark_fetched();
                ValidationRequest {
                    request_id: Some(request_id), // populate request_id
                    ..v.clone()
                }
            })
            .ok_or_else(|| Error::new(ErrorKind::FetchRequestNotFound, None))
    }

    /// Get all stored validation requests
    /// If caller is provided return caller's validations
    /// Otherwise return all requests
    /// Note: Used for debugging purpose
    pub fn list_all_requests(&self, caller: Option<&CallerId>) -> Vec<ValidationRequest> {
        self.requests.iter()
            .filter(|(key, val)|
                match caller {
                    Some(caller_id) => &val.caller_id == caller_id,
                    _ => true, // include all if no filter provided
                })
            .map(|(val_id, val)|
                ValidationRequest {
                    request_id: Some(val_id.clone()),
                    ..val.clone()
                }
            ).collect()
    }

    /// Return list of fresh canister ids
    pub fn list_fresh_requests(&self) -> Vec<&(CanisterId, RequestId)> {
        self.fresh_requests.iter()
            .map(|entry|
                entry
            ).collect()
    }

    pub fn add_response(
        &mut self,
        caller_id: &CallerId,
        resp: &ValidationResponse,
    ) -> Result<(), Error> {
        let mut data = resp.clone();
        data.validator_id = Some(caller_id.clone());
        self.contains_validation(&data.request_id)
            .not()
            .then(|| {
                self.responses.insert(data.request_id, data);
            })
            .ok_or_else(|| Error::new(ErrorKind::AddValidationError, None))
    }

    pub fn contains_request(&self, canister_id: &CanisterId) -> bool {
        self.request_by_canister_id.contains_key(canister_id)
    }

    pub fn contains_validation(&self, req_id: &RequestId) -> bool {
        self.responses.contains_key(req_id)
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
            self.requests.len()
        }
        pub fn count_fresh(&self) -> usize {
            self.fresh_requests.len()
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
            git_tag: "".into(),
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

        assert_eq!(registry.list_fresh_requests(), vec![
            &(fake_canister1(), 1),
            &(fake_canister2(), 2),
            &(fake_canister3(), 3),
        ]);
    }

    #[test]
    fn fetching_request_ok() {
        let mut registry = fake_registry();
        registry.add_request(&fake_caller1(), &fake_canister1(), &fake_build_params());
        registry.add_request(&fake_caller1(), &fake_canister2(), &fake_build_params());
        registry.add_request(&fake_caller1(), &fake_canister3(), &fake_build_params());

        assert_eq!(registry.list_fresh_requests(), vec![
            &(fake_canister1(), 1),
            &(fake_canister2(), 2),
            &(fake_canister3(), 3),
        ]);

        {
            let req1 = registry.fetch_request(&fake_canister1()).unwrap();
            assert_eq!(req1.caller_id, fake_caller1());
            assert_eq!(req1.canister_id, fake_canister1());
            assert_eq!(req1.fetched, true);
        }

        // sort ids to deal with shuffle
        assert_eq!(registry.list_fresh_requests().sort(), vec![
            &fake_canister2(),
            &fake_canister3(),
        ].sort());
        assert_eq!(registry.list_fresh_requests().len(), 2); // removed from fresh
        assert_eq!(registry.list_all_requests(None), vec![
            ValidationRequest {
                request_id: Some(1),
                canister_id: fake_canister1(),
                caller_id: fake_caller1(),
                build_settings: fake_build_params(),
                fetched: true,
            },
            ValidationRequest {
                request_id: Some(2),
                canister_id: fake_canister2(),
                caller_id: fake_caller1(),
                build_settings: fake_build_params(),
                fetched: false,
            },
            ValidationRequest {
                request_id: Some(3),
                canister_id: fake_canister3(),
                caller_id: fake_caller1(),
                build_settings: fake_build_params(),
                fetched: false,
            },
        ]);
    }
}
