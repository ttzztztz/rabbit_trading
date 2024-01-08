use crate::model::common::error::Error;

pub fn reqwest_error_to_rabbit_trading_error(error: reqwest::Error) -> Error {
    const REQWEST_ERROR_CODE: &'static str = "REQWEST_ERROR";
    Error {
        code: REQWEST_ERROR_CODE.to_owned(),
        message: format!("{}", error),
    }
}
