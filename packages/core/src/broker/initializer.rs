use std::sync::{atomic::AtomicBool, Arc};

use super::common::broker::{BrokerInterceptorFactoryTrait, BrokerTrait};
use crate::model::common::{error::Error, types::ConfigMap};

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
    const IDENTIFIER_NOT_MATCHED_ERROR_CODE: &'static str = "IDENTIFIER_NOT_MATCHED";

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

        _ => Result::Err(Error {
            code: IDENTIFIER_NOT_MATCHED_ERROR_CODE.to_owned(),
            message: format!("Broker: {}", identifier),
        }),
    }
}

#[cfg(test)]
mod test_broker_initializer {
    use std::{
        collections::HashMap,
        sync::{atomic::AtomicBool, Arc},
    };

    use crate::broker::{
        common::broker::EmptyBrokerInterceptorFactory, initializer::get_broker_instance,
    };

    #[test]
    fn test_get_broker_instance() {
        const LONGBRIDGE_IDENTIFIER: &'static str = "longbridge";
        const YAHOO_FINANCE_IDENTIFIER: &'static str = "yahoo_finance";

        assert_eq!(
            cfg!(feature = "broker__longbridge"),
            get_broker_instance(
                LONGBRIDGE_IDENTIFIER.to_owned(),
                Box::new(EmptyBrokerInterceptorFactory::new()),
                HashMap::new(),
                Arc::new(AtomicBool::new(false)),
            )
            .is_ok()
        );
        assert_eq!(
            cfg!(feature = "broker__yahoo_finance"),
            get_broker_instance(
                YAHOO_FINANCE_IDENTIFIER.to_owned(),
                Box::new(EmptyBrokerInterceptorFactory::new()),
                HashMap::new(),
                Arc::new(AtomicBool::new(false)),
            )
            .is_ok()
        );
    }
}
