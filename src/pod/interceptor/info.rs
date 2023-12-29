use tokio::sync::broadcast::Sender;

use crate::{broker::common::info::InfoInterceptorTrait, model::event::RabbitTradingEvent};

pub struct PodInfoInterceptor {
    event_sender: Sender<RabbitTradingEvent>,
}

impl PodInfoInterceptor {
    pub fn new(event_sender: Sender<RabbitTradingEvent>) -> Self {
        PodInfoInterceptor { event_sender }
    }
}

impl InfoInterceptorTrait for PodInfoInterceptor {}
