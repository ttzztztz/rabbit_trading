use std::collections::HashMap;

use broker::{
    common::broker::{BrokerTrait, EmptyBrokerInterceptorFactory},
    longbridge::broker::LongBridgeBroker,
};
use persistent_kv::{
    common::persistent_kv::{PersistentKVStore, PersistentKVStoreParameters},
    memory::memory_kv::MemoryKVStore,
};
use simple_logger::SimpleLogger;
use strategy::{
    common::strategy::{StrategyContext, StrategyTrait},
    example::print_live_price::PrintLivePriceStrategy,
};

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

    let longbridge_broker = Box::new(LongBridgeBroker::new(Box::new(
        EmptyBrokerInterceptorFactory::new(),
    )));
    let strategy_context = StrategyContext {
        broker_list: vec![longbridge_broker],
        persistent_kv_store: Box::new(
            MemoryKVStore::new(PersistentKVStoreParameters {
                configuration: HashMap::<String, ()>::new(),
            })
            .await,
        ),
    };
    let print_live_price_strategy = PrintLivePriceStrategy::new(strategy_context).await;
    print_live_price_strategy.start().await;
}
