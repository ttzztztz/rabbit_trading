use super::common::broker::{BrokerInterceptorFactoryTrait, BrokerTrait};

#[cfg(feature = "longbridge")]
use super::longbridge::broker::LongBridgeBroker;
#[cfg(feature = "yahoo_finance")]
use super::yahoo_finance::broker::YahooFinanceBroker;

pub fn get_broker_instance(
    interceptor_factory: Box<dyn BrokerInterceptorFactoryTrait>,
    identifier: String,
) -> Option<Box<dyn BrokerTrait>> {
    match identifier {
        #[cfg(feature = "longbridge")]
        identifier if identifier == LongBridgeBroker::get_broker_identifier() => {
            Option::Some(Box::new(LongBridgeBroker::new(interceptor_factory)))
        }

        #[cfg(feature = "yahoo_finance")]
        identifier if identifier == YahooFinanceBroker::get_broker_identifier() => {
            Option::Some(Box::new(YahooFinanceBroker::new(interceptor_factory)))
        }

        _ => Option::None,
    }
}
#[cfg(test)]
mod test_broker_initializer {
    use crate::broker::{
        common::broker::EmptyBrokerInterceptorFactory, initializer::get_broker_instance,
    };

    #[test]
    fn test_get_broker_instance() {
        const LONGBRIDGE_IDENTIFIER: &'static str = "longbridge";
        const YAHOO_FINANCE_IDENTIFIER: &'static str = "yahoo_finance";

        assert_eq!(
            cfg!(feature = "longbridge"),
            get_broker_instance(
                Box::new(EmptyBrokerInterceptorFactory::new()),
                LONGBRIDGE_IDENTIFIER.to_owned()
            )
            .is_some()
        );
        assert_eq!(
            cfg!(feature = "yahoo_finance"),
            get_broker_instance(
                Box::new(EmptyBrokerInterceptorFactory::new()),
                YAHOO_FINANCE_IDENTIFIER.to_owned()
            )
            .is_some()
        );
    }
}
