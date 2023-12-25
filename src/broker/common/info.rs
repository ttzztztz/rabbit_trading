use async_trait::async_trait;
use std::iter::Map;

use crate::model::{error::Error, quote::QuoteInfo, symbol::Symbol};

#[derive(Clone)]
pub struct InfoContext {
    pub symbol: Symbol,
    pub extra: Option<Map<String, String>>,
}

// todo: redesign info + subscription trait
#[async_trait]
pub trait InfoTrait {
    async fn new(context: InfoContext) -> Self
    where
        Self: Sized;
    async fn query_real_time_info(&self) -> Result<QuoteInfo, Error>;
}

#[async_trait]
pub trait InfoInterceptorTrait {
    async fn before_query_real_time_info(&self) -> Result<(), Error>;
    async fn after_query_real_time_info(
        &self,
        result: Result<QuoteInfo, Error>,
    ) -> Result<QuoteInfo, Error>;
}

pub struct InfoReflection {
    pub shadowed_transaction: Box<dyn InfoTrait + Send + Sync>,
    pub interceptor: Box<dyn InfoInterceptorTrait + Send + Sync>,
}

#[async_trait]
impl InfoTrait for InfoReflection {
    async fn new(_context: InfoContext) -> Self {
        panic!("Cannot Call \"new\" on the reflection method!");
    }

    async fn query_real_time_info(&self) -> Result<QuoteInfo, Error> {
        if let Err(err) = self.interceptor.before_query_real_time_info().await {
            return Err(err);
        }
        let result = self.shadowed_transaction.query_real_time_info().await;
        self.interceptor.after_query_real_time_info(result).await
    }
}

pub struct NoOpInfoInterceptor {}

#[async_trait]
impl InfoInterceptorTrait for NoOpInfoInterceptor {
    async fn before_query_real_time_info(&self) -> Result<(), Error> {
        Result::Ok(())
    }

    async fn after_query_real_time_info(
        &self,
        result: Result<QuoteInfo, Error>,
    ) -> Result<QuoteInfo, Error> {
        result
    }
}
