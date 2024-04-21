// https://www.interactivebrokers.com/api/doc.html#tag/PortfolioAnalyst

use reqwest_middleware::Error;

use crate::{
    client::IBClientPortal,
    model::portfolio_analyst::{
        GetPortfolioPerformanceRequest, GetPortfolioPerformanceResponse,
        GetPortfolioTransactionsRequest, GetPortfolioTransactionsResponse,
    },
};

impl IBClientPortal {
    /// Returns the performance (MTM) for the given accounts, if more than one account is passed, the result is consolidated.
    pub async fn get_portfolio_analyst_performance(
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
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }

    /// Transaction history for a given number of conids and accounts.
    /// Types of transactions include dividend payments, buy and sell transactions, transfers.
    pub async fn get_portfolio_analyst_transactions(
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
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }
}
