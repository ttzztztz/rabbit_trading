use async_trait::async_trait;

use super::registry::MetricRegistryTrait;
use crate::model::common::types::ConfigMap;

#[async_trait]
pub trait MetricRegistryFactoryTrait: Send + Sync {
    fn get_identifier() -> String
    where
        Self: Sized;
    fn new(config_map: ConfigMap) -> Self
    where
        Self: Sized;

    async fn create(&self) -> Box<dyn MetricRegistryTrait>;
}
