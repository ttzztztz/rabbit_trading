use reqwest_retry::policies::ExponentialBackoff;
use rust_decimal::Decimal;
use serial_test::serial;
use tokio_tungstenite::tungstenite::Message;

use crate::{
    client::IBClientPortal,
    model::streaming::{
        BulletinsArgs, NotificationsArgs, StreamingDataResponse, StreamingDataStructuredRequest,
        SubscribeAccountSummaryRequest, ToStructuredRequest, TopicArgsResponse,
    },
    test::utils::{get_test_account, TEST_HOST},
};

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_connect_to_websocket() {
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );

    let (sender, receiver) = ib_cp_client.connect_to_websocket().await.unwrap();
    sender.send_keep_alive_message().await.unwrap();
    sender
        .send_streaming_structured_data_request(
            SubscribeAccountSummaryRequest {
                account_id: get_test_account(),
                keys: vec!["AccruedCash-S".to_owned(), "ExcessLiquidity-S".to_owned()],
                fields: vec!["currency".to_owned(), "monetaryValue".to_owned()],
            }
            .to_structured_request(),
        )
        .await
        .unwrap();

    for _ in 1..=20 {
        match receiver.receive().await {
            Ok(message) => {
                if let StreamingDataResponse::AccountUpdate(_) = message {
                    return;
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
    panic!("Failed: AccountUpdate not received!");
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
            args: BulletinsArgs {
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
            args: NotificationsArgs {
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

#[test]
fn test_decimal_json_parse() {
    assert!(serde_json::from_str::<Decimal>("2.0280151634374067E8").is_ok());
}
