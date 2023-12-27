use tokio::sync::broadcast::{self, Receiver, Sender};

use crate::model::event::RabbitTradingEvent;

pub struct EventBus {
    sender: Sender<RabbitTradingEvent>,
}

impl EventBus {
    fn new() -> Self {
        let (sender, receiver) = broadcast::channel::<RabbitTradingEvent>(256);
        Self::start_log_task(receiver);

        EventBus { sender }
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

    fn subscribe(&self) -> Receiver<RabbitTradingEvent> {
        self.sender.subscribe()
    }

    async fn send(
        &self,
        event: RabbitTradingEvent,
    ) -> Result<usize, broadcast::error::SendError<RabbitTradingEvent>> {
        self.sender.send(event)
    }
}
