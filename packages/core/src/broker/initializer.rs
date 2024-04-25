use anyhow::{anyhow, Error};
use std::sync::{atomic::AtomicBool, Arc};

use super::common::broker::{BrokerInterceptorFactoryTrait, BrokerTrait};
use crate::model::common::types::ConfigMap;

#[cfg(feature = "broker__interactive_brokers")]
use super::interactive_brokers::broker::InteractiveBrokersBroker;
#[cfg(feature = "broker__longbridge")]
use super::longbridge::broker::LongBridgeBroker;
#[cfg(feature = "broker__yahoo_finance")]
use super::yahoo_finance::broker::YahooFinanceBroker;

pub fn get_broker_instance(
    identifier: String,
    interceptor_factory: Box<dyn BrokerInterceptorFactoryTrait>,
    config_map: ConfigMap,
    stopped_indicator: Arc<AtomicBool>,
) -> Result<Box<dyn BrokerTrait>, Error> {
    match identifier {
        #[cfg(feature = "broker__longbridge")]
        identifier if identifier == LongBridgeBroker::get_identifier() => Result::Ok(Box::new(
            LongBridgeBroker::new(interceptor_factory, config_map, stopped_indicator),
        )),

        #[cfg(feature = "broker__yahoo_finance")]
        identifier if identifier == YahooFinanceBroker::get_identifier() => Result::Ok(Box::new(
            YahooFinanceBroker::new(interceptor_factory, config_map, stopped_indicator),
        )),

        #[cfg(feature = "broker__interactive_brokers")]
        identifier if identifier == InteractiveBrokersBroker::get_identifier() => {
            Result::Ok(Box::new(InteractiveBrokersBroker::new(
                interceptor_factory,
                config_map,
                stopped_indicator,
            )))
        }

        _ => Result::Err(anyhow!("IDENTIFIER_NOT_MATCHED Broker: {}", identifier)),
    }
}
