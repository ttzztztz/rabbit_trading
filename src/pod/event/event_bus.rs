use tokio::sync::broadcast::{self, Receiver, Sender};

use super::listener::{common::listener::EventListenerTrait, log::listener::LogEventListener};
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
}

impl EventBus {
    pub fn new(pod_id: String) -> Self {
        let (sender, receiver) = broadcast::channel::<RabbitTradingEvent>(256);
        LogEventListener::new(ConfigMap::new()).start(receiver);
        EventBus { sender, pod_id }
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
        }
    }
}
