use std::collections::{BTreeMap, VecDeque};

use crate::common::types::{CallerId, CanisterId, ReqId};
use crate::service::types::{BuildSettings, ValidationResponse};

/// Batch request buffer
const MAX_BATCH_REQ: ReqId = 10;

#[derive(Debug, PartialEq)]
pub struct ValidationsRegistry {
    /// Validation request counter <=> last request id
    pub last_request_id: ReqId,

    /// Result history of validation by canisters
    pub history_canisters: BTreeMap<CanisterId, ValidationResponse>,

    /// Pending batch request queue
    /// FIFO -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    ///         [11, 12, 13, 14, 15, None, None, None, None, None] <- LILO
    pub pending_request: VecDeque<[Option<ValidationRequest>; MAX_BATCH_REQ as usize]>,

    /// On going validation
    pub in_progress: VecDeque<ReqId>,

    /// Last consumed request index
    pub last_consumed_request_index: ReqId,
}

#[derive(Debug, PartialEq)]
pub struct ValidationRequest {
    pub request_id: ReqId,
    pub caller_id: CallerId,
    pub canister_id: CanisterId,
    pub build_settings: BuildSettings,
}

impl Default for ValidationsRegistry {
    fn default() -> Self {
        Self {
            last_request_id: 0,
            history_canisters: BTreeMap::default(),
            pending_request: VecDeque::default(),
            in_progress: VecDeque::default(),
            last_consumed_request_index: 0,
        }
    }
}

impl ValidationsRegistry {
    fn current_pending_index(&self) -> ReqId {
        self.last_request_id % MAX_BATCH_REQ
    }

    fn should_create_new_batch(&self) -> bool {
        if let Some(p) = self.pending_request.back() {
            return self.current_pending_index() == 0
                && p.get(self.current_pending_index() as usize).is_some();
        }
        true
    }

    fn create_new_batch(&mut self) {
        // workaround https://github.com/rust-lang/rust/issues/44796
        let new_batch: [Option<ValidationRequest>; MAX_BATCH_REQ as usize] = Default::default();
        self.pending_request.push_back(new_batch);
    }

    pub fn add_request(
        &mut self,
        caller_id: CallerId,
        canister_id: CanisterId,
        build_settings: BuildSettings,
    ) {
        let index = self.current_pending_index();

        if self.should_create_new_batch() {
            self.create_new_batch();
        }

        let last_batch = self.pending_request.back_mut().unwrap(); // safe here

        self.last_request_id += 1;

        last_batch[index as usize] = Some(ValidationRequest {
            request_id: self.last_request_id,
            caller_id,
            canister_id,
            build_settings,
        });
    }

    pub fn get_pending_request_by_id(&self, request_id: ReqId) -> Option<&ValidationRequest> {
        if request_id <= self.last_consumed_request_index || request_id > self.last_request_id {
            return None;
        }

        let offset_batch = self.last_consumed_request_index / MAX_BATCH_REQ;

        let target_batch = (request_id - 1) / MAX_BATCH_REQ - offset_batch;

        let target_index = (request_id - 1) % MAX_BATCH_REQ;

        self.pending_request
            .get(target_batch as usize)?
            .get(target_index as usize)?
            .as_ref()
    }

    pub fn get_pending_request(&self) -> Vec<&ValidationRequest> {
        self.pending_request
            .iter()
            .flat_map(|b| {
                b.iter()
                    .filter(|p| p.is_some())
                    .map(|p| p.as_ref().unwrap())
                    .collect::<Vec<&ValidationRequest>>()
            })
            .collect::<Vec<&ValidationRequest>>()
    }

