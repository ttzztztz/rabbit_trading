use async_trait::async_trait;

use crate::model::common::{error::Error, types::ConfigMap};

#[async_trait]
pub trait HeartbeatTrait: Send + Sync {
    async fn new(config_map: ConfigMap) -> Self
    where
        Self: Sized;

    async fn start(&self) -> Result<(), Error>;
    async fn stop(&self) -> Result<(), Error>;
}
