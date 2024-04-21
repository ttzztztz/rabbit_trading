use async_once::AsyncOnce;
use lazy_static::lazy_static;
use reqwest_retry::policies::ExponentialBackoff;
use serial_test::serial;

use crate::{
    client::IBClientPortal,
    model::session::AuthStatus,
    test::utils::{get_test_account, TEST_HOST},
};

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_tickle() {
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );
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
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );
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
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );
    let response_result = ib_cp_client.sso_validate().await;
    assert!(response_result.is_ok());
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_reauthenticate() {
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );
    let response_result = ib_cp_client.reauthenticate().await;
    assert!(response_result.is_ok());
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_init_brokerage_session() {
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );
    let response_result = ib_cp_client.init_brokerage_session().await;
    assert!(response_result.is_ok());
}

lazy_static! {
    static ref ONCE_INIT_BROKAGE_SESSION: AsyncOnce<Result<AuthStatus, reqwest_middleware::Error>> =
        AsyncOnce::new(async {
            let ib_cp_client = IBClientPortal::new(
                get_test_account(),
                TEST_HOST.to_owned(),
                false,
                ExponentialBackoff::builder().build_with_max_retries(3),
            );
            ib_cp_client.init_brokerage_session().await
        });
}

pub async fn once_init_brokerage_session() {
    ONCE_INIT_BROKAGE_SESSION.get().await;
}
