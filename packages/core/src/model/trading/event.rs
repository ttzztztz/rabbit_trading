use serde::{Deserialize, Serialize};

use crate::model::{
    common::error::Error,
    trading::transaction::{
        CancelOrderRequest, CancelOrderResponse, EditOrderRequest, EditOrderResponse,
        SubmitOrderRequest, SubmitOrderResponse,
    },
};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EventContext {
    pub broker_id: String,
    pub pod_id: String,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
