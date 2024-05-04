use std::sync::{atomic::AtomicBool, Arc};

use log;
use rust_decimal_macros::dec;
use tokio::time::{sleep, Duration};

use crate::{
    broker::{
        common::subscription::SubscriptionTrait, longbridge::subscription::LongBridgeSubscription,
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
#[cfg_attr(not(feature = "flaky_test_cases"), ignore)]
async fn test_quote_real_time_info() {
    let longbridge_subscription =
        LongBridgeSubscription::new(ConfigMap::new(), Arc::new(AtomicBool::new(false)));
    let (mut receiver, _) = longbridge_subscription
        .real_time_info(QueryInfoRequest {
            symbol: Symbol {
                market: Market::HK,
                identifier: "0700".to_owned(),
            },
            kind: QuoteKind::Stock,
        })
        .await
        .unwrap();
    tokio::select! {
        quote_info = receiver.recv() => {
            assert!(quote_info.is_some());
            let quote_info = quote_info.unwrap();
            log::warn!("quote_info: {quote_info:?}");
            assert_eq!("0700.HK", quote_info.symbol.to_string());
            assert!(quote_info.current_price > dec!(0.0));
            assert!(quote_info.volume > 0u64);
            assert!(quote_info.timestamp > 0u64);
        },
        _ = sleep(Duration::from_millis(3000))=> {
            panic!("loop not working!");
        },
    };
}

#[tokio::test]
#[cfg_attr(not(feature = "flaky_test_cases"), ignore)]
async fn test_quote_depth_info() {
    let longbridge_subscription =
        LongBridgeSubscription::new(ConfigMap::new(), Arc::new(AtomicBool::new(false)));
    let (mut receiver, _) = longbridge_subscription
        .depth_info(QueryInfoRequest {
            symbol: Symbol {
                market: Market::US,
                identifier: "AAPL".to_owned(),
            },
            kind: QuoteKind::Stock,
        })
        .await
        .unwrap();
    tokio::select! {
        quote_depth_info_result = receiver.recv() => {
            assert!(quote_depth_info_result.is_some());
            let quote_depth_info = quote_depth_info_result.unwrap();
            log::warn!("quote_depth_info: {quote_depth_info:?}");
            assert_eq!("AAPL.US", quote_depth_info.symbol.to_string());
            quote_depth_info
                .ask_list
                .into_iter()
                .chain(quote_depth_info.bid_list.into_iter())
                .for_each(|depth| {
                    assert!(depth.order_count >= dec!(0.0));
                    assert!(depth.position > dec!(0.0));
                    assert!(depth.price > dec!(0.0));
                    assert!(depth.volume > dec!(0.0));
                });
        },
        _ = sleep(Duration::from_millis(3000))=> {
            panic!("loop not working!");
        },
    };
}
