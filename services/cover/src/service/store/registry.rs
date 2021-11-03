use std::collections::{BTreeMap, VecDeque};

use crate::common::types::{CallerId, CanisterId, ReqId};
use crate::service::store::error::ErrorKind;
use crate::service::types::BuildSettings;

/// Batch request buffer
const MAX_BATCH_REQ: ReqId = 10;

#[derive(Debug, PartialEq)]
pub struct ValidationsRegistry {
    /// Validation request counter <=> last request id
    pub last_request_id: ReqId,

    // consume history from - to by request id
    // allow arbitrary range from history
    pub consume_history: BTreeMap<(ReqId, ReqId), ConsumeRegistry>,

    /// Pending batch request queue
    /// FIFO -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    ///         [11, 12, 13, 14, 15, None, None, None, None, None] <- LILO
    pub pending_request: VecDeque<[Option<ValidationRequest>; MAX_BATCH_REQ as usize]>,

    /// Last consumed request id
    pub last_consumed_request_id: ReqId,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ValidationRequest {
    pub request_id: ReqId,
    pub caller_id: CallerId,
    pub canister_id: CanisterId,
    pub build_settings: BuildSettings,
    // pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, PartialEq)]
pub struct ConsumeRegistry {
    pub provider_info: ProviderInfo,
    pub batch: Vec<Option<ValidationRequest>>,
    // pub consumed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, PartialEq)]
pub struct ProviderInfo {}

impl Default for ValidationsRegistry {
    fn default() -> Self {
        Self {
            last_request_id: 0,
            consume_history: BTreeMap::default(),
            pending_request: VecDeque::default(),
            last_consumed_request_id: 0,
        }
    }
}

impl ValidationsRegistry {
    /// Current last request index in its batch
    fn current_pending_request_index(&self) -> ReqId {
        self.last_request_id % MAX_BATCH_REQ
    }

    fn should_create_new_batch(&self) -> bool {
        if let Some(p) = self.pending_request.back() {
            return self.current_pending_request_index() == 0
                && p.get(self.current_pending_request_index() as usize)
                    .is_some();
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
        let index = self.current_pending_request_index();
        if self.should_create_new_batch() {
            self.create_new_batch();
        }
        let last_batch = self.pending_request.back_mut().unwrap();
        self.last_request_id += 1;
        last_batch[index as usize] = Some(ValidationRequest {
            request_id: self.last_request_id,
            caller_id,
            canister_id,
            build_settings,
            // created_at: chrono::Utc::now(),
        });
    }

    pub fn get_pending_request_by_id(&self, request_id: ReqId) -> Option<&ValidationRequest> {
        if request_id <= self.last_consumed_request_id || request_id > self.last_request_id {
            return None;
        }
        let offset_batch = self.last_consumed_request_id / MAX_BATCH_REQ;
        let target_batch = (request_id - 1) / MAX_BATCH_REQ - offset_batch;
        let target_index = (request_id - 1) % MAX_BATCH_REQ;
        self.pending_request
            .get(target_batch as usize)?
            .get(target_index as usize)?
            .as_ref()
    }

    pub fn get_all_pending_request(&self) -> Vec<&ValidationRequest> {
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

    pub fn consume_request(
        &mut self,
        provider_info: ProviderInfo,
    ) -> Result<Vec<&ValidationRequest>, ErrorKind> {
        let from = self.last_consumed_request_id;
        let to = self.last_request_id;
        if from >= to {
            return Err(ErrorKind::PendingRequestNotFound(
                "Pending request not found.".into(),
            ));
        }
        if self.pending_request.len() > 1 {
            self.last_consumed_request_id = (self.last_consumed_request_id + MAX_BATCH_REQ)
                - (self.last_consumed_request_id % MAX_BATCH_REQ);
        } else {
            self.last_consumed_request_id = self.last_request_id;
        }
        let batch = self.pending_request.pop_front().unwrap().to_vec();
        self.consume_history.insert(
            (from, to),
            ConsumeRegistry {
                provider_info,
                batch,
            },
        );
        Ok(self
            .consume_history
            .get(&(from, to))
            .unwrap()
            .batch
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.as_ref().unwrap())
            .collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::service::store::test_data;
    use ic_kit::*;

    impl ValidationsRegistry {
        fn fake_store_with_pending_offset(&mut self, offset: ReqId, size: usize) {
            self.last_request_id += offset;
            self.last_consumed_request_id += offset;
            for i in 0..size {
                self.add_request(
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
            }
        }
    }

    #[test]
    fn initial_state_ok() {
        let store = ValidationsRegistry::default();
        assert_eq!(store.last_request_id, 0);
        assert_eq!(store.consume_history, BTreeMap::default());
        assert_eq!(store.pending_request, VecDeque::default());
        assert_eq!(store.last_consumed_request_id, 0);
    }

    #[test]
    fn add_request_ok() {
        let mut store = ValidationsRegistry::default();
        store.fake_store_with_pending_offset(0, 11);
        assert_eq!(store.last_request_id, 11);
        assert_eq!(store.consume_history, BTreeMap::default());
        assert_eq!(
            store.pending_request,
            VecDeque::from(vec![
                [
                    Some(ValidationRequest {
                        request_id: 1,
                        caller_id: mock_principals::bob(),
                        canister_id: test_data::fake_canister1(),
                        build_settings: test_data::fake_build_settings1(),
                        // created_at: get_timer()
                    }),
                    Some(ValidationRequest {
                        request_id: 2,
                        caller_id: mock_principals::alice(),
                        canister_id: test_data::fake_canister2(),
                        build_settings: test_data::fake_build_settings2(),
                        // created_at: get_timer()
                    }),
                    Some(ValidationRequest {
                        request_id: 3,
                        caller_id: mock_principals::bob(),
                        canister_id: test_data::fake_canister1(),
                        build_settings: test_data::fake_build_settings1(),
                        // created_at: get_timer()
                    }),
                    Some(ValidationRequest {
                        request_id: 4,
                        caller_id: mock_principals::alice(),
                        canister_id: test_data::fake_canister2(),
                        build_settings: test_data::fake_build_settings2(),
                        // created_at: get_timer()
                    }),
                    Some(ValidationRequest {
                        request_id: 5,
                        caller_id: mock_principals::bob(),
                        canister_id: test_data::fake_canister1(),
                        build_settings: test_data::fake_build_settings1(),
                        // created_at: get_timer()
                    }),
                    Some(ValidationRequest {
                        request_id: 6,
                        caller_id: mock_principals::alice(),
                        canister_id: test_data::fake_canister2(),
                        build_settings: test_data::fake_build_settings2(),
                        // created_at: get_timer()
                    }),
                    Some(ValidationRequest {
                        request_id: 7,
                        caller_id: mock_principals::bob(),
                        canister_id: test_data::fake_canister1(),
                        build_settings: test_data::fake_build_settings1(),
                        // created_at: get_timer()
                    }),
                    Some(ValidationRequest {
                        request_id: 8,
                        caller_id: mock_principals::alice(),
                        canister_id: test_data::fake_canister2(),
                        build_settings: test_data::fake_build_settings2(),
                        // created_at: get_timer()
                    }),
                    Some(ValidationRequest {
                        request_id: 9,
                        caller_id: mock_principals::bob(),
                        canister_id: test_data::fake_canister1(),
                        build_settings: test_data::fake_build_settings1(),
                        // created_at: get_timer()
                    }),
                    Some(ValidationRequest {
                        request_id: 10,
                        caller_id: mock_principals::alice(),
                        canister_id: test_data::fake_canister2(),
                        build_settings: test_data::fake_build_settings2(),
                        // created_at: get_timer()
                    }),
                ],
                [
                    Some(ValidationRequest {
                        request_id: 11,
                        caller_id: mock_principals::bob(),
                        canister_id: test_data::fake_canister1(),
                        build_settings: test_data::fake_build_settings1(),
                        // created_at: get_timer()
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
        assert_eq!(store.last_consumed_request_id, 0);
    }

    #[test]
    fn get_pending_request_by_id_ok() {
        let len = 15;
        for offset in 0..len {
            let mut store = ValidationsRegistry::default();
            store.fake_store_with_pending_offset(offset, len as usize);
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

    #[test]
    fn consume_request() {
        let len = 15;
        for offset in 0..len {
            let mut store = ValidationsRegistry::default();
            store.fake_store_with_pending_offset(offset, len as usize);

            let result = store.consume_request(test_data::fake_provider_info1());
            assert_eq!(
                result.unwrap().len(),
                (MAX_BATCH_REQ - (offset % MAX_BATCH_REQ)) as usize
            );

            for request_id in (offset + 1)..(len + offset + 1) {
                let result = store.get_pending_request_by_id(request_id);

                if request_id <= store.last_consumed_request_id {
                    assert_eq!(result, None); // consumed
                } else {
                    assert_eq!(result.unwrap().request_id, request_id); // remain
                }
            }
        }
    }
}
