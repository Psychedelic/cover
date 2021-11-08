use std::collections::{BTreeMap, VecDeque};

use crate::common::types::{CallerId, CanisterId, ReqId};
use crate::service::store::error::ErrorKind;
use crate::service::types::{BuildSettings, ProviderInfo, ValidationRequest};

/// Batch request buffer
const MAX_BATCH_REQ: ReqId = 10;

#[derive(Debug, PartialEq)]
pub struct RequestStore {
    /// Validation request counter <=> last request id
    last_request_id: ReqId,

    /// Consume history from - to by request id
    /// Allow arbitrary range from history
    consume_history: BTreeMap<(ReqId, ReqId), ConsumeRegistry>,

    /// Batch request queue
    /// FIFO -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    ///         [11, 12, 13, 14, 15, None, None, None, None, None] <- LILO
    request: VecDeque<[Option<ValidationRequest>; MAX_BATCH_REQ as usize]>,

    /// Last consumed request id
    last_consumed_request_id: ReqId,
}

#[derive(Debug, PartialEq)]
struct ConsumeRegistry {
    provider_info: ProviderInfo,
    batch: Vec<Option<ValidationRequest>>,
    // consumed_at: chrono::DateTime<chrono::Utc>,
}

impl Default for RequestStore {
    fn default() -> Self {
        Self {
            last_request_id: 0,
            consume_history: BTreeMap::default(),
            request: VecDeque::default(),
            last_consumed_request_id: 0,
        }
    }
}

// TODO: history api
impl RequestStore {
    /// Output a list of non-empty request
    fn filter_non_empty_request(
        batch: &[Option<ValidationRequest>],
    ) -> Vec<&ValidationRequest> {
        batch
            .iter()
            .filter(|p| p.is_some())
            .map(|p| p.as_ref().unwrap())
            .collect::<Vec<&ValidationRequest>>()
    }

    /// Calculate next consume request id
    fn next_consume_request_id(
        &self,
        last_consumed_request_id: ReqId,
        last_request_id: ReqId,
    ) -> ReqId {
        std::cmp::min(
            last_consumed_request_id + MAX_BATCH_REQ - (last_consumed_request_id % MAX_BATCH_REQ),
            last_request_id,
        )
    }

    /// Current last request index in its batch
    fn current_request_index(&self) -> ReqId {
        self.last_request_id % MAX_BATCH_REQ
    }

    /// Check if allocate batch for new batch is needed
    fn should_create_new_batch(&self) -> bool {
        if let Some(p) = self.request.back() {
            return self.current_request_index() == 0
                && p.get(self.current_request_index() as usize)
                    .is_some();
        }
        true
    }

    /// Allocate new batch to the queue
    fn create_new_batch(&mut self) {
        // workaround https://github.com/rust-lang/rust/issues/44796
        let new_batch: [Option<ValidationRequest>; MAX_BATCH_REQ as usize] = Default::default();
        self.request.push_back(new_batch);
    }

    /// Add new request
    pub fn add_request(
        &mut self,
        caller_id: CallerId,
        canister_id: CanisterId,
        build_settings: BuildSettings,
    ) {
        let index = self.current_request_index();
        if self.should_create_new_batch() {
            self.create_new_batch();
        }
        let last_batch = self.request.back_mut().unwrap();
        self.last_request_id += 1;
        last_batch[index as usize] = Some(ValidationRequest {
            request_id: self.last_request_id,
            caller_id,
            canister_id,
            build_settings,
            // created_at: chrono::Utc::now(),
        });
    }

    /// Get request by id
    pub fn get_request_by_id(&self, request_id: ReqId) -> Option<&ValidationRequest> {
        if request_id <= self.last_consumed_request_id || request_id > self.last_request_id {
            return None;
        }
        let offset_batch = self.last_consumed_request_id / MAX_BATCH_REQ;
        let target_batch = (request_id - 1) / MAX_BATCH_REQ - offset_batch;
        let target_index = (request_id - 1) % MAX_BATCH_REQ;
        self.request
            .get(target_batch as usize)?
            .get(target_index as usize)?
            .as_ref()
    }

    /// Get all request
    /// TODO: support pagination
    pub fn get_all_request(&self) -> Vec<&ValidationRequest> {
        self.request
            .iter()
            .flat_map(|b| RequestStore::filter_non_empty_request(b))
            .collect::<Vec<&ValidationRequest>>()
    }

