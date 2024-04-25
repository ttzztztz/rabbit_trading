use anyhow::Error;
use tokio::sync::broadcast::Receiver;

use crate::{
    model::{common::types::ConfigMap, trading::event::RabbitTradingEvent},
    pod::event::listener::common::listener::EventListenerTrait,
};

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
}

impl EventListenerTrait for LogEventListener {
    fn new(_config_map: ConfigMap) -> Self {
        LogEventListener {}
    }

    fn get_identifier() -> String {
        const IDENTIFIER: &'static str = "LogEventListener";
        IDENTIFIER.to_owned()
    }

    fn start(&self, receiver: Receiver<RabbitTradingEvent>) {
        tokio::task::spawn(Self::async_log_task(receiver));
    }

    fn stop(&self) -> Result<(), Error> {
        todo!()
    }
}
