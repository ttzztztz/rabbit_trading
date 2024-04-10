use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;

pub struct StreamingDataRequest {
    pub topic: String,
    pub arguments: Option<Vec<String>>,
    pub body: Option<String>,
}

impl StreamingDataRequest {
    pub fn to_message(&self) -> Message {
        let mut components = vec![self.topic.clone()];
        if let Some(arguments) = &self.arguments {
            components.append(&mut arguments.clone());
        }
        if let Some(body) = &self.body {
            components.push(body.clone());
        }
        Message::Text(components.join("+").to_owned())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(untagged)]
pub enum StreamingDataResponse {
    #[serde(rename = "blt")]
    Bulletins(TopicArgsResponse<BulletinsResponse>),
    #[serde(rename = "ntf")]
    Notifications(TopicArgsResponse<NotificationsResponse>),

    #[serde(skip_serializing)]
    Unknown(String),
}

impl StreamingDataResponse {
    pub fn from_str(str: &str) -> StreamingDataResponse {
        serde_json::from_str::<StreamingDataResponse>(&str)
            .unwrap_or(StreamingDataResponse::Unknown(str.to_string()))
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct TopicArgsResponse<T> {
    pub topic: String,
    pub args: T,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct BulletinsResponse {
    pub id: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct NotificationsResponse {
    pub id: String,
    pub text: String,
    pub title: Option<String>,
    pub url: Option<String>,
}
