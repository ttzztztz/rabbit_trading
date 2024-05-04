use async_trait::async_trait;

use super::registry::NoOpMetricRegistry;
use crate::{
    metrics::common::{factory::MetricRegistryFactoryTrait, registry::MetricRegistryTrait},
    model::common::types::ConfigMap,
};

pub struct NoOpMetricRegistryFactory {}

#[async_trait]
impl MetricRegistryFactoryTrait for NoOpMetricRegistryFactory {
    fn get_identifier() -> String {
        const IDENTIFIER: &'static str = "NoOpMetricRegistryFactory";
        IDENTIFIER.to_owned()
    }

    fn new(_config_map: ConfigMap) -> Self {
        NoOpMetricRegistryFactory {}
    }

    fn create(&self) -> Box<dyn MetricRegistryTrait> {
        Box::new(NoOpMetricRegistry {})
    }
}
