use reqwest_retry::policies::ExponentialBackoff;
use serial_test::serial;

use crate::{
    client::IBClientPortal,
    model::account::{
        GetAccountAllocationRequest, GetAccountMetadataRequest, GetAccountSummaryRequest,
        GetSubAccountsV2Request,
    },
    test::utils::{get_test_account, TEST_HOST},
};

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_account_ledger() {
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );

    let response_result = ib_cp_client.get_account_ledger().await;
    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    assert!(response.len() > 0);
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_brokerage_accounts() {
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );
    let test_account = get_test_account();

    let response_result = ib_cp_client.get_brokerage_accounts().await;
    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    assert!(response.accounts.contains(&test_account));

    // todo: support testing switch_account
    // let response_result = ib_cp_client
    //     .switch_account(SwitchAccountRequest {
    //         account_id: test_account.clone(),
    //     })
    //     .await;
    // assert!(response_result.is_ok());
    // let response = response_result.unwrap();
    // assert!(response.set);
    // assert_eq!(test_account, response.account_id);
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_portfolio_accounts() {
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );

    let response_result = ib_cp_client.get_portfolio_accounts().await;
    assert!(response_result.is_ok());
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_sub_accounts() {
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );

    let response_result = ib_cp_client.get_sub_accounts().await;
    assert!(response_result.is_ok());
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_sub_accounts_v2() {
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );

    let response_result = ib_cp_client
        .get_sub_accounts_v2(GetSubAccountsV2Request { page: 1 })
        .await;
    assert!(response_result.is_ok());
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_account_metadata() {
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );

    let response_result = ib_cp_client
        .get_account_metadata(GetAccountMetadataRequest {
            account_id: get_test_account(),
        })
        .await;
    assert!(response_result.is_ok());
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_account_summary() {
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );

    let response_result = ib_cp_client
        .get_account_summary(GetAccountSummaryRequest {
            account_id: get_test_account(),
        })
        .await;
    assert!(response_result.is_ok());
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_account_allocations() {
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );

    let response_result = ib_cp_client
        .get_account_allocations(GetAccountAllocationRequest {
            account_id: get_test_account(),
        })
        .await;
    assert!(response_result.is_ok());
}

// todo: test switch_account

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_account_pnl_partitioned() {
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );

    let response_result = ib_cp_client.get_account_pnl_partitioned().await;
    assert!(response_result.is_ok());
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_account_trades() {
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );

    let response_result = ib_cp_client.get_account_trades().await;
    assert!(response_result.is_ok());
}
