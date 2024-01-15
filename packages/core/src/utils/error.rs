use crate::model::common::error::Error;

pub fn reqwest_error_to_rabbit_trading_error(error: reqwest::Error) -> Error {
    const REQWEST_ERROR_CODE: &'static str = "REQWEST_ERROR";
    Error {
        code: REQWEST_ERROR_CODE.to_owned(),
        message: format!("{}", error),
    }
}

pub fn tokio_tungstenite_error_to_rabbit_trading_error(
    error: tokio_tungstenite::tungstenite::Error,
) -> Error {
    const TUNGSTENITE_ERROR_CODE: &'static str = "TUNGSTENITE_ERROR_CODE";
    Error {
        code: TUNGSTENITE_ERROR_CODE.to_owned(),
        message: format!("{}", error),
    }
}
