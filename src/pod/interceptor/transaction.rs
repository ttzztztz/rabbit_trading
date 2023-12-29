use tokio::sync::broadcast::Sender;

use crate::{
    broker::common::transaction::TransactionInterceptorTrait, model::event::RabbitTradingEvent,
};

pub struct PodTransactionInterceptor {
    event_sender: Sender<RabbitTradingEvent>,
}

impl PodTransactionInterceptor {
    pub fn new(event_sender: Sender<RabbitTradingEvent>) -> Self {
        PodTransactionInterceptor { event_sender }
    }
}

impl TransactionInterceptorTrait for PodTransactionInterceptor {}
