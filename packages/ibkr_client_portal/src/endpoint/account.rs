// https://www.interactivebrokers.com/api/doc.html#tag/Account

use reqwest::Error;

use crate::{
    client::IBClientPortal,
    model::account::{
        Account, Allocation, GetAccountAllocationRequest, GetAccountLedgerResponse,
        GetAccountMetadataRequest, GetAccountMetadataResponse, GetAccountPnLPartitionedResponse,
        GetAccountSummaryRequest, GetAccountSummaryResponse, GetAccountTradesResponse,
        GetAccountsResponse, GetPortfolioAccountsResponse, GetSubAccountsV2Request,
        GetSubAccountsV2Response, SwitchAccountRequest, SwitchAccountResponse,
    },
};

impl IBClientPortal {
    pub async fn account_ledger(&self) -> Result<GetAccountLedgerResponse, Error> {
        let path = format!("/portfolio/{}/ledger", self.account);
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn brokerage_accounts(&self) -> Result<GetAccountsResponse, Error> {
        let path = "/iserver/accounts";
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn portfolio_accounts(&self) -> Result<GetPortfolioAccountsResponse, Error> {
        let path = "/portfolio/accounts";
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn sub_accounts(&self) -> Result<Account, Error> {
        let path = "/portfolio/subaccounts";
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn sub_accounts_v2(
        &self,
        request: GetSubAccountsV2Request,
    ) -> Result<GetSubAccountsV2Response, Error> {
        let path = "/portfolio/subaccounts2";
        let response = self
            .client
            .get(self.get_url(&path))
            .query(&[("page", request.page.to_string())])
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Account information related to account Id /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.
    pub async fn account_metadata(
        &self,
        request: GetAccountMetadataRequest,
    ) -> Result<GetAccountMetadataResponse, Error> {
        let path = format!("/portfolio/{}/meta", request.account_id);
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Account information related to account Id /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.
    pub async fn account_summary(
        &self,
        request: GetAccountSummaryRequest,
    ) -> Result<GetAccountSummaryResponse, Error> {
        let path = format!("/portfolio/{}/summary", request.account_id);
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn account_allocations(
        &self,
        request: GetAccountAllocationRequest,
    ) -> Result<Allocation, Error> {
        let path = format!("/portfolio/{}/allocation", request.account_id);
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn switch_account(
        &self,
        request: SwitchAccountRequest,
    ) -> Result<SwitchAccountResponse, Error> {
        let response = self
            .client
            .post(self.get_url("/iserver/account"))
            .json(&request)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn get_account_pnl_partitioned(
        &self,
    ) -> Result<GetAccountPnLPartitionedResponse, Error> {
        let path = "/iserver/account/pnl/partitioned";
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn get_account_trades(&self) -> Result<GetAccountTradesResponse, Error> {
        let path = "/iserver/account/trades";
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }
}
