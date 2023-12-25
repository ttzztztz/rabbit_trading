use async_trait::async_trait;
use longbridge::quote::{SecurityDepth, SecurityQuote, SecurityStaticInfo};
use longbridge::QuoteContext;
use std::result::Result;

use super::broker::LongBridgeBroker;
use crate::broker::common::info::InfoTrait;
use crate::model::error::Error;
use crate::model::quote::{QueryInfoRequest, QuoteBasicInfo, QuoteDepthInfo, QuoteRealTimeInfo};
use crate::model::symbol::Symbol;

pub struct LongBridgeInfo {
    longbridge_context: QuoteContext,
}

impl LongBridgeInfo {
    fn to_quote_real_time_info(symbol: Symbol, security_quote: SecurityQuote) -> QuoteRealTimeInfo {
        QuoteRealTimeInfo {
            symbol,
            sequence: security_quote.timestamp.unix_timestamp() as u64,
            timestamp: security_quote.timestamp.unix_timestamp(),
            current_price: security_quote.last_done,
            low_price: Option::Some(security_quote.low),
            high_price: Option::Some(security_quote.high),
            open_price: Option::Some(security_quote.open),
            prev_close: Option::Some(security_quote.prev_close),
            volume: security_quote.volume as u64,
            turnover: Option::Some(security_quote.turnover),
            extra: Option::None,
        }
    }

    fn to_quote_basic_info(
        symbol: Symbol,
        security_static_info: SecurityStaticInfo,
    ) -> QuoteBasicInfo {
        QuoteBasicInfo {
            symbol,
            currency: LongBridgeBroker::to_currency(&security_static_info.currency),
            lot_size: security_static_info.lot_size,
            total_shares: security_static_info.total_shares,
            circulating_shares: security_static_info.circulating_shares,
            eps: security_static_info.eps,
            eps_ttm: security_static_info.eps_ttm,
            bps: security_static_info.bps,
            dividend_yield: security_static_info.dividend_yield,
        }
    }

    fn to_depth(depth: longbridge::quote::Depth) -> crate::model::quote::Depth {
        crate::model::quote::Depth {
            position: depth.position,
            price: depth.price,
            volume: depth.volume,
            order_count: depth.order_num,
        }
    }

    fn to_quote_depth_info(symbol: Symbol, security_depth: SecurityDepth) -> QuoteDepthInfo {
        QuoteDepthInfo {
            symbol,
            ask_list: security_depth
                .asks
                .into_iter()
                .map(Self::to_depth)
                .collect(),
            bid_list: security_depth
                .bids
                .into_iter()
                .map(Self::to_depth)
                .collect(),
        }
    }

    fn get_missing_element_error() -> Error {
        const ERROR_CODE: &'static str = "longbridge_api_internal_error";
        const MESSAGE: &'static str = "Missing elements from the api response.";

        Error {
            code: ERROR_CODE.to_owned(),
            message: MESSAGE.to_owned(),
        }
    }
}

#[async_trait]
impl InfoTrait for LongBridgeInfo {
    async fn new() -> Self {
        let (longbridge_context, _) = LongBridgeBroker::create_quote_context().await.unwrap();
        LongBridgeInfo { longbridge_context }
    }

    async fn query_basic_info(&self, request: QueryInfoRequest) -> Result<QuoteBasicInfo, Error> {
        let symbol_identifier = request.symbol.to_string();
        self.longbridge_context
            .static_info([symbol_identifier])
            .await
            .map_err(LongBridgeBroker::to_rabbit_trading_err)
            .and_then(|result_vec| match result_vec.into_iter().nth(0) {
                Some(static_info) => {
                    Result::Ok(Self::to_quote_basic_info(request.symbol, static_info))
                }
                None => Result::Err(Self::get_missing_element_error()),
            })
    }

    async fn query_real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<QuoteRealTimeInfo, Error> {
        // todo: support option
        let symbol_identifier = request.symbol.to_string();
        self.longbridge_context
            .quote([symbol_identifier])
            .await
            .map_err(LongBridgeBroker::to_rabbit_trading_err)
            .and_then(|result_vec| match result_vec.into_iter().nth(0) {
                Some(real_time_info) => Result::Ok(Self::to_quote_real_time_info(
                    request.symbol,
                    real_time_info,
                )),
                None => Result::Err(Self::get_missing_element_error()),
            })
    }

    async fn query_depth(&self, request: QueryInfoRequest) -> Result<QuoteDepthInfo, Error> {
        let symbol_identifier = request.symbol.to_string();
        self.longbridge_context
            .depth(symbol_identifier)
            .await
            .map(|depth_info| Self::to_quote_depth_info(request.symbol, depth_info))
            .map_err(LongBridgeBroker::to_rabbit_trading_err)
    }
}

#[cfg(test)]
mod test_longbridge_info {
    use log;
    use rust_decimal_macros::dec;

    use super::LongBridgeInfo;
    use crate::broker::common::info::InfoTrait;
    use crate::model::currency::Currency;
    use crate::model::quote::{QueryInfoRequest, QuoteKind};
    use crate::model::{market::Market, symbol::Symbol};

    #[tokio::test]
    #[cfg_attr(feature = "ci", ignore)]
    async fn test_query_real_time_info() {
        let longbridge_info = LongBridgeInfo::new().await;
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
        assert!(quote_real_time_info.volume > 0u64);
        assert!(quote_real_time_info.high_price.unwrap() > dec!(0.0));
        assert!(quote_real_time_info.low_price.unwrap() > dec!(0.0));
        assert!(quote_real_time_info.open_price.unwrap() > dec!(0.0));
        assert!(quote_real_time_info.prev_close.unwrap() > dec!(0.0));
        assert!(quote_real_time_info.turnover.unwrap() > dec!(0.0));
        assert!(quote_real_time_info.volume > 0u64);
        assert!(quote_real_time_info.timestamp > 0);
    }

    #[tokio::test]
    #[cfg_attr(feature = "ci", ignore)]
    async fn test_query_basic_info() {
        let longbridge_info = LongBridgeInfo::new().await;
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
        assert!(quote_basic_info.circulating_shares > 0i64);
        assert!(quote_basic_info.total_shares > 0i64);
    }

    #[tokio::test]
    #[cfg_attr(feature = "ci", ignore)]
    async fn test_query_depth() {
        let longbridge_info = LongBridgeInfo::new().await;
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
        assert!(quote_depth_info.ask_list.len() > 0);
        assert!(quote_depth_info.bid_list.len() > 0);
        quote_depth_info
            .ask_list
            .into_iter()
            .chain(quote_depth_info.bid_list.into_iter())
            .for_each(|depth| {
                println!("order_count={}", depth.order_count);
                assert!(depth.order_count >= 0i64);
                assert!(depth.position > 0i32);
                assert!(depth.price > dec!(0.0));
                assert!(depth.volume > 0i64);
            });
    }
}
