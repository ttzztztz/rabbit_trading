use log;
use rust_decimal_macros::dec;

use crate::{
    broker::{common::info::InfoTrait, yahoo_finance::info::YahooFinanceInfo},
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
async fn test_query_quote_info() {
    let yahoo_finance_info = YahooFinanceInfo::new(ConfigMap::new());

    let quote_info_result = yahoo_finance_info
        .query_real_time_info(QueryInfoRequest {
            symbol: Symbol {
                market: Market::US,
                identifier: "ABNB".to_owned(),
            },
            kind: QuoteKind::Stock,
        })
        .await;
    assert!(quote_info_result.is_ok());
    let quote_info = quote_info_result.unwrap();
    log::warn!("quote_info: {quote_info:?}");
    assert_eq!("ABNB.US", quote_info.symbol.to_string());
    assert!(quote_info.current_price > dec!(0.0));
    assert!(quote_info.volume > 0u64);
    assert!(quote_info.timestamp > 0u64);
}
