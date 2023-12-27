use async_trait::async_trait;

use crate::model::{
    error::Error,
    quote::{QueryInfoRequest, QuoteBasicInfo, QuoteDepthInfo, QuoteRealTimeInfo},
};

#[async_trait]
pub trait InfoTrait {
    async fn new() -> Self
    where
        Self: Sized;

    async fn query_basic_info(&self, request: QueryInfoRequest) -> Result<QuoteBasicInfo, Error>;
    async fn query_real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<QuoteRealTimeInfo, Error>;
    async fn query_depth(&self, request: QueryInfoRequest) -> Result<QuoteDepthInfo, Error>;
}

#[async_trait]
pub trait InfoInterceptorTrait {
    async fn before_query_basic_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<QueryInfoRequest, Error> {
        Result::Ok(request)
    }

    async fn after_query_basic_info(
        &self,
        result: Result<QuoteBasicInfo, Error>,
    ) -> Result<QuoteBasicInfo, Error> {
        result
    }

    async fn before_query_real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<QueryInfoRequest, Error> {
        Result::Ok(request)
    }

    async fn after_query_real_time_info(
        &self,
        result: Result<QuoteRealTimeInfo, Error>,
    ) -> Result<QuoteRealTimeInfo, Error> {
        result
    }

    async fn before_query_depth(
        &self,
        request: QueryInfoRequest,
    ) -> Result<QueryInfoRequest, Error> {
        Result::Ok(request)
    }

    async fn after_query_depth(
        &self,
        result: Result<QuoteDepthInfo, Error>,
    ) -> Result<QuoteDepthInfo, Error> {
        result
    }
}

pub struct InfoReflection {
    pub shadowed_transaction: Box<dyn InfoTrait + Send + Sync>,
    pub interceptor: Box<dyn InfoInterceptorTrait + Send + Sync>,
}

#[async_trait]
impl InfoTrait for InfoReflection {
    async fn new() -> Self {
        panic!("Cannot Call \"new\" on the reflection method!");
    }

    async fn query_basic_info(&self, request: QueryInfoRequest) -> Result<QuoteBasicInfo, Error> {
        match self.interceptor.before_query_basic_info(request).await {
            Ok(request) => {
                let result = self.shadowed_transaction.query_basic_info(request).await;
                self.interceptor.after_query_basic_info(result).await
            }
            Err(err) => Result::Err(err),
        }
    }

    async fn query_real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<QuoteRealTimeInfo, Error> {
        match self.interceptor.before_query_real_time_info(request).await {
            Ok(request) => {
                let result = self
                    .shadowed_transaction
                    .query_real_time_info(request)
                    .await;
                self.interceptor.after_query_real_time_info(result).await
            }
            Err(err) => Result::Err(err),
        }
    }

    async fn query_depth(&self, request: QueryInfoRequest) -> Result<QuoteDepthInfo, Error> {
        match self.interceptor.before_query_depth(request).await {
            Ok(request) => {
                let result = self.shadowed_transaction.query_depth(request).await;
                self.interceptor.after_query_depth(result).await
            }
            Err(err) => Result::Err(err),
        }
    }
}

pub struct NoOpInfoInterceptor {}

impl InfoInterceptorTrait for NoOpInfoInterceptor {}
