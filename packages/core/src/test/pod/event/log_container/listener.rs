use rust_decimal_macros::dec;
use std::time::Duration;
use tokio::{sync::broadcast, time::sleep};

use crate::{
    model::{
        common::types::ConfigMap,
        trading::{
            event::{EventContext, RabbitTradingEvent},
            market::Market,
            symbol::Symbol,
            transaction::{
                Direction, Expire, Price, RegularTradingTime, SubmitOrderRequest,
                SubmitOrderResponse,
            },
        },
    },
    pod::event::listener::{
        common::listener::EventListenerTrait, log_container::listener::LogContainerEventListener,
    },
    utils::time::get_now_unix_timestamp,
};

#[tokio::test]
async fn test_whole_lifecycle() {
    const SYMBOL_IDENTIFIER: &'static str = "QQQ";
    const BROKER_ID: &'static str = "broker_id_1";
    const ORDER_ID: &'static str = "order_id_1";
    const POD_ID: &'static str = "test_pod_1";

    let log_container_event_listener = LogContainerEventListener::new(ConfigMap::new());
    let (sender, receiver) = broadcast::channel::<RabbitTradingEvent>(256);
    log_container_event_listener.start(receiver);
    let event = RabbitTradingEvent::SubmitOrder {
        context: EventContext {
            broker_id: BROKER_ID.to_owned(),
            pod_id: POD_ID.to_owned(),
            timestamp: get_now_unix_timestamp(),
        },
        request: SubmitOrderRequest {
            symbol: Symbol {
                market: Market::US,
                identifier: SYMBOL_IDENTIFIER.to_owned(),
            },
            quantity: dec!(100),
            direction: Direction::Buy,
            regular_trading_time: RegularTradingTime::OnlyRegularTradingTime,
            expire: Expire::Day,
            price: Price::MarketOrder,
        },
        result: Result::Ok(SubmitOrderResponse {
            order_id: ORDER_ID.to_owned(),
        }),
    };
    let send_result = sender.send(event.clone());
    assert!(send_result.is_ok());
    sleep(Duration::from_millis(1000)).await;
    let container = log_container_event_listener.inspect_log().await;
    assert!(container.len() == 1);
    let container_log_option = container.front();
    assert!(container_log_option.is_some());
    let container_log = container_log_option.unwrap();
    assert_eq!(event, container_log.clone());
    log_container_event_listener.stop().unwrap();
}
