use anyhow::Error;
use async_trait::async_trait;
use std::time::{Duration, Instant};

use crate::model::{
    common::types::ConfigMap,
    trading::{
        balance::BalanceHashMap,
        position::PositionList,
        transaction::{
            BuyingPower, CancelOrderRequest, CancelOrderResponse, EditOrderRequest,
            EditOrderResponse, EstimateMaxBuyingPowerRequest, OrderDetail, OrderDetailRequest,
            SubmitOrderRequest, SubmitOrderResponse,
        },
    },
};

#[async_trait]
pub trait TransactionTrait: Send + Sync {
    async fn new(config_map: ConfigMap) -> Self
    where
        Self: Sized;

    // <-- Read APIs
    async fn account_balance(&self) -> Result<BalanceHashMap, Error>;
    async fn positions(&self) -> Result<PositionList, Error>;
    async fn estimate_max_buying_power(
        &self,
        request: EstimateMaxBuyingPowerRequest,
    ) -> Result<BuyingPower, Error>;
    async fn order_detail(&self, request: OrderDetailRequest) -> Result<OrderDetail, Error>;

    // <-- Mutate APIs
    async fn submit_order(
        &mut self,
        request: SubmitOrderRequest,
    ) -> Result<SubmitOrderResponse, Error>;
    async fn edit_order(&mut self, request: EditOrderRequest) -> Result<EditOrderResponse, Error>;
    async fn cancel_order(
        &mut self,
        request: CancelOrderRequest,
    ) -> Result<CancelOrderResponse, Error>;
}

#[async_trait]
pub trait TransactionInterceptorTrait: Send + Sync {
    async fn before_account_balance(&self) -> Result<(), Error> {
        Result::Ok(())
    }
    async fn after_account_balance(
        &self,
        _request: (),
        result: Result<BalanceHashMap, Error>,
        _duration: Duration,
    ) -> Result<BalanceHashMap, Error> {
        result
    }

    async fn before_positions(&self) -> Result<(), Error> {
        Result::Ok(())
    }
    async fn after_positions(
        &self,
        _request: (),
        result: Result<PositionList, Error>,
        _duration: Duration,
    ) -> Result<PositionList, Error> {
        result
    }

    async fn before_estimate_max_buying_power(
        &self,
        request: EstimateMaxBuyingPowerRequest,
    ) -> Result<EstimateMaxBuyingPowerRequest, Error> {
        Result::Ok(request)
    }
    async fn after_estimate_max_buying_power(
        &self,
        _request: EstimateMaxBuyingPowerRequest,
        result: Result<BuyingPower, Error>,
        _duration: Duration,
    ) -> Result<BuyingPower, Error> {
        result
    }

    async fn before_order_detail(
        &self,
        request: OrderDetailRequest,
    ) -> Result<OrderDetailRequest, Error> {
        Result::Ok(request)
    }
    async fn after_order_detail(
        &self,
        _request: OrderDetailRequest,
        result: Result<OrderDetail, Error>,
        _duration: Duration,
    ) -> Result<OrderDetail, Error> {
        result
    }

    async fn before_submit_order(
        &self,
        request: SubmitOrderRequest,
    ) -> Result<SubmitOrderRequest, Error> {
        Result::Ok(request)
    }
    async fn after_submit_order(
        &self,
        _request: SubmitOrderRequest,
        result: Result<SubmitOrderResponse, Error>,
        _duration: Duration,
    ) -> Result<SubmitOrderResponse, Error> {
        result
    }

    async fn before_edit_order(
        &self,
        request: EditOrderRequest,
    ) -> Result<EditOrderRequest, Error> {
        Result::Ok(request)
    }
    async fn after_edit_order(
        &self,
        _request: EditOrderRequest,
        result: Result<EditOrderResponse, Error>,
        _duration: Duration,
    ) -> Result<EditOrderResponse, Error> {
        result
    }

    async fn before_cancel_order(
        &self,
        request: CancelOrderRequest,
    ) -> Result<CancelOrderRequest, Error> {
        Result::Ok(request)
    }
    async fn after_cancel_order(
        &self,
        _request: CancelOrderRequest,
        result: Result<CancelOrderResponse, Error>,
        _duration: Duration,
    ) -> Result<CancelOrderResponse, Error> {
        result
    }
}

