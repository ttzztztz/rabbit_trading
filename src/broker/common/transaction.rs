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
