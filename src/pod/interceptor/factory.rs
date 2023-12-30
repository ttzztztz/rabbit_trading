use super::{
    info::PodInfoInterceptor, subscription::PodSubscriptionInterceptor,
    transaction::PodTransactionInterceptor,
};
use crate::{
    broker::common::{
        broker::BrokerInterceptorFactoryTrait, info::InfoInterceptorTrait,
        subscription::SubscriptionInterceptorTrait, transaction::TransactionInterceptorTrait,
    },
    pod::event::event_bus::EventBus,
};

pub struct PodBrokerInterceptorCollectionFactory {
    event_bus: EventBus,
}

impl PodBrokerInterceptorCollectionFactory {
    pub fn new(event_bus: EventBus) -> Self {
        PodBrokerInterceptorCollectionFactory { event_bus }
    }
}

impl BrokerInterceptorFactoryTrait for PodBrokerInterceptorCollectionFactory {
    fn create_info_interceptor(&self) -> Option<Box<dyn InfoInterceptorTrait>> {
        let info_interceptor = PodInfoInterceptor::new(self.event_bus.clone());
        Option::Some(Box::new(info_interceptor))
    }

    fn create_subscription_interceptor(&self) -> Option<Box<dyn SubscriptionInterceptorTrait>> {
        let subscription_interceptor = PodSubscriptionInterceptor::new(self.event_bus.clone());
        Option::Some(Box::new(subscription_interceptor))
    }

    fn create_transaction_interceptor(&self) -> Option<Box<dyn TransactionInterceptorTrait>> {
        let transaction_interceptor = PodTransactionInterceptor::new(self.event_bus.clone());
        Option::Some(Box::new(transaction_interceptor))
    }
}
