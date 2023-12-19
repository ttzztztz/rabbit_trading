use async_trait::async_trait;
use rust_decimal::Decimal;
use std::result::Result;
use yahoo_finance_api::YahooConnector;

use crate::broker::common::info_trait::{Info, InfoContext};
use crate::broker::yahoo_finance::broker::YahooFinanceBroker;
use crate::model::error::Error;
use crate::model::quote::QuoteInfo;

pub struct YahooFinanceInfo {
    provider: YahooConnector,
    pub(super) context: InfoContext,
}

impl YahooFinanceInfo {
    const YAHOO_LAST_QUOTES_INTERVAL: &'static str = "1d";
}

#[async_trait]
impl Info for YahooFinanceInfo {
    async fn new(context: InfoContext) -> Self {
        let provider = YahooConnector::new();
        YahooFinanceInfo { provider, context }
    }

    async fn query_real_time_info(&self) -> Result<QuoteInfo, Error> {
        let symbol = &self.context.symbol;

        match self
            .provider
            .get_latest_quotes(symbol.identifier.as_str(), Self::YAHOO_LAST_QUOTES_INTERVAL)
            .await
            .and_then(|y_response| y_response.last_quote())
        {
            Result::Ok(yahoo_quote) => {
                log::info!("Received yahoo_quote = {yahoo_quote:?} successfully");

                Result::Ok(QuoteInfo {
                    symbol: symbol.clone(),
                    sequence: yahoo_quote.timestamp,
                    timestamp: yahoo_quote.timestamp as i64,
                    current_price: Decimal::from_str_exact(
                        format!("{:.2}", yahoo_quote.close).as_str(),
                    )
                    .unwrap(),
                    low_price: Option::None,
                    high_price: Option::None,
                    open_price: Option::None,
                    prev_close: Option::None,
                    volume: yahoo_quote.volume,
                    turnover: Option::None,
                    extra: Option::None,
                })
            }
            Result::Err(err) => {
                log::error!("error {}", err);
                Result::Err(YahooFinanceBroker::to_rabbit_trading_err(err))
            }
        }
    }
}

#[cfg(test)]
mod test_yahoo_finance_info {
    use log;
    use rust_decimal_macros::dec;

    use super::YahooFinanceInfo;
    use crate::{
        broker::common::info_trait::{Info, InfoContext},
        model::{market::Market, symbol::Symbol},
    };

    #[tokio::test]
    async fn test_query_quote_info() {
        let yahoo_finance_info = YahooFinanceInfo::new(InfoContext {
            symbol: Symbol {
                identifier: "ABNB".to_owned(),
                market: Market::US,
            },
            extra: Option::None,
        })
        .await;

        let quote_info_result = yahoo_finance_info.query_real_time_info().await;
        assert!(quote_info_result.is_ok());
        let quote_info = quote_info_result.unwrap();
        log::warn!("quote_info: {quote_info:?}");
        assert_eq!("ABNB.US", quote_info.symbol.to_string());
        assert!(quote_info.current_price > dec!(0.0));
        assert!(quote_info.volume > 0u64);
        assert!(quote_info.timestamp > 0i64);
    }
}
