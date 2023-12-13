use async_trait::async_trait;
use longbridge::QuoteContext;
use std::result::Result;

use crate::broker::long_bridge::LongBridgeBroker;
use crate::info::info_trait::{Info, InfoContext};
use crate::model::error::Error;
use crate::model::quote::QuoteInfo;

pub struct LongBridgeInfo {
    context: InfoContext,
    longbridge_context: QuoteContext,
}

#[async_trait]
impl Info for LongBridgeInfo {
    async fn new(context: InfoContext) -> Self {
        let (ctx, _) = LongBridgeBroker::create_context().await.unwrap();

        LongBridgeInfo {
            context,
            longbridge_context: ctx,
        }
    }

    async fn query_real_time_info(&self) -> Result<QuoteInfo, Error> {
        let quote_result = self
            .longbridge_context
            .quote([self.context.quote.identifier.clone()])
            .await
            .map(|result_vec| result_vec[0].clone());

        match quote_result {
            Ok(quote_info) => Result::Ok(QuoteInfo {
                quote: self.context.quote.clone(),
                sequence: quote_info.timestamp.unix_timestamp() as u64,
                timestamp: quote_info.timestamp.unix_timestamp(),
                current_price: quote_info.last_done,
                low_price: Option::Some(quote_info.low),
                high_price: Option::Some(quote_info.high),
                open_price: Option::Some(quote_info.open),
                prev_close: Option::Some(quote_info.prev_close),
                volume: quote_info.volume as u64,
                turnover: Option::Some(quote_info.turnover),
                extra: Option::None,
            }),
            Err(err) => Result::Err(LongBridgeBroker::to_rabbit_trading_err(err)),
        }
    }
}

#[cfg(test)]
mod test_long_bridge_info {
    use log;
    use longbridge::decimal;

    use super::LongBridgeInfo;
    use crate::info::info_trait::{Info, InfoContext};
    use crate::model::quote::Quote;

    #[tokio::test]
    #[cfg_attr(feature = "ci", ignore)]
    async fn test_query_real_time_info() {
        let long_bridge_info = LongBridgeInfo::new(InfoContext {
            quote: Quote {
                kind: crate::model::quote::QuoteKind::Stock,
                identifier: "0700.HK".to_owned(),
            },
            extra: Option::None,
        })
        .await;

        let quote_info_result = long_bridge_info.query_real_time_info().await;
        let quote_info = quote_info_result.unwrap();
        log::warn!("quote_info: {quote_info:?}");
        assert_eq!("Stock:0700.HK", quote_info.quote.to_string());
        assert!(quote_info.current_price > decimal!(0.0));
        assert!(quote_info.volume > 0u64);
        assert!(quote_info.high_price.unwrap() > decimal!(0.0));
        assert!(quote_info.low_price.unwrap() > decimal!(0.0));
        assert!(quote_info.open_price.unwrap() > decimal!(0.0));
        assert!(quote_info.prev_close.unwrap() > decimal!(0.0));
        assert!(quote_info.turnover.unwrap() > decimal!(0.0));
        assert!(quote_info.volume > 0u64);
        assert!(quote_info.timestamp > 0);
    }
}
