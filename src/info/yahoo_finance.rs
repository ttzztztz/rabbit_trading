use rust_decimal::Decimal;
use std::result::Result;
use yahoo_finance_api::YahooConnector;

use crate::broker::yahoo_finance::YahooFinanceBroker;
use crate::info::info_trait::{Info, InfoContext};
use crate::model::error::Error;
use crate::model::quote::QuoteInfo;

pub struct YahooFinanceInfo {
    provider: YahooConnector,
    pub(crate) context: InfoContext,
}

impl YahooFinanceInfo {
    const YAHOO_LAST_QUOTES_INTERVAL: &'static str = "1d";

    pub(crate) async fn query_quote_info(&self) -> Result<QuoteInfo, Error> {
        let quote = &self.context.quote;

        match self
            .provider
            .get_latest_quotes(quote.identifier.as_str(), Self::YAHOO_LAST_QUOTES_INTERVAL)
            .await
            .and_then(|y_response| y_response.last_quote())
        {
            Result::Ok(yahoo_quote) => {
                log::info!("Received yahoo_quote = {yahoo_quote:?} successfully");

                Result::Ok(QuoteInfo {
                    quote: quote.clone(),
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

impl Info for YahooFinanceInfo {
    fn new(context: InfoContext) -> Self {
        let provider = YahooConnector::new();
        YahooFinanceInfo { provider, context }
    }

    fn query_real_time(&self) -> Result<QuoteInfo, Error> {
        self.context.runtime.block_on(self.query_quote_info())
    }
}

#[cfg(test)]
mod test_yahoo_finance_info {
    use log;
    use longbridge::decimal;
    use std::sync::Arc;
    use tokio::runtime::Runtime;

    use super::YahooFinanceInfo;
    use crate::info::info_trait::{Info, InfoContext};
    use crate::model::quote::Quote;

    #[test]
    fn test_query_quote_info() {
        let runtime = Arc::new(Runtime::new().unwrap());
        let yahoo_finance_info = YahooFinanceInfo::new(InfoContext {
            quote: Quote {
                kind: crate::model::quote::QuoteKind::Stock,
                identifier: "ABNB".to_owned(),
            },
            runtime: runtime.clone(),
            extra: Option::None,
        });

        let quote_info_result = runtime.block_on(yahoo_finance_info.query_quote_info());
        assert!(quote_info_result.is_ok());
        let quote_info = quote_info_result.unwrap();
        log::warn!("quote_info: {quote_info:?}");
        assert_eq!("Stock:ABNB", quote_info.quote.to_string());
        assert!(quote_info.current_price > decimal!(0.0));
        assert!(quote_info.volume > 0u64);
        assert!(quote_info.timestamp > 0i64);
    }
}