    //   pub fn fetch_request(&mut self, canister_id: &CanisterId) -> Result<ValidationRequest, Error> {
    //     let index = self
    //       .fresh_requests
    //       .iter()
    //       .position(|(c_id, _vid)| c_id == canister_id)
    //       .unwrap();
    //     let (_cid, request_id) = self.fresh_requests.swap_remove(index); // use faster swap_remove
    //     self.pull_request(request_id)
    //   }
    //
    //   pub fn fetch_next_request(&mut self) -> Result<ValidationRequest, Error> {
    //     if self.fresh_requests.is_empty() {
    //       return Result::Err(Error::new(ErrorKind::RequestNotFound, None));
    //     }
    //
    //     match self.fresh_requests.remove(0) {
    //       (_canister_id, request_id) => self.pull_request(request_id),
    //       _ => Result::Err(Error::new(ErrorKind::RequestNotFound, None)),
    //     }
    //   }
    //
    //   /// pull requests from fresh list (mark fetched)
    //   fn pull_request(&mut self, request_id: ReqCnter) -> Result<ValidationRequest, Error> {
    //     self.requests
    //       .get_mut(&request_id)
    //       .map(|v| {
    //         v.mark_fetched();
    //         ValidationRequest {
    //           request_id: Some(request_id), // populate request_id
    //           ..v.clone()
    //         }
    //       })
    //       .ok_or_else(|| Error::new(ErrorKind::FetchRequestNotFound, None))
    //   }
    //
    //   /// Get all stored validation requests
    //   /// If caller is provided return caller's validations
    //   /// Otherwise return all requests
    //   /// Note: Used for debugging purpose
    //   pub fn list_all_requests(&self, caller: Option<&CallerId>) -> Vec<ValidationRequest> {
    //     self.requests
    //       .iter()
    //       .filter(|(key, val)| match caller {
    //         Some(caller_id) => &val.caller_id == caller_id,
    //         _ => true, // include all if no filter provided
    //       })
    //       .map(|(val_id, val)| ValidationRequest {
    //         request_id: Some(val_id.clone()),
    //         ..val.clone()
    //       })
    //       .collect()
    //   }
    //
    //   /// Return list of fresh canister ids
    //   pub fn list_fresh_requests(&self) -> Vec<&(CanisterId, ReqCnter)> {
    //     self.fresh_requests.iter().map(|entry| entry).collect()
    //   }
    //
    //   pub fn list_all_responses(&self, caller: Option<&CallerId>) -> Vec<ValidationResponse> {
    //     self.responses
    //       .iter()
    //       .filter(|(key, val)| match caller {
    //         Some(caller_id) => &val.validator_id.unwrap() == caller_id,
    //         _ => true, // include all if no filter provided
    //       })
    //       .map(|(val_id, val)| val.clone())
    //       .collect()
    //   }
    //
    //   pub fn add_response(
    //     &mut self,
    //     caller_id: &CallerId,
    //     resp: &ValidationResponse,
    //   ) -> Result<(), Error> {
    //     let mut data = resp.clone();
    //     data.validator_id = Some(caller_id.clone());
    //     self.contains_validation(&data.request_id)
    //       .not()
    //       .then(|| {
    //         self.responses.insert(data.request_id, data);
    //       })
    //       .ok_or_else(|| Error::new(ErrorKind::AddValidationError, None))
    //   }
    //
    //   pub fn get_response(&self, request_id: ReqCnter) -> Result<&ValidationResponse, Error> {
    //     Ok(self.responses.get(&request_id).unwrap())
    //   }
    //
    //   pub fn contains_request(&self, canister_id: &CanisterId) -> bool {
    //     self.request_by_canister_id.contains_key(canister_id)
    //   }
    //
    //   pub fn contains_validation(&self, request_id: &ReqCnter) -> bool {
    //     self.responses.contains_key(request_id)
    //   }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::service::store::test_data;
    use ic_kit::*;

