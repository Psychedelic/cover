use std::collections::{BTreeMap, VecDeque};

use crate::common::types::{CallerId, CanisterId, ReqId};
use crate::service::store::error::Error;
use crate::service::types::{BuildSettings, ValidationResponse};

/// Batch request buffer
const MAX_BATCH_REQ: ReqId = 10;

#[derive(Debug, PartialEq)]
pub struct ValidationsRegistry {
    /// Validation request counter <=> last request id
    pub req_counter: ReqId,

    /// Result history of validations by canisters
    pub history_canisters: BTreeMap<CanisterId, ValidationResponse>,

    /// Pending batch quest queue
    /// FIFO -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    ///         [11, 12, 13, 14, 15, None, None, None, None, None] <- LILO
    pub pending_req: VecDeque<[Option<ValidationRequest>; MAX_BATCH_REQ as usize]>,

    /// On going validation
    pub in_progress: VecDeque<ReqId>,

    /// Last consumed request index
    pub last_consumed_req_idx: ReqId,
}

#[derive(Debug, PartialEq)]
pub struct ValidationRequest {
    pub req_id: ReqId,
    pub caller_id: CallerId,
    pub canister_id: CanisterId,
    pub build_settings: BuildSettings,
}

impl Default for ValidationsRegistry {
    fn default() -> Self {
        Self {
            req_counter: 0,
            history_canisters: BTreeMap::default(),
            pending_req: VecDeque::default(),
            in_progress: VecDeque::default(),
            last_consumed_req_idx: 0,
        }
    }
}

impl ValidationsRegistry {
    pub fn add_request(
        &mut self,
        caller_id: CallerId,
        canister_id: CanisterId,
        build_settings: BuildSettings,
    ) -> Result<(), Error> {
        // index of current req in last pending batch
        let index = self.req_counter % MAX_BATCH_REQ;

        // get last pending batch
        let last_batch = match self.pending_req.back_mut() {
            Some(p) => {
                if index == 0 && p.get(index as usize).is_some() {
                    // workaround https://github.com/rust-lang/rust/issues/44796
                    let new_batch: [Option<ValidationRequest>; MAX_BATCH_REQ as usize] =
                        Default::default();
                    self.pending_req.push_back(new_batch);
                    self.pending_req.back_mut().unwrap()
                } else {
                    p
                }
            }
            None => {
                // workaround https://github.com/rust-lang/rust/issues/44796
                let new_batch: [Option<ValidationRequest>; MAX_BATCH_REQ as usize] =
                    Default::default();
                self.pending_req.push_back(new_batch);
                self.pending_req.back_mut().unwrap()
            }
        };

        // increasing request counter
        self.req_counter += 1;

        // add new request
        last_batch[index as usize] = Some(ValidationRequest {
            req_id: self.req_counter,
            caller_id,
            canister_id,
            build_settings,
        });

        Ok(())
    }

    pub fn get_pend_req_by_id(&self, req_id: ReqId) -> Option<&ValidationRequest> {
        if req_id <= self.last_consumed_req_idx || req_id > self.req_counter {
            return None;
        }

        let offset = self.last_consumed_req_idx / MAX_BATCH_REQ;

        let is_even_batch = req_id / MAX_BATCH_REQ > 0 && req_id % MAX_BATCH_REQ == 0;

        let target_batch = req_id / MAX_BATCH_REQ - offset - if is_even_batch { 1 } else { 0 };

        let cur_batch = self.pending_req.get(target_batch as usize)?;

        let index = if is_even_batch {
            MAX_BATCH_REQ - 1
        } else {
            req_id % MAX_BATCH_REQ - 1
        };

        cur_batch.get(index as usize)?.as_ref()
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
    //   pub fn contains_validation(&self, req_id: &ReqCnter) -> bool {
    //     self.responses.contains_key(req_id)
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
        assert_eq!(store.req_counter, 0);
        assert_eq!(store.history_canisters, BTreeMap::default());
        assert_eq!(store.pending_req, VecDeque::default());
        assert_eq!(store.in_progress, VecDeque::default());
        assert_eq!(store.last_consumed_req_idx, 0);
    }

