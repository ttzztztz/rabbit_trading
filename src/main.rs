use std::collections::HashMap;

use broker::{common::broker::BrokerTrait, longbridge::broker::LongBridgeBroker};
use persistent_kv::{
    memory_kv::MemoryKVStore,
    persistent_kv_trait::{PersistentKVStore, PersistentKVStoreParameters},
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

    let longbridge_broker = LongBridgeBroker {};
    let strategy_context = StrategyContext {
        broker_info_list: vec![],
        broker_transaction_list: vec![],
        broker_subscription_list: vec![longbridge_broker.create_subscription(Option::None).await],
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
