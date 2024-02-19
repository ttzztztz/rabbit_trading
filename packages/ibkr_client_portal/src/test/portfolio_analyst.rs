use serial_test::serial;

use crate::{
    client::IBClientPortal,
    model::portfolio_analyst::{GetPortfolioPerformanceRequest, GetPortfolioTransactionsRequest},
    test::utils::{get_test_account, CONTRACT_ID_AAPL, TEST_HOST},
};

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_portfolio_analyst_performance() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client
        .get_portfolio_analyst_performance(GetPortfolioPerformanceRequest {
            account_id_list: Option::Some(vec![get_test_account()]),
            freq: Option::None,
        })
        .await;
    assert!(response_result.is_ok());
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_portfolio_analyst_transactions() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client
        .get_portfolio_analyst_transactions(GetPortfolioTransactionsRequest {
            account_id_list: vec![get_test_account()],
            conid_list: vec![CONTRACT_ID_AAPL],
            currency: Option::Some("USD".to_owned()),
            days: Option::Some(90),
        })
        .await;
    assert!(response_result.is_ok());
}
