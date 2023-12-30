use std::time::SystemTime;
use tokio::sync::broadcast::{self, Receiver, Sender};

use crate::model::event::{EventContext, RabbitTradingEvent};

pub struct EventBus {
    sender: Sender<RabbitTradingEvent>,
    pod_id: String,
}

impl EventBus {
    pub fn new(pod_id: String) -> Self {
        let (sender, receiver) = broadcast::channel::<RabbitTradingEvent>(256);
        Self::start_log_task(receiver);

        EventBus { sender, pod_id }
    }

    fn start_log_task(receiver: Receiver<RabbitTradingEvent>) {
        tokio::task::spawn(Self::async_log_task(receiver));
    }

    async fn async_log_task(mut receiver: Receiver<RabbitTradingEvent>) {
        loop {
            match receiver.recv().await {
                Ok(event) => {
                    log::info!("Received a log: {:?}", event);
                }
                Err(error) => {
                    log::error!("Error when polling data from event bus, {}", error);
                    return;
                }
            }
        }
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
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        EventContext {
            pod_id: self.pod_id.clone(),
            timestamp,
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
