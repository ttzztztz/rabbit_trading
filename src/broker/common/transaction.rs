use async_trait::async_trait;

use crate::model::{
    common::error::Error,
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
pub trait TransactionTrait {
    async fn new() -> Self
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
    async fn submit_order(&self, request: SubmitOrderRequest)
        -> Result<SubmitOrderResponse, Error>;
    async fn edit_order(&self, request: EditOrderRequest) -> Result<EditOrderResponse, Error>;
    async fn cancel_order(&self, request: CancelOrderRequest)
        -> Result<CancelOrderResponse, Error>;
}

#[async_trait]
pub trait TransactionInterceptorTrait {
    async fn before_account_balance(&self) -> Result<(), Error> {
        Result::Ok(())
    }
    async fn after_account_balance(
        &self,
        request: (),
        result: Result<BalanceHashMap, Error>,
    ) -> Result<BalanceHashMap, Error> {
        result
    }

    async fn before_positions(&self) -> Result<(), Error> {
        Result::Ok(())
    }
    async fn after_positions(
        &self,
        request: (),
        result: Result<PositionList, Error>,
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
        request: EstimateMaxBuyingPowerRequest,
        result: Result<BuyingPower, Error>,
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
        request: OrderDetailRequest,
        result: Result<OrderDetail, Error>,
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
        request: SubmitOrderRequest,
        result: Result<SubmitOrderResponse, Error>,
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
        request: EditOrderRequest,
        result: Result<EditOrderResponse, Error>,
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
        request: CancelOrderRequest,
        result: Result<CancelOrderResponse, Error>,
    ) -> Result<CancelOrderResponse, Error> {
        result
    }
}

pub struct TransactionProxy {
    pub shadowed_transaction: Box<dyn TransactionTrait + Send + Sync>,
    pub interceptor: Box<dyn TransactionInterceptorTrait + Send + Sync>,
}

impl TransactionProxy {
    pub fn new(
        shadowed_transaction: Box<dyn TransactionTrait + Send + Sync>,
        interceptor_option: Option<Box<dyn TransactionInterceptorTrait + Send + Sync>>,
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
    async fn new() -> Self {
        panic!("Cannot Call \"new\" on the proxy method!");
    }

    async fn account_balance(&self) -> Result<BalanceHashMap, Error> {
        if let Err(err) = self.interceptor.before_account_balance().await {
            return Err(err);
        }
        let result = self.shadowed_transaction.account_balance().await;
        self.interceptor.after_account_balance((), result).await
    }

    async fn positions(&self) -> Result<PositionList, Error> {
        if let Err(err) = self.interceptor.before_positions().await {
            return Err(err);
        }
        let result = self.shadowed_transaction.positions().await;
        self.interceptor.after_positions((), result).await
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
                let result = self
                    .shadowed_transaction
                    .estimate_max_buying_power(request.clone())
                    .await;

                self.interceptor
                    .after_estimate_max_buying_power(request, result)
                    .await
            }
            Err(err) => Result::Err(err),
        }
    }

    async fn order_detail(&self, request: OrderDetailRequest) -> Result<OrderDetail, Error> {
        match self.interceptor.before_order_detail(request).await {
            Ok(request) => {
                let result = self
                    .shadowed_transaction
                    .order_detail(request.clone())
                    .await;
                self.interceptor.after_order_detail(request, result).await
            }
            Err(err) => Result::Err(err),
        }
    }

    async fn submit_order(
        &self,
        request: SubmitOrderRequest,
    ) -> Result<SubmitOrderResponse, Error> {
        match self.interceptor.before_submit_order(request).await {
            Ok(request) => {
                let result = self
                    .shadowed_transaction
                    .submit_order(request.clone())
                    .await;
                self.interceptor.after_submit_order(request, result).await
            }
            Err(err) => Result::Err(err),
        }
    }

    async fn edit_order(&self, request: EditOrderRequest) -> Result<EditOrderResponse, Error> {
        match self.interceptor.before_edit_order(request).await {
            Ok(request) => {
                let result = self.shadowed_transaction.edit_order(request.clone()).await;
                self.interceptor.after_edit_order(request, result).await
            }
            Err(err) => Result::Err(err),
        }
    }

    async fn cancel_order(
        &self,
        request: CancelOrderRequest,
    ) -> Result<CancelOrderResponse, Error> {
        match self.interceptor.before_cancel_order(request).await {
            Ok(request) => {
                let result = self
                    .shadowed_transaction
                    .cancel_order(request.clone())
                    .await;
                self.interceptor.after_cancel_order(request, result).await
            }
            Err(err) => Result::Err(err),
        }
    }
}

pub struct NoOpTransactionInterceptor {}

impl TransactionInterceptorTrait for NoOpTransactionInterceptor {}
