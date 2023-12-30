use super::{
    event::event_bus::EventBus,
    interceptor::{
        info::PodInfoInterceptor, subscription::PodSubscriptionInterceptor,
        transaction::PodTransactionInterceptor,
    },
};
use crate::{broker::initializer::BrokerInitializer, model::config::pod::PodConfig};

pub struct Pod {
    pod_config: PodConfig,
    broker_initializer: BrokerInitializer,
    event_bus: EventBus,
}

impl Pod {
    pub fn new(pod_config: PodConfig) -> Self {
        let pod_id = pod_config.pod_id.clone();
        Pod {
            pod_config,
            broker_initializer: BrokerInitializer::new(),
            event_bus: EventBus::new(pod_id),
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
