use ic_kit::candid::candid_method;
use ic_kit::macros::query;

use crate::service::model::stats::Stats;
use crate::service::stats;

#[query(name = "getStats")]
#[candid_method(query, rename = "getStats")]
fn get_stats() -> Stats {
    stats::get_stats()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::service::store::test_data::*;
    use crate::service::verification::*;
    use ic_kit::*;

    fn init_test_data() {
        MockContext::new()
            .with_caller(mock_principals::bob())
            .inject();

        submit_verification(fake_success_verification(
            &mock_principals::bob(),
            &fake_canister1(),
        ));

        submit_verification(fake_error_verification(
            &mock_principals::alice(),
            &fake_canister2(),
        ));

        assert_eq!(
            register_verification(fake_register_verification(&fake_canister3())),
            Ok(())
        );
    }

    #[test]
    fn get_stats_ok() {
        init_test_data();

        assert_eq!(
            get_stats(),
            Stats {
                total_canisters: 3,
                motoko_canisters_count: 1,
                rust_canisters_count: 1,
                build_pending_count: 1,
                build_in_progress_count: 0,
                build_error_count: 1,
                build_success_count: 1
            }
        );

        assert_eq!(
            register_verification(fake_register_verification(&fake_canister1())),
            Ok(())
        );

        assert_eq!(
            get_stats(),
            Stats {
                total_canisters: 3,
                motoko_canisters_count: 1,
                rust_canisters_count: 0,
                build_pending_count: 2,
                build_in_progress_count: 0,
                build_error_count: 1,
                build_success_count: 0
            }
        );

        submit_verification(fake_error_verification(
            &mock_principals::alice(),
            &fake_canister3(),
        ));

        assert_eq!(
            get_stats(),
            Stats {
                total_canisters: 3,
                motoko_canisters_count: 2,
                rust_canisters_count: 0,
                build_pending_count: 1,
                build_in_progress_count: 0,
                build_error_count: 2,
                build_success_count: 0
            }
        );
    }
}