    /// Consume a batch of request by provider
    pub fn consume_request(
        &mut self,
        provider_info: ProviderInfo,
    ) -> Result<Vec<&ValidationRequest>, ErrorKind> {
        let from = self.last_consumed_request_id;
        if from >= self.last_request_id {
            return Err(ErrorKind::RequestNotFound);
        }
        self.last_consumed_request_id =
            self.next_consume_request_id(self.last_consumed_request_id, self.last_request_id);
        let batch = self.request.pop_front().unwrap().to_vec();
        self.consume_history.insert(
            (from, self.last_consumed_request_id),
            ConsumeRegistry {
                provider_info,
                batch,
            },
        );
        Ok(self
            .consume_history
            .get(&(from, self.last_consumed_request_id))
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
    use ic_kit::*;

    use crate::service::store::test_data;

    use super::*;

    impl RequestStore {
        fn fake_store_with_offset(&mut self, offset: ReqId, size: usize) {
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

        fn assert_request_utils(&self, offset: ReqId, len: ReqId) {
            // outbound left most check
            let result = self.get_request_by_id(offset);
            assert_eq!(result, None);

            // outbound right most check
            let result = self.get_request_by_id(len + offset + 1);
            assert_eq!(result, None);

            // inbound check
            for request_id in (offset + 1)..(len + offset + 1) {
                let result = self.get_request_by_id(request_id);
                // consumed check
                if request_id <= self.last_consumed_request_id {
                    assert_eq!(result, None);
                } else {
                    // actual data check
                    let result = result.unwrap();
                    assert_eq!(result.request_id, request_id);
                    if (request_id - offset - 1) % 2 == 0 {
                        assert_eq!(result.caller_id, mock_principals::bob());
                        assert_eq!(result.canister_id, test_data::fake_canister1());
                        assert_eq!(result.build_settings, test_data::fake_build_settings1());
                    } else {
                        assert_eq!(result.caller_id, mock_principals::alice());
                        assert_eq!(result.canister_id, test_data::fake_canister2());
                        assert_eq!(result.build_settings, test_data::fake_build_settings2());
                    }
                }
            }
        }
    }

    #[test]
    fn initial_state_ok() {
        let store = RequestStore::default();
        assert_eq!(store.last_request_id, 0);
        assert_eq!(store.consume_history, BTreeMap::default());
        assert_eq!(store.request, VecDeque::default());
        assert_eq!(store.last_consumed_request_id, 0);
    }

    #[test]
    fn add_request_ok() {
        let mut store = RequestStore::default();
        store.fake_store_with_offset(0, 11);
        assert_eq!(store.last_request_id, 11);
        assert_eq!(store.consume_history, BTreeMap::default());
        assert_eq!(
            store.request,
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
                ],
            ])
        );
        assert_eq!(store.last_consumed_request_id, 0);
    }

    #[test]
    fn get_request_by_id_ok() {
        let len = 15;
        for offset in 0..len {
            let mut store = RequestStore::default();
            store.fake_store_with_offset(offset, len as usize);
            assert_eq!(store.last_request_id, len + offset);
            assert_eq!(store.consume_history, BTreeMap::default());
            assert_eq!(store.last_consumed_request_id, offset);
            store.assert_request_utils(offset, len);
        }
    }

    #[test]
    fn get_all_request_ok() {
        let len = 15;
        for offset in 0..len {
            let mut store = RequestStore::default();

            // empty request
            let result = store.get_all_request();
            assert_eq!(result, Vec::<&ValidationRequest>::default());

            store.fake_store_with_offset(offset, len as usize);

            // all requests
            let result = store.get_all_request();
            assert_eq!(
                result,
                store
                    .request
                    .iter()
                    .flat_map(|b| RequestStore::filter_non_empty_request(b))
                    .collect::<Vec<&ValidationRequest>>()
            );
        }
    }

    #[test]
    fn consume_request_ok() {
        let len = 15;
        for offset in 0..len {
            let mut store = RequestStore::default();

            // error consume when no request
            let result = store.consume_request(test_data::fake_provider_info1());
            assert_eq!(result, Err(ErrorKind::RequestNotFound));

            store.fake_store_with_offset(offset, len as usize);

            // previous state
            let mut from = store.last_consumed_request_id;
            let mut first_batch = store.request.front().unwrap().to_vec();

            while let Ok(result) = store.consume_request(test_data::fake_provider_info1()) {
                // check valid consume result
                assert_eq!(
                  result,
                  RequestStore::filter_non_empty_request(&first_batch)
                );

                // check valid state
                assert_eq!(store.last_request_id, len + offset);
                assert_eq!(
                    store.last_consumed_request_id,
                    store.next_consume_request_id(from, len + offset)
                );

                // check valid history
                let history = store
                    .consume_history
                    .get(&(from, store.last_consumed_request_id))
                    .unwrap();
                assert_eq!(history.provider_info, test_data::fake_provider_info1());
                assert_eq!(history.batch, first_batch);

                // check valid request
                store.assert_request_utils(offset, len);

                // update new state
                from = store.last_consumed_request_id;
                if let Some(batch) = store.request.front() {
                    first_batch = batch.to_vec();
                }
            }

            // back to empty state when
            let request = store.get_all_request();
            assert_eq!(request.len(), 0);
        }
    }
}