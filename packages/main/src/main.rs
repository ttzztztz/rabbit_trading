use model::config::metrics_registry::MetricsRegistryConfig;
use simple_logger::SimpleLogger;
use std::collections::HashMap;

use crate::model::config::{
    broker::BrokerConfig, persistent_kv_store::PersistentKVStoreConfig, pod::PodConfig,
    strategy::StrategyConfig,
};
use crate::pod::pod::Pod;

mod broker;
mod control_plane;
mod metrics;
mod model;
mod persistent_kv;
mod pod;
mod strategy;
mod utils;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .env()
        .with_level(::log::LevelFilter::Info)
        .init()
        .unwrap();

    let pod = Pod::new(PodConfig {
        pod_id: "DEMO_POD".to_owned(),
        broker_list: vec![BrokerConfig {
            identifier: "longbridge".to_owned(),
            config_map: HashMap::new(),
        }],
        persistent_kv_store: PersistentKVStoreConfig {
            identifier: "MemoryKVStore".to_owned(),
            config_map: HashMap::new(),
        },
        strategy: StrategyConfig {
            identifier: "ExamplePrintLivePriceStrategy".to_owned(),
            config_map: HashMap::new(),
        },
        metrics_registry: MetricsRegistryConfig {
            identifier: "NoOpMetricRegistryFactory".to_owned(),
            config_map: HashMap::new(),
        },
        event_listener_list: vec![],
    });

    pod.start().await.unwrap();
}
