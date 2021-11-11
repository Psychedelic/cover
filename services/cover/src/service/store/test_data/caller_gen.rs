use crate::common::types::CallerId;
use ic_kit::*;

pub fn caller_gen(seed: u8) -> CallerId {
    if seed % 3 == 0 {
        mock_principals::alice()
    } else if seed % 3 == 1 {
        mock_principals::bob()
    } else {
        mock_principals::john()
    }
}
