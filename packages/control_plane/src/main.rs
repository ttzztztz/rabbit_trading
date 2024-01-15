use rabbit_trading_core::{
    model::config::{
        broker::BrokerConfig, metrics_registry::MetricsRegistryConfig,
        persistent_kv_store::PersistentKVStoreConfig, pod::PodConfig, strategy::StrategyConfig,
    },
    pod::pod::Pod,
};
use simple_logger::SimpleLogger;
use std::collections::HashMap;

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
