use reqwest_retry::policies::ExponentialBackoff;
use serial_test::serial;

use crate::{
    client::IBClientPortal,
    test::utils::{get_test_account, TEST_HOST},
};

// todo: test get_list_of_available_alerts

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_mobile_trading_assistant_alert() {
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );
    let response_result = ib_cp_client.get_mobile_trading_assistant_alert().await;
    assert!(response_result.is_ok());
}

// todo: test get_alert_details, delete_alert, toggle_alert_activation, upsert_alert
