use tokio::sync::broadcast::Receiver;

use crate::model::{
    common::{error::Error, types::ConfigMap},
    trading::event::RabbitTradingEvent,
};

pub trait EventListenerTrait {
    fn new(config_map: ConfigMap) -> Self
    where
        Self: Sized;
    fn get_identifier() -> String
    where
        Self: Sized;

    fn start(&self, receiver: Receiver<RabbitTradingEvent>);
    fn stop(&self) -> Result<(), Error>;
}
