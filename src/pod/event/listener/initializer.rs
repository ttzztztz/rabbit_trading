use super::common::listener::EventListenerTrait;
use crate::{
    model::common::{error::Error, types::ConfigMap},
    pod::event::listener::log::listener::LogEventListener,
};

pub fn get_event_listener(
    identifier: String,
    config_map: ConfigMap,
) -> Result<Box<dyn EventListenerTrait>, Error> {
    const IDENTIFIER_NOT_MATCHED_ERROR_CODE: &'static str = "IDENTIFIER_NOT_MATCHED";

    match identifier {
        identifier if identifier == LogEventListener::get_identifier() => {
            Result::Ok(Box::new(LogEventListener::new(config_map)))
        }

        _ => Result::Err(Error {
            code: IDENTIFIER_NOT_MATCHED_ERROR_CODE.to_owned(),
            message: format!("Broker: {}", identifier),
        }),
    }
}
