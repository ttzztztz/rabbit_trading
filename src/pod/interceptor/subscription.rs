use crate::{broker::common::subscription::SubscriptionInterceptorTrait, pod::event_bus::EventBus};

pub struct PodSubscriptionInterceptor {
    event_bus: EventBus,
}

impl PodSubscriptionInterceptor {
    pub fn new(event_bus: EventBus) -> Self {
        PodSubscriptionInterceptor { event_bus }
    }
}

impl SubscriptionInterceptorTrait for PodSubscriptionInterceptor {}
