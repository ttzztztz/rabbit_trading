// https://www.interactivebrokers.com/api/doc.html#tag/PortfolioAnalyst

use reqwest::Error;

use crate::{
    client::IBClientPortal,
    model::portfolio_analyst::{
        GetPortfolioPerformanceRequest, GetPortfolioPerformanceResponse,
        GetPortfolioTransactionsRequest, GetPortfolioTransactionsResponse,
    },
};

impl IBClientPortal {
    pub async fn portfolio_analyst_performance(
        &self,
        request: GetPortfolioPerformanceRequest,
    ) -> Result<GetPortfolioPerformanceResponse, Error> {
        let path = "/pa/performance";
        let response = self
            .client
            .post(self.get_url(path))
            .json(&request)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn portfolio_analyst_transactions(
        &self,
        request: GetPortfolioTransactionsRequest,
    ) -> Result<GetPortfolioTransactionsResponse, Error> {
        let path = "/pa/transactions";
        let response = self
            .client
            .post(self.get_url(path))
            .json(&request)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }
}
