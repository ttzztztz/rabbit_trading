use std::collections::HashMap;

use model::config::broker::BrokerConfig;
use model::config::persistent_kv_store::PersistentKVStoreConfig;
use model::config::strategy::StrategyConfig;
use simple_logger::SimpleLogger;

use crate::model::config::pod::PodConfig;
use crate::pod::pod::Pod;

mod broker;
mod control_plane;
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
    });

    pod.start().await.unwrap();
}