    #[test]
    fn initial_state_ok() {
        let store = ValidationsRegistry::default();
        assert_eq!(store.last_request_id, 0);
        assert_eq!(store.history_canisters, BTreeMap::default());
        assert_eq!(store.pending_request, VecDeque::default());
        assert_eq!(store.in_progress, VecDeque::default());
        assert_eq!(store.last_consumed_request_index, 0);

        assert_eq!(
            store.get_pending_request(),
            Vec::<&ValidationRequest>::default()
        );
    }

    #[test]
    fn add_request_ok() {
        let store = test_data::fake_store_with_pending(0, 11);
        assert_eq!(store.last_request_id, 11);
        assert_eq!(store.history_canisters, BTreeMap::default());
        assert_eq!(
            store.pending_request,
            VecDeque::from(vec![
                [
                    Some(ValidationRequest {
                        request_id: 1,
                        caller_id: mock_principals::bob(),
                        canister_id: test_data::fake_canister1(),
                        build_settings: test_data::fake_build_settings1(),
                    }),
                    Some(ValidationRequest {
                        request_id: 2,
                        caller_id: mock_principals::alice(),
                        canister_id: test_data::fake_canister2(),
                        build_settings: test_data::fake_build_settings2(),
                    }),
                    Some(ValidationRequest {
                        request_id: 3,
                        caller_id: mock_principals::bob(),
                        canister_id: test_data::fake_canister1(),
                        build_settings: test_data::fake_build_settings1(),
                    }),
                    Some(ValidationRequest {
                        request_id: 4,
                        caller_id: mock_principals::alice(),
                        canister_id: test_data::fake_canister2(),
                        build_settings: test_data::fake_build_settings2(),
                    }),
                    Some(ValidationRequest {
                        request_id: 5,
                        caller_id: mock_principals::bob(),
                        canister_id: test_data::fake_canister1(),
                        build_settings: test_data::fake_build_settings1(),
                    }),
                    Some(ValidationRequest {
                        request_id: 6,
                        caller_id: mock_principals::alice(),
                        canister_id: test_data::fake_canister2(),
                        build_settings: test_data::fake_build_settings2(),
                    }),
                    Some(ValidationRequest {
                        request_id: 7,
                        caller_id: mock_principals::bob(),
                        canister_id: test_data::fake_canister1(),
                        build_settings: test_data::fake_build_settings1(),
                    }),
                    Some(ValidationRequest {
                        request_id: 8,
                        caller_id: mock_principals::alice(),
                        canister_id: test_data::fake_canister2(),
                        build_settings: test_data::fake_build_settings2(),
                    }),
                    Some(ValidationRequest {
                        request_id: 9,
                        caller_id: mock_principals::bob(),
                        canister_id: test_data::fake_canister1(),
                        build_settings: test_data::fake_build_settings1(),
                    }),
                    Some(ValidationRequest {
                        request_id: 10,
                        caller_id: mock_principals::alice(),
                        canister_id: test_data::fake_canister2(),
                        build_settings: test_data::fake_build_settings2(),
                    }),
                ],
                [
                    Some(ValidationRequest {
                        request_id: 11,
                        caller_id: mock_principals::bob(),
                        canister_id: test_data::fake_canister1(),
                        build_settings: test_data::fake_build_settings1(),
                    }),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ]
            ])
        );
        assert_eq!(store.in_progress, VecDeque::default());
        assert_eq!(store.last_consumed_request_index, 0);
    }

    #[test]
    fn get_pending_request_by_id_ok() {
        let len = 15;
        for offset in 0..len {
            let store = test_data::fake_store_with_pending(offset, len as usize);
            assert_eq!(store.last_request_id, len + offset);

            // outbound
            let result = store.get_pending_request_by_id(offset);
            assert_eq!(result, None);

            // outbound
            let result = store.get_pending_request_by_id(len + offset + 1);
            assert_eq!(result, None);

            // inbound
            for request_id in (offset + 1)..(len + offset + 1) {
                let result = store.get_pending_request_by_id(request_id);
                assert_eq!(result.unwrap().request_id, request_id);
            }
        }
    }
}
