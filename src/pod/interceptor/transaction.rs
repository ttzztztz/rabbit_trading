use async_trait::async_trait;

use crate::{
    broker::common::transaction::TransactionInterceptorTrait,
    model::{
        error::Error,
        event::RabbitTradingEvent,
        transaction::{
            CancelOrderRequest, CancelOrderResponse, EditOrderRequest, EditOrderResponse,
            SubmitOrderRequest, SubmitOrderResponse,
        },
    },
    pod::event_bus::EventBus,
};

pub struct PodTransactionInterceptor {
    event_bus: EventBus,
}

impl PodTransactionInterceptor {
    pub fn new(event_bus: EventBus) -> Self {
        PodTransactionInterceptor { event_bus }
    }
}

#[async_trait]
impl TransactionInterceptorTrait for PodTransactionInterceptor {
    async fn after_submit_order(
        &self,
        request: SubmitOrderRequest,
        result: Result<SubmitOrderResponse, Error>,
    ) -> Result<SubmitOrderResponse, Error> {
        if let Some(err) = self
            .event_bus
            .send(RabbitTradingEvent::SubmitOrder {
                context: self.event_bus.create_event_context(),
                request,
                result: result.clone(),
            })
            .await
            .err()
        {
            log::error!("Error when sending message into event_bus, {}", err);
        }

        result
    }

    async fn after_edit_order(
        &self,
        request: EditOrderRequest,
        result: Result<EditOrderResponse, Error>,
    ) -> Result<EditOrderResponse, Error> {
        if let Some(err) = self
            .event_bus
            .send(RabbitTradingEvent::EditOrder {
                context: self.event_bus.create_event_context(),
                request,
                result: result.clone(),
            })
            .await
            .err()
        {
            log::error!("Error when sending message into event_bus, {}", err);
        }

        result
    }

    async fn after_cancel_order(
        &self,
        request: CancelOrderRequest,
        result: Result<CancelOrderResponse, Error>,
    ) -> Result<CancelOrderResponse, Error> {
        if let Some(err) = self
            .event_bus
            .send(RabbitTradingEvent::CancelOrder {
                context: self.event_bus.create_event_context(),
                request,
                result: result.clone(),
            })
            .await
            .err()
        {
            log::error!("Error when sending message into event_bus, {}", err);
        }

        result
    }
}
