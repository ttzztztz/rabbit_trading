use reqwest_retry::policies::ExponentialBackoff;
use rust_decimal_macros::dec;
use serial_test::serial;
use std::time::SystemTime;

use crate::{
    client::IBClientPortal,
    model::order::{
        CancelOrderRequest, GetOrderStatusRequest, ModifyOrderRequest, OrderRequest,
        PlaceOrdersRequest, PreviewOrderRequest,
    },
    test::{
        session::once_init_brokerage_session,
        utils::{get_test_account, CONTRACT_ID_AAPL, TEST_HOST},
    },
    utils::reply::handle_reply_order_requests,
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
                limit_offset: Option::None,
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
                originator: Option::None,
            }],
        })
        .await;
    assert!(response_result.is_ok());
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_order_operations() {
    once_init_brokerage_session().await;
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );

    let place_order_response_result = ib_cp_client
        .place_orders(PlaceOrdersRequest {
            account_id: get_test_account(),
            orders: vec![OrderRequest {
                account_id: Option::Some(get_test_account()),
                conid: Option::None,
                conidex: Option::Some(CONTRACT_ID_AAPL.to_string()),
                sec_type: Option::None,
                c_oid: Option::Some(format!(
                    "test_{}",
                    SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as u64
                )),
                parent_id: Option::None,
                order_type: "LMT".to_owned(),
                listing_exchange: Option::None,
                limit_offset: Option::None,
                is_single_group: Option::None,
                outside_regular_trading_hours: false,
                price: Option::Some(dec!(99.99)),
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
                originator: Option::Some("Side-Order".to_owned()),
            }],
        })
        .await;

    assert!(place_order_response_result.is_ok());
    let place_order_response = place_order_response_result.unwrap();
    let order_id = handle_reply_order_requests(ib_cp_client.clone(), place_order_response, 2i32)
        .await
        .unwrap();

    let order_status_result = ib_cp_client
        .get_order_status(GetOrderStatusRequest {
            order_id: order_id.clone(),
        })
        .await;
    assert!(order_status_result.is_ok());
    let order_status = order_status_result.unwrap();
    assert_eq!(Option::Some(CONTRACT_ID_AAPL), order_status.conid);
    assert_eq!(
        Option::Some(order_id.clone()),
        order_status.order_id.map(|v| v.to_string())
    );
    assert_eq!(Option::Some(dec!(99.99)), order_status.limit_price);

    let modify_order_response_result = ib_cp_client
        .modify_order(ModifyOrderRequest {
            account_id_or_financial_advisors_group: get_test_account(),
            order_id: order_id.clone(),
            account_id: Option::Some(get_test_account()),
            conid: Option::None,
            conidex: Option::Some(CONTRACT_ID_AAPL.to_string()),
            order_type: Option::Some("LMT".to_owned()),
            outside_regular_trading_hours: Option::Some(false),
            price: Option::Some(dec!(88.88)),
            aux_price: Option::None,
            side: Option::Some("BUY".to_owned()),
            listing_exchange: Option::None,
            ticker: Option::None,
            time_in_force: Option::Some("DAY".to_owned()),
            quantity: Option::Some(dec!(100)),
            deactivated: Option::None,
            use_adaptive: Option::Some(false),
            limit_offset: Option::None,
            trailing_amount: Option::None,
            trailing_type: Option::None,
        })
        .await;
    assert!(modify_order_response_result.is_ok());
    let modify_order_response = modify_order_response_result.unwrap();
    handle_reply_order_requests(ib_cp_client.clone(), modify_order_response, 2i32)
        .await
        .unwrap();

    let order_status_result = ib_cp_client
        .get_order_status(GetOrderStatusRequest {
            order_id: order_id.clone(),
        })
        .await;
    assert!(order_status_result.is_ok());
    let order_status = order_status_result.unwrap();
    assert_eq!(Option::Some(CONTRACT_ID_AAPL), order_status.conid);
    assert_eq!(
        Option::Some(order_id.clone()),
        order_status.order_id.map(|v| v.to_string())
    );
    assert_eq!(Option::Some(dec!(88.88)), order_status.limit_price);

    let cancel_order_result = ib_cp_client
        .cancel_order(CancelOrderRequest {
            account_id: get_test_account(),
            order_id,
        })
        .await;
    assert!(cancel_order_result.is_ok());
}