pub struct TransactionProxy {
    pub shadowed_transaction: Box<dyn TransactionTrait>,
    pub interceptor: Box<dyn TransactionInterceptorTrait>,
}

impl TransactionProxy {
    pub fn new(
        shadowed_transaction: Box<dyn TransactionTrait>,
        interceptor_option: Option<Box<dyn TransactionInterceptorTrait>>,
    ) -> Self {
        TransactionProxy {
            shadowed_transaction,
            interceptor: match interceptor_option {
                Some(interceptor) => interceptor,
                None => Box::new(NoOpTransactionInterceptor {}),
            },
        }
    }
}

#[async_trait]
impl TransactionTrait for TransactionProxy {
    async fn new(_config_map: ConfigMap) -> Self {
        panic!("Cannot Call \"new\" on the proxy method!");
    }

    async fn account_balance(&self) -> Result<BalanceHashMap, Error> {
        if let Err(err) = self.interceptor.before_account_balance().await {
            return Err(err);
        }
        let instant = Instant::now();
        let result = self.shadowed_transaction.account_balance().await;
        let duration = instant.elapsed();
        self.interceptor
            .after_account_balance((), result, duration)
            .await
    }

    async fn positions(&self) -> Result<PositionList, Error> {
        if let Err(err) = self.interceptor.before_positions().await {
            return Err(err);
        }
        let instant = Instant::now();
        let result = self.shadowed_transaction.positions().await;
        let duration = instant.elapsed();
        self.interceptor.after_positions((), result, duration).await
    }

    async fn estimate_max_buying_power(
        &self,
        request: EstimateMaxBuyingPowerRequest,
    ) -> Result<BuyingPower, Error> {
        match self
            .interceptor
            .before_estimate_max_buying_power(request)
            .await
        {
            Ok(request) => {
                let instant = Instant::now();
                let result = self
                    .shadowed_transaction
                    .estimate_max_buying_power(request.clone())
                    .await;
                let duration = instant.elapsed();
                self.interceptor
                    .after_estimate_max_buying_power(request, result, duration)
                    .await
            }
            Err(err) => Result::Err(err),
        }
    }

    async fn order_detail(&self, request: OrderDetailRequest) -> Result<OrderDetail, Error> {
        match self.interceptor.before_order_detail(request).await {
            Ok(request) => {
                let instant = Instant::now();
                let result = self
                    .shadowed_transaction
                    .order_detail(request.clone())
                    .await;
                let duration = instant.elapsed();
                self.interceptor
                    .after_order_detail(request, result, duration)
                    .await
            }
            Err(err) => Result::Err(err),
        }
    }

    async fn submit_order(
        &mut self,
        request: SubmitOrderRequest,
    ) -> Result<SubmitOrderResponse, Error> {
        match self.interceptor.before_submit_order(request).await {
            Ok(request) => {
                let instant = Instant::now();
                let result = self
                    .shadowed_transaction
                    .submit_order(request.clone())
                    .await;
                let duration = instant.elapsed();
                self.interceptor
                    .after_submit_order(request, result, duration)
                    .await
            }
            Err(err) => Result::Err(err),
        }
    }

    async fn edit_order(&mut self, request: EditOrderRequest) -> Result<EditOrderResponse, Error> {
        match self.interceptor.before_edit_order(request).await {
            Ok(request) => {
                let instant = Instant::now();
                let result = self.shadowed_transaction.edit_order(request.clone()).await;
                let duration = instant.elapsed();
                self.interceptor
                    .after_edit_order(request, result, duration)
                    .await
            }
            Err(err) => Result::Err(err),
        }
    }

    async fn cancel_order(
        &mut self,
        request: CancelOrderRequest,
    ) -> Result<CancelOrderResponse, Error> {
        match self.interceptor.before_cancel_order(request).await {
            Ok(request) => {
                let instant = Instant::now();
                let result = self
                    .shadowed_transaction
                    .cancel_order(request.clone())
                    .await;
                let duration = instant.elapsed();
                self.interceptor
                    .after_cancel_order(request, result, duration)
                    .await
            }
            Err(err) => Result::Err(err),
        }
    }
}

pub struct NoOpTransactionInterceptor {}

impl TransactionInterceptorTrait for NoOpTransactionInterceptor {}
