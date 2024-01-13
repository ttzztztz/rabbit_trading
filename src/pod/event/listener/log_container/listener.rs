use std::{collections::LinkedList, sync::Arc};
use tokio::sync::{broadcast::Receiver, RwLock, RwLockReadGuard};

use crate::{
    model::{
        common::{error::Error, types::ConfigMap},
        trading::event::RabbitTradingEvent,
    },
    pod::event::listener::common::listener::EventListenerTrait,
};

pub struct LogContainerEventListener {
    data: Arc<RwLock<LinkedList<RabbitTradingEvent>>>,
}

impl LogContainerEventListener {
    async fn async_log_task(
        mut receiver: Receiver<RabbitTradingEvent>,
        data_container: Arc<RwLock<LinkedList<RabbitTradingEvent>>>,
    ) {
        loop {
            match receiver.recv().await {
                Ok(event) => {
                    let mut write_guard = data_container.write().await;
                    write_guard.push_back(event);
                }
                Err(error) => {
                    log::error!("Error when polling data from event bus, {}", error);
                    return;
                }
            }
        }
    }

    pub async fn inspect_log(&self) -> RwLockReadGuard<'_, LinkedList<RabbitTradingEvent>> {
        self.data.read().await
    }
}

impl EventListenerTrait for LogContainerEventListener {
    fn new(_config_map: ConfigMap) -> Self {
        LogContainerEventListener {
            data: Arc::new(RwLock::new(LinkedList::new())),
        }
    }

    fn get_identifier() -> String {
        const IDENTIFIER: &'static str = "LogContainerEventListener";
        return IDENTIFIER.to_owned();
    }

    fn start(&self, receiver: Receiver<RabbitTradingEvent>) {
        tokio::task::spawn(Self::async_log_task(receiver, self.data.clone()));
    }

    fn stop(&self) -> Result<(), Error> {
        todo!()
    }
}

impl Clone for LogContainerEventListener {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(), // shallow clone the Arc pointer
        }
    }
}

#[cfg(test)]
mod test_log_container_event_listener {
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
            common::listener::EventListenerTrait,
            log_container::listener::LogContainerEventListener,
        },
        utils::time::get_now_unix_timestamp,
    };

    #[tokio::test]
    async fn test_whole_lifecycle() {
        const SYMBOL_IDENTIFIER: &'static str = "QQQ";
        const ORDER_ID: &'static str = "order_id_1";
        const POD_ID: &'static str = "test_pod_1";

        let log_container_event_listener = LogContainerEventListener::new(ConfigMap::new());
        let (sender, receiver) = broadcast::channel::<RabbitTradingEvent>(256);
        log_container_event_listener.start(receiver);
        let event = RabbitTradingEvent::SubmitOrder {
            context: EventContext {
                pod_id: POD_ID.to_owned(),
                timestamp: get_now_unix_timestamp(),
            },
            request: SubmitOrderRequest {
                symbol: Symbol {
                    market: Market::US,
                    identifier: SYMBOL_IDENTIFIER.to_owned(),
                },
                quantity: 100i64,
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
        // log_container_event_listener.stop().unwrap();
        // todo: support gracefully stop
    }
}
