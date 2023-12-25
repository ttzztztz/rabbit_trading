use async_trait::async_trait;
use longbridge::quote::SecurityQuote;
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
}

#[async_trait]
impl InfoTrait for LongBridgeInfo {
    async fn new() -> Self {
        let (longbridge_context, _) = LongBridgeBroker::create_quote_context().await.unwrap();
        LongBridgeInfo {
            longbridge_context,
        }
    }

    async fn query_basic_info(&self, request: QueryInfoRequest) -> Result<QuoteBasicInfo, Error> {
        todo!()
    }

    async fn query_real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<QuoteRealTimeInfo, Error> {
        let symbol_identifier = request.symbol.to_string();
        let quote_result = self
            .longbridge_context
            .quote([symbol_identifier])
            .await
            .map(|result_vec| result_vec[0].clone());

        match quote_result {
            Ok(quote_info) => Result::Ok(Self::to_quote_real_time_info(request.symbol, quote_info)),
            Err(err) => Result::Err(LongBridgeBroker::to_rabbit_trading_err(err)),
        }
    }

    async fn query_real_time_depth(
        &self,
        request: QueryInfoRequest,
    ) -> Result<QuoteDepthInfo, Error> {
        todo!()
    }
}

#[cfg(test)]
mod test_longbridge_info {
    use log;
    use rust_decimal_macros::dec;

    use super::LongBridgeInfo;
    use crate::broker::common::info::InfoTrait;
    use crate::model::quote::{QueryInfoRequest, QuoteKind};
    use crate::model::{market::Market, symbol::Symbol};

    #[tokio::test]
    #[cfg_attr(feature = "ci", ignore)]
    async fn test_query_real_time_info() {
        let longbridge_info = LongBridgeInfo::new().await;

        let quote_info_result = longbridge_info
            .query_real_time_info(QueryInfoRequest {
                symbol: Symbol {
                    market: Market::HK,
                    identifier: "0700".to_owned(),
                },
                kind: QuoteKind::Stock,
            })
            .await;
        let quote_info = quote_info_result.unwrap();
        log::warn!("quote_info: {quote_info:?}");
        assert_eq!("0700.HK", quote_info.symbol.to_string());
        assert!(quote_info.current_price > dec!(0.0));
        assert!(quote_info.volume > 0u64);
        assert!(quote_info.high_price.unwrap() > dec!(0.0));
        assert!(quote_info.low_price.unwrap() > dec!(0.0));
        assert!(quote_info.open_price.unwrap() > dec!(0.0));
        assert!(quote_info.prev_close.unwrap() > dec!(0.0));
        assert!(quote_info.turnover.unwrap() > dec!(0.0));
        assert!(quote_info.volume > 0u64);
        assert!(quote_info.timestamp > 0);
    }
}
