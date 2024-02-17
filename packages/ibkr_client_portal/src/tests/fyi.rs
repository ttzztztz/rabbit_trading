use serial_test::serial;

use crate::{
    client::IBClientPortal,
    model::fyi::GetNotificationListRequest,
    tests::utils::{get_test_account, TEST_HOST},
};

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
/// this api is unstable
async fn test_get_fyi_unread_number() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client.get_fyi_unread_number().await;
    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    assert!(response.bn.is_some());
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_fyi_settings() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client.get_fyi_settings().await;
    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    assert!(response.len() > 0);
}

// todo: test toggle_fyi_setting, get_fyi_disclaimer, read_fyi_disclaimer

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_notification_list() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client
        .get_notification_list(GetNotificationListRequest {
            max: 10,
            exclude: Option::None,
            include: Option::None,
        })
        .await;
    assert!(response_result.is_ok());
}

// todo; test get_more_notification_list

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn get_fyi_delivery_options() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client.get_fyi_delivery_options().await;
    assert!(response_result.is_ok());
}

// todo: test toggle_fyi_delivery_options_for_email, toggle_fyi_delivery_options_for_device, delete_fyi_delivery_options_for_device, read_notification
