use anyhow::{anyhow, Error};

use super::common::factory::MetricRegistryFactoryTrait;
use crate::model::common::types::ConfigMap;

#[cfg(feature = "metrics__noops")]
use crate::metrics::noops::factory::NoOpMetricRegistryFactory;
#[cfg(feature = "metrics__statsd")]
use crate::metrics::statsd::factory::StatsDMetricRegistryFactory;

pub fn get_metrics_registry_factory(
    identifier: String,
    config_map: ConfigMap,
) -> Result<Box<dyn MetricRegistryFactoryTrait>, Error> {
    match identifier {
        #[cfg(feature = "metrics__noops")]
        identifier if identifier == NoOpMetricRegistryFactory::get_identifier() => {
            Result::Ok(Box::new(NoOpMetricRegistryFactory::new(config_map)))
        }

        #[cfg(feature = "metrics__statsd")]
        identifier if identifier == StatsDMetricRegistryFactory::get_identifier() => {
            Result::Ok(Box::new(StatsDMetricRegistryFactory::new(config_map)))
        }

        _ => Result::Err(anyhow!(
            "IDENTIFIER_NOT_MATCHED MetricsRegistryFactory: {}",
            identifier
        )),
    }
}
