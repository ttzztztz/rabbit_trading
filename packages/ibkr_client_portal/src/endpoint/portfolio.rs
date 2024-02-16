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
    /// Returns a list of positions for the given account. The endpoint supports paging, page's default size is 30 positions. /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.
    pub async fn get_portfolio_positions(
        &self,
        request: GetPortfolioPositionsRequest,
    ) -> Result<GetPortfolioPositionsResponse, Error> {
        let path = format!("/portfolio/{}/positions/{}", self.account, request.page);
        let response = self
            .client
            .get(self.get_url(&path))
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Information about the account's portfolio allocation by Asset Class, Industry and Category. /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.
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

    /// Returns a list of all positions matching the conid. For portfolio models the conid could be in more than one model, returning an array with the name of the model it belongs to. /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.
    pub async fn get_portfolio_position_by_account_and_conid(
        &self,
        request: GetPortfolioPositionByAccountAndConIdRequest,
    ) -> Result<GetPortfolioPositionByAccountAndConIdResponse, Error> {
        let path = format!(
            "/portfolio/{}/position/{}",
            request.account_id, request.conid,
        );
        let response = self
            .client
            .get(self.get_url(&path))
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Invalidates the backend cache of the Portfolio
    pub async fn invalidate_portfolio_cache(
        &self,
        request: InvalidatePortfolioCacheRequest,
    ) -> Result<(), Error> {
        let path = format!("/portfolio/{}/positions/invalidate", request.account_id,);
        let response = self
            .client
            .post(self.get_url(&path))
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .send()
            .await?;

        response.error_for_status_ref()?;
        Result::Ok(())
    }

    /// Returns an object of all positions matching the conid for all the selected accounts. For portfolio models the conid could be in more than one model, returning an array with the name of the model it belongs to. /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.
    pub async fn get_portfolio_position_by_conid(
        &self,
        request: GetPortfolioPositionByConIdRequest,
    ) -> Result<GetPortfolioPositionByConIdResponse, Error> {
        let path = format!("/portfolio/positions/{}", request.conid);
        let response = self
            .client
            .get(self.get_url(&path))
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }
}
