use longbridge::{Config, QuoteContext, Result, TradeContext};
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedReceiver;

use super::broker::LongBridgeBroker;
use crate::model::{currency::Currency, error::Error, market::Market, symbol::Symbol};

impl LongBridgeBroker {
    const OTHER_ERROR_CODE: &'static str = "other";

    pub async fn create_quote_context() -> Result<(
        QuoteContext,
        UnboundedReceiver<longbridge::quote::PushEvent>,
    )> {
        let config = Arc::new(Config::from_env().unwrap());
        QuoteContext::try_new(config.clone()).await
    }

    pub async fn create_trade_context() -> Result<(
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

    pub fn to_currency(currency: &str) -> Option<Currency> {
        match currency.to_uppercase().as_str() {
            "HKD" => Option::Some(Currency::HKD),
            "USD" => Option::Some(Currency::USD),
            "CNH" => Option::Some(Currency::CNH),
            _ => {
                log::error!("Error when parsing currency {}", currency);
                Option::None
            }
        }
    }

    pub fn to_market(market: &str) -> Option<Market> {
        match market.to_uppercase().as_str() {
            "US" => Option::Some(Market::US),
            "HK" => Option::Some(Market::HK),
            "CN" => Option::Some(Market::CN),
            _ => {
                log::error!("Error when parsing market {}", market);
                Option::None
            }
        }
    }

    pub fn to_symbol(symbol: &str) -> Option<Symbol> {
        let splitted_vec: Vec<&str> = symbol.split('.').collect();
        if splitted_vec.len() != 2 {
            log::error!("Error when parsing symbol {}", symbol);
            return Option::None;
        }

        Option::Some(Symbol {
            market: Self::to_market(splitted_vec[1])?,
            identifier: splitted_vec[0].to_owned(),
        })
    }
}

#[cfg(test)]
mod test_long_bridge_broker_utils {
    use crate::broker::long_bridge::broker::LongBridgeBroker;
    use crate::model::{currency::Currency, market::Market, symbol::Symbol};

    #[test]
    fn test_to_currency() {
        assert_eq!(Option::None, LongBridgeBroker::to_currency("JPY"));
        assert_eq!(
            Option::Some(Currency::CNH),
            LongBridgeBroker::to_currency("CNH")
        );
        assert_eq!(
            Option::Some(Currency::USD),
            LongBridgeBroker::to_currency("USD")
        );
        assert_eq!(
            Option::Some(Currency::HKD),
            LongBridgeBroker::to_currency("HKD")
        );
    }

    #[test]
    fn test_to_market() {
        assert_eq!(Option::None, LongBridgeBroker::to_market("JP"));
        assert_eq!(Option::Some(Market::CN), LongBridgeBroker::to_market("CN"));
        assert_eq!(Option::Some(Market::US), LongBridgeBroker::to_market("US"));
        assert_eq!(Option::Some(Market::HK), LongBridgeBroker::to_market("HK"));
    }

    #[test]
    fn test_to_symbol() {
        assert_eq!(Option::None, LongBridgeBroker::to_symbol("8316.JP"));
        assert_eq!(
            Option::Some(Symbol {
                market: Market::US,
                identifier: "META".to_owned(),
            }),
            LongBridgeBroker::to_symbol("META.US")
        );
        assert_eq!(
            Option::Some(Symbol {
                market: Market::HK,
                identifier: "0700".to_owned(),
            }),
            LongBridgeBroker::to_symbol("0700.HK")
        );
    }
}
