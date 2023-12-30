use crate::{broker::common::info::InfoInterceptorTrait, pod::event::event_bus::EventBus};

pub struct PodInfoInterceptor {
    event_bus: EventBus,
}

impl PodInfoInterceptor {
    pub fn new(event_bus: EventBus) -> Self {
        PodInfoInterceptor { event_bus }
    }
}

impl InfoInterceptorTrait for PodInfoInterceptor {}
