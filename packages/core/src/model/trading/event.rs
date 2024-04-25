use serde::{Deserialize, Serialize};

use crate::model::trading::transaction::{
    CancelOrderRequest, CancelOrderResponse, EditOrderRequest, EditOrderResponse,
    SubmitOrderRequest, SubmitOrderResponse,
};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EventContext {
    pub broker_id: String,
    pub pod_id: String,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EventError {
    pub message: String,
}

pub fn from_anyhow_result<T>(result: &Result<T, anyhow::Error>) -> Result<T, EventError>
where
    T: Sized + Clone,
{
    result.as_ref().map(|val| val.clone()).map_err(|e| EventError {
        message: e.to_string(),
    })
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum RabbitTradingEvent {
    // todo: add more events
    SubmitOrder {
        context: EventContext,
        request: SubmitOrderRequest,
        result: Result<SubmitOrderResponse, EventError>,
    },
    EditOrder {
        context: EventContext,
        request: EditOrderRequest,
        result: Result<EditOrderResponse, EventError>,
    },
    CancelOrder {
        context: EventContext,
        request: CancelOrderRequest,
        result: Result<CancelOrderResponse, EventError>,
    },
}
