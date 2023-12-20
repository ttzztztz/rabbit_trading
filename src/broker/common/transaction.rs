use async_trait::async_trait;

use crate::model::{
    error::Error,
    transaction::{SubmitOrderRequest, SubmitOrderResponse},
};

#[async_trait]
pub trait TransactionTrait {
    async fn new() -> Self
    where
        Self: Sized;
    async fn submit_order(&self, request: SubmitOrderRequest)
        -> Result<SubmitOrderResponse, Error>;
}
