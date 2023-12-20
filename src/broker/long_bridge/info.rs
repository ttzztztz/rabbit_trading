use async_trait::async_trait;
use longbridge::quote::SecurityQuote;
use longbridge::QuoteContext;
use std::result::Result;

use super::broker::LongBridgeBroker;
use crate::broker::common::info::{InfoContext, InfoTrait};
use crate::model::error::Error;
use crate::model::quote::QuoteInfo;
use crate::model::symbol::Symbol;

pub struct LongBridgeInfo {
    context: InfoContext,
    longbridge_context: QuoteContext,
}

impl LongBridgeInfo {
    fn to_quote_info(symbol: Symbol, security_quote: SecurityQuote) -> QuoteInfo {
        QuoteInfo {
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
    async fn new(context: InfoContext) -> Self {
        let (ctx, _) = LongBridgeBroker::create_quote_context().await.unwrap();

        LongBridgeInfo {
            context,
            longbridge_context: ctx,
        }
    }

    async fn query_real_time_info(&self) -> Result<QuoteInfo, Error> {
        let symbol_identifier = self.context.symbol.to_string();
        let quote_result = self
            .longbridge_context
            .quote([symbol_identifier])
            .await
            .map(|result_vec| result_vec[0].clone());

        match quote_result {
            Ok(quote_info) => {
                Result::Ok(Self::to_quote_info(self.context.symbol.clone(), quote_info))
            }
            Err(err) => Result::Err(LongBridgeBroker::to_rabbit_trading_err(err)),
        }
    }
}

#[cfg(test)]
mod test_long_bridge_info {
    use log;
    use rust_decimal_macros::dec;

    use super::LongBridgeInfo;
    use crate::broker::common::info::{InfoContext, InfoTrait};
    use crate::model::{market::Market, symbol::Symbol};

    #[tokio::test]
    #[cfg_attr(feature = "ci", ignore)]
    async fn test_query_real_time_info() {
        let long_bridge_info = LongBridgeInfo::new(InfoContext {
            symbol: Symbol {
                identifier: "0700".to_owned(),
                market: Market::HK,
            },
            extra: Option::None,
        })
        .await;

        let quote_info_result = long_bridge_info.query_real_time_info().await;
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
