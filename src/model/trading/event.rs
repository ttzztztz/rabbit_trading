use crate::model::{
    common::error::Error,
    trading::transaction::{
        CancelOrderRequest, CancelOrderResponse, EditOrderRequest, EditOrderResponse,
        SubmitOrderRequest, SubmitOrderResponse,
    },
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventContext {
    pub pod_id: String,
    pub timestamp: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RabbitTradingEvent {
    // todo: add more events
    SubmitOrder {
        context: EventContext,
        request: SubmitOrderRequest,
        result: Result<SubmitOrderResponse, Error>,
    },
    EditOrder {
        context: EventContext,
        request: EditOrderRequest,
        result: Result<EditOrderResponse, Error>,
    },
    CancelOrder {
        context: EventContext,
        request: CancelOrderRequest,
        result: Result<CancelOrderResponse, Error>,
    },
}
