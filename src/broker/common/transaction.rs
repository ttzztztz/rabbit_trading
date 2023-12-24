use async_trait::async_trait;

use crate::model::{
    balance::BalanceHashMap,
    error::Error,
    position::PositionList,
    transaction::{
        BuyingPower, CancelOrderRequest, CancelOrderResponse, EditOrderRequest, EditOrderResponse,
        EstimateMaxBuyingPowerRequest, SubmitOrderRequest, SubmitOrderResponse,
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

    // <-- Mutate APIs
    async fn submit_order(&self, request: SubmitOrderRequest)
        -> Result<SubmitOrderResponse, Error>;
    async fn edit_order(&self, request: EditOrderRequest) -> Result<EditOrderResponse, Error>;
    async fn cancel_order(&self, request: CancelOrderRequest)
        -> Result<CancelOrderResponse, Error>;
}

#[async_trait]
pub trait TransactionInteceptorTrait {
    async fn before_account_balance(&self) -> Result<(), Error>;
    async fn after_account_balance(
        &self,
        result: Result<BalanceHashMap, Error>,
    ) -> Result<BalanceHashMap, Error>;

    async fn before_positions(&self) -> Result<(), Error>;
    async fn after_positions(
        &self,
        result: Result<PositionList, Error>,
    ) -> Result<PositionList, Error>;

    async fn befoer_estimate_max_buying_power(
        &self,
        request: EstimateMaxBuyingPowerRequest,
    ) -> Result<EstimateMaxBuyingPowerRequest, Error>;
    async fn after_estimate_max_buying_power(
        &self,
        result: Result<BuyingPower, Error>,
    ) -> Result<BuyingPower, Error>;

    async fn before_submit_order(
        &self,
        request: SubmitOrderRequest,
    ) -> Result<SubmitOrderRequest, Error>;
    async fn after_submit_order(
        &self,
        result: Result<SubmitOrderResponse, Error>,
    ) -> Result<SubmitOrderResponse, Error>;

    async fn before_edit_order(&self, request: EditOrderRequest)
        -> Result<EditOrderRequest, Error>;
    async fn after_edit_order(
        &self,
        result: Result<EditOrderResponse, Error>,
    ) -> Result<EditOrderResponse, Error>;

    async fn before_cancel_order(
        &self,
        request: CancelOrderRequest,
    ) -> Result<CancelOrderRequest, Error>;
    async fn after_cancel_order(
        &self,
        result: Result<CancelOrderResponse, Error>,
    ) -> Result<CancelOrderResponse, Error>;
}

pub struct TransactionReflection {
    pub shadowed_transaction: Box<dyn TransactionTrait + Send + Sync>,
    pub inteceptor: Box<dyn TransactionInteceptorTrait + Send + Sync>,
}

impl TransactionReflection {
    pub fn new(
        shadowed_transaction: Box<dyn TransactionTrait + Send + Sync>,
        inteceptor: Option<Box<dyn TransactionInteceptorTrait + Send + Sync>>,
    ) -> Self {
        TransactionReflection {
            shadowed_transaction,
            inteceptor: inteceptor.unwrap(), // todo: add an empty inteceptor
        }
    }
}

#[async_trait]
impl TransactionTrait for TransactionReflection {
    async fn new() -> Self {
        panic!("Cannot Call \"new\" on the reflection method!");
    }

    async fn account_balance(&self) -> Result<BalanceHashMap, Error> {
        if let Err(err) = self.inteceptor.before_account_balance().await {
            return Err(err);
        }
        let result = self.shadowed_transaction.account_balance().await;
        self.inteceptor.after_account_balance(result).await
    }

    async fn positions(&self) -> Result<PositionList, Error> {
        if let Err(err) = self.inteceptor.before_positions().await {
            return Err(err);
        }
        let result = self.shadowed_transaction.positions().await;
        self.inteceptor.after_positions(result).await
    }

    async fn estimate_max_buying_power(
        &self,
        request: EstimateMaxBuyingPowerRequest,
    ) -> Result<BuyingPower, Error> {
        match self
            .inteceptor
            .befoer_estimate_max_buying_power(request)
            .await
        {
            Ok(request) => {
                let result = self
                    .shadowed_transaction
                    .estimate_max_buying_power(request)
                    .await;

                self.inteceptor
                    .after_estimate_max_buying_power(result)
                    .await
            }
            Err(err) => Result::Err(err),
        }
    }

    async fn submit_order(
        &self,
        request: SubmitOrderRequest,
    ) -> Result<SubmitOrderResponse, Error> {
        match self.inteceptor.before_submit_order(request).await {
            Ok(request) => {
                let result = self.shadowed_transaction.submit_order(request).await;
                self.inteceptor.after_submit_order(result).await
            }
            Err(err) => Result::Err(err),
        }
    }

    async fn edit_order(&self, request: EditOrderRequest) -> Result<EditOrderResponse, Error> {
        match self.inteceptor.before_edit_order(request).await {
            Ok(request) => {
                let result = self.shadowed_transaction.edit_order(request).await;
                self.inteceptor.after_edit_order(result).await
            }
            Err(err) => Result::Err(err),
        }
    }

    async fn cancel_order(
        &self,
        request: CancelOrderRequest,
    ) -> Result<CancelOrderResponse, Error> {
        match self.inteceptor.before_cancel_order(request).await {
            Ok(request) => {
                let result = self.shadowed_transaction.cancel_order(request).await;
                self.inteceptor.after_cancel_order(result).await
            }
            Err(err) => Result::Err(err),
        }
    }
}

pub struct EmptyTransactionInteceptor {}

#[async_trait]
impl TransactionInteceptorTrait for EmptyTransactionInteceptor {
    async fn before_account_balance(&self) -> Result<(), Error> {
        Result::Ok(())
    }
    async fn after_account_balance(
        &self,
        result: Result<BalanceHashMap, Error>,
    ) -> Result<BalanceHashMap, Error> {
        result
    }

    async fn before_positions(&self) -> Result<(), Error> {
        Result::Ok(())
    }
    async fn after_positions(
        &self,
        result: Result<PositionList, Error>,
    ) -> Result<PositionList, Error> {
        result
    }

    async fn befoer_estimate_max_buying_power(
        &self,
        request: EstimateMaxBuyingPowerRequest,
    ) -> Result<EstimateMaxBuyingPowerRequest, Error> {
        Result::Ok(request)
    }
    async fn after_estimate_max_buying_power(
        &self,
        result: Result<BuyingPower, Error>,
    ) -> Result<BuyingPower, Error> {
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
        result: Result<CancelOrderResponse, Error>,
    ) -> Result<CancelOrderResponse, Error> {
        result
    }
}
