use serial_test::serial;
use tokio_tungstenite::tungstenite::Message;

use crate::model::streaming::{
    BulletinsResponse, NotificationsResponse, StreamingDataResponse,
    StreamingDataStructuredRequest, TopicArgsResponse,
};

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "flaky_test_cases"), ignore)]
async fn test_connect_to_websocket() {
    todo!()
}

#[test]
fn test_stream_data_request_to_message() {
    let body_str_1: &'static str =
        r#"{"keys":["AccruedCash-S","ExcessLiquidity-S"],"fields":["currency","monetaryValue"]}"#;
    let stream_data_reqeust_1 = StreamingDataStructuredRequest {
        topic: "ssd".to_owned(),
        arguments: Option::Some(vec!["DU1234567".to_owned()]),
        body: Option::Some(body_str_1.to_owned()),
    };
    assert_eq!(
        Message::Text(format!("ssd+DU1234567+{}", body_str_1)),
        stream_data_reqeust_1.to_message()
    );

    let body_str_2: &'static str = "{}";
    let stream_data_reqeust_2 = StreamingDataStructuredRequest {
        topic: "usd".to_owned(),
        arguments: Option::Some(vec!["DU1234567".to_owned()]),
        body: Option::Some(body_str_2.to_owned()),
    };
    assert_eq!(
        Message::Text(format!("usd+DU1234567+{}", body_str_2)),
        stream_data_reqeust_2.to_message()
    );

    let stream_data_reqeust_3 = StreamingDataStructuredRequest {
        topic: "umh".to_owned(),
        arguments: Option::Some(vec!["12345".to_owned()]),
        body: Option::None,
    };
    assert_eq!(
        Message::Text("umh+12345".to_owned()),
        stream_data_reqeust_3.to_message()
    );

    let stream_data_reqeust_4 = StreamingDataStructuredRequest {
        topic: "tic".to_owned(),
        arguments: Option::None,
        body: Option::None,
    };
    assert_eq!(
        Message::Text("tic".to_owned()),
        stream_data_reqeust_4.to_message()
    );

    let stream_data_reqeust_5 = StreamingDataStructuredRequest {
        topic: "upl".to_owned(),
        arguments: Option::None,
        body: Option::Some("{}".to_owned()),
    };
    assert_eq!(
        Message::Text("upl+{}".to_owned()),
        stream_data_reqeust_5.to_message()
    );
}

#[test]
fn parse_stream_data_response_serde_parse() {
    assert_eq!(
        StreamingDataResponse::Bulletins(TopicArgsResponse {
            topic: "blt".to_owned(),
            args: BulletinsResponse {
                id: "id".to_owned(),
                message: "message".to_owned()
            }
        }),
        StreamingDataResponse::from_str(
            r#"{"topic":"blt","args":{"id":"id","message":"message"}}"#
        )
    );

    assert_eq!(
        StreamingDataResponse::Notifications(TopicArgsResponse {
            topic: "ntf".to_owned(),
            args: NotificationsResponse {
                id: "id".to_owned(),
                text: "text".to_owned(),
                title: Option::Some("title".to_owned()),
                url: Option::Some("url".to_owned()),
            }
        }),
        StreamingDataResponse::from_str(
            r#"{"topic":"ntf","args":{"id":"id","text":"text","title":"title","url":"url"}}"#
        )
    );

    const UNKNOWN_STR: &'static str = r#"{"unknown":"unknown"}"#;
    assert_eq!(
        StreamingDataResponse::Unknown(UNKNOWN_STR.to_owned()),
        StreamingDataResponse::from_str(UNKNOWN_STR)
    );
}
