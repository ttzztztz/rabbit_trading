use async_trait::async_trait;

use crate::model::{
    balance::BalanceHashMap,
    error::Error,
    position::PositionList,
    transaction::{
        BuyingPower, EstimateMaxBuyingPowerRequest, SubmitOrderRequest, SubmitOrderResponse,
    },
};

#[async_trait]
pub trait TransactionTrait {
    async fn new() -> Self
    where
        Self: Sized;
    async fn account_balance(&self) -> Result<BalanceHashMap, Error>;
    async fn positions(&self) -> Result<PositionList, Error>;
    async fn submit_order(&self, request: SubmitOrderRequest)
        -> Result<SubmitOrderResponse, Error>;
    async fn estimate_max_buying_power(
        &self,
        request: EstimateMaxBuyingPowerRequest,
    ) -> Result<BuyingPower, Error>;
}
