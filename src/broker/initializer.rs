use std::collections::HashMap;

use super::common::broker::BrokerTrait;

#[cfg(feature = "longbridge")]
use super::longbridge::broker::LongBridgeBroker;
#[cfg(feature = "yahoo_finance")]
use super::yahoo_finance::broker::YahooFinanceBroker;

pub struct BrokerInitializer {
    pub broker_map: HashMap<String, Box<dyn BrokerTrait + Send + Sync>>,
}

impl BrokerInitializer {
    pub fn new() -> Self {
        let brokers: Vec<Box<dyn BrokerTrait + Send + Sync>> = vec![
            #[cfg(feature = "longbridge")]
            Box::new(LongBridgeBroker {}),
            #[cfg(feature = "yahoo_finance")]
            Box::new(YahooFinanceBroker {}),
        ];

        BrokerInitializer {
            broker_map: brokers
                .into_iter()
                .map(|broker| (broker.get_broker_identifier(), broker))
                .collect(),
        }
    }

    pub fn get_broker_instance(
        &self,
        identifier: String,
    ) -> Option<&Box<dyn BrokerTrait + Send + Sync>> {
        self.broker_map.get(&identifier)
    }
}

#[cfg(test)]
mod test_broker_initializer {
    use super::BrokerInitializer;

    const longbridge_IDENTIFIER: &'static str = "longbridge";
    const YAHOO_FINANCE_IDENTIFIER: &'static str = "yahoo_finance";

    #[test]
    fn test_get_broker_instance() {
        let initializer = BrokerInitializer::new();

        assert_eq!(
            cfg!(feature = "longbridge"),
            initializer
                .get_broker_instance(longbridge_IDENTIFIER.to_owned())
                .is_some()
        );
        assert_eq!(
            cfg!(feature = "yahoo_finance"),
            initializer
                .get_broker_instance(YAHOO_FINANCE_IDENTIFIER.to_owned())
                .is_some()
        );
    }
}
