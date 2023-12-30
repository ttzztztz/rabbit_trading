use super::{
    event::event_bus::EventBus,
    interceptor::{
        info::PodInfoInterceptor, subscription::PodSubscriptionInterceptor,
        transaction::PodTransactionInterceptor,
    },
};
use crate::{broker::initializer::BrokerInitializer, strategy::common::strategy::StrategyTrait};

pub struct Pod {
    pod_id: String,
    broker_initializer: BrokerInitializer,
    event_bus: EventBus,
    strategy: Box<dyn StrategyTrait + Send + Sync>,
}

impl Pod {
    pub fn new(pod_id: String, strategy: Box<dyn StrategyTrait + Send + Sync>) -> Self {
        Pod {
            pod_id: pod_id.clone(),
            broker_initializer: BrokerInitializer::new(),
            event_bus: EventBus::new(pod_id),
            strategy,
        }
    }

    fn initialize(&self) {
        let info_interceptor = PodInfoInterceptor::new(self.event_bus.clone());
        let subscription_interceptor = PodSubscriptionInterceptor::new(self.event_bus.clone());
        let transaction_interceptor = PodTransactionInterceptor::new(self.event_bus.clone());
    }

    pub fn start(&self) {
        self.initialize();
    }

    pub fn stop(&self) {}
}