    #[test]
    fn add_request_ok() {
        let mut store = ValidationsRegistry::default();
        for i in 0..11 {
            let result = store.add_request(
                if i % 2 == 0 {
                    mock_principals::bob()
                } else {
                    mock_principals::alice()
                },
                if i % 2 == 0 {
                    test_data::fake_canister1()
                } else {
                    test_data::fake_canister2()
                },
                if i % 2 == 0 {
                    test_data::fake_build_settings1()
                } else {
                    test_data::fake_build_settings2()
                },
            );
            assert_eq!(result, Ok(()));
        }
        assert_eq!(store.req_counter, 11);
        assert_eq!(store.history_canisters, BTreeMap::default());
        assert_eq!(
            store.pending_req,
            VecDeque::from(vec![
                [
                    Some(ValidationRequest {
                        req_id: 1,
                        caller_id: mock_principals::bob(),
                        canister_id: test_data::fake_canister1(),
                        build_settings: test_data::fake_build_settings1(),
                    }),
                    Some(ValidationRequest {
                        req_id: 2,
                        caller_id: mock_principals::alice(),
                        canister_id: test_data::fake_canister2(),
                        build_settings: test_data::fake_build_settings2(),
                    }),
                    Some(ValidationRequest {
                        req_id: 3,
                        caller_id: mock_principals::bob(),
                        canister_id: test_data::fake_canister1(),
                        build_settings: test_data::fake_build_settings1(),
                    }),
                    Some(ValidationRequest {
                        req_id: 4,
                        caller_id: mock_principals::alice(),
                        canister_id: test_data::fake_canister2(),
                        build_settings: test_data::fake_build_settings2(),
                    }),
                    Some(ValidationRequest {
                        req_id: 5,
                        caller_id: mock_principals::bob(),
                        canister_id: test_data::fake_canister1(),
                        build_settings: test_data::fake_build_settings1(),
                    }),
                    Some(ValidationRequest {
                        req_id: 6,
                        caller_id: mock_principals::alice(),
                        canister_id: test_data::fake_canister2(),
                        build_settings: test_data::fake_build_settings2(),
                    }),
                    Some(ValidationRequest {
                        req_id: 7,
                        caller_id: mock_principals::bob(),
                        canister_id: test_data::fake_canister1(),
                        build_settings: test_data::fake_build_settings1(),
                    }),
                    Some(ValidationRequest {
                        req_id: 8,
                        caller_id: mock_principals::alice(),
                        canister_id: test_data::fake_canister2(),
                        build_settings: test_data::fake_build_settings2(),
                    }),
                    Some(ValidationRequest {
                        req_id: 9,
                        caller_id: mock_principals::bob(),
                        canister_id: test_data::fake_canister1(),
                        build_settings: test_data::fake_build_settings1(),
                    }),
                    Some(ValidationRequest {
                        req_id: 10,
                        caller_id: mock_principals::alice(),
                        canister_id: test_data::fake_canister2(),
                        build_settings: test_data::fake_build_settings2(),
                    }),
                ],
                [
                    Some(ValidationRequest {
                        req_id: 11,
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
        assert_eq!(store.last_consumed_req_idx, 0);
    }

    #[test]
    fn get_pend_req_by_id_ok() {
        let len = 15;
        for offset in 0..len {
            let store = test_data::fake_store_with_pending(offset, len as usize);
            assert_eq!(store.req_counter, len + offset);

            // outbound
            let result = store.get_pend_req_by_id(offset);
            assert_eq!(result, None);

            // outbound
            let result = store.get_pend_req_by_id(len + offset + 1);
            assert_eq!(result, None);

            for req_id in 1..(len + offset + 1) {
                let result = store.get_pend_req_by_id(req_id);
                if offset >= req_id {
                    assert_eq!(result, None); // outbound
                } else {
                    assert_eq!(result.unwrap().req_id, req_id); // inbound
                }
            }
        }
    }
}
