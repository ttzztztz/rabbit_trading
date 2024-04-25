use anyhow::{anyhow, Error};

use super::common::listener::EventListenerTrait;
use crate::{
    model::common::types::ConfigMap,
    pod::event::listener::{
        log::listener::LogEventListener, log_container::listener::LogContainerEventListener,
    },
};

pub fn get_event_listener(
    identifier: String,
    config_map: ConfigMap,
) -> Result<Box<dyn EventListenerTrait>, Error> {
    match identifier {
        identifier if identifier == LogEventListener::get_identifier() => {
            Result::Ok(Box::new(LogEventListener::new(config_map)))
        }

        identifier if identifier == LogContainerEventListener::get_identifier() => {
            Result::Ok(Box::new(LogContainerEventListener::new(config_map)))
        }

        _ => Result::Err(anyhow!("IDENTIFIER_NOT_MATCHED Broker: {}", identifier)),
    }
}
