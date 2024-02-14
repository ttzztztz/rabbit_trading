use std::{
    collections::LinkedList,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::sync::{broadcast::Receiver, RwLock, RwLockReadGuard};

use crate::{
    model::{
        common::{error::Error, types::ConfigMap},
        trading::event::RabbitTradingEvent,
    },
    pod::event::listener::common::listener::EventListenerTrait,
};

pub struct LogContainerEventListener {
    config_map: ConfigMap,
    data: Arc<RwLock<LinkedList<RabbitTradingEvent>>>,
    stopped_indicator: Arc<AtomicBool>,
}

impl LogContainerEventListener {
    async fn async_log_task(
        mut receiver: Receiver<RabbitTradingEvent>,
        data_container: Arc<RwLock<LinkedList<RabbitTradingEvent>>>,
        stopped_indicator: Arc<AtomicBool>,
    ) {
        loop {
            if stopped_indicator.load(Ordering::Relaxed) {
                return;
            }

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
    fn new(config_map: ConfigMap) -> Self {
        LogContainerEventListener {
            config_map,
            data: Arc::new(RwLock::new(LinkedList::new())),
            stopped_indicator: Arc::new(AtomicBool::new(false)),
        }
    }

    fn get_identifier() -> String {
        const IDENTIFIER: &'static str = "LogContainerEventListener";
        IDENTIFIER.to_owned()
    }

    fn start(&self, receiver: Receiver<RabbitTradingEvent>) {
        tokio::task::spawn(Self::async_log_task(
            receiver,
            self.data.clone(),
            self.stopped_indicator.clone(),
        ));
    }

    fn stop(&self) -> Result<(), Error> {
        self.stopped_indicator.store(true, Ordering::Relaxed);
        Result::Ok(())
    }
}

impl Clone for LogContainerEventListener {
    fn clone(&self) -> Self {
        Self {
            config_map: self.config_map.clone(),
            // shallow clone the Arc pointer
            data: self.data.clone(),
            stopped_indicator: self.stopped_indicator.clone(),
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
        log_container_event_listener.stop().unwrap();
    }
}
