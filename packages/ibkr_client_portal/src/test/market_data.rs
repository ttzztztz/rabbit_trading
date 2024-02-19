use serial_test::serial;

use crate::{
    client::IBClientPortal,
    model::{
        definition::TickType,
        market_data::{
            GetMarketDataHistoryBetaRequest, GetMarketDataHistoryRequest, GetMarketDataRequest,
            GetMarketDataSnapshotRequest,
        },
    },
    test::utils::{get_test_account, CONTRACT_ID_AAPL, TEST_HOST},
};

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_market_data() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client
        .get_market_data(GetMarketDataRequest {
            conid_list: vec![CONTRACT_ID_AAPL],
            since: Option::None,
            fields: Option::Some(vec![TickType::LastPrice, TickType::Low, TickType::High]),
        })
        .await;

    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    assert!(response.len() > 0);
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "flaky_test_cases"), ignore)]
async fn test_get_market_data_history() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client
        .get_market_data_history(GetMarketDataHistoryRequest {
            conid: CONTRACT_ID_AAPL,
            exchange: Option::None,
            period: "1w".to_owned(),
            bar: Option::Some("1d".to_owned()),
            outside_regular_trading_hours: Option::Some(false),
            start_time: Option::None,
        })
        .await;

    assert!(response_result.is_ok());
}

// todo: test unsubscribe_all_market_data, unsubscribe_market_data

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "flaky_test_cases"), ignore)]
async fn test_get_market_data_history_beta() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client
        .get_market_data_history_beta(GetMarketDataHistoryBetaRequest {
            conid: CONTRACT_ID_AAPL,
            period: "30d".to_owned(),
            bar: Option::Some("1d".to_owned()),
            outside_regular_trading_hours: Option::Some(false),
        })
        .await;

    assert!(response_result.is_ok());
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_market_data_snapshot_beta() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client
        .get_market_data_snapshot_beta(GetMarketDataSnapshotRequest {
            conid_list: vec![CONTRACT_ID_AAPL],
            field_list: vec![TickType::LastPrice],
        })
        .await;

    assert!(response_result.is_ok());
}
