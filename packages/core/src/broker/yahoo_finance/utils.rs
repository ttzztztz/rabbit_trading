use anyhow::{anyhow, Error};
use yahoo_finance_api::YahooError;

use super::broker::YahooFinanceBroker;

impl YahooFinanceBroker {
    pub fn to_rabbit_trading_err(err: YahooError) -> Error {
        let code = match err {
            YahooError::FetchFailed(_) => "FetchFailed",
            YahooError::DeserializeFailed(_) => "DeserializeFailed",
            YahooError::ConnectionFailed(_) => "ConnectionFailed",
            YahooError::InvalidJson => "InvalidJson",
            YahooError::EmptyDataSet => "EmptyDataSet",
            YahooError::DataInconsistency => "DataInconsistency",
            YahooError::BuilderFailed => "BuilderFailed",
        };

        anyhow!("YahooError {}, message:{}", code, err.to_string())
    }
}
