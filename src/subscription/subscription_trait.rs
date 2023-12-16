use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;

use crate::{
    info::info_trait::InfoContext,
    model::{error::Error, quote::QuoteInfo},
};

#[async_trait]
pub trait Subscription {
    async fn create(context: InfoContext) -> Self
    where
        Self: Sized;
    async fn subscribe(&self) -> Result<Receiver<QuoteInfo>, Error>;
    async fn unsubscribe(&self) -> Result<(), Error>;
}
