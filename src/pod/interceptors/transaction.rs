use async_trait::async_trait;

use crate::broker::common::transaction::TransactionInterceptorTrait;

pub struct PodTransactionInterceptor {}

#[async_trait]
impl TransactionInterceptorTrait for PodTransactionInterceptor {}
