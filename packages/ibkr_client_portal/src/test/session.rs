use serial_test::serial;

use crate::{
    client::IBClientPortal,
    test::utils::{get_test_account, TEST_HOST},
};

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_tickle() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client.tickle().await;
    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    assert!(response.session.len() > 0);
    assert!(response.user_id > 0);
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_auth_status() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client.get_auth_status().await;
    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    assert!(response.authenticated.unwrap());
}

// todo: test logout

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_sso_validate() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client.sso_validate().await;
    assert!(response_result.is_ok());
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_reauthenticate() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client.reauthenticate().await;
    assert!(response_result.is_ok());
}
