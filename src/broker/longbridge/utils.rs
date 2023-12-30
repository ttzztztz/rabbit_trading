use longbridge::{Config, QuoteContext, TradeContext};
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedReceiver;

use super::broker::LongBridgeBroker;
use crate::model::{
    common::error::Error,
    trading::{currency::Currency, market::Market, symbol::Symbol},
};

impl LongBridgeBroker {
    pub(super) const OTHER_ERROR_CODE: &'static str = "OTHER_ERROR";
    pub(super) const PARSING_ERROR_CODE: &'static str = "PARSING_ERROR";

    pub async fn create_quote_context() -> longbridge::Result<(
        QuoteContext,
        UnboundedReceiver<longbridge::quote::PushEvent>,
    )> {
        let config = Arc::new(Config::from_env().unwrap());
        QuoteContext::try_new(config.clone()).await
    }

    pub async fn create_trade_context() -> longbridge::Result<(
        TradeContext,
        UnboundedReceiver<longbridge::trade::PushEvent>,
    )> {
        let config = Arc::new(Config::from_env().unwrap());
        TradeContext::try_new(config.clone()).await
    }

    pub fn to_rabbit_trading_err(err: longbridge::Error) -> Error {
        match err.into_simple_error() {
            longbridge::SimpleError::Response {
                code,
                message,
                trace_id,
            } => Error {
                code: format!("{}", code),
                message: format!("{}, trace_id={}", message, trace_id),
            },
            longbridge::SimpleError::Other(message) => Error {
                code: Self::OTHER_ERROR_CODE.to_owned(),
                message,
            },
        }
    }

    pub fn to_currency(currency: &str) -> Result<Currency, Error> {
        match currency.to_uppercase().as_str() {
            "HKD" => Result::Ok(Currency::HKD),
            "USD" => Result::Ok(Currency::USD),
            "CNH" => Result::Ok(Currency::CNH),
            _ => Result::Err(Error {
                code: Self::PARSING_ERROR_CODE.to_owned(),
                message: format!("Error when parsing currency {}", currency),
            }),
        }
    }

    pub fn to_market(market: &str) -> Result<Market, Error> {
        match market.to_uppercase().as_str() {
            "US" => Result::Ok(Market::US),
            "HK" => Result::Ok(Market::HK),
            "CN" => Result::Ok(Market::CN),
            _ => Result::Err(Error {
                code: Self::PARSING_ERROR_CODE.to_owned(),
                message: format!("Error when parsing market {}", market),
            }),
        }
    }

    pub fn to_symbol(symbol: &str) -> Result<Symbol, Error> {
        let splitted_vec: Vec<&str> = symbol.split('.').collect();
        if splitted_vec.len() != 2 {
            return Result::Err(Error {
                code: Self::PARSING_ERROR_CODE.to_owned(),
                message: format!("Error when parsing symbol {}", symbol),
            });
        }

        Result::Ok(Symbol {
            market: Self::to_market(splitted_vec[1])?,
            identifier: splitted_vec[0].to_owned(),
        })
    }
}

#[cfg(test)]
mod test_longbridge_broker_utils {
    use crate::broker::longbridge::broker::LongBridgeBroker;
    use crate::model::trading::{currency::Currency, market::Market, symbol::Symbol};

    #[test]
    fn test_to_currency() {
        assert!(LongBridgeBroker::to_currency("JPY").is_err());
        assert_eq!(
            Result::Ok(Currency::CNH),
            LongBridgeBroker::to_currency("CNH")
        );
        assert_eq!(
            Result::Ok(Currency::USD),
            LongBridgeBroker::to_currency("USD")
        );
        assert_eq!(
            Result::Ok(Currency::HKD),
            LongBridgeBroker::to_currency("HKD")
        );
    }

    #[test]
    fn test_to_market() {
        assert!(LongBridgeBroker::to_market("JP").is_err());
        assert_eq!(Result::Ok(Market::CN), LongBridgeBroker::to_market("CN"));
        assert_eq!(Result::Ok(Market::US), LongBridgeBroker::to_market("US"));
        assert_eq!(Result::Ok(Market::HK), LongBridgeBroker::to_market("HK"));
    }

    #[test]
    fn test_to_symbol() {
        assert!(LongBridgeBroker::to_symbol("8316.JP").is_err());
        assert_eq!(
            Result::Ok(Symbol {
                market: Market::US,
                identifier: "META".to_owned(),
            }),
            LongBridgeBroker::to_symbol("META.US")
        );
        assert_eq!(
            Result::Ok(Symbol {
                market: Market::HK,
                identifier: "0700".to_owned(),
            }),
            LongBridgeBroker::to_symbol("0700.HK")
        );
    }
}
