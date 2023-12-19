use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;

use super::info::InfoContext;
use crate::model::{error::Error, quote::QuoteInfo};

#[async_trait]
pub trait SubscriptionTrait {
    async fn new(context: InfoContext) -> Self
    where
        Self: Sized;
    async fn subscribe(&self) -> Result<Receiver<QuoteInfo>, Error>;
    async fn unsubscribe(&self) -> Result<(), Error>;
}
