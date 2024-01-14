use super::common::factory::MetricRegistryFactoryTrait;
use crate::model::common::{error::Error, types::ConfigMap};

#[cfg(feature = "metrics__noops")]
use crate::metrics::noops::factory::NoOpMetricRegistryFactory;
#[cfg(feature = "metrics__statsd")]
use crate::metrics::statsd::factory::StatsDMetricRegistryFactory;

pub fn get_metrics_registry_factory(
    identifier: String,
    config_map: ConfigMap,
) -> Result<Box<dyn MetricRegistryFactoryTrait>, Error> {
    const IDENTIFIER_NOT_MATCHED_ERROR_CODE: &'static str = "IDENTIFIER_NOT_MATCHED";

    match identifier {
        #[cfg(feature = "metrics__noops")]
        identifier if identifier == NoOpMetricRegistryFactory::get_identifier() => {
            Result::Ok(Box::new(NoOpMetricRegistryFactory::new(config_map)))
        }

        #[cfg(feature = "metrics__statsd")]
        identifier if identifier == StatsDMetricRegistryFactory::get_identifier() => {
            Result::Ok(Box::new(StatsDMetricRegistryFactory::new(config_map)))
        }

        _ => Result::Err(Error {
            code: IDENTIFIER_NOT_MATCHED_ERROR_CODE.to_owned(),
            message: format!("MetricsRegistryFactory: {}", identifier),
        }),
    }
}
