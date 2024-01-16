use async_trait::async_trait;

use crate::{
    broker::common::broker::BrokerTrait,
    model::common::{error::Error, types::ConfigMap},
    persistent_kv::common::store::PersistentKVStoreTrait,
};

pub struct StrategyContext {
    pub broker_list: Vec<Box<dyn BrokerTrait>>,
    pub persistent_kv_store: Box<dyn PersistentKVStoreTrait>,
    pub config_map: ConfigMap,
}

#[async_trait]
pub trait StrategyTrait: Send + Sync {
    fn new(strategy_context: StrategyContext) -> Self
    where
        Self: Sized;
    fn get_identifier() -> String
    where
        Self: Sized;

    async fn start(&self) -> Result<(), Error>;
    async fn stop(&self) -> Result<(), Error>;
}
