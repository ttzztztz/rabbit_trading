use std::iter::Map;
use std::sync::Arc;
use tokio::{runtime::Runtime, sync::mpsc::Sender};

use crate::model::quote::{Quote, QuoteInfo};

pub struct InfoContext {
    pub quote: Quote,
    pub runtime: Arc<Runtime>,
    pub sender: Arc<Sender<QuoteInfo>>,
    pub extra: Option<Map<String, String>>,
}

pub trait InfoWorker {
    fn new(context: InfoContext) -> Self;
    fn start(&self);
    fn stop(&self);
}
