use log;
use rust_decimal_macros::dec;

use crate::{
    broker::{common::info::InfoTrait, longbridge::info::LongBridgeInfo},
    model::{
        common::types::ConfigMap,
        trading::{
            currency::Currency,
            market::Market,
            quote::{QueryInfoRequest, QuoteKind},
            symbol::Symbol,
        },
    },
};

#[tokio::test]
#[cfg_attr(feature = "ci", ignore)]
async fn test_query_basic_info() {
    let longbridge_info = LongBridgeInfo::new(ConfigMap::new());
    let quote_basic_info_result = longbridge_info
        .query_basic_info(QueryInfoRequest {
            symbol: Symbol {
                market: Market::US,
                identifier: "AAPL".to_owned(),
            },
            kind: QuoteKind::Stock,
        })
        .await;
    let quote_basic_info = quote_basic_info_result.unwrap();
    log::warn!("quote_basic_info: {quote_basic_info:?}");
    assert_eq!("AAPL.US", quote_basic_info.symbol.to_string());
    assert_eq!(Option::Some(Currency::USD), quote_basic_info.currency);
    assert!(quote_basic_info.bps >= dec!(0.0));
    assert!(quote_basic_info.dividend_yield >= dec!(0.0));
    assert!(quote_basic_info.eps >= dec!(0.0));
    assert!(quote_basic_info.eps_ttm >= dec!(0.0));
    assert!(quote_basic_info.lot_size > 0i32);
    assert!(quote_basic_info.circulating_shares > dec!(0.0));
    assert!(quote_basic_info.total_shares > dec!(0.0));
}

#[tokio::test]
#[cfg_attr(feature = "ci", ignore)]
async fn test_query_real_time_info() {
    let longbridge_info = LongBridgeInfo::new(ConfigMap::new());
    let quote_real_time_info_result = longbridge_info
        .query_real_time_info(QueryInfoRequest {
            symbol: Symbol {
                market: Market::HK,
                identifier: "0700".to_owned(),
            },
            kind: QuoteKind::Stock,
        })
        .await;
    let quote_real_time_info = quote_real_time_info_result.unwrap();
    log::warn!("quote_real_time_info: {quote_real_time_info:?}");
    assert_eq!("0700.HK", quote_real_time_info.symbol.to_string());
    assert!(quote_real_time_info.current_price > dec!(0.0));
    assert!(quote_real_time_info.volume > dec!(0.0));
    assert!(quote_real_time_info.high_price.unwrap() > dec!(0.0));
    assert!(quote_real_time_info.low_price.unwrap() > dec!(0.0));
    assert!(quote_real_time_info.open_price.unwrap() > dec!(0.0));
    assert!(quote_real_time_info.prev_close.unwrap() > dec!(0.0));
    assert!(quote_real_time_info.turnover.unwrap() > dec!(0.0));
    assert!(quote_real_time_info.volume > dec!(0.0));
    assert!(quote_real_time_info.timestamp > 0);
}

#[tokio::test]
#[cfg_attr(feature = "ci", ignore)]
async fn test_query_depth() {
    let longbridge_info = LongBridgeInfo::new(ConfigMap::new());
    let quote_depth_info_result = longbridge_info
        .query_depth(QueryInfoRequest {
            symbol: Symbol {
                market: Market::US,
                identifier: "MSFT".to_owned(),
            },
            kind: QuoteKind::Stock,
        })
        .await;
    let quote_depth_info = quote_depth_info_result.unwrap();
    log::warn!("quote_depth_info: {quote_depth_info:?}");
    assert_eq!("MSFT.US", quote_depth_info.symbol.to_string());
    quote_depth_info
        .ask_list
        .into_iter()
        .chain(quote_depth_info.bid_list.into_iter())
        .for_each(|depth| {
            assert!(depth.order_count.unwrap() >= dec!(0.0));
            assert!(depth.position.unwrap() > dec!(0.0));
            assert!(depth.price > dec!(0.0));
            assert!(depth.volume > dec!(0.0));
        });
}
