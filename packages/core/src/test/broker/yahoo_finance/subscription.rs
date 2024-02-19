use log;
use rust_decimal_macros::dec;
use tokio::time::{sleep, Duration};

use crate::{
    broker::{
        common::subscription::SubscriptionTrait,
        yahoo_finance::subscription::YahooFinanceSubscription,
    },
    model::{
        common::types::ConfigMap,
        trading::{
            market::Market,
            quote::{QueryInfoRequest, QuoteKind},
            symbol::Symbol,
        },
    },
};

#[tokio::test]
async fn test_subscribe_quote_real_time_info() {
    let yahoo_finance_subscription = YahooFinanceSubscription::new(ConfigMap::new()).await;
    let subscription_instance_result = yahoo_finance_subscription
        .real_time_info(QueryInfoRequest {
            symbol: Symbol {
                market: Market::US,
                identifier: "ABNB".to_owned(),
            },
            kind: QuoteKind::Stock,
        })
        .await;
    assert!(subscription_instance_result.is_ok());
    let (mut receiver, _) = subscription_instance_result.unwrap();
    tokio::select! {
        quote_info = receiver.recv() => {
            assert!(quote_info.is_some());
            let quote_info = quote_info.unwrap();
            log::warn!("quote_info: {quote_info:?}");
            assert_eq!("ABNB.US", quote_info.symbol.to_string());
            assert!(quote_info.current_price > dec!(0.0));
            assert!(quote_info.volume > 0u64);
            assert!(quote_info.timestamp > 0u64);
        },
        _ = sleep(Duration::from_millis(5000))=> {
            panic!("loop not working!");
        },
    };
}
