pub enum StreamingError {
    RequestError(reqwest::Error),
    RequestMiddlewareError(anyhow::Error),
    WebSocketError(tokio_tungstenite::tungstenite::Error),
    OtherError(String),
}

pub fn tokio_tungstenite_error_to_streaming_error(
    error: tokio_tungstenite::tungstenite::Error,
) -> StreamingError {
    StreamingError::WebSocketError(error)
}

pub fn reqwest_error_to_streaming_error(error: reqwest_middleware::Error) -> StreamingError {
    match error {
        reqwest_middleware::Error::Middleware(e) => StreamingError::RequestMiddlewareError(e),
        reqwest_middleware::Error::Reqwest(e) => StreamingError::RequestError(e),
    }
}
