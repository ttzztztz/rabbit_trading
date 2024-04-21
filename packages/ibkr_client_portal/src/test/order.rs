use reqwest_retry::policies::ExponentialBackoff;
use rust_decimal_macros::dec;
use serial_test::serial;

use crate::{
    client::IBClientPortal,
    model::order::{OrderRequest, PreviewOrderRequest},
    test::{
        session::once_init_brokerage_session,
        utils::{get_test_account, CONTRACT_ID_AAPL, TEST_HOST},
    },
};

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_live_orders() {
    once_init_brokerage_session().await;
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );
    let response_result = ib_cp_client.get_live_orders().await;
    assert!(response_result.is_ok());
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_preview_order() {
    once_init_brokerage_session().await;
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );
    let response_result = ib_cp_client
        .preview_order(PreviewOrderRequest {
            account_id: get_test_account(),
            orders: vec![OrderRequest {
                account_id: Option::Some(get_test_account()),
                conid: Option::None,
                conidex: Option::Some(CONTRACT_ID_AAPL.to_string()),
                sec_type: Option::None,
                c_oid: Option::Some("c_oid".to_owned()),
                parent_id: Option::None,
                order_type: "LMT".to_owned(),
                listing_exchange: Option::None,
                is_single_group: Option::None,
                outside_regular_trading_hours: false,
                price: Option::Some(dec!(168.88)),
                aux_price: Option::None,
                side: "BUY".to_owned(),
                ticker: Option::None,
                time_in_force: "DAY".to_owned(),
                trailing_amount: Option::None,
                trailing_type: Option::None,
                referrer: Option::None,
                quantity: Option::Some(dec!(100)),
                cash_quantity: Option::None,
                fx_quantity: Option::None,
                use_adaptive: Option::Some(false),
                is_currency_conv: Option::None,
                allocation_method: Option::None,
                strategy: Option::None,
                strategy_parameters: Option::None,
            }],
        })
        .await;
    assert!(response_result.is_ok());
}
