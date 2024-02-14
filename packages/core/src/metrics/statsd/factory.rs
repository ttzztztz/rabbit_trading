use async_trait::async_trait;
use dogstatsd::{Client, Options};

use super::registry::StatsDMetricRegistry;
use crate::{
    metrics::common::{factory::MetricRegistryFactoryTrait, registry::MetricRegistryTrait},
    model::common::types::ConfigMap,
};

pub struct StatsDMetricRegistryFactory {
    config_map: ConfigMap,
}

#[async_trait]
impl MetricRegistryFactoryTrait for StatsDMetricRegistryFactory {
    fn get_identifier() -> String {
        const IDENTIFIER: &'static str = "StatsDMetricRegistryFactory";
        IDENTIFIER.to_owned()
    }

    fn new(config_map: ConfigMap) -> Self {
        StatsDMetricRegistryFactory { config_map }
    }

    async fn create(&self) -> Box<dyn MetricRegistryTrait> {
        const CONFIG_KEY_CLIENT_FROM_ADDRESS: &'static str = "metrics.statsd.from.address";
        const CONFIG_KEY_CLIENT_TO_ADDRESS: &'static str = "metrics.statsd.to.address";
        const CONFIG_KEY_CLIENT_PREFIX: &'static str = "metrics.statsd.prefix";
        const CONFIG_DEFAULT_VALUE_CLIENT_PREFIX: &'static str = "rabbit.trading.";

        let from_address = self
            .config_map
            .get(CONFIG_KEY_CLIENT_FROM_ADDRESS)
            .unwrap()
            .clone();
        let to_address = self
            .config_map
            .get(CONFIG_KEY_CLIENT_TO_ADDRESS)
            .unwrap()
            .clone();
        let prefix = self
            .config_map
            .get(CONFIG_KEY_CLIENT_PREFIX)
            .map(|value| value.to_string())
            .unwrap_or(CONFIG_DEFAULT_VALUE_CLIENT_PREFIX.to_owned());

        let client_options = Options::new(
            from_address.as_str(),
            to_address.as_str(),
            prefix.as_str(),
            vec![],
            Option::None,
            Option::None,
        );
        let client = Client::new(client_options).unwrap();
        Box::new(StatsDMetricRegistry::new(client))
    }
}
