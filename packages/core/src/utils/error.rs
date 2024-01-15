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
    const TUNGSTENITE_ERROR_CODE: &'static str = "TUNGSTENITE_ERROR";
    Error {
        code: TUNGSTENITE_ERROR_CODE.to_owned(),
        message: format!("{}", error),
    }
}

pub fn env_var_error_to_rabbit_trading_error(error: std::env::VarError) -> Error {
    const ENV_VAR_ERROR_CODE: &'static str = "ENV_VAR_ERROR";
    const VALUE_NOT_PRESENT_ERROR_MESSAGE: &'static str = "Value not present!";
    const VALUE_NOT_UNICODE_ERROR_MESSAGE: &'static str = "Value not unicode!";

    match error {
        std::env::VarError::NotPresent => Error {
            code: ENV_VAR_ERROR_CODE.to_owned(),
            message: VALUE_NOT_PRESENT_ERROR_MESSAGE.to_owned(),
        },
        std::env::VarError::NotUnicode(_) => Error {
            code: ENV_VAR_ERROR_CODE.to_owned(),
            message: VALUE_NOT_UNICODE_ERROR_MESSAGE.to_owned(),
        },
    }
}
