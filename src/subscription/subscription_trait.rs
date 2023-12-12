use std::sync::mpsc::Receiver;

use crate::{info::info_trait::InfoContext, model::quote::QuoteInfo};

pub trait Subscription {
    fn new(context: InfoContext) -> Self;
    fn subscribe(&self) -> Result<Receiver<QuoteInfo>>;
    fn unsubscribe(&self) -> Result<()>;
}
