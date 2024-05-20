use rust_decimal_macros::dec;

use crate::{
    broker::{
        common::transaction::TransactionTrait,
        interactive_brokers::transaction::InteractiveBrokersTransaction,
    },
    model::trading::{
        market::Market,
        symbol::Symbol,
        transaction::{
            CancelOrderRequest, Direction, EditOrderRequest, Expire, OrderDetailRequest, Price,
            RegularTradingTime, SubmitOrderRequest,
        },
    },
    test::broker::interactive_brokers::test_helper::get_config_map,
};

#[tokio::test]
#[cfg_attr(feature = "ci", ignore)]
async fn test_interactive_brokers_transaction() {
    let config_map = get_config_map();
    let mut transaction = InteractiveBrokersTransaction::new(config_map);

    let account_balance_result = transaction.account_balance().await;
    assert!(account_balance_result.is_ok());

    let create_order_result = transaction
        .submit_order(SubmitOrderRequest {
            symbol: Symbol {
                market: Market::US,
                identifier: "AAPL".to_owned(),
            },
            quantity: dec!(100),
            direction: Direction::Buy,
            regular_trading_time: RegularTradingTime::OnlyRegularTradingTime,
            expire: Expire::Day,
            price: Price::LimitOrder { price: dec!(88.88) },
        })
        .await;
    assert!(create_order_result.is_ok());

    let order_id = create_order_result.unwrap().order_id;
    let order_detail_result = transaction
        .order_detail(OrderDetailRequest {
            order_id: order_id.clone(),
        })
        .await;
    assert!(order_detail_result.is_ok());

    let order_detail = order_detail_result.unwrap();
    assert_eq!(dec!(100), order_detail.quantity);
    assert_eq!(Direction::Buy, order_detail.direction);
    assert_eq!(Expire::Day, order_detail.expire);
    assert_eq!(
        RegularTradingTime::OnlyRegularTradingTime,
        order_detail.regular_trading_time
    );
    assert_eq!(Price::LimitOrder { price: dec!(88.88) }, order_detail.price);

    let edit_order_result = transaction
        .edit_order(EditOrderRequest {
            order_id: order_id.clone(),
            symbol: Symbol {
                market: Market::US,
                identifier: "AAPL".to_owned(),
            },
            quantity: dec!(150),
            direction: Direction::Buy,
            expire: Expire::Day,
            price: Price::LimitOrder { price: dec!(99.99) },
        })
        .await;
    assert!(edit_order_result.is_ok());

    let order_detail_result = transaction
        .order_detail(OrderDetailRequest {
            order_id: order_id.clone(),
        })
        .await;
    assert!(order_detail_result.is_ok());

    let order_detail = order_detail_result.unwrap();
    assert_eq!(dec!(150), order_detail.quantity);
    assert_eq!(Direction::Buy, order_detail.direction);
    assert_eq!(Expire::Day, order_detail.expire);
    assert_eq!(
        RegularTradingTime::OnlyRegularTradingTime,
        order_detail.regular_trading_time
    );
    assert_eq!(Price::LimitOrder { price: dec!(99.99) }, order_detail.price);

    let cancel_order_result = transaction
        .cancel_order(CancelOrderRequest {
            order_id: order_id.clone(),
        })
        .await;
    assert!(cancel_order_result.is_ok());
}
