// https://www.interactivebrokers.com/api/doc.html#tag/Portfolio

use reqwest::Error;

use crate::{
    client::IBClientPortal,
    model::portfolio::{
        GetPortfolioAllocationRequest, GetPortfolioAllocationResponse,
        GetPortfolioPositionByAccountAndConIdRequest,
        GetPortfolioPositionByAccountAndConIdResponse, GetPortfolioPositionByConIdRequest,
        GetPortfolioPositionByConIdResponse, GetPortfolioPositionsRequest,
        GetPortfolioPositionsResponse, InvalidatePortfolioCacheRequest,
    },
};

impl IBClientPortal {
    pub async fn get_portfolio_positions(
        &self,
        request: GetPortfolioPositionsRequest,
    ) -> Result<GetPortfolioPositionsResponse, Error> {
        let path = format!("/portfolio/{}/positions/{}", self.account, request.page);
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn get_portfolio_allocation(
        &self,
        request: GetPortfolioAllocationRequest,
    ) -> Result<GetPortfolioAllocationResponse, Error> {
        let path = "/portfolio/allocation";
        let response = self
            .client
            .post(self.get_url(&path))
            .json(&request)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn get_portfolio_position_by_account_and_conid(
        &self,
        request: GetPortfolioPositionByAccountAndConIdRequest,
    ) -> Result<GetPortfolioPositionByAccountAndConIdResponse, Error> {
        let path = format!(
            "/portfolio/{}/position/{}",
            request.account_id, request.conid,
        );
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn invalidate_portfolio_cache(
        &self,
        request: InvalidatePortfolioCacheRequest,
    ) -> Result<(), Error> {
        let path = format!("/portfolio/{}/positions/invalidate", request.account_id,);
        let response = self.client.post(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        Result::Ok(())
    }

    pub async fn get_portfolio_position_by_conid(
        &self,
        request: GetPortfolioPositionByConIdRequest,
    ) -> Result<GetPortfolioPositionByConIdResponse, Error> {
        let path = format!("/portfolio/positions/{}", request.conid);
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }
}
