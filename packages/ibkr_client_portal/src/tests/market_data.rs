use crate::{
    client::IBClientPortal,
    model::{market_data::MarketDataRequest, definition::TickType},
    tests::utils::{get_test_account, CONTRACT_ID_AAPL, TEST_HOST},
};

#[tokio::test]
#[cfg_attr(feature = "ci", ignore)]
async fn test_market_data() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client
        .market_data(MarketDataRequest {
            conids: vec![CONTRACT_ID_AAPL.to_string()],
            since: Option::None,
            fields: Option::Some(vec![TickType::LastPrice, TickType::Low, TickType::High]),
        })
        .await;

    // todo: fix this ut failure
    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    assert!(response.len() > 0);
    let first_contract = response.first().unwrap();
    assert!(first_contract
        .get(TickType::LastPrice.to_string().as_str())
        .is_some());
    assert!(first_contract
        .get(TickType::Low.to_string().as_str())
        .is_some());
    assert!(first_contract
        .get(TickType::High.to_string().as_str())
        .is_some());
}
