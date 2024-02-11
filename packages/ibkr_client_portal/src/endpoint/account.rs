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
    /// Information regarding settled cash, cash balances, etc. in the account's base currency and any other cash balances hold in other currencies. /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint. The list of supported currencies is available at https://www.interactivebrokers.com/en/index.php?f=3185.
    pub async fn get_account_ledger(&self) -> Result<GetAccountLedgerResponse, Error> {
        let path = format!("/portfolio/{}/ledger", self.account);
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Returns a list of accounts the user has trading access to, their respective aliases and the currently selected account. Note this endpoint must be called before modifying an order or querying open orders.
    pub async fn get_brokerage_accounts(&self) -> Result<GetAccountsResponse, Error> {
        let path = "/iserver/accounts";
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// In non-tiered account structures, returns a list of accounts for which the user can view position and account information. This endpoint must be called prior to calling other /portfolio endpoints for those accounts. For querying a list of accounts which the user can trade, see /iserver/accounts. For a list of subaccounts in tiered account structures (e.g. financial advisor or ibroker accounts) see /portfolio/subaccounts.
    pub async fn get_portfolio_accounts(&self) -> Result<GetPortfolioAccountsResponse, Error> {
        let path = "/portfolio/accounts";
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Used in tiered account structures (such as Financial Advisor and IBroker Accounts) to return a list of up to 100 sub-accounts for which the user can view position and account-related information. This endpoint must be called prior to calling other /portfolio endpoints for those sub-accounts. If you have more than 100 sub-accounts use /portfolio/subaccounts2. To query a list of accounts the user can trade, see /iserver/accounts.
    pub async fn get_sub_accounts(&self) -> Result<Account, Error> {
        let path = "/portfolio/subaccounts";
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Used in tiered account structures (such as Financial Advisor and IBroker Accounts) to return a list of sub-accounts, paginated up to 20 accounts per page, for which the user can view position and account-related information. This endpoint must be called prior to calling other /portfolio endpoints for those sub-accounts. If you have less than 100 sub-accounts use /portfolio/subaccounts. To query a list of accounts the user can trade, see /iserver/accounts.
    pub async fn get_sub_accounts_v2(
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
    pub async fn get_account_metadata(
        &self,
        request: GetAccountMetadataRequest,
    ) -> Result<GetAccountMetadataResponse, Error> {
        let path = format!("/portfolio/{}/meta", request.account_id);
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Account information related to account Id /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.
    pub async fn get_account_summary(
        &self,
        request: GetAccountSummaryRequest,
    ) -> Result<GetAccountSummaryResponse, Error> {
        let path = format!("/portfolio/{}/summary", request.account_id);
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Information about the account's portfolio allocation by Asset Class, Industry and Category. /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.
    pub async fn get_account_allocations(
        &self,
        request: GetAccountAllocationRequest,
    ) -> Result<Allocation, Error> {
        let path = format!("/portfolio/{}/allocation", request.account_id);
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// If an user has multiple accounts, and user wants to get orders, trades, etc. of an account other than currently selected account, then user can update the currently selected account using this API and then can fetch required information for the newly updated account.
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

    /// Returns an object containing PnL for the selected account and its models (if any).
    /// To receive streaming PnL the endpoint /ws can be used. Refer to Streaming WebSocket Data for details.
    pub async fn get_account_pnl_partitioned(
        &self,
    ) -> Result<GetAccountPnLPartitionedResponse, Error> {
        let path = "/iserver/account/pnl/partitioned";
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Returns a list of trades for the currently selected account for current day and six previous days. It is advised to call this endpoint once per session.
    pub async fn get_account_trades(&self) -> Result<GetAccountTradesResponse, Error> {
        let path = "/iserver/account/trades";
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }
}
