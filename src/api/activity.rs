use ic_kit::candid::candid_method;
use ic_kit::macros::{query, update};

use crate::service::activity;
use crate::service::guard::is_builder;
use crate::service::model::activity::Activity;
use crate::service::model::pagination::{Pagination, PaginationInfo};

#[update(name = "addActivity", guard = "is_builder")]
#[candid_method(update, rename = "addActivity")]
fn add_activity(new_activity: Activity) {
    activity::add_activity(new_activity)
}

#[query(name = "getActivities", guard = "is_builder")]
#[candid_method(query, rename = "getActivities")]
fn get_activities(pagination_info: PaginationInfo) -> Pagination<&'static Activity> {
    activity::get_activities(&pagination_info)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::service::model::verification::BuildStatus;
    use crate::service::store::test_data::*;
    use ic_kit::*;

    fn init_test_data() {
        MockContext::new()
            .with_caller(mock_principals::john())
            .inject();

        add_activity(fake_activity(&fake_canister1(), BuildStatus::Pending));
        add_activity(fake_activity(&fake_canister2(), BuildStatus::Building));
        add_activity(fake_activity(&fake_canister3(), BuildStatus::Success));
        add_activity(fake_activity(&fake_canister3(), BuildStatus::Error));
        add_activity(fake_activity(&fake_canister2(), BuildStatus::Building));
        add_activity(fake_activity(&fake_canister1(), BuildStatus::Success));
    }

    #[test]
    fn get_activities_ok() {
        init_test_data();

        assert_eq!(
            get_activities(PaginationInfo {
                page_index: 2,
                items_per_page: 4
            }),
            fake_pagination(
                vec![
                    &fake_activity(&fake_canister2(), BuildStatus::Building),
                    &fake_activity(&fake_canister1(), BuildStatus::Pending)
                ],
                &PaginationInfo {
                    page_index: 2,
                    items_per_page: 4
                },
                6
            )
        );

        assert_eq!(
            get_activities(PaginationInfo {
                page_index: 100,
                items_per_page: 0
            }),
            fake_pagination(
                vec![],
                &PaginationInfo {
                    page_index: 100,
                    items_per_page: 0
                },
                6
            )
        );
    }

    #[test]
    fn add_activity_ok() {
        init_test_data();

        add_activity(fake_activity(&fake_canister2(), BuildStatus::Error));

        assert_eq!(
            get_activities(PaginationInfo {
                page_index: 1,
                items_per_page: 3
            }),
            fake_pagination(
                vec![
                    &fake_activity(&fake_canister2(), BuildStatus::Error),
                    &fake_activity(&fake_canister1(), BuildStatus::Success),
                    &fake_activity(&fake_canister2(), BuildStatus::Building)
                ],
                &PaginationInfo {
                    page_index: 1,
                    items_per_page: 3
                },
                7
            )
        );
    }
}
