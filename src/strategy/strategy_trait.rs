use crate::{info::info_trait::Info, subscription::subscription_trait::Subscription};
use async_trait::async_trait;

pub struct StrategyContext {
    pub info: Box<dyn Info>,
    // pub position: Position, # todo
    pub subscription: Box<dyn Subscription>,
    // pub tarnsaction: Transaction, # todo
}

#[async_trait]
pub trait Strategy {
    async fn new() -> Self
    where
        Self: Sized;
    async fn start(&self, context: StrategyContext);
    async fn stop(&self);
}
