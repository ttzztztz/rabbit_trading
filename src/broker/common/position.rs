use async_trait::async_trait;

use crate::model::{balance::BalanceHashMap, error::Error, position::PositionList};

#[async_trait]
pub trait PositionTrait {
    async fn new() -> Self
    where
        Self: Sized;
    async fn account_balance(&self) -> Result<BalanceHashMap, Error>;
    async fn positions(&self) -> Result<PositionList, Error>;
}
