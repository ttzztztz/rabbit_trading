use async_trait::async_trait;

use crate::model::{
    common::error::Error,
    trading::quote::{QueryInfoRequest, QuoteBasicInfo, QuoteDepthInfo, QuoteRealTimeInfo},
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
        request: QueryInfoRequest,
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
        request: QueryInfoRequest,
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
        request: QueryInfoRequest,
        result: Result<QuoteDepthInfo, Error>,
    ) -> Result<QuoteDepthInfo, Error> {
        result
    }
}

pub struct InfoProxy {
    pub shadowed_info: Box<dyn InfoTrait + Send + Sync>,
    pub interceptor: Box<dyn InfoInterceptorTrait + Send + Sync>,
}

impl InfoProxy {
    pub fn new(
        shadowed_info: Box<dyn InfoTrait + Send + Sync>,
        interceptor_option: Option<Box<dyn InfoInterceptorTrait + Send + Sync>>,
    ) -> Self {
        InfoProxy {
            shadowed_info,
            interceptor: match interceptor_option {
                Some(interceptor) => interceptor,
                None => Box::new(NoOpInfoInterceptor {}),
            },
        }
    }
}

#[async_trait]
impl InfoTrait for InfoProxy {
    async fn new() -> Self {
        panic!("Cannot Call \"new\" on the proxy method!");
    }

    async fn query_basic_info(&self, request: QueryInfoRequest) -> Result<QuoteBasicInfo, Error> {
        match self.interceptor.before_query_basic_info(request).await {
            Ok(request) => {
                let result = self.shadowed_info.query_basic_info(request.clone()).await;
                self.interceptor
                    .after_query_basic_info(request, result)
                    .await
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
                    .shadowed_info
                    .query_real_time_info(request.clone())
                    .await;
                self.interceptor
                    .after_query_real_time_info(request, result)
                    .await
            }
            Err(err) => Result::Err(err),
        }
    }

    async fn query_depth(&self, request: QueryInfoRequest) -> Result<QuoteDepthInfo, Error> {
        match self.interceptor.before_query_depth(request).await {
            Ok(request) => {
                let result = self.shadowed_info.query_depth(request.clone()).await;
                self.interceptor.after_query_depth(request, result).await
            }
            Err(err) => Result::Err(err),
        }
    }
}

pub struct NoOpInfoInterceptor {}

impl InfoInterceptorTrait for NoOpInfoInterceptor {}
