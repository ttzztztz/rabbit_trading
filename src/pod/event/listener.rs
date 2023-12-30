use tokio::sync::broadcast::Receiver;

use super::event_bus::EventBus;
use crate::model::trading::event::RabbitTradingEvent;

pub trait EventListenerTrait {
    fn new(event_bus: &EventBus) -> Self
    where
        Self: Sized;

    fn stop(&self);
}

pub struct LogEventListener {}

impl LogEventListener {
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

    pub fn new(receiver: Receiver<RabbitTradingEvent>) -> Self {
        tokio::task::spawn(Self::async_log_task(receiver));
        LogEventListener {}
    }
}

impl EventListenerTrait for LogEventListener {
    fn new(event_bus: &EventBus) -> Self {
        let receiver = event_bus.subscribe();
        Self::new(receiver)
    }

    fn stop(&self) {
        todo!()
    }
}
