use serial_test::serial;

use crate::{
    client::IBClientPortal,
    model::portfolio::GetPortfolioPositionsRequest,
    tests::utils::{get_test_account, TEST_HOST},
};

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_positions() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client
        .get_portfolio_positions(GetPortfolioPositionsRequest { page: 1 })
        .await;
    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    response.into_iter().for_each(|position| {
        assert!(position.conid.is_some());
    });
}
