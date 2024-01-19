use std::collections::LinkedList;
use tokio::sync::{
    broadcast::{self, Receiver, Sender},
    RwLockReadGuard,
};

use super::listener::{
    common::listener::EventListenerTrait, log_container::listener::LogContainerEventListener,
};
use crate::{
    model::{
        common::types::ConfigMap,
        trading::event::{EventContext, RabbitTradingEvent},
    },
    utils::time::get_now_unix_timestamp,
};

pub struct EventBus {
    sender: Sender<RabbitTradingEvent>,
    broker_id: String,
    pod_id: String,
    log_container_event_listener: LogContainerEventListener,
}

impl EventBus {
    pub fn new(broker_id: String, pod_id: String) -> Self {
        let (sender, receiver) = broadcast::channel::<RabbitTradingEvent>(256);
        let log_container_event_listener = LogContainerEventListener::new(ConfigMap::new());
        log_container_event_listener.start(receiver);
        EventBus {
            sender,
            broker_id,
            pod_id,
            log_container_event_listener,
        }
    }

    pub async fn inspect_log(&self) -> RwLockReadGuard<'_, LinkedList<RabbitTradingEvent>> {
        self.log_container_event_listener.inspect_log().await
    }

    pub fn subscribe(&self) -> Receiver<RabbitTradingEvent> {
        self.sender.subscribe()
    }

    pub async fn send(
        &self,
        event: RabbitTradingEvent,
    ) -> Result<usize, broadcast::error::SendError<RabbitTradingEvent>> {
        self.sender.send(event)
    }

    pub fn create_event_context(&self) -> EventContext {
        EventContext {
            broker_id: self.broker_id.clone(),
            pod_id: self.pod_id.clone(),
            timestamp: get_now_unix_timestamp(),
        }
    }

    pub fn shallow_clone(&self, broker_id: Option<String>) -> Self {
        EventBus {
            sender: self.sender.clone(),
            broker_id: broker_id.unwrap_or(self.broker_id.clone()),
            pod_id: self.pod_id.clone(),
            log_container_event_listener: self.log_container_event_listener.clone(),
        }
    }
}
