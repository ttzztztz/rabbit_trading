use async_trait::async_trait;

use super::registry::StatsDMetricRegistry;
use crate::{
    metrics::common::{factory::MetricRegistryFactoryTrait, registry::MetricRegistryTrait},
    model::common::types::ConfigMap,
};

pub struct StatsDMetricRegistryFactory {}

#[async_trait]
impl MetricRegistryFactoryTrait for StatsDMetricRegistryFactory {
    fn get_identifier() -> String {
        const IDENTIFIER: &'static str = "StatsDMetricRegistryFactory";
        return IDENTIFIER.to_owned();
    }

    fn new(_config_map: ConfigMap) -> Self {
        StatsDMetricRegistryFactory {}
    }

    async fn create(&self) -> Box<dyn MetricRegistryTrait> {
        Box::new(StatsDMetricRegistry {})
    }
}
