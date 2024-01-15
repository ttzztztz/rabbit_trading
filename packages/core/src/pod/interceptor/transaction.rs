use async_trait::async_trait;
use std::{collections::HashMap, time::Duration};

use crate::{
    broker::common::transaction::TransactionInterceptorTrait,
    metrics::common::registry::MetricRegistryTrait,
    model::{
        common::error::Error,
        trading::{
            balance::BalanceHashMap,
            event::RabbitTradingEvent,
            position::PositionList,
            transaction::{
                BuyingPower, CancelOrderRequest, CancelOrderResponse, EditOrderRequest,
                EditOrderResponse, EstimateMaxBuyingPowerRequest, OrderDetail, OrderDetailRequest,
                SubmitOrderRequest, SubmitOrderResponse,
            },
        },
    },
    pod::event::event_bus::EventBus,
};

pub struct PodTransactionInterceptor {
    event_bus: EventBus,
    metric_registry: Box<dyn MetricRegistryTrait>,
}

impl PodTransactionInterceptor {
    pub fn new(event_bus: EventBus, metric_registry: Box<dyn MetricRegistryTrait>) -> Self {
        PodTransactionInterceptor {
            event_bus,
            metric_registry,
        }
    }
}

#[async_trait]
impl TransactionInterceptorTrait for PodTransactionInterceptor {
    async fn after_account_balance(
        &self,
        _request: (),
        result: Result<BalanceHashMap, Error>,
        duration: Duration,
    ) -> Result<BalanceHashMap, Error> {
        self.metric_registry
            .timer(
                "system.pod.counter".to_owned(),
                HashMap::from([
                    ("component".to_owned(), "transaction".to_owned()),
                    ("method".to_owned(), "account_balance".to_owned()),
                    ("is_success".to_owned(), result.is_ok().to_string()),
                ]),
                duration,
            )
            .await;

        result
    }

    async fn after_positions(
        &self,
        _request: (),
        result: Result<PositionList, Error>,
        duration: Duration,
    ) -> Result<PositionList, Error> {
        self.metric_registry
            .timer(
                "system.pod.counter".to_owned(),
                HashMap::from([
                    ("component".to_owned(), "transaction".to_owned()),
                    ("method".to_owned(), "position".to_owned()),
                    ("is_success".to_owned(), result.is_ok().to_string()),
                ]),
                duration,
            )
            .await;

        result
    }

    async fn after_estimate_max_buying_power(
        &self,
        _request: EstimateMaxBuyingPowerRequest,
        result: Result<BuyingPower, Error>,
        duration: Duration,
    ) -> Result<BuyingPower, Error> {
        self.metric_registry
            .timer(
                "system.pod.counter".to_owned(),
                HashMap::from([
                    ("component".to_owned(), "transaction".to_owned()),
                    ("method".to_owned(), "estimate_max_buying_power".to_owned()),
                    ("is_success".to_owned(), result.is_ok().to_string()),
                ]),
                duration,
            )
            .await;

        result
    }

    async fn after_order_detail(
        &self,
        _request: OrderDetailRequest,
        result: Result<OrderDetail, Error>,
        duration: Duration,
    ) -> Result<OrderDetail, Error> {
        self.metric_registry
            .timer(
                "system.pod.counter".to_owned(),
                HashMap::from([
                    ("component".to_owned(), "transaction".to_owned()),
                    ("method".to_owned(), "order_detail".to_owned()),
                    ("is_success".to_owned(), result.is_ok().to_string()),
                ]),
                duration,
            )
            .await;

        result
    }

    async fn after_submit_order(
        &self,
        request: SubmitOrderRequest,
        result: Result<SubmitOrderResponse, Error>,
        duration: Duration,
    ) -> Result<SubmitOrderResponse, Error> {
        self.metric_registry
            .timer(
                "system.pod.counter".to_owned(),
                HashMap::from([
                    ("component".to_owned(), "transaction".to_owned()),
                    ("method".to_owned(), "submit_order".to_owned()),
                    ("is_success".to_owned(), result.is_ok().to_string()),
                ]),
                duration,
            )
            .await;

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
        duration: Duration,
    ) -> Result<EditOrderResponse, Error> {
        self.metric_registry
            .timer(
                "system.pod.counter".to_owned(),
                HashMap::from([
                    ("component".to_owned(), "transaction".to_owned()),
                    ("method".to_owned(), "edit_order".to_owned()),
                    ("is_success".to_owned(), result.is_ok().to_string()),
                ]),
                duration,
            )
            .await;

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
        duration: Duration,
    ) -> Result<CancelOrderResponse, Error> {
        self.metric_registry
            .timer(
                "system.pod.counter".to_owned(),
                HashMap::from([
                    ("component".to_owned(), "transaction".to_owned()),
                    ("method".to_owned(), "cancel_order".to_owned()),
                    ("is_success".to_owned(), result.is_ok().to_string()),
                ]),
                duration,
            )
            .await;

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
