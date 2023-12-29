use tokio::sync::broadcast::Sender;

use crate::{
    broker::common::subscription::SubscriptionInterceptorTrait, model::event::RabbitTradingEvent,
};

pub struct PodSubscriptionInterceptor {
    event_sender: Sender<RabbitTradingEvent>,
}

impl PodSubscriptionInterceptor {
    pub fn new(event_sender: Sender<RabbitTradingEvent>) -> Self {
        PodSubscriptionInterceptor { event_sender }
    }
}

impl SubscriptionInterceptorTrait for PodSubscriptionInterceptor {}
