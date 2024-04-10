pub enum StreamingError {
    RequestError(reqwest::Error),
    WebSocketError(tokio_tungstenite::tungstenite::Error),
    ParseError(serde_json::Error),
    OtherError(String),
}

pub fn tokio_tungstenite_error_to_streaming_error(
    error: tokio_tungstenite::tungstenite::Error,
) -> StreamingError {
    StreamingError::WebSocketError(error)
}

pub fn reqwest_error_to_streaming_error(error: reqwest::Error) -> StreamingError {
    StreamingError::RequestError(error)
}
