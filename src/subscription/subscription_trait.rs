use tokio::sync::mpsc::Receiver;

use crate::{
    info::info_trait::InfoContext,
    model::{error::Error, quote::QuoteInfo},
};

pub trait Subscription {
    fn new(context: InfoContext) -> Self;
    fn subscribe(&self) -> Result<Receiver<QuoteInfo>, Error>;
    fn unsubscribe(&self) -> Result<(), Error>;
}
