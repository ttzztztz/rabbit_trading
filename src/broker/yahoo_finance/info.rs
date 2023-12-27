use async_trait::async_trait;
use rust_decimal::Decimal;
use std::result::Result;
use yahoo_finance_api::YahooConnector;

use crate::broker::common::info::InfoTrait;
use crate::broker::yahoo_finance::broker::YahooFinanceBroker;
use crate::model::error::Error;
use crate::model::quote::{QueryInfoRequest, QuoteBasicInfo, QuoteDepthInfo, QuoteRealTimeInfo};
use crate::model::symbol::Symbol;

pub struct YahooFinanceInfo {
    provider: YahooConnector,
}

impl YahooFinanceInfo {
    const YAHOO_LAST_QUOTES_INTERVAL: &'static str = "1d";

    fn to_quote_real_time_info(
        symbol: Symbol,
        yahoo_quote: yahoo_finance_api::Quote,
    ) -> QuoteRealTimeInfo {
        QuoteRealTimeInfo {
            symbol,
            sequence: yahoo_quote.timestamp,
            timestamp: yahoo_quote.timestamp as i64,
            current_price: Decimal::from_str_exact(format!("{:.2}", yahoo_quote.close).as_str())
                .unwrap(),
            volume: yahoo_quote.volume,
            low_price: Option::None,
            high_price: Option::None,
            open_price: Option::None,
            prev_close: Option::None,
            turnover: Option::None,
            extra: Option::None,
        }
    }
}

#[async_trait]
impl InfoTrait for YahooFinanceInfo {
    async fn new() -> Self {
        let provider = YahooConnector::new();
        YahooFinanceInfo { provider }
    }

    async fn query_basic_info(&self, _request: QueryInfoRequest) -> Result<QuoteBasicInfo, Error> {
        todo!()
    }

    async fn query_real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<QuoteRealTimeInfo, Error> {
        let symbol = request.symbol;

        match self
            .provider
            .get_latest_quotes(symbol.identifier.as_str(), Self::YAHOO_LAST_QUOTES_INTERVAL)
            .await
            .and_then(|y_response| y_response.last_quote())
        {
            Result::Ok(yahoo_quote) => {
                log::info!("Received yahoo_quote = {yahoo_quote:?} successfully");
                Result::Ok(Self::to_quote_real_time_info(symbol.clone(), yahoo_quote))
            }
            Result::Err(err) => {
                log::error!("error {}", err);
                Result::Err(YahooFinanceBroker::to_rabbit_trading_err(err))
            }
        }
    }

    async fn query_depth(&self, _request: QueryInfoRequest) -> Result<QuoteDepthInfo, Error> {
        todo!()
    }
}

#[cfg(test)]
mod test_yahoo_finance_info {
    use log;
    use rust_decimal_macros::dec;

    use super::YahooFinanceInfo;
    use crate::{
        broker::common::info::InfoTrait,
        model::{
            market::Market,
            quote::{QueryInfoRequest, QuoteKind},
            symbol::Symbol,
        },
    };

    #[tokio::test]
    async fn test_query_quote_info() {
        let yahoo_finance_info = YahooFinanceInfo::new().await;

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
        assert!(quote_info.timestamp > 0i64);
    }
}
