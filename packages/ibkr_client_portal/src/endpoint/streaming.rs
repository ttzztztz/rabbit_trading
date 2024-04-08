// https://www.interactivebrokers.com/api/doc.html#tag/Streaming

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde_json::json;
use std::cell::RefCell;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::{
    client::IBClientPortal,
    model::error::{
        reqwest_error_to_streaming_error, tokio_tungstenite_error_to_streaming_error,
        StreamingError,
    },
};

impl IBClientPortal {
    fn get_ws_url(&self) -> String {
        let protocol = if self.listen_ssl { "wss" } else { "ws" };
        format!("{protocol}://{}/v1/api/ws", self.host)
    }

    async fn send_auth_message(&self, sender: &IBStreamingSender) -> Result<(), StreamingError> {
        let session = self
            .tickle()
            .await
            .map_err(reqwest_error_to_streaming_error)?
            .session;
        let message = Message::Text(json!({ "session": session }).to_string());
        sender.send_raw_data(message).await
    }

    pub async fn connect_to_websocket(
        &self,
    ) -> Result<(IBStreamingSender, IBStreamingReceiver), StreamingError> {
        let url = self.get_ws_url();
        let (ws_stream, _) = tokio_tungstenite::connect_async(url)
            .await
            .map_err(tokio_tungstenite_error_to_streaming_error)?;
        let (ws_out, ws_in) = ws_stream.split();

        let sender = IBStreamingSender::new(ws_out);
        let receiver = IBStreamingReceiver::new(ws_in);

        self.send_auth_message(&sender).await?;
        Ok((sender, receiver))
    }
}

pub struct IBStreamingReceiver {
    stream: RefCell<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
}

impl IBStreamingReceiver {
    pub fn new(stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>) -> Self
    where
        Self: Sized,
    {
        IBStreamingReceiver {
            stream: RefCell::new(stream),
        }
    }

    pub async fn receive_raw_data(&self) -> Result<Message, StreamingError> {
        const STREAM_ENDED_MESSAGE: &'static str = "stream ended";

        let mut in_stream = self.stream.borrow_mut();
        if let Some(message) = in_stream.next().await {
            return message.map_err(tokio_tungstenite_error_to_streaming_error);
        }
        Result::Err(StreamingError::OtherError(STREAM_ENDED_MESSAGE.to_owned()))
    }
}

pub struct IBStreamingSender {
    stream: RefCell<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
}

impl IBStreamingSender {
    pub fn new(stream: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>) -> Self
    where
        Self: Sized,
    {
        IBStreamingSender {
            stream: RefCell::new(stream),
        }
    }

    pub async fn send_raw_data(&self, message: Message) -> Result<(), StreamingError> {
        let mut out_stream = self.stream.borrow_mut();
        out_stream
            .send(message)
            .await
            .map_err(tokio_tungstenite_error_to_streaming_error)?;
        Result::Ok(())
    }

    pub async fn close(&self) -> Result<(), StreamingError> {
        let mut out_stream = self.stream.borrow_mut();
        out_stream
            .close()
            .await
            .map_err(tokio_tungstenite_error_to_streaming_error)
    }

    pub async fn send_keep_alive_message(&self) -> Result<(), StreamingError> {
        self.send_raw_data(Message::Text("tic".to_owned())).await
    }

    pub async fn run_keep_alive_loop(&self) -> Result<(), StreamingError> {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(58));
        loop {
            interval.tick().await;
            self.send_keep_alive_message().await?;
        }
    }
}
