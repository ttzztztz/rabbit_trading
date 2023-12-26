use super::transaction::{CancelOrderRequest, EditOrderRequest, SubmitOrderRequest};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventContext {
    pub pod: String,
    pub timestamp: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RabbitTradingEvent { // todo: add more events
    SubmitOrder {
        context: EventContext,
        request: SubmitOrderRequest,
    },
    EditOrder {
        context: EventContext,
        request: EditOrderRequest,
    },
    CancelOrder {
        context: EventContext,
        request: CancelOrderRequest,
    },
}
