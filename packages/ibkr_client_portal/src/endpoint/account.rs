// https://www.interactivebrokers.com/api/doc.html#tag/Account

use reqwest::Error;

use crate::{
    client::IBClientPortal,
    model::account::{
        GetAccountLedgerResponse, GetAccountsResponse, SwitchAccountRequest, SwitchAccountResponse,
    },
};

impl IBClientPortal {
    pub async fn account_ledger(&self) -> Result<GetAccountLedgerResponse, Error> {
        let path = format!("/portfolio/{}/ledger", self.account);
        let response = self.client.get(self.get_url(&path)).body("").send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn brokerage_accounts(&self) -> Result<GetAccountsResponse, Error> {
        let path = "/iserver/accounts";
        let response = self.client.get(self.get_url(&path)).body("").send().await?;

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

    // todo
    // /portfolio/accounts
    // /portfolio/subaccounts
    // /portfolio/subaccounts2
    // /portfolio/{accountId}/meta
    // /portfolio/{accountId}/summary
    // /portfolio/{accountId}/allocation
    // /iserver/account/pnl/partitioned
    // /iserver/account/trades
}
