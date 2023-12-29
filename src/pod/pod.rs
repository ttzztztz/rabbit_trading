use super::{
    event_bus::EventBus,
    interceptor::{
        info::PodInfoInterceptor, subscription::PodSubscriptionInterceptor,
        transaction::PodTransactionInterceptor,
    },
};
use crate::{broker::initializer::BrokerInitializer, strategy::common::strategy::StrategyTrait};

pub struct Pod {
    broker_initializer: BrokerInitializer,
    event_bus: EventBus,
    strategy: Box<dyn StrategyTrait + Send + Sync>,
}

impl Pod {
    fn new(strategy: Box<dyn StrategyTrait + Send + Sync>) -> Self {
        Pod {
            broker_initializer: BrokerInitializer::new(),
            event_bus: EventBus::new(),
            strategy,
        }
    }

    fn initialize(&self) {
        let info_interceptor = PodInfoInterceptor::new(self.event_bus.get_sender());
        let subscription_interceptor = PodSubscriptionInterceptor::new(self.event_bus.get_sender());
        let transaction_interceptor = PodTransactionInterceptor::new(self.event_bus.get_sender());
    }
}
