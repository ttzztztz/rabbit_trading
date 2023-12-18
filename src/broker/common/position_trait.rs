use async_trait::async_trait;

use crate::model::{balance::BalanceHashMap, error::Error};

#[async_trait]
pub trait Position {
    async fn new() -> Self
    where
        Self: Sized;
    async fn account_balance(&self) -> Result<BalanceHashMap, Error>;
}
