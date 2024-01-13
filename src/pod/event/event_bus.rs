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
    pod_id: String,
    log_container_event_listener: LogContainerEventListener,
}

impl EventBus {
    pub fn new(pod_id: String) -> Self {
        let (sender, receiver) = broadcast::channel::<RabbitTradingEvent>(256);
        let log_container_event_listener = LogContainerEventListener::new(ConfigMap::new());
        log_container_event_listener.start(receiver);
        EventBus {
            sender,
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
            // todo: add broker_id
            pod_id: self.pod_id.clone(),
            timestamp: get_now_unix_timestamp(),
        }
    }
}

impl Clone for EventBus {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            pod_id: self.pod_id.clone(),
            log_container_event_listener: self.log_container_event_listener.clone(),
        }
    }
}
