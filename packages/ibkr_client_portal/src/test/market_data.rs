use std::str::FromStr;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
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
    test::{
        session::once_init_brokerage_session,
        utils::{get_test_account, CONTRACT_ID_AAPL, TEST_HOST},
    },
};

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_market_data() {
    once_init_brokerage_session().await;
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let request = GetMarketDataRequest {
        conid_list: vec![CONTRACT_ID_AAPL],
        since: Option::None,
        fields: Option::Some(vec![TickType::LastPrice, TickType::Low, TickType::High]),
    };
    let first_response_result = ib_cp_client.get_market_data(request.clone()).await;
    assert!(first_response_result.is_ok());

    // first response won't return anything
    let second_response_result = ib_cp_client.get_market_data(request).await;

    assert!(second_response_result.is_ok());
    let second_response_result = second_response_result.unwrap();
    assert!(second_response_result.len() > 0);
    let body = &second_response_result[0];

    assert!(
        Decimal::from_str(
            body.get(TickType::LastPrice.to_string().as_str())
                .unwrap()
                .as_str()
                .unwrap()
        )
        .unwrap()
            > dec!(1.0)
    );
    assert!(
        Decimal::from_str(
            body.get(TickType::LastPrice.to_string().as_str())
                .unwrap()
                .as_str()
                .unwrap()
        )
        .unwrap()
            > dec!(1.0)
    );
    assert!(
        Decimal::from_str(
            body.get(TickType::LastPrice.to_string().as_str())
                .unwrap()
                .as_str()
                .unwrap()
        )
        .unwrap()
            > dec!(1.0)
    );
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_market_data_history() {
    once_init_brokerage_session().await;
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client
        .get_market_data_history(GetMarketDataHistoryRequest {
            conid: CONTRACT_ID_AAPL,
            exchange: Option::None,
            period: "1d".to_owned(),
            bar: Option::Some("15min".to_owned()),
            outside_regular_trading_hours: Option::Some(false),
            start_time: Option::Some("20240101-00:00:00".to_owned()),
        })
        .await;

    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    assert!(response.data.is_some());
    assert!(response.data.unwrap().len() > 0);
}

// todo: test unsubscribe_all_market_data, unsubscribe_market_data

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "flaky_test_cases"), ignore)]
async fn test_get_market_data_history_beta() {
    once_init_brokerage_session().await;
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
    once_init_brokerage_session().await;
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client
        .get_market_data_snapshot_beta(GetMarketDataSnapshotRequest {
            conid_list: vec![CONTRACT_ID_AAPL],
            field_list: vec![TickType::LastPrice],
        })
        .await;

    assert!(response_result.is_ok());
}
